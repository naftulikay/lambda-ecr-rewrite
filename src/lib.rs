pub mod requests;
pub mod responses;
#[cfg(test)]
mod tests;

use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use lambda_runtime::Context;
use std::env;

use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderValue;
use parking_lot::RwLock;
use requests::ApiGatewayRequestType;
use responses::{ApiGatewayGenericResponse, ApiGatewayResponseType};
use std::sync::{LazyLock, OnceLock};
use std::time::{Duration, Instant};

static REGISTRY_URL: OnceLock<Option<String>> = OnceLock::new();

static CACHE_MAX_AGE: OnceLock<usize> = OnceLock::new();

/// If the `DEBUG` environment variable is set to `y | yes | true`, then we enable debug logging.
static IS_DEBUG: LazyLock<bool> = LazyLock::new(|| {
    if let Ok(value) = std::env::var("DEBUG") {
        value.trim().starts_with("y") || value.trim() == "true"
    } else {
        false
    }
});

/// The name of the environment variable containing the ECR registry host FQDN.
pub const ECR_REGISTRY_ENV_VAR: &str = "ECR_REGISTRY_HOST";

/// The name of the environment variable containing the cache max age in seconds to return with
/// responses.
pub const CACHE_MAX_AGE_ENV_VAR: &str = "CACHE_MAX_AGE";

/// The default max age in seconds to specify in the `Cache-Control` header.
pub const CACHE_MAX_AGE_DEFAULT: usize = 60;

/// The minimum amount of time to wait before logging failed requests.
pub const MIN_LOG_INTERVAL: Duration = Duration::from_secs(60);

static LAST_LOG_TIME: RwLock<Option<Instant>> = RwLock::new(None);

static HTML_ERROR_RESPONSE: &str = r#"<!doctype html>
<html lang="en-us">
  <head>
    <title>Error: 500 (Internal Server Error)</title>
  </head>
  <body>
    <h1>Error: 500 (Internal Server Error)</h1>
    <p>Destination host name not set.</p>
  </body>
</html>"#;

static JSON_ERROR_RESPONSE: &str = r#"{
  "status": {
    "code": 500,
    "msg": "Internal Server Error"
  },
  "errors": ["Destination host name not set."]
}"#;

/// Take an API Gateway proxy request and rewrite it into an API Gateway proxy response containing
/// the redirect or an error message if no host is defined.
pub fn rewrite(req: serde_json::Value, _ctx: Context) -> ApiGatewayResponseType {
    // dump the event if logging is enabled
    debug_log(|| {
        format!(
            "Event Payload: {}",
            serde_json::to_string(&req).unwrap_or_else(|e| { format!("(error: {e:?})") })
        )
    });

    let req_backup = req.clone();

    // try to get the request as either v1 or v2 of api gateway
    let req = match serde_json::from_value::<ApiGatewayRequestType>(req) {
        Ok(req) => req,
        Err(e) => {
            eprintln!(
                "ERROR: Unable to deserialize event as either version of API gateway request: {e:?}"
            );
            eprintln!(
                "Actual payload: {}",
                serde_json::to_string_pretty(&req_backup)
                    .unwrap_or_else(|e| format!("(error: {e:?})"))
            );

            // here we cannot determine what kind of response to issue so we return a v1
            return ApiGatewayResponseType::V1(ApiGatewayProxyResponse {
                status_code: 500,
                headers: Default::default(),
                multi_value_headers: Default::default(),
                body: Some("Invalid event received".into()),
                is_base64_encoded: false,
            });
        }
    };

    let resp = if let Some(hostname) = ecr_registry_url() {
        create_rewrite_response(&req, hostname, cache_max_age())
    } else {
        eprintln!(
            "ERROR: Misconfiguration; please set the {} environment variable to the FQDN of the ECR registry",
            ECR_REGISTRY_ENV_VAR
        );
        create_error_response(&req)
    };

    debug_log(|| {
        format!(
            "Response Payload: {}",
            serde_json::to_string(&req_backup).unwrap_or_else(|e| { format!("(error: {e:?})") })
        )
    });

    resp
}

/// Emit a debug log only if debug logging is enabled.
pub fn debug_log<S: AsRef<str>>(f: impl FnOnce() -> S) {
    if *IS_DEBUG {
        eprintln!("DEBUG: {}", f().as_ref());
    }
}

/// Only emit logs after a given interval of time, preventing masses of redundant logs.
pub fn log_infrequently<S: AsRef<str>>(message: S) -> LogStatus {
    let now = Instant::now();

    // get a read-only guard
    let guard = LAST_LOG_TIME.read();

    // if LAST_LOG_TIME is unset or it has been MIN_LOG_INTERVAL since the last log, then log and
    // store the new last log time.
    if guard.is_none() || now.duration_since(guard.unwrap()).ge(&MIN_LOG_INTERVAL) {
        eprintln!("{}", message.as_ref());
        drop(guard);

        let mut guard = LAST_LOG_TIME.write();
        *guard = Some(now);

        LogStatus::Emitted
    } else {
        LogStatus::Ignored
    }
}

/// Simple enum representing whether a log message was emitted or ignored.
#[derive(Debug, Eq, PartialEq)]
pub enum LogStatus {
    Emitted,
    Ignored,
}

/// Fetch (and cache) the ECR registry host from the [ECR_REGISTRY_ENV_VAR] environment variable.
pub fn ecr_registry_url() -> Option<&'static String> {
    REGISTRY_URL
        .get_or_init(|| env::var(ECR_REGISTRY_ENV_VAR).ok())
        .as_ref()
}

/// Determine the `max-age` setting for the `Cache-Control` header.
pub fn cache_max_age() -> usize {
    *CACHE_MAX_AGE.get_or_init(|| {
        if let Ok(v) = env::var(CACHE_MAX_AGE_ENV_VAR) {
            v.parse().unwrap_or(CACHE_MAX_AGE_DEFAULT)
        } else {
            CACHE_MAX_AGE_DEFAULT
        }
    })
}

/// Determines whether to serve a JSON response for a given request.
pub fn should_return_json(req: &ApiGatewayRequestType) -> bool {
    for header_name in ["Accept", "Content-Type"] {
        if let Some(value) = req.headers().get(header_name) {
            // if html is NOT in the content type, return json
            if !value.to_str().unwrap_or("").contains("html") {
                return true;
            }
        }
    }

    false
}

/// Creates a 500 error response in either JSON or HTML for the circumstance in which we lack the
/// [ECR_REGISTRY_ENV_VAR] fqdn of the ECR registry.
pub fn create_error_response(req: &ApiGatewayRequestType) -> ApiGatewayResponseType {
    let mut resp = ApiGatewayGenericResponse::builder()
        .req(req)
        .status_code(500)
        .build();

    if should_return_json(req) {
        resp.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        resp.body = Some(Body::Text(JSON_ERROR_RESPONSE.to_string()))
    } else {
        resp.headers.insert(
            "Content-Type",
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        resp.body = Some(Body::Text(HTML_ERROR_RESPONSE.to_string()));
    }

    resp.into()
}

/// Creates a 307 rewrite response, redirecting the client to the ECR registry.
pub fn create_rewrite_response<S: AsRef<str>>(
    req: &ApiGatewayRequestType,
    host: S,
    max_age: usize,
) -> ApiGatewayResponseType {
    let mut resp = ApiGatewayGenericResponse::builder()
        .req(req)
        .status_code(307)
        .build();

    let path = req
        .path()
        .as_ref()
        .map(|p| {
            if !p.starts_with("/") {
                format!("/{}", p)
            } else {
                p.to_string()
            }
        })
        .unwrap_or("/".into());

    let query = {
        let qs = req.query().to_query_string();

        if qs.is_empty() {
            qs
        } else {
            format!("?{}", qs)
        }
    };

    let location = format!("https://{host}{path}{query}", host = host.as_ref());

    resp.status_code = 307;
    resp.headers.insert(
        "Cache-Control",
        HeaderValue::from_str(format!("max-age={max_age}").as_str()).unwrap(),
    );
    resp.headers.insert(
        "Location",
        HeaderValue::from_str(location.as_str()).unwrap(),
    );

    resp.into()
}

#[cfg(test)]
mod tests;

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::Context;
use std::env;

use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderValue;
use parking_lot::RwLock;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

static REGISTRY_URL: OnceLock<Option<String>> = OnceLock::new();

static CACHE_MAX_AGE: OnceLock<usize> = OnceLock::new();

/// The name of the environment variable containing the ECR registry host FQDN.
pub const ECR_REGISTRY_ENV_VAR: &'static str = "ECR_REGISTRY_HOST";

/// The name of the environment variable containing the cache max age in seconds to return with
/// responses.
pub const CACHE_MAX_AGE_ENV_VAR: &'static str = "CACHE_MAX_AGE";

/// The default max age in seconds to specify in the `Cache-Control` header.
pub const CACHE_MAX_AGE_DEFAULT: usize = 60;

/// The minimum amount of time to wait before logging failed requests.
pub const MIN_LOG_INTERVAL: Duration = Duration::from_secs(60);

static LAST_LOG_TIME: RwLock<Option<Instant>> = RwLock::new(None);

static HTML_ERROR_RESPONSE: &'static str = r#"<!doctype html>
<html lang="en-us">
  <head>
    <title>Error: 500 (Internal Server Error)</title>
  </head>
  <body>
    <h1>Error: 500 (Internal Server Error)</h1>
    <p>Destination host name not set.</p>
  </body>
</html>"#;

static JSON_ERROR_RESPONSE: &'static str = r#"{
  "status": {
    "code": 500,
    "msg": "Internal Server Error"
  },
  "errors": ["Destination host name not set."]
}"#;

/// Take an API Gateway proxy request and rewrite it into an API Gateway proxy response containing
/// the redirect or an error message if no host is defined.
pub fn rewrite(req: ApiGatewayProxyRequest, _ctx: Context) -> ApiGatewayProxyResponse {
    if let Some(hostname) = ecr_registry_url() {
        create_rewrite_response(&req, hostname, cache_max_age())
    } else {
        eprintln!("ERROR: Misconfiguration; please set the {} environment variable to the FQDN of the ECR registry", ECR_REGISTRY_ENV_VAR);
        create_error_response(&req)
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
        .get_or_init(|| {
            if let Ok(v) = env::var(ECR_REGISTRY_ENV_VAR) {
                Some(v)
            } else {
                None
            }
        })
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
pub fn should_return_json(req: &ApiGatewayProxyRequest) -> bool {
    for header_name in ["Accept", "Content-Type"] {
        if let Some(value) = req.headers.get(header_name) {
            if value.to_str().unwrap_or("text/html").contains("json") {
                return true;
            }
        }
    }

    false
}

/// Creates a 500 error response in either JSON or HTML for the circumstance in which we lack the
/// [ECR_REGISTRY_ENV_VAR] fqdn of the ECR registry.
pub fn create_error_response(req: &ApiGatewayProxyRequest) -> ApiGatewayProxyResponse {
    let mut resp = ApiGatewayProxyResponse::default();
    resp.status_code = 500;

    if should_return_json(&req) {
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

    resp
}

/// Creates a 307 rewrite response, redirecting the client to the ECR registry.
pub fn create_rewrite_response<S: AsRef<str>>(
    req: &ApiGatewayProxyRequest,
    host: S,
    max_age: usize,
) -> ApiGatewayProxyResponse {
    let mut resp = ApiGatewayProxyResponse::default();

    let path = req
        .path
        .as_ref()
        .map(|p| {
            if !p.starts_with("/") {
                format!("/{}", p)
            } else {
                format!("{}", p)
            }
        })
        .unwrap_or("/".into());

    let query = {
        let qs = req.query_string_parameters.to_query_string();

        if qs.len() == 0 {
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

    resp
}

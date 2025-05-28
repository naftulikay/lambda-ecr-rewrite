use crate::requests::ApiGatewayRequestType;
use aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayV2httpResponse};
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderMap;
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiGatewayResponseType {
    V1(ApiGatewayProxyResponse),
    V2(ApiGatewayV2httpResponse),
}

impl ApiGatewayResponseType {
    pub fn is_v1(&self) -> bool {
        matches!(self, Self::V1(_))
    }

    pub fn is_v2(&self) -> bool {
        matches!(self, Self::V2(_))
    }

    pub fn body(&self) -> Option<&Body> {
        match self {
            Self::V1(resp) => resp.body.as_ref(),
            Self::V2(resp) => resp.body.as_ref(),
        }
    }

    pub fn body_mut(&mut self) -> Option<&mut Body> {
        match self {
            Self::V1(resp) => resp.body.as_mut(),
            Self::V2(resp) => resp.body.as_mut(),
        }
    }

    pub fn is_base64_encoded(&self) -> bool {
        match self {
            Self::V1(resp) => resp.is_base64_encoded,
            Self::V2(resp) => resp.is_base64_encoded,
        }
    }

    pub fn set_is_base64_encoded(&mut self, is_base64_encoded: bool) {
        match self {
            Self::V1(resp) => resp.is_base64_encoded = is_base64_encoded,
            Self::V2(resp) => resp.is_base64_encoded = is_base64_encoded,
        }
    }

    pub fn headers(&self) -> &HeaderMap {
        match self {
            Self::V1(resp) => &resp.headers,
            Self::V2(resp) => &resp.headers,
        }
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        match self {
            Self::V1(resp) => &mut resp.headers,
            Self::V2(resp) => &mut resp.headers,
        }
    }

    pub fn status_code(&self) -> i64 {
        match self {
            Self::V1(resp) => resp.status_code,
            Self::V2(resp) => resp.status_code,
        }
    }

    pub fn set_status_code(&mut self, code: i64) {
        match self {
            Self::V1(resp) => resp.status_code = code,
            Self::V2(resp) => resp.status_code = code,
        }
    }

    pub fn cookies(&self) -> Option<&Vec<String>> {
        match self {
            Self::V1(_) => None,
            Self::V2(resp) => Some(&resp.cookies),
        }
    }

    pub fn cookies_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            Self::V1(_) => None,
            Self::V2(resp) => Some(&mut resp.cookies),
        }
    }
}

#[derive(Debug, Clone, Builder)]
pub struct ApiGatewayGenericResponse<'a> {
    pub(crate) req: &'a ApiGatewayRequestType,
    pub(crate) body: Option<Body>,
    #[builder(default)]
    pub(crate) is_base64_encoded: bool,
    #[builder(default)]
    pub(crate) headers: HeaderMap,
    pub(crate) status_code: i64,
    #[builder(into)]
    pub(crate) cookies: Option<Vec<String>>,
}

impl From<ApiGatewayGenericResponse<'_>> for ApiGatewayResponseType {
    fn from(value: ApiGatewayGenericResponse<'_>) -> Self {
        match &value.req {
            ApiGatewayRequestType::V1(_) => ApiGatewayResponseType::V1(ApiGatewayProxyResponse {
                status_code: value.status_code,
                headers: value.headers.clone(),
                multi_value_headers: value.headers,
                body: value.body,
                is_base64_encoded: value.is_base64_encoded,
            }),
            ApiGatewayRequestType::V2(_) => ApiGatewayResponseType::V2(ApiGatewayV2httpResponse {
                status_code: value.status_code,
                headers: value.headers.clone(),
                multi_value_headers: value.headers.clone(),
                body: value.body,
                is_base64_encoded: value.is_base64_encoded,
                // priority: use cookies specified in the builder, otherwise copy the request cookies
                cookies: value
                    .cookies
                    .unwrap_or(value.req.cookies().cloned().unwrap_or_default()),
            }),
        }
    }
}

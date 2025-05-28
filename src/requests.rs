use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayV2httpRequest};
use aws_lambda_events::http::HeaderMap;
use aws_lambda_events::query_map::QueryMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiGatewayRequestType {
    // note: v2 should be tried first as it is more specific
    V2(ApiGatewayV2httpRequest),
    V1(ApiGatewayProxyRequest),
}

impl ApiGatewayRequestType {
    pub fn path(&self) -> Option<&String> {
        match &self {
            Self::V1(req) => req.path.as_ref(),
            Self::V2(req) => req.raw_path.as_ref(),
        }
    }

    pub fn set_path(&mut self, path: impl Into<String>) {
        match self {
            Self::V1(req) => req.path = Some(path.into()),
            Self::V2(req) => req.raw_path = Some(path.into()),
        }
    }

    pub fn query(&self) -> &QueryMap {
        match &self {
            Self::V1(req) => &req.query_string_parameters,
            Self::V2(req) => &req.query_string_parameters,
        }
    }

    pub fn query_mut(&mut self) -> &mut QueryMap {
        match self {
            Self::V1(req) => &mut req.query_string_parameters,
            Self::V2(req) => &mut req.query_string_parameters,
        }
    }

    pub fn headers(&self) -> &HeaderMap {
        match &self {
            Self::V1(req) => &req.multi_value_headers,
            Self::V2(req) => &req.headers,
        }
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        match self {
            Self::V1(req) => &mut req.multi_value_headers,
            Self::V2(req) => &mut req.headers,
        }
    }
}

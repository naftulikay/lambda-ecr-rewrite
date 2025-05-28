mod fixtures;
mod tests_v1;
mod tests_v2;

use crate::requests::ApiGatewayRequestType;
use crate::responses::{ApiGatewayGenericResponse, ApiGatewayResponseType};
use crate::{HTML_ERROR_RESPONSE, JSON_ERROR_RESPONSE, LogStatus, log_infrequently};
use fixtures::{APIGW_REQ_V1, APIGW_REQ_V1_WITH_VERSION, APIGW_REQ_V2};
use serde_json::Value;

#[test]
fn test_valid_error_json() {
    let _: Value = serde_json::from_str(JSON_ERROR_RESPONSE).unwrap();
}

#[test]
fn test_valid_error_html() {
    assert!(html_parser::Dom::parse(HTML_ERROR_RESPONSE).is_ok());
}

/// Utility: deserialize a string into a request type enum
fn deserialize_req(value: &str) -> Result<ApiGatewayRequestType, serde_json::Error> {
    serde_json::from_str::<ApiGatewayRequestType>(value)
}

/// Test that example payload deserialization works
#[test]
fn test_api_gateway_req_deserialize_versions() {
    deserialize_req(APIGW_REQ_V1).unwrap();
    deserialize_req(APIGW_REQ_V1_WITH_VERSION).unwrap();
    deserialize_req(APIGW_REQ_V2).unwrap();
}

/// Test that deserializing different versions of request events yields the correct version
#[test]
fn test_api_gateway_req_version_mapping() {
    assert!(
        matches!(
            deserialize_req(APIGW_REQ_V1),
            Ok(ApiGatewayRequestType::V1(_))
        ),
        "should deserialize a v1 request when version not present"
    );

    assert!(
        matches!(
            deserialize_req(APIGW_REQ_V1_WITH_VERSION),
            Ok(ApiGatewayRequestType::V1(_))
        ),
        "should deserialize a v1 request when version 1.0 present"
    );

    assert!(
        matches!(
            deserialize_req(APIGW_REQ_V2),
            Ok(ApiGatewayRequestType::V2(_))
        ),
        "should deserialize a v2 request when version 2.0 present"
    );
}

/// Tests that for V1 requests, V1 responses are returned, and the same for V2 requests/responses
#[test]
fn test_api_gateway_req_resp_mapping() {
    assert!(
        matches!(
            ApiGatewayGenericResponse::builder()
                .req(&deserialize_req(APIGW_REQ_V1).unwrap())
                .status_code(200)
                .build()
                .into(),
            ApiGatewayResponseType::V1(_)
        ),
        "should create a v1 response from a v1 request without an explicit version"
    );

    assert!(
        matches!(
            ApiGatewayGenericResponse::builder()
                .req(&deserialize_req(APIGW_REQ_V1_WITH_VERSION).unwrap())
                .status_code(200)
                .build()
                .into(),
            ApiGatewayResponseType::V1(_)
        ),
        "should create a v1 response from a v1 request with version 1.0"
    );

    assert!(
        matches!(
            ApiGatewayGenericResponse::builder()
                .req(&deserialize_req(APIGW_REQ_V2).unwrap())
                .status_code(200)
                .build()
                .into(),
            ApiGatewayResponseType::V2(_)
        ),
        "should create a v2 response from a v2 request"
    );
}

#[test]
fn test_log_infrequently() {
    assert_eq!(LogStatus::Emitted, log_infrequently("log message A"));
    assert_eq!(LogStatus::Ignored, log_infrequently("log message B"));
}

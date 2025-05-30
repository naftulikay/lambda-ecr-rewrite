use crate::requests::ApiGatewayRequestType;
use crate::responses::{ApiGatewayGenericResponse, ApiGatewayResponseType};
use crate::{
    CACHE_MAX_AGE_DEFAULT, HTML_ERROR_RESPONSE, JSON_ERROR_RESPONSE, create_error_response,
    create_rewrite_response, should_return_json,
};
use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderValue;
use aws_lambda_events::query_map::QueryMap;
use std::collections::HashMap;

#[test]
fn test_v2_copy_req_cookies() {
    let req = ApiGatewayRequestType::V2(ApiGatewayV2httpRequest {
        cookies: Some(vec!["TEST-COOKIE".to_string()]),
        ..Default::default()
    });

    let generic_response = ApiGatewayGenericResponse::builder()
        .req(&req)
        .status_code(200)
        .build();

    let resp: ApiGatewayResponseType = generic_response.into();

    assert_eq!(
        Some(&"TEST-COOKIE".to_string()),
        resp.cookies().unwrap().first(),
        "should copy cookies from the request"
    );
}

#[test]
fn test_v2_override_cookies() {
    let req = ApiGatewayRequestType::V2(ApiGatewayV2httpRequest {
        cookies: Some(vec!["OVERRIDDEN-COOKIE".to_string()]),
        ..Default::default()
    });

    let generic_response = ApiGatewayGenericResponse::builder()
        .req(&req)
        .status_code(200)
        .cookies(vec!["TEST-COOKIE".to_string()])
        .build();

    let resp: ApiGatewayResponseType = generic_response.into();

    // should not contain the cookies from the original request
    assert!(
        !resp
            .cookies()
            .unwrap()
            .contains(&"OVERRIDDEN-COOKIE".to_string())
    );
    // should contain the cookies from the override
    assert!(resp.cookies().unwrap().contains(&"TEST-COOKIE".to_string()));
}

#[test]
fn test_v2_should_return_json() {
    let mut req = ApiGatewayRequestType::V2(Default::default());

    // test accept json header
    req.headers_mut()
        .insert("Accept", HeaderValue::from_static("application/json"));

    assert!(should_return_json(&req));

    req.headers_mut().remove("Accept");

    // test content type json header
    req.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));

    assert!(should_return_json(&req));

    req.headers_mut().remove("Content-Type");

    // test empty
    assert!(!should_return_json(&req));

    // test accept html
    req.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("text/html"));

    assert!(!should_return_json(&req));

    req.headers_mut().remove("Content-Type");

    // test content type html
    req.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("text/html"));

    assert!(!should_return_json(&req));
}

#[test]
fn test_v2_create_rewrite_response_cache() {
    let mut req = ApiGatewayRequestType::V2(Default::default());
    req.set_path("a/b/c");

    let resp = create_rewrite_response(&req, "ecr.registry.com", 12345);

    assert!(resp.headers().contains_key("Cache-Control"));
    assert_eq!(
        "max-age=12345",
        resp.headers().get("Cache-Control").unwrap()
    );
}

#[test]
fn test_v2_create_rewrite_response_path() {
    let mut req = ApiGatewayRequestType::V2(Default::default());
    req.set_path("a/b/c");

    let resp = create_rewrite_response(&req, "ecr.registry.com", CACHE_MAX_AGE_DEFAULT);

    assert_eq!(307, resp.status_code());

    assert!(resp.headers().contains_key("Location"));
    assert_eq!(
        "https://ecr.registry.com/a/b/c",
        resp.headers().get("Location").unwrap().to_str().unwrap()
    );

    assert_eq!(
        "max-age=60",
        resp.headers()
            .get("Cache-Control")
            .unwrap_or(&HeaderValue::from_static(""))
    );
}

#[test]
fn test_v2_create_rewrite_response_no_path() {
    let req = ApiGatewayRequestType::V2(Default::default());
    let resp = create_rewrite_response(&req, "ecr.myhost.com", 100);

    assert!(resp.headers().contains_key("Location"));
    assert_eq!(
        "https://ecr.myhost.com/",
        resp.headers().get("Location").unwrap()
    );
}

#[test]
fn test_v2_create_rewrite_response_qs() {
    let qs = QueryMap::from(HashMap::from([("size".to_string(), "lorge".to_string())]));

    let mut req = ApiGatewayRequestType::V2(Default::default());
    *req.query_mut() = qs;

    let resp = create_rewrite_response(&req, "ecr.myhost.com", 60);

    assert!(resp.headers().contains_key("Location"));
    assert_eq!(
        "https://ecr.myhost.com/?size=lorge",
        resp.headers().get("Location").unwrap()
    )
}

#[test]
fn test_v2_create_rewrite_response_qs_extra() {
    let qs = QueryMap::from(HashMap::from([
        ("a".to_string(), "b".to_string()),
        ("c".to_string(), "22".to_string()),
    ]));

    let mut req = ApiGatewayRequestType::V2(Default::default());
    req.set_path("/twenty");
    *req.query_mut() = qs;

    let resp = create_rewrite_response(&req, "ecr.myhost.com", 120);

    assert!(resp.headers().contains_key("Location"));

    let location = resp.headers().get("Location").unwrap().to_str().unwrap();

    assert!(location.contains("?"));
    assert!(location.contains("a=b"));
    assert!(location.contains("c=22"));
}

#[test]
fn test_v2_create_error_response() {
    let mut req = ApiGatewayRequestType::V1(Default::default());
    req.headers_mut()
        .insert("Accept", HeaderValue::from_static("application/json"));
    req.set_path("/a/b/c");

    let resp = create_error_response(&req);

    // test json body
    match resp.body() {
        Some(Body::Text(body)) => assert_eq!(JSON_ERROR_RESPONSE, body),
        _ => panic!("returned non-text body"),
    }

    // test html body
    req.headers_mut().remove("Accept");

    let resp = create_error_response(&req);

    match resp.body() {
        Some(Body::Text(body)) => assert_eq!(HTML_ERROR_RESPONSE, body),
        _ => panic!("returned non-text body"),
    }
}

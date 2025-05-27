use crate::{
    ApiGatewayProxyEventType, CACHE_MAX_AGE_DEFAULT, HTML_ERROR_RESPONSE, JSON_ERROR_RESPONSE,
    LogStatus, create_error_response, create_rewrite_response, log_infrequently,
    should_return_json,
};
use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::HeaderValue;
use aws_lambda_events::query_map::QueryMap;
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn test_valid_error_json() {
    let _: Value = serde_json::from_str(JSON_ERROR_RESPONSE).unwrap();
}

#[test]
fn test_valid_error_html() {
    assert!(html_parser::Dom::parse(HTML_ERROR_RESPONSE).is_ok());
}

const APIGW_REQ_V1: &str = r#"
{
	"resource": "/{proxy+}",
	"path": "/hello/world",
	"httpMethod": "POST",
	"headers": {
		"Accept": "*/*",
		"Accept-Encoding": "gzip, deflate",
		"cache-control": "no-cache",
		"CloudFront-Forwarded-Proto": "https",
		"CloudFront-Is-Desktop-Viewer": "true",
		"CloudFront-Is-Mobile-Viewer": "false",
		"CloudFront-Is-SmartTV-Viewer": "false",
		"CloudFront-Is-Tablet-Viewer": "false",
		"CloudFront-Viewer-Country": "US",
		"Content-Type": "application/json",
		"headerName": "headerValue",
		"Host": "gy415nuibc.execute-api.us-east-1.amazonaws.com",
		"Postman-Token": "9f583ef0-ed83-4a38-aef3-eb9ce3f7a57f",
		"User-Agent": "PostmanRuntime/2.4.5",
		"Via": "1.1 d98420743a69852491bbdea73f7680bd.cloudfront.net (CloudFront)",
		"X-Amz-Cf-Id": "pn-PWIJc6thYnZm5P0NMgOUglL1DYtl0gdeJky8tqsg8iS_sgsKD1A==",
		"X-Forwarded-For": "54.240.196.186, 54.182.214.83",
		"X-Forwarded-Port": "443",
		"X-Forwarded-Proto": "https"
	},
	"multiValueHeaders": {
		"Accept": [
			"*/*"
		],
		"Accept-Encoding": [
			"gzip, deflate"
		],
		"cache-control": [
			"no-cache"
		],
		"CloudFront-Forwarded-Proto": [
			"https"
		],
		"CloudFront-Is-Desktop-Viewer": [
			"true"
		],
		"CloudFront-Is-Mobile-Viewer": [
			"false"
		],
		"CloudFront-Is-SmartTV-Viewer": [
			"false"
		],
		"CloudFront-Is-Tablet-Viewer": [
			"false"
		],
		"CloudFront-Viewer-Country": [
			"US"
		],
		"Content-Type": [
			"application/json"
		],
		"headerName": [
			"headerValue"
		],
		"Host": [
			"gy415nuibc.execute-api.us-east-1.amazonaws.com"
		],
		"Postman-Token": [
			"9f583ef0-ed83-4a38-aef3-eb9ce3f7a57f"
		],
		"User-Agent": [
			"PostmanRuntime/2.4.5"
		],
		"Via": [
			"1.1 d98420743a69852491bbdea73f7680bd.cloudfront.net (CloudFront)"
		],
		"X-Amz-Cf-Id": [
			"pn-PWIJc6thYnZm5P0NMgOUglL1DYtl0gdeJky8tqsg8iS_sgsKD1A=="
		],
		"X-Forwarded-For": [
			"54.240.196.186, 54.182.214.83"
		],
		"X-Forwarded-Port": [
			"443"
		],
		"X-Forwarded-Proto": [
			"https"
		]
	},
	"queryStringParameters": {
		"name": "me"
	},
	"multiValueQueryStringParameters": {
		"name": [
			"me"
		]
	},
	"pathParameters": {
		"proxy": "hello/world"
	},
	"stageVariables": {
		"stageVariableName": "stageVariableValue"
	},
	"requestContext": {
		"accountId": "12345678912",
		"resourceId": "roq9wj",
		"path": "/hello/world",
		"stage": "testStage",
		"domainName": "gy415nuibc.execute-api.us-east-2.amazonaws.com",
		"domainPrefix": "y0ne18dixk",
		"requestId": "deef4878-7910-11e6-8f14-25afc3e9ae33",
		"protocol": "HTTP/1.1",
		"identity": {
			"cognitoIdentityPoolId": "theCognitoIdentityPoolId",
			"accountId": "theAccountId",
			"cognitoIdentityId": "theCognitoIdentityId",
			"caller": "theCaller",
			"apiKey": "theApiKey",
			"apiKeyId": "theApiKeyId",
			"accessKey": "ANEXAMPLEOFACCESSKEY",
			"sourceIp": "192.168.196.186",
			"cognitoAuthenticationType": "theCognitoAuthenticationType",
			"cognitoAuthenticationProvider": "theCognitoAuthenticationProvider",
			"userArn": "theUserArn",
			"userAgent": "PostmanRuntime/2.4.5",
			"user": "theUser"
		},
		"authorizer": {
			"principalId": "admin",
			"clientId": 1,
			"clientName": "Exata"
		},
		"resourcePath": "/{proxy+}",
		"httpMethod": "POST",
		"requestTime": "15/May/2020:06:01:09 +0000",
		"requestTimeEpoch": 1589522469693,
		"apiId": "gy415nuibc"
	},
	"body": "{\r\n\t\"a\": 1\r\n}"
}
"#;

const APIGW_REQ_V1_WITH_VERSION: &str = r#"
{
	"resource": "/{proxy+}",
	"path": "/hello/world",
	"httpMethod": "POST",
	"headers": {
		"Accept": "*/*",
		"Accept-Encoding": "gzip, deflate",
		"cache-control": "no-cache",
		"CloudFront-Forwarded-Proto": "https",
		"CloudFront-Is-Desktop-Viewer": "true",
		"CloudFront-Is-Mobile-Viewer": "false",
		"CloudFront-Is-SmartTV-Viewer": "false",
		"CloudFront-Is-Tablet-Viewer": "false",
		"CloudFront-Viewer-Country": "US",
		"Content-Type": "application/json",
		"headerName": "headerValue",
		"Host": "gy415nuibc.execute-api.us-east-1.amazonaws.com",
		"Postman-Token": "9f583ef0-ed83-4a38-aef3-eb9ce3f7a57f",
		"User-Agent": "PostmanRuntime/2.4.5",
		"Via": "1.1 d98420743a69852491bbdea73f7680bd.cloudfront.net (CloudFront)",
		"X-Amz-Cf-Id": "pn-PWIJc6thYnZm5P0NMgOUglL1DYtl0gdeJky8tqsg8iS_sgsKD1A==",
		"X-Forwarded-For": "54.240.196.186, 54.182.214.83",
		"X-Forwarded-Port": "443",
		"X-Forwarded-Proto": "https"
	},
	"multiValueHeaders": {
		"Accept": [
			"*/*"
		],
		"Accept-Encoding": [
			"gzip, deflate"
		],
		"cache-control": [
			"no-cache"
		],
		"CloudFront-Forwarded-Proto": [
			"https"
		],
		"CloudFront-Is-Desktop-Viewer": [
			"true"
		],
		"CloudFront-Is-Mobile-Viewer": [
			"false"
		],
		"CloudFront-Is-SmartTV-Viewer": [
			"false"
		],
		"CloudFront-Is-Tablet-Viewer": [
			"false"
		],
		"CloudFront-Viewer-Country": [
			"US"
		],
		"Content-Type": [
			"application/json"
		],
		"headerName": [
			"headerValue"
		],
		"Host": [
			"gy415nuibc.execute-api.us-east-1.amazonaws.com"
		],
		"Postman-Token": [
			"9f583ef0-ed83-4a38-aef3-eb9ce3f7a57f"
		],
		"User-Agent": [
			"PostmanRuntime/2.4.5"
		],
		"Via": [
			"1.1 d98420743a69852491bbdea73f7680bd.cloudfront.net (CloudFront)"
		],
		"X-Amz-Cf-Id": [
			"pn-PWIJc6thYnZm5P0NMgOUglL1DYtl0gdeJky8tqsg8iS_sgsKD1A=="
		],
		"X-Forwarded-For": [
			"54.240.196.186, 54.182.214.83"
		],
		"X-Forwarded-Port": [
			"443"
		],
		"X-Forwarded-Proto": [
			"https"
		]
	},
	"queryStringParameters": {
		"name": "me"
	},
	"multiValueQueryStringParameters": {
		"name": [
			"me"
		]
	},
	"pathParameters": {
		"proxy": "hello/world"
	},
	"stageVariables": {
		"stageVariableName": "stageVariableValue"
	},
	"requestContext": {
		"accountId": "12345678912",
		"resourceId": "roq9wj",
		"path": "/hello/world",
		"stage": "testStage",
		"domainName": "gy415nuibc.execute-api.us-east-2.amazonaws.com",
		"domainPrefix": "y0ne18dixk",
		"requestId": "deef4878-7910-11e6-8f14-25afc3e9ae33",
		"protocol": "HTTP/1.1",
		"identity": {
			"cognitoIdentityPoolId": "theCognitoIdentityPoolId",
			"accountId": "theAccountId",
			"cognitoIdentityId": "theCognitoIdentityId",
			"caller": "theCaller",
			"apiKey": "theApiKey",
			"apiKeyId": "theApiKeyId",
			"accessKey": "ANEXAMPLEOFACCESSKEY",
			"sourceIp": "192.168.196.186",
			"cognitoAuthenticationType": "theCognitoAuthenticationType",
			"cognitoAuthenticationProvider": "theCognitoAuthenticationProvider",
			"userArn": "theUserArn",
			"userAgent": "PostmanRuntime/2.4.5",
			"user": "theUser"
		},
		"authorizer": {
			"principalId": "admin",
			"clientId": 1,
			"clientName": "Exata"
		},
		"resourcePath": "/{proxy+}",
		"httpMethod": "POST",
		"requestTime": "15/May/2020:06:01:09 +0000",
		"requestTimeEpoch": 1589522469693,
		"apiId": "gy415nuibc"
	},
	"body": "{\r\n\t\"a\": 1\r\n}"
}
"#;

const APIGW_REQ_V2: &str = r#"
{
  "version": "2.0",
  "routeKey": "$default",
  "rawPath": "/",
  "rawQueryString": "",
  "headers": {
    "accept": "*/*",
    "content-length": "0",
    "host": "aaaaaaaaaa.execute-api.us-west-2.amazonaws.com",
    "user-agent": "curl/7.58.0",
    "x-amzn-trace-id": "Root=1-5e9f0c65-1de4d666d4dd26aced652b6c",
    "x-forwarded-for": "1.2.3.4",
    "x-forwarded-port": "443",
    "x-forwarded-proto": "https"
  },
  "requestContext": {
    "accountId": "123456789012",
    "apiId": "aaaaaaaaaa",
    "authentication": {
      "clientCert": {
        "clientCertPem": "-----BEGIN CERTIFICATE-----\nMIIEZTCCAk0CAQEwDQ...",
        "issuerDN": "C=US,ST=Washington,L=Seattle,O=Amazon Web Services,OU=Security,CN=My Private CA",
        "serialNumber": "1",
        "subjectDN": "C=US,ST=Washington,L=Seattle,O=Amazon Web Services,OU=Security,CN=My Client",
        "validity": {
          "notAfter": "Aug  5 00:28:21 2120 GMT",
          "notBefore": "Aug 29 00:28:21 2020 GMT"
        }
      }
    },
    "domainName": "aaaaaaaaaa.execute-api.us-west-2.amazonaws.com",
    "domainPrefix": "aaaaaaaaaa",
    "http": {
      "method": "GET",
      "path": "/",
      "protocol": "HTTP/1.1",
      "sourceIp": "1.2.3.4",
      "userAgent": "curl/7.58.0"
    },
    "requestId": "LV7fzho-PHcEJPw=",
    "routeKey": "$default",
    "stage": "$default",
    "time": "21/Apr/2020:15:08:21 +0000",
    "timeEpoch": 1587481701067
  },
  "isBase64Encoded": false
}
"#;

#[test]
fn test_api_gateway_versions() {
    serde_json::from_str::<ApiGatewayProxyEventType>(APIGW_REQ_V1).unwrap();
    serde_json::from_str::<ApiGatewayProxyEventType>(APIGW_REQ_V1_WITH_VERSION).unwrap();
    serde_json::from_str::<ApiGatewayProxyEventType>(APIGW_REQ_V2).unwrap();
}

#[test]
fn test_should_return_json() {
    let mut req = ApiGatewayProxyEventType::V1(ApiGatewayProxyRequest::default());

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
fn test_create_rewrite_response_cache() {
    let mut req = ApiGatewayProxyEventType::V1(ApiGatewayProxyRequest::default());
    req.set_path("a/b/c");

    let resp = create_rewrite_response(&req, "ecr.registry.com", 12345);

    assert!(resp.headers.contains_key("Cache-Control"));
    assert_eq!("max-age=12345", resp.headers.get("Cache-Control").unwrap());
}

#[test]
fn test_create_rewrite_response_path() {
    let mut req = ApiGatewayProxyEventType::V1(ApiGatewayProxyRequest::default());
    req.set_path("a/b/c");

    let resp = create_rewrite_response(&req, "ecr.registry.com", CACHE_MAX_AGE_DEFAULT);

    assert_eq!(307, resp.status_code);

    assert!(resp.headers.contains_key("Location"));
    assert_eq!(
        "https://ecr.registry.com/a/b/c",
        resp.headers.get("Location").unwrap().to_str().unwrap()
    );

    assert_eq!(
        "max-age=60",
        resp.headers
            .get("Cache-Control")
            .unwrap_or(&HeaderValue::from_static(""))
    );
}

#[test]
fn test_create_rewrite_response_no_path() {
    let req = ApiGatewayProxyEventType::V1(ApiGatewayProxyRequest::default());
    let resp = create_rewrite_response(&req, "ecr.myhost.com", 100);

    assert!(resp.headers.contains_key("Location"));
    assert_eq!(
        "https://ecr.myhost.com/",
        resp.headers.get("Location").unwrap()
    );
}

#[test]
fn test_create_rewrite_response_qs() {
    let qs = QueryMap::from({
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert("size".into(), "lorge".into());
        m
    });

    let mut req = ApiGatewayProxyEventType::V1(ApiGatewayProxyRequest::default());
    *req.query_mut() = qs;

    let resp = create_rewrite_response(&req, "ecr.myhost.com", 60);

    assert!(resp.headers.contains_key("Location"));
    assert_eq!(
        "https://ecr.myhost.com/?size=lorge",
        resp.headers.get("Location").unwrap()
    )
}

#[test]
fn test_create_rewrite_response_qs_extra() {
    let qs = QueryMap::from({
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert("a".into(), "b".into());
        m.insert("c".into(), "22".into());
        m
    });

    let mut req = ApiGatewayProxyEventType::V1(ApiGatewayProxyRequest::default());
    req.set_path("/twenty");
    *req.query_mut() = qs;

    let resp = create_rewrite_response(&req, "ecr.myhost.com", 120);

    assert!(resp.headers.contains_key("Location"));

    let location = resp.headers.get("Location").unwrap().to_str().unwrap();

    assert!(location.contains("?"));
    assert!(location.contains("a=b"));
    assert!(location.contains("c=22"));
}

#[test]
fn test_create_error_response() {
    let mut req = ApiGatewayProxyEventType::V1(ApiGatewayProxyRequest::default());
    req.headers_mut()
        .insert("Accept", HeaderValue::from_static("application/json"));
    req.set_path("/a/b/c");

    let resp = create_error_response(&req);

    // test json body
    match resp.body {
        Some(Body::Text(body)) => assert_eq!(JSON_ERROR_RESPONSE, body),
        _ => panic!("returned non-text body"),
    }

    // test html body
    req.headers_mut().remove("Accept");

    let resp = create_error_response(&req);

    match resp.body {
        Some(Body::Text(body)) => assert_eq!(HTML_ERROR_RESPONSE, body),
        _ => panic!("returned non-text body"),
    }
}

#[test]
fn test_log_infrequently() {
    assert_eq!(LogStatus::Emitted, log_infrequently("log message A"));
    assert_eq!(LogStatus::Ignored, log_infrequently("log message B"));
}

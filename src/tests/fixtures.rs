pub const APIGW_REQ_V1: &str = r#"
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
pub const APIGW_REQ_V1_WITH_VERSION: &str = r#"
{
    "version": "1.0",
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
pub const APIGW_REQ_V2: &str = r#"
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

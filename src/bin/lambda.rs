use aws_lambda_events::apigw::ApiGatewayProxyResponse;
use lambda_runtime::{Error, LambdaEvent, service_fn};
use std::env;

use lambda_ecr_rewrite::rewrite;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if let Some(s) = env::args().nth(1) {
        if s == "test" {
            eprintln!("Startup test passed.");
            return Ok(());
        }
    }

    lambda_runtime::run(service_fn(handler)).await
}

async fn handler(event: LambdaEvent<serde_json::Value>) -> Result<ApiGatewayProxyResponse, Error> {
    Ok(rewrite(event.payload, event.context))
}

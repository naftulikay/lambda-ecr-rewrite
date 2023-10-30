use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{service_fn, Error, LambdaEvent};
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

async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    Ok(rewrite(event.payload, event.context))
}

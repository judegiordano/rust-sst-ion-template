use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::Value;
use test_api::logger;

pub async fn handler(_: LambdaEvent<Value>) -> Result<(), Error> {
    tracing::info!("cron hit");
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    run(service_fn(handler)).await
}

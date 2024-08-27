use aws_lambda_events::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use {{crate_name}}::{aws::sqs::parse_records, logger};

#[derive(Debug, Deserialize, Serialize)]
pub struct MyMessage {
    pub name: String,
}

pub async fn handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    let messages = event.payload.records;
    let messages = parse_records::<MyMessage>(messages)?;
    '_messages: for message in messages {
        tracing::info!("{:?}", message);
    }
    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    logger::init()?;
    run(service_fn(handler)).await
}

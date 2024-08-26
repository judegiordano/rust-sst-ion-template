use aws_lambda_events::sqs::SqsMessage;
use aws_sdk_sqs::{operation::send_message::SendMessageOutput, Client};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

use super::config;

pub fn parse_records<T: for<'a> Deserialize<'a>>(
    records: impl IntoIterator<Item = SqsMessage>,
) -> Result<Vec<T>, AppError> {
    let mut items = vec![];
    for record in records {
        if let Some(body) = record.body {
            items.push(serde_json::from_str::<T>(&body).map_err(AppError::internal_server_error)?)
        }
    }
    Ok(items)
}

pub struct SqsFifoMessageOptions {
    pub message_group_id: String,
    pub message_deduplication_id: String,
}

impl Default for SqsFifoMessageOptions {
    fn default() -> Self {
        Self {
            message_group_id: nanoid::nanoid!(),
            message_deduplication_id: nanoid::nanoid!(),
        }
    }
}

pub struct Queue {
    queue_url: String,
    client: Client,
}

impl Queue {
    fn _serialize_body(message: impl Serialize) -> Result<String, AppError> {
        serde_json::to_string(&message).map_err(AppError::bad_request)
    }

    pub async fn new(queue_url: impl ToString) -> Self {
        Self {
            queue_url: queue_url.to_string(),
            client: Client::new(&config().await),
        }
    }

    pub async fn send_fifo_message(
        &self,
        message: impl Serialize,
        options: SqsFifoMessageOptions,
    ) -> Result<SendMessageOutput, AppError> {
        self.client
            .send_message()
            .queue_url(&self.queue_url)
            .message_deduplication_id(options.message_deduplication_id)
            .message_group_id(options.message_group_id)
            .message_body(Self::_serialize_body(message)?)
            .send()
            .await
            .map_err(AppError::internal_server_error)
    }
}

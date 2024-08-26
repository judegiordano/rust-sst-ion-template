use aws_config::{BehaviorVersion, SdkConfig};

{% if s3_bucket %}pub mod s3;{% endif %}
{% if sqs_queue %}pub mod sqs;{% endif %}

pub async fn config() -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest()).load().await
}

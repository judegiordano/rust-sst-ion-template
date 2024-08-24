use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct PingMessage {
    ok: bool,
}

pub async fn ping() -> impl IntoResponse {
    Json(PingMessage { ok: true })
}

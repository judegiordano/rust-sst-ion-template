use axum::{http::StatusCode, response::IntoResponse, Json};
use meme_cache::{get, set};
use serde::{Deserialize, Serialize};

use crate::{
    env::{Env, Stage},
    types::ApiResponse,
};

#[derive(Serialize, Deserialize)]
struct PingMessage {
    ok: bool,
    stage: Stage,
}

pub async fn ping() -> ApiResponse {
    if let Some(exists) = get::<PingMessage>("ping").await {
        return Ok(Json(exists).into_response());
    }
    let Env { stage, .. } = Env::load()?;
    let response = PingMessage { ok: true, stage };
    set("ping", &response, 10_000).await;
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

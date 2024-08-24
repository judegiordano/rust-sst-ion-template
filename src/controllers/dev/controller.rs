use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::{
    env::{Env, Stage},
    types::ApiResponse,
};

#[derive(Serialize)]
struct PingMessage {
    ok: bool,
    stage: Stage,
}

pub async fn ping() -> ApiResponse {
    let Env { stage, .. } = Env::load()?;
    Ok(Json(PingMessage { ok: true, stage }).into_response())
}

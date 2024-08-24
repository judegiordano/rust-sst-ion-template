use serde::{Deserialize, Serialize};
use tracing::Level;

use crate::errors::AppError;

#[derive(Debug, Deserialize, Serialize)]
pub enum Stage {
    Local,
    Test,
    Prod,
    Other(String),
}

#[derive(Debug)]
pub struct Env {
    pub stage: Stage,
    pub log_level: Level,
    pub mongo_uri: String,
}

impl Env {
    fn _get_optional_string(key: &str) -> Option<String> {
        match std::env::var(key.trim().to_uppercase()) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    fn _get_required_string(key: &str) -> Result<String, AppError> {
        match std::env::var(key.trim().to_uppercase()) {
            Ok(value) => Ok(value),
            Err(err) => {
                eprintln!("{key} not found: {err}");
                return Err(AppError::env_error(err));
            }
        }
    }

    pub fn log_level() -> Level {
        match Self::_get_optional_string("LOG_LEVEL") {
            Some(value) => match value.to_uppercase().as_str() {
                "DEBUG" => Level::DEBUG,
                "INFO" => Level::INFO,
                "ERROR" => Level::ERROR,
                "WARN" => Level::WARN,
                "TRACE" => Level::TRACE,
                _ => Level::ERROR,
            },
            None => Level::ERROR,
        }
    }

    pub fn stage() -> Result<Stage, AppError> {
        match Self::_get_required_string("STAGE")?.to_uppercase().as_str() {
            "LOCAL" => Ok(Stage::Local),
            "PROD" => Ok(Stage::Prod),
            "TEST" => Ok(Stage::Test),
            other => Ok(Stage::Other(other.to_string())),
        }
    }

    pub fn mongo_uri() -> Result<String, AppError> {
        Self::_get_required_string("MONGO_URI")
    }

    pub fn load() -> Result<Self, AppError> {
        if cfg!(debug_assertions) {
            use dotenv::dotenv;
            dotenv().ok();
        }
        Ok(Self {
            stage: Self::stage()?,
            log_level: Self::log_level(),
            mongo_uri: Self::mongo_uri()?,
        })
    }
}

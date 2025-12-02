pub mod health;

use axum::{Router, routing::get};
use crate::app_state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/health", get(health::health_check))
}

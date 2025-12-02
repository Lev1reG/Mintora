pub mod health;
pub mod auth;

use axum::{Router, routing::get};
use crate::app_state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
        .nest("/auth", auth::auth_routes())
}

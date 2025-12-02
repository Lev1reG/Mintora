use axum::{Json, http::StatusCode, extract::State};
use serde::Serialize;
use crate::app_state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    database: DatabaseHealth,
}

#[derive(Serialize)]
pub struct DatabaseHealth {
    connected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub async fn health_check(
    State(state): State<AppState>,
) -> (StatusCode, Json<HealthResponse>) {
    // Test database connectivity
    let db_health = match sqlx::query("SELECT 1 as health_check")
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => DatabaseHealth {
            connected: true,
            error: None,
        },
        Err(e) => DatabaseHealth {
            connected: false,
            error: Some(e.to_string()),
        },
    };

    let status_code = if db_health.connected {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(HealthResponse {
            status: if db_health.connected { "healthy" } else { "unhealthy" }.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database: db_health,
        }),
    )
}

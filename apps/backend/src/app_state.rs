use sqlx::PgPool;
use crate::config::Config;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}

impl AppState {
    pub fn new(db: PgPool, config: Config) -> Self {
        Self { db, config }
    }
}

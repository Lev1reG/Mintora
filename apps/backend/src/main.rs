mod config;
mod routes;
mod db;
mod app_state;
mod error;

use app_state::AppState;
use config::Config;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mintora_backend=debug,tower_http=debug,sqlx=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    tracing::info!("Starting Mintora Backend API");
    tracing::info!("Connecting to database...");

    // Create database connection pool
    let db_pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection pool created successfully");

    // Run migrations
    tracing::info!("Running database migrations...");
    db::run_migrations(&db_pool)
        .await
        .expect("Failed to run database migrations");

    tracing::info!("Database migrations completed successfully");

    // Create application state
    let app_state = AppState::new(db_pool);

    // Build application with routes and middleware
    let app = routes::create_router()
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    // Create TCP listener
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on {}", addr);

    // Start server
    axum::serve(listener, app).await.expect("Server failed");
}

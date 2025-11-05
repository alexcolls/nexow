mod db;
mod routes;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url_app = format!(
        "postgres://{}:{}@{}:{}/{}",
        std::env::var("POSTGRES_APP_USER").unwrap(),
        std::env::var("POSTGRES_APP_PASSWORD").unwrap(),
        std::env::var("POSTGRES_APP_HOST").unwrap(),
        std::env::var("POSTGRES_APP_PORT").unwrap(),
        std::env::var("POSTGRES_APP_DB").unwrap(),
    );

    let database_url_ts = format!(
        "postgres://{}:{}@{}:{}/{}",
        std::env::var("TIMESCALEDB_USER").unwrap(),
        std::env::var("TIMESCALEDB_PASSWORD").unwrap(),
        std::env::var("TIMESCALEDB_HOST").unwrap(),
        std::env::var("TIMESCALEDB_PORT").unwrap(),
        std::env::var("TIMESCALEDB_DB").unwrap(),
    );

    let pool_app = db::connect(&database_url_app).await;
    let pool_ts = db::connect(&database_url_ts).await;

    let state = std::sync::Arc::new(state::AppState {
        engine: std::sync::Mutex::new(None),
        pool_app,
        pool_ts,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/assets", get(|| async { axum::Json(Vec::<String>::new()) }))
        .route("/api/sim/start", post(routes::start_sim))
        .route("/api/sim/status", get(routes::sim_status))
        .route("/ws/stream", get(routes::ws_stream))
        .with_state(state)
        .layer(cors);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("{}:{}", host, port).parse().unwrap();

    tracing::info!("server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

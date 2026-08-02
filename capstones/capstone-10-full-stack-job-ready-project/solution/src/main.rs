use capstone_10_solution::{build_router, init_db, AppState};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:url_shortener.db?mode=rwc".to_string());
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let pool = init_db(&database_url).await;
    let state = Arc::new(AppState { db: pool, base_url });

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("URL shortener listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

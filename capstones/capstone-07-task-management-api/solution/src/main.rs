use capstone_07_task_management_api_solution::build_app;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:task-api.db?mode=rwc".into());
    let jwt_secret =
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-in-production".into());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("failed to connect to database");

    let app = build_app(pool, jwt_secret).await;

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");

    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

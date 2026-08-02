use capstone_10_solution::{build_router, AppState};
use reqwest::{Client, StatusCode};
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use uuid::Uuid;

async fn setup() -> (String, tokio::task::JoinHandle<()>) {
    let db_url = format!(
        "sqlite:file:test_{}.sqlite?mode=memory&cache=shared",
        Uuid::new_v4().to_string().replace('-', "")
    );
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS short_links (\
         id TEXT PRIMARY KEY, short_code TEXT NOT NULL UNIQUE, \
         original_url TEXT NOT NULL, created_at TEXT NOT NULL, \
         click_count INTEGER NOT NULL DEFAULT 0);\
         CREATE TABLE IF NOT EXISTS click_events (\
         id TEXT PRIMARY KEY, short_code TEXT NOT NULL, \
         timestamp TEXT NOT NULL, user_agent TEXT);",
    )
    .execute(&pool)
    .await
    .unwrap();

    let state = Arc::new(AppState {
        db: pool,
        base_url: "http://localhost".to_string(),
    });
    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{}", addr);
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    (base, server)
}

#[tokio::test]
async fn test_create_short_link() {
    let (base, _server) = setup().await;
    let client = Client::new();

    let res = client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "https://www.rust-lang.org" }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let json: serde_json::Value = res.json().await.unwrap();
    let code = json["short_code"].as_str().unwrap();
    assert_eq!(code.len(), 8);
    assert_eq!(json["original_url"], "https://www.rust-lang.org");
    assert!(json["short_url"].as_str().unwrap().contains(code));
}

#[tokio::test]
async fn test_redirect_to_original() {
    let (base, _server) = setup().await;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let create_res = client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "https://example.com/redirect-test" }))
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = create_res.json().await.unwrap();
    let code = json["short_code"].as_str().unwrap();

    let redirect_res = client
        .get(format!("{}/{}", base, code))
        .send()
        .await
        .unwrap();

    assert_eq!(redirect_res.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        redirect_res
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap(),
        "https://example.com/redirect-test"
    );
}

#[tokio::test]
async fn test_click_count_increments() {
    let (base, _server) = setup().await;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let create_res = client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "https://example.com/click-test" }))
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = create_res.json().await.unwrap();
    let code = json["short_code"].as_str().unwrap();

    let get_before: serde_json::Value = client
        .get(format!("{}/api/links/{}", base, code))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(get_before["click_count"].as_i64().unwrap(), 0);

    for _ in 0..3 {
        client
            .get(format!("{}/{}", base, code))
            .send()
            .await
            .unwrap();
    }

    let get_after: serde_json::Value = client
        .get(format!("{}/api/links/{}", base, code))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(get_after["click_count"].as_i64().unwrap(), 3);
}

#[tokio::test]
async fn test_list_links() {
    let (base, _server) = setup().await;
    let client = Client::new();

    client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "https://example.com/a" }))
        .send()
        .await
        .unwrap();

    client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "https://example.com/b" }))
        .send()
        .await
        .unwrap();

    let list: Vec<serde_json::Value> = client
        .get(format!("{}/api/links", base))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn test_delete_link() {
    let (base, _server) = setup().await;
    let client = Client::new();

    let create_res = client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "https://example.com/delete-test" }))
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = create_res.json().await.unwrap();
    let code = json["short_code"].as_str().unwrap();

    let delete_res = client
        .delete(format!("{}/api/links/{}", base, code))
        .send()
        .await
        .unwrap();

    assert_eq!(delete_res.status(), StatusCode::OK);
    let del_json: serde_json::Value = delete_res.json().await.unwrap();
    assert_eq!(del_json["deleted"], true);

    let get_res = client
        .get(format!("{}/api/links/{}", base, code))
        .send()
        .await
        .unwrap();

    assert_eq!(get_res.status(), StatusCode::NOT_FOUND);

    let redirect_res = client
        .get(format!("{}/{}", base, code))
        .send()
        .await
        .unwrap();

    assert_eq!(redirect_res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_missing_short_code_returns_404() {
    let (base, _server) = setup().await;
    let client = Client::new();

    let res = client
        .get(format!("{}/nonexistent123", base))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_invalid_url_returns_400() {
    let (base, _server) = setup().await;
    let client = Client::new();

    let empty_res = client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "" }))
        .send()
        .await
        .unwrap();

    assert_eq!(empty_res.status(), StatusCode::BAD_REQUEST);

    let no_scheme_res = client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "example.com" }))
        .send()
        .await
        .unwrap();

    assert_eq!(no_scheme_res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_health_check() {
    let (base, _server) = setup().await;
    let client = Client::new();

    let res: serde_json::Value = client
        .get(format!("{}/api/health", base))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(res["status"], "ok");
}

#[tokio::test]
async fn test_analytics_stats_endpoint() {
    let (base, _server) = setup().await;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let create_res = client
        .post(format!("{}/api/links", base))
        .json(&serde_json::json!({ "url": "https://example.com/stats-test" }))
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = create_res.json().await.unwrap();
    let code = json["short_code"].as_str().unwrap();

    for i in 0..5 {
        let _ = client
            .get(format!("{}/{}", base, code))
            .header("User-Agent", format!("test-agent-{}", i))
            .send()
            .await;
    }

    let stats: serde_json::Value = client
        .get(format!("{}/api/links/{}/stats", base, code))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(stats["link"]["click_count"].as_i64().unwrap(), 5);
    assert_eq!(stats["events"].as_array().unwrap().len(), 5);
    assert!(stats["clicks_by_hour"].as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_dashboard_returns_html() {
    let (base, _server) = setup().await;
    let client = Client::new();

    let res = client.get(&base).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let content_type = res.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(content_type.contains("text/html"));
    let body = res.text().await.unwrap();
    assert!(body.contains("URL Shortener"));
    assert!(body.contains("<!DOCTYPE html>"));
}

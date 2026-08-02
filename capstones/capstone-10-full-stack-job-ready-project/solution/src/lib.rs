use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{delete, get, post},
    Json, Router,
};
use chrono::Utc;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShortLink {
    pub id: String,
    pub short_code: String,
    pub original_url: String,
    pub created_at: String,
    pub click_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClickEvent {
    pub id: String,
    pub short_code: String,
    pub timestamp: String,
    pub user_agent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLinkRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkResponse {
    pub short_code: String,
    pub original_url: String,
    pub short_url: String,
}

#[derive(Debug, Serialize)]
pub struct LinkStatsResponse {
    pub link: ShortLink,
    pub events: Vec<ClickEvent>,
    pub clicks_by_hour: Vec<(String, i64)>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}

pub struct AppState {
    pub db: SqlitePool,
    pub base_url: String,
}

const MIGRATION: &str = r#"
CREATE TABLE IF NOT EXISTS short_links (
    id TEXT PRIMARY KEY,
    short_code TEXT NOT NULL UNIQUE,
    original_url TEXT NOT NULL,
    created_at TEXT NOT NULL,
    click_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS click_events (
    id TEXT PRIMARY KEY,
    short_code TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    user_agent TEXT
);

CREATE INDEX IF NOT EXISTS idx_click_events_short_code ON click_events(short_code);
CREATE INDEX IF NOT EXISTS idx_short_links_short_code ON short_links(short_code);
"#;

pub async fn init_db(database_url: &str) -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query(MIGRATION)
        .execute(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

fn generate_short_code() -> String {
    nanoid!(8)
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

fn error_response(status: StatusCode, message: &str) -> Response {
    let body = Json(ApiError {
        error: message.to_string(),
    });
    (status, body).into_response()
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn create_link(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateLinkRequest>,
) -> Response {
    let url = payload.url.trim().to_string();

    if url.is_empty() || !is_valid_url(&url) {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Invalid URL. Must start with http:// or https://",
        );
    }

    let id = Uuid::new_v4().to_string();
    let short_code = generate_short_code();
    let now = Utc::now().to_rfc3339();

    eprintln!(
        "create_link: inserting short_code={:?}, url={:?}",
        short_code, url
    );

    let result = sqlx::query(
        "INSERT INTO short_links (id, short_code, original_url, created_at, click_count) VALUES (?, ?, ?, ?, 0)",
    )
    .bind(&id)
    .bind(&short_code)
    .bind(&url)
    .bind(&now)
    .execute(&state.db)
    .await;

    eprintln!(
        "create_link: insert result={:?}",
        result
            .as_ref()
            .map(|r| format!("rows={}", r.rows_affected()))
    );

    match result {
        Ok(_) => {
            let short_url = format!("{}/{}", state.base_url.trim_end_matches('/'), short_code);
            let response = CreateLinkResponse {
                short_code,
                original_url: url,
                short_url,
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            if e.to_string().contains("UNIQUE") {
                error_response(
                    StatusCode::CONFLICT,
                    "Short code collision, please try again",
                )
            } else {
                error_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create link")
            }
        }
    }
}

async fn list_links(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let links = sqlx::query_as::<_, ShortLink>(
        "SELECT id, short_code, original_url, created_at, click_count FROM short_links ORDER BY created_at DESC",
    )
    .fetch_all(&state.db)
    .await;

    match links {
        Ok(links) => Json(links).into_response(),
        Err(_) => error_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch links")
            .into_response(),
    }
}

async fn get_link(State(state): State<Arc<AppState>>, Path(code): Path<String>) -> Response {
    let link = sqlx::query_as::<_, ShortLink>(
        "SELECT id, short_code, original_url, created_at, click_count FROM short_links WHERE short_code = ?",
    )
    .bind(&code)
    .fetch_optional(&state.db)
    .await;

    match link {
        Ok(Some(link)) => Json(link).into_response(),
        Ok(None) => error_response(StatusCode::NOT_FOUND, "Link not found"),
        Err(_) => error_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch link"),
    }
}

async fn get_link_stats(State(state): State<Arc<AppState>>, Path(code): Path<String>) -> Response {
    let link = sqlx::query_as::<_, ShortLink>(
        "SELECT id, short_code, original_url, created_at, click_count FROM short_links WHERE short_code = ?",
    )
    .bind(&code)
    .fetch_optional(&state.db)
    .await;

    let link = match link {
        Ok(Some(link)) => link,
        Ok(None) => return error_response(StatusCode::NOT_FOUND, "Link not found"),
        Err(_) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch link"),
    };

    let events = sqlx::query_as::<_, ClickEvent>(
        "SELECT id, short_code, timestamp, user_agent FROM click_events WHERE short_code = ? ORDER BY timestamp DESC LIMIT 100",
    )
    .bind(&code)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let clicks_by_hour = sqlx::query_as::<_, (String, i64)>(
        "SELECT strftime('%Y-%m-%d %H:00:00', timestamp) as hour, COUNT(*) as count \
         FROM click_events WHERE short_code = ? GROUP BY hour ORDER BY hour DESC LIMIT 24",
    )
    .bind(&code)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let response = LinkStatsResponse {
        link,
        events,
        clicks_by_hour,
    };

    Json(response).into_response()
}

async fn delete_link(State(state): State<Arc<AppState>>, Path(code): Path<String>) -> Response {
    let _ = sqlx::query("DELETE FROM click_events WHERE short_code = ?")
        .bind(&code)
        .execute(&state.db)
        .await;

    let result = sqlx::query("DELETE FROM short_links WHERE short_code = ?")
        .bind(&code)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            Json(serde_json::json!({ "deleted": true })).into_response()
        }
        Ok(_) => error_response(StatusCode::NOT_FOUND, "Link not found"),
        Err(_) => error_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete link"),
    }
}

async fn redirect_to_original(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
    headers: HeaderMap,
) -> Response {
    eprintln!("redirect_to_original called with code: {:?}", code);

    let link = sqlx::query_as::<_, ShortLink>(
        "SELECT id, short_code, original_url, created_at, click_count FROM short_links WHERE short_code = ?",
    )
    .bind(&code)
    .fetch_optional(&state.db)
    .await;

    eprintln!(
        "DB query result: {:?}",
        link.as_ref()
            .ok()
            .map(|o| o.as_ref().map(|l| l.short_code.as_str()))
    );

    let link = match link {
        Ok(Some(link)) => link,
        Ok(None) => {
            eprintln!("Link not found in DB for code: {}", code);
            return error_response(StatusCode::NOT_FOUND, "Link not found");
        }
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            return error_response(StatusCode::INTERNAL_SERVER_ERROR, "Database error");
        }
    };

    let click_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let _ = sqlx::query(
        "INSERT INTO click_events (id, short_code, timestamp, user_agent) VALUES (?, ?, ?, ?)",
    )
    .bind(&click_id)
    .bind(&code)
    .bind(&now)
    .bind(&user_agent)
    .execute(&state.db)
    .await;

    let _ =
        sqlx::query("UPDATE short_links SET click_count = click_count + 1 WHERE short_code = ?")
            .bind(&code)
            .execute(&state.db)
            .await;

    Redirect::temporary(&link.original_url).into_response()
}

const DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>URL Shortener</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Segoe UI', system-ui, sans-serif; background: #f0f2f5; color: #1a1a2e; }
        .container { max-width: 900px; margin: 0 auto; padding: 2rem; }
        h1 { text-align: center; margin-bottom: 2rem; color: #16213e; }
        .card { background: white; border-radius: 12px; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 2px 8px rgba(0,0,0,0.08); }
        .card h2 { margin-bottom: 1rem; color: #0f3460; }
        .form-group { display: flex; gap: 0.5rem; }
        .form-group input { flex: 1; padding: 0.75rem; border: 2px solid #e0e0e0; border-radius: 8px; font-size: 1rem; }
        .form-group input:focus { outline: none; border-color: #0f3460; }
        .form-group button { padding: 0.75rem 1.5rem; background: #0f3460; color: white; border: none; border-radius: 8px; font-size: 1rem; cursor: pointer; }
        .form-group button:hover { background: #16213e; }
        table { width: 100%; border-collapse: collapse; }
        th, td { padding: 0.75rem; text-align: left; border-bottom: 1px solid #eee; }
        th { color: #666; font-weight: 600; font-size: 0.85rem; text-transform: uppercase; }
        td a { color: #0f3460; text-decoration: none; }
        .stats { font-size: 0.9rem; color: #666; }
    </style>
</head>
<body>
    <div class="container">
        <h1>URL Shortener</h1>
        <div id="message" class="message" style="display:none;padding:0.75rem;border-radius:8px;margin-bottom:1rem;"></div>
        <div class="card">
            <h2>Create Short Link</h2>
            <form id="create-form" class="form-group">
                <input type="url" id="url-input" placeholder="https://example.com/very/long/url" required>
                <button type="submit">Shorten</button>
            </form>
        </div>
        <div class="card">
            <h2>My Links</h2>
            <div id="links-container"><div class="empty-state" style="text-align:center;padding:2rem;color:#999;">Loading...</div></div>
        </div>
    </div>
    <script>
        const API = '/api/links';
        async function loadLinks() {
            const c = document.getElementById('links-container');
            try {
                const links = await (await fetch(API)).json();
                if (!links.length) { c.innerHTML = '<div style="text-align:center;padding:2rem;color:#999;">No links yet.</div>'; return; }
                const o = window.location.origin;
                let h = '<table><thead><tr><th>Short URL</th><th>Original</th><th>Clicks</th><th>Actions</th></tr></thead><tbody>';
                for (const l of links) {
                    const su = o + '/' + l.short_code;
                    h += `<tr><td><a href="${su}" target="_blank">/${l.short_code}</a></td><td><a href="${l.original_url}" target="_blank" style="max-width:300px;display:inline-block;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">${l.original_url}</a></td><td><span class="stats">${l.click_count}</span></td><td><button onclick="navigator.clipboard.writeText('${su}').then(()=>msg('Copied!'))" style="border:1px solid #ddd;padding:0.25rem 0.5rem;border-radius:4px;cursor:pointer;font-size:0.8rem;">Copy</button> <button onclick="showStats('${l.short_code}')" style="border:1px solid #ddd;padding:0.25rem 0.5rem;border-radius:4px;cursor:pointer;font-size:0.8rem;">Stats</button> <button onclick="del('${l.short_code}')" style="border:1px solid #e74c3c;color:#e74c3c;padding:0.25rem 0.5rem;border-radius:4px;cursor:pointer;font-size:0.8rem;">Del</button></td></tr>`;
                }
                h += '</tbody></table>'; c.innerHTML = h;
            } catch(e) { c.innerHTML = '<div style="text-align:center;padding:2rem;color:#999;">Error loading links.</div>'; }
        }
        function msg(t) { const m = document.getElementById('message'); m.textContent = t; m.style.display = 'block'; m.style.background = '#d4edda'; m.style.color = '#155724'; setTimeout(() => m.style.display = 'none', 3000); }
        document.getElementById('create-form').onsubmit = async (e) => {
            e.preventDefault();
            const inp = document.getElementById('url-input'), url = inp.value.trim();
            if (!url) return;
            try {
                const r = await fetch(API, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ url }) });
                const d = await r.json();
                if (r.ok) { msg('Created: ' + d.short_url); inp.value = ''; loadLinks(); }
                else { const m = document.getElementById('message'); m.textContent = d.error || 'Failed'; m.style.display = 'block'; m.style.background = '#f8d7da'; m.style.color = '#721c24'; setTimeout(() => m.style.display = 'none', 3000); }
            } catch(e) { msg('Network error'); }
        };
        async function showStats(code) {
            try {
                const r = await fetch(API + '/' + code + '/stats');
                const d = await r.json();
                let t = 'Stats for /' + code + '\n\nTotal Clicks: ' + d.link.click_count + '\n\nClicks by Hour:\n';
                for (const [h, c] of (d.clicks_by_hour || [])) t += '  ' + h + ': ' + c + '\n';
                alert(t);
            } catch(e) { alert('Failed to load stats'); }
        }
        async function del(code) { if (!confirm('Delete /' + code + '?')) return; await fetch(API + '/' + code, { method: 'DELETE' }); msg('Deleted!'); loadLinks(); }
        loadLinks();
    </script>
</body>
</html>"#;

async fn serve_dashboard() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

async fn fallback_404(uri: axum::http::Uri) -> Response {
    eprintln!("fallback_404: unhandled request -> {}", uri);
    error_response(StatusCode::NOT_FOUND, "Route not found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_db_insert_and_query() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(MIGRATION).execute(&pool).await.unwrap();

        let code = "test1234";
        let url = "https://example.com";
        sqlx::query(
            "INSERT INTO short_links (id, short_code, original_url, created_at, click_count) VALUES (?, ?, ?, ?, 0)",
        )
        .bind("id1")
        .bind(code)
        .bind(url)
        .bind("2024-01-01T00:00:00Z")
        .execute(&pool)
        .await
        .unwrap();

        let link = sqlx::query_as::<_, ShortLink>(
            "SELECT id, short_code, original_url, created_at, click_count FROM short_links WHERE short_code = ?",
        )
        .bind(code)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(link.short_code, code);
        assert_eq!(link.original_url, url);
    }

    #[tokio::test]
    async fn test_redirect_route_direct() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(MIGRATION).execute(&pool).await.unwrap();

        let code = "abcd1234";
        sqlx::query(
            "INSERT INTO short_links (id, short_code, original_url, created_at, click_count) VALUES ('id1', ?, 'https://example.com', '2024-01-01T00:00:00Z', 0)",
        )
        .bind(code)
        .execute(&pool)
        .await
        .unwrap();

        let state = Arc::new(AppState {
            db: pool,
            base_url: "http://localhost".to_string(),
        });

        let app = build_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/{}", code))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        eprintln!("Response status: {}", response.status());
        let loc = response.headers().get("location").cloned();
        eprintln!("Location header: {:?}", loc);
        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    }
}

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(serve_dashboard))
        .route("/api/health", get(health_check))
        .route("/api/links", post(create_link))
        .route("/api/links", get(list_links))
        .route("/api/links/:code", get(get_link))
        .route("/api/links/:code/stats", get(get_link_stats))
        .route("/api/links/:code", delete(delete_link))
        .route("/:code", get(redirect_to_original))
        .fallback(fallback_404)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#![allow(dead_code, unused_imports)]

//! Capstone 10: Full-Stack URL Shortener with Analytics
//!
//! Build a URL shortener with an Axum backend and HTML frontend.
//!
//! ## Data Model
//!
//! - `ShortLink { id, short_code, original_url, created_at, click_count }`
//! - `ClickEvent { id, short_code, timestamp, user_agent }`
//!
//! ## API Routes
//!
//! - `GET    /`                  — Serve the HTML dashboard
//! - `GET    /api/health`        — Health check
//! - `POST   /api/links`         — Create a short link (JSON body: `{ "url": "..." }`)
//! - `GET    /api/links`         — List all links
//! - `GET    /api/links/{code}`  — Get link details
//! - `GET    /api/links/{code}/stats` — Get click analytics
//! - `DELETE /api/links/{code}`  — Delete a link
//! - `GET    /{code}`            — Redirect to original URL (record click)
//!
//! ## Your Tasks
//!
//! 1. Implement `create_link` handler — validate URL, generate code, insert into DB
//! 2. Implement `list_links` handler — SELECT all, return JSON
//! 3. Implement `get_link` handler — SELECT by short_code, 404 if missing
//! 4. Implement `get_link_stats` handler — aggregate clicks by hour
//! 5. Implement `delete_link` handler — delete events + link
//! 6. Implement `redirect_to_original` handler — lookup, record click, redirect
//! 7. Add the remaining routes in `build_router`
//! 8. Run `cargo test -p capstone-10-starter` to verify all tests pass

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
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

// ============================================================================
// Models
// ============================================================================

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

// ============================================================================
// App State
// ============================================================================

pub struct AppState {
    pub db: SqlitePool,
    pub base_url: String,
}

// ============================================================================
// Database
// ============================================================================

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

// ============================================================================
// Helpers (provided — no changes needed)
// ============================================================================

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

// ============================================================================
// Route Handlers
// ============================================================================

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn create_link(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateLinkRequest>,
) -> Response {
    // TODO: Implement create_link handler
    // 1. Validate the URL (use is_valid_url; reject empty or invalid with 400)
    // 2. Generate a unique ID (Uuid::new_v4()) and short code (generate_short_code())
    // 3. Get current time with Utc::now().to_rfc3339()
    // 4. INSERT into short_links (id, short_code, original_url, created_at, click_count=0)
    // 5. Build short_url from state.base_url and short_code
    // 6. Return StatusCode::CREATED with CreateLinkResponse as JSON
    // 7. Handle UNIQUE constraint violations with 409
    let _ = state;
    let _ = payload;
    error_response(StatusCode::NOT_IMPLEMENTED, "TODO: implement create_link")
}

async fn list_links(State(state): State<Arc<AppState>>) -> Response {
    // TODO: Implement list_links handler
    // 1. SELECT all links ordered by created_at DESC using sqlx::query_as::<_, ShortLink>
    // 2. Return the list as JSON
    let _ = state;
    error_response(StatusCode::NOT_IMPLEMENTED, "TODO: implement list_links")
}

async fn get_link(State(state): State<Arc<AppState>>, Path(code): Path<String>) -> Response {
    // TODO: Implement get_link handler
    // 1. SELECT link by short_code using fetch_optional
    // 2. Return 200 with link JSON if found, 404 if not found
    let _ = state;
    let _ = code;
    error_response(StatusCode::NOT_IMPLEMENTED, "TODO: implement get_link")
}

async fn get_link_stats(State(state): State<Arc<AppState>>, Path(code): Path<String>) -> Response {
    // TODO: Implement get_link_stats handler
    // 1. Fetch the ShortLink by code (404 if not found)
    // 2. Fetch ClickEvents for this code (last 100, ordered by timestamp DESC)
    // 3. Aggregate clicks by hour using strftime (last 24 hours)
    // 4. Return LinkStatsResponse as JSON
    let _ = state;
    let _ = code;
    error_response(
        StatusCode::NOT_IMPLEMENTED,
        "TODO: implement get_link_stats",
    )
}

async fn delete_link(State(state): State<Arc<AppState>>, Path(code): Path<String>) -> Response {
    // TODO: Implement delete_link handler
    // 1. Delete click_events for this short_code
    // 2. Delete short_link for this short_code
    // 3. Return { "deleted": true } if a row was deleted, 404 if not found
    let _ = state;
    let _ = code;
    error_response(StatusCode::NOT_IMPLEMENTED, "TODO: implement delete_link")
}

async fn redirect_to_original(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
    headers: HeaderMap,
) -> Response {
    // TODO: Implement redirect_to_original handler
    // 1. Look up the ShortLink by short_code (404 if not found)
    // 2. Record a ClickEvent: generate UUID, timestamp, extract User-Agent from headers
    // 3. Increment click_count on the short_link
    // 4. Return Redirect::temporary(&link.original_url)
    let _ = state;
    let _ = code;
    let _ = headers;
    error_response(
        StatusCode::NOT_IMPLEMENTED,
        "TODO: implement redirect_to_original",
    )
}

// ============================================================================
// HTML Dashboard (provided — no changes needed)
// ============================================================================

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
            <div id="links-container"><div style="text-align:center;padding:2rem;color:#999;">Loading...</div></div>
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

// ============================================================================
// Router
// ============================================================================

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(serve_dashboard))
        .route("/api/health", get(health_check))
        // TODO: Uncomment and implement the remaining routes:
        // .route("/api/links", post(create_link))
        // .route("/api/links", get(list_links))
        // .route("/api/links/{code}", get(get_link))
        // .route("/api/links/{code}/stats", get(get_link_stats))
        // .route("/api/links/{code}", delete(delete_link))
        // .route("/{code}", get(redirect_to_original))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

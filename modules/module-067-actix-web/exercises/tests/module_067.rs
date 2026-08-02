//! Integration tests for module 067. These exercise the actix-web service
//! via `actix_web::test` — no sockets, no running server.

use actix_web::{test, web, App};
use serde_json::{json, Value};
use std::sync::atomic::AtomicUsize;

use module_067_exercises::{configure_app, AppState};

fn app_data() -> web::Data<AppState> {
    web::Data::new(AppState {
        counter: AtomicUsize::new(0),
    })
}

#[actix_web::test]
async fn hello_returns_200() {
    let app = test::init_service(App::new().configure(configure_app).app_data(app_data())).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let body = test::read_body(resp).await;
    assert_eq!(
        String::from_utf8(body.to_vec()).unwrap(),
        "Hello from actix-web!"
    );
}

#[actix_web::test]
async fn hello_name_captures_path() {
    let app = test::init_service(App::new().configure(configure_app).app_data(app_data())).await;
    let req = test::TestRequest::get().uri("/hello/world").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let body = test::read_body(resp).await;
    assert_eq!(String::from_utf8(body.to_vec()).unwrap(), "Hello, world!");
}

#[actix_web::test]
async fn create_item_returns_201_with_id() {
    let app = test::init_service(App::new().configure(configure_app).app_data(app_data())).await;
    let req = test::TestRequest::post()
        .uri("/items")
        .set_json(&json!({"name": "buy milk"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);
    let body: Value = test::try_read_body_json(resp).await.unwrap();
    assert_eq!(body["id"], 1);
    assert_eq!(body["name"], "buy milk");
}

#[actix_web::test]
async fn ids_increment_across_creates() {
    let app = test::init_service(App::new().configure(configure_app).app_data(app_data())).await;
    let req1 = test::TestRequest::post()
        .uri("/items")
        .set_json(&json!({"name": "one"}))
        .to_request();
    let resp1 = test::call_service(&app, req1).await;
    let first: Value = test::try_read_body_json(resp1).await.unwrap();

    let req2 = test::TestRequest::post()
        .uri("/items")
        .set_json(&json!({"name": "two"}))
        .to_request();
    let resp2 = test::call_service(&app, req2).await;
    let second: Value = test::try_read_body_json(resp2).await.unwrap();
    assert_eq!(first["id"], 1);
    assert_eq!(second["id"], 2);
}

#[actix_web::test]
async fn item_count_tracks_creates() {
    let app = test::init_service(App::new().configure(configure_app).app_data(app_data())).await;
    let req = test::TestRequest::post()
        .uri("/items")
        .set_json(&json!({"name": "one"}))
        .to_request();
    let _ = test::call_service(&app, req).await;
    let req = test::TestRequest::post()
        .uri("/items")
        .set_json(&json!({"name": "two"}))
        .to_request();
    let _ = test::call_service(&app, req).await;

    let req = test::TestRequest::get().uri("/items/count").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let body = test::read_body(resp).await;
    assert_eq!(String::from_utf8(body.to_vec()).unwrap(), "2");
}

#[actix_web::test]
async fn unknown_route_returns_404() {
    let app = test::init_service(App::new().configure(configure_app).app_data(app_data())).await;
    let req = test::TestRequest::get().uri("/nope").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
}

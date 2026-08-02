//! Integration tests for module 061. These define "done": a correct
//! minimal HTTP server must pass them. The tests talk to a real server
//! over a real TCP socket (bound to an ephemeral port), exactly like a
//! client would.

use std::io::{Read, Write};
use std::net::TcpStream;

use module_061_exercises::{build_response, parse_request_line, HttpServer};

/// Sends a raw HTTP request to the server and returns the raw response.
fn request(server: &HttpServer, raw: &str) -> String {
    let mut stream =
        TcpStream::connect(("127.0.0.1", server.port())).expect("connect to test server");
    stream.write_all(raw.as_bytes()).expect("write request");
    let mut response = Vec::new();
    stream.read_to_end(&mut response).expect("read response");
    String::from_utf8_lossy(&response).into_owned()
}

/// Sends a `GET` request to the given path with HTTP/1.1 framing.
fn get(server: &HttpServer, path: &str) -> String {
    request(
        server,
        &format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"),
    )
}

fn status_line(response: &str) -> &str {
    response.lines().next().unwrap_or_default()
}

fn body(response: &str) -> &str {
    response.split("\r\n\r\n").nth(1).unwrap_or_default()
}

fn header<'a>(response: &'a str, name: &str) -> Option<&'a str> {
    response.lines().find_map(|line| {
        line.split_once(':')
            .filter(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.trim())
    })
}

#[test]
fn root_returns_200_with_hello_body() {
    let server = HttpServer::start().expect("start server");
    let response = get(&server, "/");
    assert_eq!(status_line(&response), "HTTP/1.1 200 OK");
    assert!(body(&response).contains("Hello from Rust.Stack"));
}

#[test]
fn echo_returns_the_requested_text() {
    let server = HttpServer::start().expect("start server");
    let response = get(&server, "/echo/hello-world");
    assert_eq!(status_line(&response), "HTTP/1.1 200 OK");
    assert_eq!(body(&response), "hello-world");
}

#[test]
fn echo_keeps_slashes_in_the_target() {
    let server = HttpServer::start().expect("start server");
    let response = get(&server, "/echo/a/b/c");
    assert_eq!(body(&response), "a/b/c");
}

#[test]
fn unknown_path_returns_404() {
    let server = HttpServer::start().expect("start server");
    let response = get(&server, "/does-not-exist");
    assert_eq!(status_line(&response), "HTTP/1.1 404 Not Found");
}

#[test]
fn unsupported_method_returns_405() {
    let server = HttpServer::start().expect("start server");
    let response = request(
        &server,
        "POST / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    );
    assert_eq!(status_line(&response), "HTTP/1.1 405 Method Not Allowed");
}

#[test]
fn malformed_request_returns_400() {
    let server = HttpServer::start().expect("start server");
    let response = request(&server, "GARBAGE\r\n\r\n");
    assert_eq!(status_line(&response), "HTTP/1.1 400 Bad Request");
}

#[test]
fn content_length_matches_body_size() {
    let server = HttpServer::start().expect("start server");
    let response = get(&server, "/echo/rust");
    let response_body = body(&response);
    let length: usize = header(&response, "Content-Length")
        .expect("Content-Length header present")
        .parse()
        .expect("Content-Length is numeric");
    assert_eq!(length, response_body.len());
    assert_eq!(response_body, "rust");
}

#[test]
fn parse_request_line_splits_method_and_target() {
    assert_eq!(
        parse_request_line("GET /echo/rust HTTP/1.1"),
        Some(("GET", "/echo/rust"))
    );
    assert_eq!(parse_request_line("not-a-request"), None);
}

#[test]
fn build_response_writes_status_and_length() {
    let response = build_response("200 OK", "hi");
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Length: 2"));
    assert!(response.ends_with("\r\n\r\nhi"));
}

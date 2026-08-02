//! Module 061: HTTP & Web Fundamentals.
//!
//! A minimal HTTP/1.1 server built directly on [`std::net::TcpListener`]:
//! accept a TCP connection, read raw request bytes, route them, and write
//! raw response bytes back. No framework involved — this is essentially
//! what the `hyper` crate does for real web servers, and what axum
//! (Module 062) builds on top of.

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// A tiny single-threaded HTTP/1.1 server.
pub struct HttpServer {
    listener: TcpListener,
}

impl HttpServer {
    /// Binds a server to an ephemeral port (`127.0.0.1:0`) and starts
    /// accepting connections on a background thread.
    pub fn start() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let acceptor = listener.try_clone()?;
        thread::spawn(move || {
            for stream in acceptor.incoming() {
                match stream {
                    Ok(mut stream) => {
                        // A malformed request must not take the whole
                        // server down: respond with an error and move on.
                        let _ = handle_connection(&mut stream);
                    }
                    Err(_) => break,
                }
            }
        });
        Ok(Self { listener })
    }

    /// The TCP port the server is listening on.
    pub fn port(&self) -> u16 {
        self.listener
            .local_addr()
            .expect("bound listener has a local address")
            .port()
    }
}

/// Reads a raw HTTP request from the stream, up to and including the
/// blank line (`\r\n\r\n`) that terminates the header section.
pub fn read_request(stream: &mut TcpStream) -> io::Result<String> {
    let mut bytes = Vec::new();
    let mut buf = [0u8; 1];
    loop {
        if stream.read(&mut buf)? == 0 {
            break;
        }
        bytes.push(buf[0]);
        if bytes.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// Splits an HTTP request line (`METHOD TARGET HTTP/1.1`) into its method
/// and target. Returns `None` for a malformed line.
pub fn parse_request_line(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.split_whitespace();
    let method = parts.next()?;
    let target = parts.next()?;
    Some((method, target))
}

/// Routes a request to a response. Returns the status line (e.g. `"200 OK"`)
/// and the response body.
pub fn route(method: &str, target: &str) -> (String, String) {
    if method != "GET" {
        return (
            "405 Method Not Allowed".to_string(),
            "Method Not Allowed".to_string(),
        );
    }
    match target {
        "/" => ("200 OK".to_string(), "Hello from Rust.Stack!".to_string()),
        _ => match target.strip_prefix("/echo/") {
            Some(text) => ("200 OK".to_string(), text.to_string()),
            None => ("404 Not Found".to_string(), "Not Found".to_string()),
        },
    }
}

/// Wraps a status line and body into a complete HTTP/1.1 response with a
/// correct `Content-Length` header.
pub fn build_response(status: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

/// Handles a single client connection: read the request, produce a
/// response, and write it back.
pub fn handle_connection(stream: &mut TcpStream) -> io::Result<()> {
    let request = read_request(stream)?;
    let (status, body) = match request.lines().next().and_then(parse_request_line) {
        Some((method, target)) => route(method, target),
        None => ("400 Bad Request".to_string(), "Bad Request".to_string()),
    };
    let response = build_response(&status, &body);
    stream.write_all(response.as_bytes())?;
    stream.flush()
}

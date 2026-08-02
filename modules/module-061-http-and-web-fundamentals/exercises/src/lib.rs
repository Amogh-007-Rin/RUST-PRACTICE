//! Module 061: HTTP & Web Fundamentals — exercise scaffold.
//!
//! A minimal HTTP/1.1 server built directly on [`std::net::TcpListener`].
//! The server itself is complete; your job is to implement the HTTP
//! semantics: parsing the request line, routing, and building responses.
//!
//! Find the `// TODO(module-061)` comments below and fill them in until
//! `cargo test -p module-061-exercises` passes.

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
    // TODO(module-061): Split `line` on whitespace and return the first two
    // words as (method, target). Return `None` if either is missing.
    panic!("not implemented: parse_request_line({line:?})")
}

/// Routes a request to a response. Returns the status line (e.g. `"200 OK"`)
/// and the response body.
pub fn route(method: &str, target: &str) -> (String, String) {
    // TODO(module-061): Implement routing:
    // - any method other than GET -> ("405 Method Not Allowed", "Method Not Allowed")
    // - GET "/"                        -> ("200 OK", "Hello from Rust.Stack!")
    // - GET "/echo/<text>"             -> ("200 OK", <text>)
    // - anything else                  -> ("404 Not Found", "Not Found")
    panic!("not implemented: route({method:?}, {target:?})")
}

/// Wraps a status line and body into a complete HTTP/1.1 response with a
/// correct `Content-Length` header.
pub fn build_response(status: &str, body: &str) -> String {
    // TODO(module-061): Format an HTTP/1.1 response with the status line,
    // a Content-Type header, a Content-Length header matching `body.len()`,
    // and the body after the mandatory blank line. Use `\r\n` line endings.
    panic!("not implemented: build_response({status:?}, {body:?})")
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

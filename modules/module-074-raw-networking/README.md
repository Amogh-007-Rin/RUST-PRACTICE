# Module 074: Raw Networking

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 60–90 min
**Prerequisites:** Module 031 (Threads), familiarity with `std::io::Read`/`Write`

## Learning Objectives
- Create a TCP server and client using `std::net::{TcpListener, TcpStream}`
- Understand the client-server model at the socket level
- Design and parse a simple wire protocol with length-delimited framing
- Use threads to handle server connections
- Write testable networking code (bind to port 0, use localhost)

## Why This Matters
Every networked Rust application — from `axum` web servers to `tonic` gRPC services — is built on top of TCP sockets. Frameworks abstract the socket layer away, but understanding what happens underneath is essential when debugging connection issues, designing custom protocols, or working with systems that don't fit the standard HTTP/gRPC mould. This is the foundation Block H's capstone (a distributed key-value store) rests on.

## Concept

Before HTTP, before gRPC, before any application-layer protocol, there are sockets. A socket is an endpoint for sending and receiving data across a network. Rust's standard library provides `TcpListener` (for servers) and `TcpStream` (for clients) in `std::net`. Together they form the lowest-level building block of networked Rust code.

### The TCP server loop

A TCP server binds to an address, listens for incoming connections, and handles each one — typically in a separate thread or async task:

```rust
use std::net::TcpListener;
use std::io::{Read, Write};

let listener = TcpListener::bind("127.0.0.1:8080").expect("bind failed");

for stream in listener.incoming() {
    let mut stream = stream.expect("accept failed");
    std::thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match stream.read(&mut buf) {
                Ok(0) => break, // EOF — client disconnected
                Ok(n) => { let _ = stream.write_all(&buf[..n]); }
                Err(_) => break,
            }
        }
    });
}
```

Key details:
- `TcpListener::bind("127.0.0.1:8080")` reserves the port. Using port `0` tells the OS to assign a free port — critical for tests, so they don't conflict.
- `.incoming()` returns an iterator of connection attempts. Each `Ok(stream)` is a `TcpStream` representing a single client connection.
- `stream.read(&mut buf)` returns the number of bytes read, or `0` when the client closes the connection.
- `stream.write_all(&buf[..n])` echoes the bytes back.

### The TCP client

The client side is simpler: connect, send, read:

```rust
use std::net::TcpStream;
use std::io::{Read, Write};

let mut stream = TcpStream::connect("127.0.0.1:8080").expect("connect failed");
stream.write_all(b"hello").expect("write failed");

let mut buf = [0u8; 5];
stream.read_exact(&mut buf).expect("read failed");
assert_eq!(&buf, b"hello");
```

`read_exact` reads exactly the number of bytes needed to fill the buffer — it blocks until that many bytes arrive or the connection closes.

### Port 0 and testing

When writing tests for networking code, you can't hardcode a port number — another test or process might already be using it. The solution: bind to port `0`, which tells the OS "give me any free port." The server's `local_addr()` method returns the actual assigned address:

```rust
let listener = TcpListener::bind("127.0.0.1:0").unwrap();
let addr = listener.local_addr().unwrap();
println!("Listening on {}", addr); // e.g. 127.0.0.1:57432
```

The test starts the server on port 0, reads the assigned address, and gives it to the client. No port conflicts.

### Designing a wire protocol

TCP is a byte stream. When you call `write_all(b"hello")` followed by `write_all(b"world")`, the receiver might get `hello world` in one `read` call, or `hel` in one and `lo world` in the next. TCP guarantees delivery and ordering but not message boundaries.

A wire protocol defines where one message ends and the next begins. The simplest approach is **length-delimited framing**:

```
[1 byte: payload length N] [N bytes: payload]
```

Parsing this is straightforward:

```rust
fn parse_message(data: &[u8]) -> Option<(u8, Vec<u8>)> {
    let len = *data.first()? as usize;
    if data.len() < 1 + len {
        return None; // incomplete message
    }
    let payload = data[1..1 + len].to_vec();
    Some((len as u8, payload))
}
```

Real protocols use this pattern extensively: HTTP uses `Content-Length`, gRPC uses a 5-byte length prefix, and Redis uses `$<length>\r\n<data>\r\n`.

### Blocking vs non-blocking I/O

`std::net` sockets are **blocking**: `read` and `write` block the calling thread until the operation completes. This is fine for one-connection-at-a-time servers or when you spawn a thread per connection. For high-concurrency servers (thousands of connections), async I/O with Tokio (Modules 041-049) is a better fit — but the socket concepts are identical.

### Putting it together: echo server lifecycle

```
Server thread                           Client
-----------                             ------
bind 127.0.0.1:0 →
local_addr = 127.0.0.1:54321
spawn thread
accept() blocks ───────── connect to 127.0.0.1:54321 ──────→
                                write_all("hello")
accept returns stream ──────────────────────────────────────→
read() → "hello"
write_all("hello") ─────────────────────────────────────────→
                                read_exact() → "hello"
accept() → Err (listener dropped, thread exits)
```

### Common pitfalls
- **Forgetting to call `local_addr()` before spawning the thread**: the `bind` may succeed but if you spawn the thread first, `local_addr()` might error. Call it on the main thread before spawning.
- **Assuming `read` returns a complete message**: it doesn't. Always check the return value and handle partial reads.
- **Not handling the empty-read case**: `read` returning `0` means the peer closed the connection. If you ignore it, you'll spin in an infinite loop.
- **Using `read_to_end` with TCP**: `read_to_end` reads until EOF, which only happens when the remote end closes the connection. If the server is waiting for a response before closing, you get a deadlock.

## Common Pitfalls
- **Hardcoding ports in tests**: use port `0` and `local_addr()` to avoid conflicts.
- **Expecting one `read` per `write`**: TCP is a stream. Use length-delimited framing or a higher-level protocol.
- **Not joining server threads in tests**: if you don't join, the test finishes before the server accepts, causing flaky client `connect` failures.
- **Ignoring `read` return value of `0`**: means the peer disconnected. Handle it.

## Key Terms
- **Socket**: an endpoint for network communication (IP + port)
- **TcpListener**: a server-side socket that listens for incoming connections
- **TcpStream**: a bidirectional connection between two sockets
- **Length-delimited framing**: a wire format where each message is prefixed with its byte length
- **Port 0**: an ephemeral port assignment — the OS picks any available port

## Exercise

In `exercises/`, fill in the `TODO(module-074)` markers to:

1. **`start_echo_server`**: Bind a `TcpListener`, spawn a thread that accepts one connection, reads data, echoes it back.
2. **`send_echo_message`**: Connect to the server, send a message, read the response back.
3. **`parse_protocol_message`**: Parse a length-delimited message from raw bytes.

Run `cargo test -p module-074-exercises` to verify.

## Further Reading
- [std::net::TcpListener docs](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
- [std::net::TcpStream docs](https://doc.rust-lang.org/std/net/struct.TcpStream.html)
- [Beej's Guide to Network Programming](https://beej.us/guide/bgnet/) (C-focused but the concepts are universal)

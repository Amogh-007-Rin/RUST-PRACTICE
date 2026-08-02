# Module 061: HTTP & Web Fundamentals in Rust

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 033 (channels/message passing), Module 041–043 (async fundamentals, tokio runtime, async I/O). Comfort reading socket-level I/O.

## Learning Objectives

- Read a raw HTTP/1.1 request off a TCP socket: request line, headers, and the blank-line terminator.
- Write a valid HTTP/1.1 response with the correct status line, headers, and `Content-Length`.
- Route requests to handlers by method and target, mapping unknown paths to `404` and unsupported methods to `405`.
- Explain what `hyper` is, what a web framework is, and where a framework like axum sits relative to `hyper`.
- Test a network server with a real `TcpStream` client on an ephemeral port.

## Why This Matters

Every Rust backend framework you will touch — axum, actix-web, warp — is a thin, ergonomic layer over `hyper`, which is itself a thin, fast layer over TCP sockets. When you call `app.await` on a router in Module 062, you are watching the same request/response dance this module implements by hand, just done for you. Debugging "why is my response missing `Content-Length`" or "why did the client hang" requires knowing exactly what the bytes on the wire look like — which is precisely what you build in this module's exercise.

## Concept

### The contract on the wire

HTTP is a text protocol spoken over a byte stream (TCP). When a client sends a request, it writes a specific sequence of bytes; a server replies with another specific sequence. There is no magic — it is a conversation protocol with a defined grammar. HTTP/1.1's grammar looks like this:

```
Request:                    Response:
GET /echo/rust HTTP/1.1     HTTP/1.1 200 OK
Host: localhost             Content-Type: text/plain; charset=utf-8
Connection: close           Content-Length: 4
                            Connection: close
<blank line>                <blank line>
                            rust
```

Two structural facts matter more than any header:

1. **Lines are terminated with `\r\n`** (carriage return + line feed), not just `\n`. A parser that accepts `\n` only will fail against real clients, which send `\r\n` per the spec.
2. **The header section ends with a completely empty line** — the sequence `\r\n\r\n`. That's how the reader knows the headers are done and the body (if any) begins.

The full exchange, end to end:

```
┌──────────┐  connect  ┌──────────┐
│  Client  │──────────►│  Server  │
└──────────┘           └────┬─────┘
                            │ accept() → socket
    ┌───────────────────────┼──────────────────────┐
    │ 1. read until "\r\n\r\n"                     │
    │ 2. parse request line: METHOD TARGET VER     │
    │ 3. look up headers (Host, Content-Length...) │
    │ 4. route (method, target) → (status, body)   │
    │ 5. write status line + headers + blank line  │
    │    + body; close the connection              │
    └──────────────────────────────────────────────┘
    │ response bytes flow back
◄───┘
Client reads until the connection closes (Connection: close)
```

### The request line

The first line of a request — the *request line* — carries the verb, the target, and the protocol version:

```
POST /api/todos/42 HTTP/1.1
^^^  ^^^^^^^^^^^^^  ^^^^^^^
verb     target      version
```

Parsing it is a whitespace split on the first line. The verb (method) tells the server what the client wants to do — `GET` fetch, `POST` create, `PUT`/`PATCH` update, `DELETE` remove. The target is what it wants to do it to. That (method, target) pair is the entire routing key for the simplest servers, and it's still the fundamental key for the most complex frameworks — axum just matches more sophisticated target patterns (as you'll see in Module 062 with routes like `/todos/{id}`).

### The response

A response is a *status line*, headers, a blank line, and a body:

- `HTTP/1.1 200 OK` — version, three-digit status code, and a human-readable reason phrase. The code is what clients and programs actually branch on: `2xx` success, `3xx` redirect, `4xx` client error (your fault — `400` malformed, `404` not found, `405` method not allowed, `422` unprocessable), `5xx` server error.
- Headers — one per line, `Name: value`. At a minimum, a response must send `Content-Length: N`, telling the client exactly how many bytes of body follow. Get `N` wrong and the client will hang waiting for body bytes that never come, or truncate a body it should have fully received.
- A blank line, then the raw body bytes.

### What is `hyper`, then?

`hyper` is the de-facto standard Rust HTTP implementation: it parses the bytes of requests, formats responses, handles connection pooling, keep-alive, chunked transfer encoding, and all the corner cases of HTTP/1.1 and HTTP/2 that a hand-rolled server like the one in this module will never see. It is what production Rust services use "underneath."

Using `hyper` directly works but is verbose. A minimal server with it looks like:

```rust,ignore
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming, Request, Response};

async fn handle(_req: Request<Incoming>) -> Result<Response<String>, hyper::Error> {
    Ok(Response::new("Hello, hyper!".to_string()))
}

// ...per connection, this would be spawned as a task:
// http1::Builder::new().serve_connection(stream, service_fn(handle)).await?;
```

Notice what's still missing: path matching, extraction of query strings or JSON bodies, shared state, error-to-status mapping. That's precisely the gap frameworks fill. Axum (Module 062) is built on `hyper` and `tower`: it hands you `Router`, handlers, and extractors, and compiles down to a `hyper` server under the hood. In a sense, everything in the rest of Block G is a power tool bolted onto the raw conversation you just built by hand.

There's a good reason to have built it by hand first: when something goes wrong at the socket level — a hung connection, a missing header, a client sending `\n` instead of `\r\n` — you now know exactly where to look in the byte stream.

### Content-Length is a promise

The single most important correctness rule in this module: **`Content-Length` must equal the byte length of the body you write**. For text, `body.len()` is the right number. (Note it's bytes, not characters — a non-ASCII body makes those differ, which is why `String::len` is used rather than a "character count" method.) Sending a wrong length is a protocol violation: clients will either read too few bytes and report a truncated body, or block forever waiting for bytes that were never promised.

### Why status codes and routing matter

A web server's job decomposes into a small pipeline: *parse → route → execute → format*. Each stage fails independently, and each failure mode has a canonical status code:

| What went wrong | Status |
|---|---|
| Request can't be parsed at all | `400 Bad Request` |
| Verb not supported for any route | `405 Method Not Allowed` |
| Target matches no route | `404 Not Found` |
| Everything worked | `200 OK` |

This module's exercise implements exactly this pipeline, and the tests check each of the four rows.

## Common Pitfalls

- **Using `\n` instead of `\r\n`.** Real HTTP clients terminate lines with `\r\n`. If your server only handles `\n`, or your response uses bare `\n`, real browsers and `curl` will misbehave. Always write `\r\n`.
- **Forgetting the blank line.** Headers end with an empty line; without it the client keeps waiting for more headers and never sees the body. The wire sequence is `...header\r\n\r\nbody`.
- **Wrong `Content-Length`.** It must be the byte length of the body, not a character count, and not "the length of the string in the test you copied." The client trusts it absolutely.
- **Letting one bad connection crash the server.** A malformed request is a client problem, not a reason to panic the accept loop. Handle the error per-connection and keep serving — the code you were given does this with `let _ = handle_connection(...)`.
- **Blocking forever waiting for a body that never arrives.** A naive `read` on a request with no `Content-Length`/`Connection: close` hangs. Reading until `\r\n\r\n` (or EOF) is the safe stopping rule for GET-style requests without a body.

## Key Terms

- **Request line:** the first line of an HTTP request: `METHOD TARGET VERSION`.
- **Status line:** the first line of an HTTP response: `VERSION CODE REASON`.
- **Method (verb):** `GET`, `POST`, `PUT`, `PATCH`, `DELETE` — what the client wants to do.
- **Target:** the path (and query) the request applies to.
- **Header:** a `Name: value` line carrying metadata about the request or response.
- **`Content-Length`:** the byte count of the message body, which the reader uses to know when the body ends.
- **`hyper`:** the Rust crate that implements HTTP/1.1 and HTTP/2 for real; most frameworks, axum included, are built on it.
- **Ephemeral port:** port `0` — the OS picks a free port at bind time; used by tests to avoid collisions.

## Exercise

In `exercises/`, the server, connection handling, and request reading are complete. Three functions are stubbed with `panic!` and marked `// TODO(module-061)`:

1. `parse_request_line` — split a request line into `(method, target)`; return `None` for a malformed line.
2. `route` — map `(method, target)` to `(status_line, body)`: `405` for non-GET, `200` for `/` and `/echo/<text>`, `404` otherwise.
3. `build_response` — assemble a full HTTP/1.1 response with `Content-Type`, a correct `Content-Length`, `Connection: close`, and the body after the blank line.

Run the tests with:

```bash
cargo test -p module-061-exercises
```

The tests in `tests/module_061.rs` start a real server on an ephemeral port and speak raw HTTP to it over `TcpStream` — including the `400`/`404`/`405` cases. When all nine pass, compare with `solutions/`.

## Further Reading

- [RFC 9112: HTTP/1.1 semantics — the message format you implemented](https://www.rfc-editor.org/rfc/rfc9112)
- [MDN: An overview of HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview)
- [The `hyper` crate documentation](https://docs.rs/hyper)
- [Module 043: Async I/O — sockets over tokio instead of std](modules/module-043-async-io/README.md)

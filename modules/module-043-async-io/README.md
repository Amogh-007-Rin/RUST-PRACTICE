# Module 043: Async I/O

**Block:** Block E — Async Rust
**Estimated time:** 60–90 min
**Prerequisites:** Module 042 (runtime, spawn); Module 041 (Future/waker)

## Learning Objectives

- Read and write files asynchronously with `tokio::fs`.
- Copy byte streams with `tokio::io::copy` and buffer async readers correctly.
- Accept TCP connections and speak a simple line protocol over `tokio::net`.
- Explain why async I/O yields the thread instead of parking it, and where the OS event loop comes in.

## Why This Matters

Real async services are I/O-bound: a web server is mostly waiting on sockets and disks, and the entire performance case for async Rust rests on making those waits cheap. `tokio::fs` and `tokio::net` are what `axum`, `sqlx`, and every networking crate sit on top of. Also, the "async read" contract — buffer carefully, never block, flush before close — is exactly the contract you will debug in production proxy and service code.

## Concept

### The one idea: waiting is a yield

In Module 041 you built a `Delay` future: polled, it returns `Pending` and a helper thread wakes it later. Async I/O is the same shape, with the helper replaced by the operating system:

- `tokio::fs::read_to_string(path).await` — the task registers interest with the runtime's I/O driver, returns `Pending`, and the thread moves on to other tasks. When the kernel finishes the read, it fires the registered waker and the task is re-polled with the data.
- `TcpStream::read/write` — identical machinery, just with a socket instead of a file.

There is a `std::io::Error` at the end of it all, by the way: async I/O APIs return `io::Result<T>` just like the blocking ones, and `?` works unchanged. The only real difference from `std::fs`/`std::net` is the `.await` in the middle.

### Files: `tokio::fs`

The file API mirrors `std`:

```rust
use std::path::Path;
use tokio::io;

async fn backup(path: &Path) -> io::Result<u64> {
    let mut reader = tokio::fs::File::open(path).await?;
    let mut writer = tokio::fs::File::create(format!("{}.bak", path.display())).await?;
    tokio::io::copy(&mut reader, &mut writer).await
}
# fn main() {}
```

Three notes. First, the convenience functions (`read_to_string`, `write`) read or write the whole file — perfect for small files like configs. Second, `copy` streams in chunks instead of slurping the file into memory; it returns the number of bytes copied, which is also how you verify a copy is complete. Third — and this matters — `tokio::fs` performs blocking system calls on a **dedicated blocking thread pool** under the hood, then resumes your task when they finish. Disk operations are not really "event-driven" the way sockets are; this is exactly how you will write your own blocking work in Module 049.

### The buffered-read pattern

`read_line` needs somewhere to buffer bytes until a newline arrives, so the standard pattern is to wrap the stream in a `BufReader`:

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

async fn echo_line(stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    if reader.read_line(&mut line).await? > 0 {
        let mut stream = reader.into_inner();
        stream.write_all(line.as_bytes()).await?;
        stream.flush().await?;
    }
    Ok(())
}
# fn main() {}
```

Two details are easy to get wrong. `BufReader` *owns* the stream, so to write back you must recover it with `into_inner()` — the borrow checker will otherwise rightly refuse to let you write to a stream that a reader still owns. And `write_all` does not guarantee the bytes reach the peer's kernel immediately: `flush` is what pushes buffered data out, so flush before you let the stream drop, or the client may read nothing at all.

### Sockets: `tokio::net`

`TcpListener` and `TcpStream` work like their `std` siblings with `.await` inserted:

```rust
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("connection from {addr}");
        tokio::spawn(async move { echo_line(stream).await });
    }
}
```

The pattern to internalize: `accept()` yields while no client is connecting; each connection becomes its own spawned task, so one slow client cannot stall the others. The client side is equally simple: `TcpStream::connect(addr).await?`, then read/write on it.

Where does the waker come from? The runtime's I/O driver asks the kernel "tell me when this socket is readable" (that is what `epoll`/`kqueue`/`IOCP` do), and the kernel's answer fires the task's waker. That integration is the whole reason a Tokio service can hold tens of thousands of open connections on a handful of threads.

### The async I/O flow, end to end

```
  task: TcpStream::read()            runtime thread              kernel
        │                                │                        │
        ▼                                │                        │
   register interest with                │                        │
   the I/O driver (epoll)                │                        │
        │                                │                        │
        ▼                                │                        │
   Poll::Pending ────────────► park the thread;                  │
        │                      run other ready tasks             │
        │                                │                        │
        │                                │◄──── data arrives ─────┘
        │                                │   (epoll fires)
        │◄──────────── wake() ───────────┘
        ▼
   re-poll: bytes available ──► Poll::Ready(data)
```

The task never spins, never sleeps, and never holds a thread while waiting. The thread only ever does useful work: poll a task, let it yield, poll the next.

## Common Pitfalls

- **Using `std::fs`/`std::net` inside async code.** They park the worker thread. When you need blocking APIs, route them through `spawn_blocking` (Module 049).
- **Reading a line with `BufReader` and then writing to the same stream.** The reader owns the stream — recover it with `into_inner()` first.
- **Forgetting `flush()` before the stream drops.** Buffered data may sit in userspace and the peer waits forever.
- **Treating "async file" as faster.** Async I/O is about *many concurrent waits per thread*, not raw speed; for a single file copy, `std::fs` is comparable or faster.
- **Holding a `TcpListener` and accepting only once.** A real server loops on `accept` and spawns each connection.

## Key Terms

- **I/O driver:** the runtime component that registers socket interests with the OS and fires wakers on events.
- **`tokio::io::copy`:** streams bytes between an async reader and writer, returning the count.
- **`BufReader`:** wraps a stream with an internal buffer so line-based reads don't syscall per byte.
- **`into_inner()`:** recover the underlying stream from a `BufReader`/`BufWriter`.
- **Half-open connection:** peer sent data and closed; you must detect EOF (read returns 0 bytes) before the peer's close is visible.

## Exercise

Work in `exercises/` and make `cargo test -p module-043-exercises` pass. Four TODOs in `src/lib.rs`:

1. `write_file_async` / `read_file_async` — one-line `tokio::fs` calls.
2. `copy_file_async` — open both `tokio::fs::File`s and `tokio::io::copy` between them.
3. `echo_line` — the `BufReader`/`read_line`/`into_inner`/`write_all`/`flush` dance from the Concept section.

The tests write and read files in temp directories (including asserting a missing file yields `ErrorKind::NotFound`), verify a copy is byte-exact, and run a real TCP echo round trip: a listener, a spawned echo handler, and a client that sends `hello world\n` and reads it back. Compare with `solutions/` when done.

## Further Reading

- [Tokio docs: tokio::fs](https://docs.rs/tokio/latest/tokio/fs/) and [tokio::net](https://docs.rs/tokio/latest/tokio/net/)
- [Tokio tutorial: Async in depth](https://tokio.rs/tokio/tutorial/async)
- [The Async Book: I/O and state machines](https://rust-lang.github.io/async-book/03_async_await/02_async_await_example.html)

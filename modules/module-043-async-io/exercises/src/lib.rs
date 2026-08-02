//! Module 043: Async I/O — exercise scaffold.
//!
//! Tokio's async file and network APIs. The signatures mirror `std`'s
//! blocking APIs; the difference is that every call yields the current
//! task instead of parking the whole thread while the OS works.

use std::path::Path;
use tokio::io;
use tokio::net::TcpStream;

/// Write `contents` to the file at `path`, asynchronously.
pub async fn write_file_async(_path: &Path, _contents: &str) -> io::Result<()> {
    // TODO(module-043): `tokio::fs::write(path, contents).await`
    panic!("TODO(module-043): implement write_file_async")
}

/// Read the file at `path` into a `String`, asynchronously.
pub async fn read_file_async(_path: &Path) -> io::Result<String> {
    // TODO(module-043): `tokio::fs::read_to_string(path).await`
    panic!("TODO(module-043): implement read_file_async")
}

/// Copy the contents of `from` to `to`, returning the number of bytes
/// copied. Both file handles must be `tokio::fs::File`s.
pub async fn copy_file_async(_from: &Path, _to: &Path) -> io::Result<u64> {
    // TODO(module-043): open `from` for reading and `to` for writing
    // (`tokio::fs::File::open` / `File::create`), then
    // `tokio::io::copy(&mut reader, &mut writer).await`.
    panic!("TODO(module-043): implement copy_file_async")
}

/// Read one line from `stream` and write it straight back, then close the
/// connection. If the peer sends an empty line (EOF), do nothing.
pub async fn echo_line(_stream: TcpStream) -> io::Result<()> {
    // TODO(module-043): wrap the stream in a `tokio::io::BufReader` so
    // `read_line` has somewhere to buffer. Read a line with
    // `AsyncBufReadExt::read_line`. If anything was read, turn the reader
    // back into the stream with `.into_inner()`, write the line back
    // (`AsyncWriteExt::write_all`) and `flush` it. You need both trait
    // imports:
    //   use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    panic!("TODO(module-043): implement echo_line")
}

//! Module 043: Async I/O — reference solution.
//!
//! Async file handling (`tokio::fs`), a byte-stream copy
//! (`tokio::io::copy`), and a line-based TCP echo over
//! `tokio::net` with buffered reads.

use std::path::Path;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

/// Write `contents` to the file at `path`, asynchronously.
pub async fn write_file_async(path: &Path, contents: &str) -> io::Result<()> {
    tokio::fs::write(path, contents).await
}

/// Read the file at `path` into a `String`, asynchronously.
pub async fn read_file_async(path: &Path) -> io::Result<String> {
    tokio::fs::read_to_string(path).await
}

/// Copy the contents of `from` to `to`, returning the number of bytes
/// copied.
pub async fn copy_file_async(from: &Path, to: &Path) -> io::Result<u64> {
    let mut reader = tokio::fs::File::open(from).await?;
    let mut writer = tokio::fs::File::create(to).await?;
    tokio::io::copy(&mut reader, &mut writer).await
}

/// Read one line from `stream` and write it straight back, then close the
/// connection. If the peer sends an empty line (EOF), do nothing.
pub async fn echo_line(stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let read = reader.read_line(&mut line).await?;
    if read > 0 {
        let mut stream = reader.into_inner();
        stream.write_all(line.as_bytes()).await?;
        stream.flush().await?;
    }
    Ok(())
}

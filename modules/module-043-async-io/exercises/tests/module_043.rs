//! Module 043: Async I/O — integration tests.
//!
//! File tests use unique per-test temp directories; the echo test runs a
//! real TCP round trip against a `TcpListener` on `127.0.0.1`.

use module_043_exercises::{copy_file_async, echo_line, read_file_async, write_file_async};

use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("module_043_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[tokio::test]
async fn write_then_read_roundtrips() {
    let dir = temp_dir("roundtrip");
    let path = dir.join("hello.txt");
    write_file_async(&path, "hello async").await.unwrap();
    let contents = read_file_async(&path).await.unwrap();
    assert_eq!(contents, "hello async");
    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn reading_a_missing_file_is_an_error() {
    let dir = temp_dir("missing");
    let err = read_file_async(&dir.join("nope.txt")).await.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn copy_copies_every_byte() {
    let dir = temp_dir("copy");
    let src = dir.join("src.bin");
    let dst = dir.join("dst.bin");
    write_file_async(&src, "payload").await.unwrap();
    let copied = copy_file_async(&src, &dst).await.unwrap();
    assert_eq!(copied, 7);
    assert_eq!(read_file_async(&dst).await.unwrap(), "payload");
    std::fs::remove_dir_all(&dir).ok();
}

#[tokio::test]
async fn echo_line_roundtrips_over_tcp() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _peer) = listener.accept().await.unwrap();
        echo_line(stream).await.unwrap();
    });

    let mut client = tokio::net::TcpStream::connect(addr).await.unwrap();
    client.write_all(b"hello world\n").await.unwrap();
    let mut reply = String::new();
    let bytes = client.read_to_string(&mut reply).await.unwrap();
    assert_eq!(bytes, 12);
    assert_eq!(reply, "hello world\n");
    server.await.unwrap();
}

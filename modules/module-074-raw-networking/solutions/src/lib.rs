//! Module 074: Raw Networking — reference solution.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::JoinHandle;

pub fn start_echo_server(addr: &str) -> (std::net::SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind(addr).expect("failed to bind");
    let local_addr = listener.local_addr().expect("failed to get local addr");
    let handle = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 1024];
            if let Ok(n) = stream.read(&mut buf) {
                let _ = stream.write_all(&buf[..n]);
            }
        }
    });
    (local_addr, handle)
}

pub fn send_echo_message(addr: &str, msg: &str) -> String {
    let mut stream = TcpStream::connect(addr).expect("failed to connect");
    stream.write_all(msg.as_bytes()).expect("failed to write");
    let mut buf = vec![0u8; msg.len()];
    stream.read_exact(&mut buf).expect("failed to read");
    String::from_utf8(buf).expect("invalid utf8")
}

pub fn parse_protocol_message(data: &[u8]) -> Option<(u8, Vec<u8>)> {
    let len = *data.first()? as usize;
    if data.len() < 1 + len {
        return None;
    }
    let payload = data[1..1 + len].to_vec();
    Some((len as u8, payload))
}

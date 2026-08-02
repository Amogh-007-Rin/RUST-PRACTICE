//! Module 074: Raw Networking — exercise scaffold.
//!
//! Implement a TCP echo server, client, and a simple wire protocol parser.

use std::thread::JoinHandle;

/// Spawn a thread running a TCP echo server on the given address (e.g. "127.0.0.1:0").
///
/// The server should accept a single connection, read data, and echo it back.
/// Bind to the address and return the bound address and a JoinHandle.
///
/// Hint: use TcpListener::bind. If the address has port 0, grab the local address
/// with listener.local_addr() so the caller knows the actual port.
pub fn start_echo_server(_addr: &str) -> (std::net::SocketAddr, JoinHandle<()>) {
    // TODO(module-074): bind a TcpListener, get the local address,
    // spawn a thread that accepts one connection, reads into a buffer,
    // writes the buffer back, and exits.
    panic!("TODO(module-074): implement start_echo_server")
}

/// Connect to a TCP echo server at `addr`, send `msg`, and read the response.
///
/// The server echoes exactly what was sent. Read back up to `msg.len()` bytes.
/// Return the response as a String (assume UTF-8).
pub fn send_echo_message(_addr: &str, _msg: &str) -> String {
    // TODO(module-074): connect to addr, write msg.as_bytes(),
    // read back the same number of bytes into a buffer, return as String.
    panic!("TODO(module-074): implement send_echo_message")
}

/// Parse a simple length-delimited protocol message.
///
/// Format:
///   - First byte: length of the payload (u8, little-endian, value = payload length)
///   - Remaining bytes: payload
///
/// Returns `Some((length, payload))` if the message is valid.
/// Returns `None` if the data is too short to contain the length byte,
/// or if the data doesn't have enough bytes for the declared length.
pub fn parse_protocol_message(_data: &[u8]) -> Option<(u8, Vec<u8>)> {
    // TODO(module-074): read the first byte as the length,
    // check that data has at least 1 + length bytes,
    // return the length and the payload (everything after the first byte).
    panic!("TODO(module-074): implement parse_protocol_message")
}

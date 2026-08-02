//! Module 074: integration tests.

use module_074_exercises::{parse_protocol_message, send_echo_message, start_echo_server};

#[test]
fn test_echo_server() {
    let (addr, handle) = start_echo_server("127.0.0.1:0");
    let response = send_echo_message(&addr.to_string(), "hello");
    assert_eq!(response, "hello");
    // Clean up: the server thread will finish after one connection.
    handle.join().unwrap();
}

#[test]
fn test_echo_server_empty_message() {
    let (addr, handle) = start_echo_server("127.0.0.1:0");
    let response = send_echo_message(&addr.to_string(), "");
    assert_eq!(response, "");
    handle.join().unwrap();
}

#[test]
fn test_echo_server_long_message() {
    let (addr, handle) = start_echo_server("127.0.0.1:0");
    let msg = "The quick brown fox jumps over the lazy dog";
    let response = send_echo_message(&addr.to_string(), msg);
    assert_eq!(response, msg);
    handle.join().unwrap();
}

#[test]
fn test_parse_protocol_message_valid() {
    let data = [5, b'h', b'e', b'l', b'l', b'o'];
    let result = parse_protocol_message(&data);
    assert_eq!(result, Some((5, b"hello".to_vec())));
}

#[test]
fn test_parse_protocol_message_empty_payload() {
    let data = [0];
    let result = parse_protocol_message(&data);
    assert_eq!(result, Some((0, vec![])));
}

#[test]
fn test_parse_protocol_message_too_short() {
    let data = [5, b'h', b'e'];
    let result = parse_protocol_message(&data);
    assert_eq!(result, None);
}

#[test]
fn test_parse_protocol_message_empty() {
    let data: [u8; 0] = [];
    let result = parse_protocol_message(&data);
    assert_eq!(result, None);
}

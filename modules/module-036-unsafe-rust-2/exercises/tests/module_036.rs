use std::sync::Arc;

use module_036_exercises::{
    global_increment, global_reset, global_value, read_u32_unaligned, GlobalCounter, IntOrFloat,
};

#[test]
fn global_counter_tracks_increments() {
    global_reset();
    assert_eq!(global_value(), 0);
    assert_eq!(global_increment(1), 1);
    assert_eq!(global_increment(2), 3);
    assert_eq!(global_value(), 3);
    global_reset();
}

#[test]
fn global_counter_handle_matches_free_functions() {
    global_reset();
    let handle = GlobalCounter::new();
    assert_eq!(handle.total(), 0);
    assert_eq!(handle.increment(5), 5);
    assert_eq!(handle.increment(5), 10);
    assert_eq!(global_value(), 10);
    global_reset();
}

#[test]
fn global_counter_is_thread_safe() {
    global_reset();
    let handle = Arc::new(GlobalCounter::new());
    let mut handles = Vec::new();
    for _ in 0..4 {
        let handle = Arc::clone(&handle);
        handles.push(std::thread::spawn(move || {
            for _ in 0..250 {
                handle.increment(1);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(global_value(), 1000);
    assert_eq!(handle.total(), 1000);
    global_reset();
}

#[test]
fn union_int_roundtrip() {
    let value = IntOrFloat::from_int(42);
    assert_eq!(value.as_int(), 42);
}

#[test]
fn union_float_roundtrip() {
    let value = IntOrFloat::from_float(3.5);
    assert_eq!(value.as_float(), 3.5);
}

#[test]
fn union_reinterprets_bits_not_values() {
    let value = IntOrFloat::from_int(-1);
    assert_eq!(value.as_float().to_bits(), (-1i32) as u32);
}

#[test]
fn unaligned_read_matches_from_ne_bytes() {
    let bytes = [0x11u8, 0x22, 0x33, 0x44];
    assert_eq!(read_u32_unaligned(&bytes), u32::from_ne_bytes(bytes));
}

#[test]
fn unaligned_read_from_inside_a_buffer() {
    let buffer = [0u8, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD, 0, 0];
    let window = &buffer[4..8];
    assert_eq!(
        read_u32_unaligned(window),
        u32::from_ne_bytes([0xAA, 0xBB, 0xCC, 0xDD])
    );
}

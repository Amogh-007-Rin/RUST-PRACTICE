use module_033_solutions::{roundtrip, sum_chunks_via_channel};

#[test]
fn roundtrip_doubles_the_value() {
    assert_eq!(roundtrip(21), 42);
    assert_eq!(roundtrip(0), 0);
    assert_eq!(roundtrip(7), 14);
}

#[test]
fn sum_chunks_single_chunk() {
    assert_eq!(sum_chunks_via_channel(vec![vec![1, 2, 3]]), 6);
}

#[test]
fn sum_chunks_multiple_chunks() {
    let chunks = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    assert_eq!(sum_chunks_via_channel(chunks), 45);
}

#[test]
fn sum_chunks_empty_chunks() {
    assert_eq!(sum_chunks_via_channel(Vec::new()), 0);
}

#[test]
fn sum_chunks_with_empty_inner_chunks() {
    assert_eq!(sum_chunks_via_channel(vec![vec![], vec![10], vec![]]), 10);
}

#[test]
fn sum_chunks_many_chunks() {
    let chunks: Vec<Vec<u32>> = (0..16).map(|i| vec![i as u32; 100]).collect();
    let expected: u64 = (0..16).map(|i| (i as u64) * 100).sum();
    assert_eq!(sum_chunks_via_channel(chunks), expected);
}

//! Module 033: reference solution.
//!
//! Round-tripping through two channels, and fan-in aggregation through a
//! single shared channel.

use std::sync::mpsc;

/// Sends `value` to a worker thread over one channel; the worker doubles it
/// and sends the result back over a second channel. Returns the received
/// value.
pub fn roundtrip(value: u32) -> u32 {
    let (tx_to_worker, rx_to_worker) = mpsc::channel();
    let (tx_to_main, rx_to_main) = mpsc::channel();

    std::thread::spawn(move || {
        let received = rx_to_worker.recv().unwrap();
        tx_to_main.send(received * 2).unwrap();
    });

    tx_to_worker.send(value).unwrap();
    rx_to_main.recv().unwrap()
}

/// Spawns one worker thread per chunk. Each worker sends its chunk's sum to
/// a shared channel. Joins every worker, then returns the sum of all
/// messages received. Returns 0 for an empty input.
pub fn sum_chunks_via_channel(chunks: Vec<Vec<u32>>) -> u64 {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    for chunk in chunks {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || {
            let sum: u64 = chunk.iter().map(|&x| x as u64).sum();
            tx.send(sum).unwrap();
        }));
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    rx.iter().sum()
}

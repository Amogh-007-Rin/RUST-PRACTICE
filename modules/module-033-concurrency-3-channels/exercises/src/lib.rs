//! Module 033: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Sends `value` to a worker thread over one channel; the worker doubles it
/// and sends the result back over a second channel. Returns the received
/// value.
pub fn roundtrip(value: u32) -> u32 {
    // TODO(module-033): `use std::sync::mpsc;`, then create two channels
    // (main→worker and worker→main), spawn a worker that receives on the
    // first and sends `received * 2` on the second, then send `value` and
    // return what you receive back.
    let _ = &value;
    panic!("TODO(module-033): implement roundtrip")
}

/// Spawns one worker thread per chunk. Each worker sends its chunk's sum to
/// a shared channel. Joins every worker, then returns the sum of all
/// messages received. Returns 0 for an empty input.
pub fn sum_chunks_via_channel(_chunks: Vec<Vec<u32>>) -> u64 {
    // TODO(module-033): use `mpsc::channel()`, clone the `Sender` into each
    // worker (each computes its chunk's sum and sends it), then `drop` your
    // own sender, join every worker, and return `rx.iter().sum()`.
    panic!("TODO(module-033): implement sum_chunks_via_channel")
}

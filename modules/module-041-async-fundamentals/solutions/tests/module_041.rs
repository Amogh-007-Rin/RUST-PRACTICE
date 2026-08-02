//! Module 041: Async Fundamentals — integration tests.
//!
//! These tests define "done" for the exercise. They exercise the two
//! pieces you implement in `src/lib.rs`: the `Future` implementation for
//! `Delay` and the tiny executor `block_on`.

use module_041_solutions::{block_on, Delay};

use std::future::Future;
use std::task::{Context, Waker};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn block_on_completes_a_delay() {
    block_on(Delay::new(Duration::from_millis(20)));
}

#[test]
fn block_on_runs_async_blocks() {
    let value = block_on(async {
        Delay::new(Duration::from_millis(10)).await;
        7u32
    });
    assert_eq!(value, 7);
}

#[test]
fn delay_waits_at_least_its_duration() {
    let start = Instant::now();
    block_on(Delay::new(Duration::from_millis(30)));
    assert!(start.elapsed() >= Duration::from_millis(29));
}

#[test]
fn first_poll_returns_pending() {
    let mut delay = Box::pin(Delay::new(Duration::from_secs(60)));
    let waker = Waker::noop();
    let mut cx = Context::from_waker(&waker);
    assert!(delay.as_mut().poll(&mut cx).is_pending());
}

#[test]
fn poll_after_deadline_is_ready() {
    let mut delay = Box::pin(Delay::new(Duration::from_millis(5)));
    thread::sleep(Duration::from_millis(15));
    let waker = Waker::noop();
    let mut cx = Context::from_waker(&waker);
    assert!(delay.as_mut().poll(&mut cx).is_ready());
}

//! Module 041: Async Fundamentals — exercise scaffold.
//!
//! This module is **pure `std`**: no Tokio, no `futures` crate. You will
//! build the machinery that makes `async`/`.await` work, by hand:
//!
//! 1. `Delay` — a hand-written `Future` that completes once a deadline
//!    passes, arming a helper thread that wakes the waiting task.
//! 2. `block_on` — a tiny executor that polls a future to completion,
//!    parking the current thread between polls.
//!
//! The integration tests in `tests/module_041.rs` define "done". Fill in
//! the TODOs below.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

/// A future that completes once its deadline has passed.
///
/// The first time it is polled it arms a helper thread that sleeps until
/// the deadline and then wakes the task waiting on it.
#[allow(dead_code)] // fields are read only once you implement `poll`
pub struct Delay {
    deadline: Instant,
    state: Arc<DelayState>,
}

/// Shared state between the `Delay` and its helper thread.
#[allow(dead_code)] // read only once you implement `poll`
struct DelayState {
    /// The waker registered by the most recent poll. The helper thread
    /// takes this waker and calls `wake()` on it when the deadline passes.
    waker: Mutex<Option<Waker>>,
    /// Whether the helper thread has already been spawned. A `Delay` must
    /// arm its thread exactly once, no matter how many times it is polled.
    started: AtomicBool,
}

#[allow(dead_code)] // read only once you implement `poll`
impl Delay {
    /// Create a `Delay` that completes `duration` from now.
    pub fn new(duration: Duration) -> Delay {
        Delay {
            deadline: Instant::now() + duration,
            state: Arc::new(DelayState {
                waker: Mutex::new(None),
                started: AtomicBool::new(false),
            }),
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let _ = (self, cx); // consumed once you implement the state machine
                            // TODO(module-041): implement the future's poll state machine.
                            //
                            // 1. If `self.deadline` has already passed, return
                            //    `Poll::Ready(())` — do not spawn anything.
                            // 2. Otherwise, arm the wake-up thread **exactly once**: use
                            //    `self.state.started.swap(true, std::sync::atomic::Ordering::SeqCst)`
                            //    to detect the first poll, then `std::thread::spawn` a thread
                            //    that sleeps until `self.deadline` (hint:
                            //    `thread::sleep(self.deadline - Instant::now())`) and then
                            //    calls `wake()` on the stored waker, taking it out of the
                            //    `Mutex` with `.take()`.
                            // 3. Store `cx.waker().clone()` in `self.state.waker` so the
                            //    thread knows what to wake.
                            // 4. Return `Poll::Pending`.
                            //
                            // The state machine the tests exercise:
                            //
                            //        poll()
                            //          |
                            //          v
                            //    deadline passed? ---- yes ----> Poll::Ready(())
                            //          |
                            //          no
                            //          |
                            //          v
                            //    first poll? ---- yes ----> spawn sleeper thread
                            //          |                        (sleeps until deadline,
                            //          no                       then wakes the waker)
                            //          |
                            //          v
                            //    store waker, return Poll::Pending
        panic!("TODO(module-041): implement Delay::poll")
    }
}

/// A tiny executor: polls `fut` to completion, parking the current thread
/// in between polls and letting its waker `unpark` it.
///
/// This is the same loop every async runtime runs, minus the I/O
/// integration: poll, and if the future says "not yet", sleep until
/// something wakes you, then poll again.
pub fn block_on<F: Future>(_fut: F) -> F::Output {
    // TODO(module-041): implement `block_on`.
    //
    // 1. Pin the future so it can be polled: `let mut fut = Box::pin(fut);`.
    // 2. Build a waker from an `Arc` of a type that implements
    //    `std::task::Wake`; its `wake` method should call
    //    `self.0.unpark()` on the thread it holds.
    //    (Store the thread: `struct ThreadWaker(thread::Thread)`,
    //    `let waker = Waker::from(Arc::new(ThreadWaker(thread::current())))`.)
    // 3. Loop: build a `Context::from_waker(&waker)`, poll
    //    `fut.as_mut()`, return the value on `Poll::Ready`, otherwise
    //    `thread::park()` and loop again.
    //
    // `thread::park`/`unpark` have exactly the "no lost wakeups"
    // semantics the loop needs: if `unpark` fires before `park` is
    // called, the next `park` returns immediately.
    panic!("TODO(module-041): implement block_on")
}

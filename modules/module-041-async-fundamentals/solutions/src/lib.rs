//! Module 041: Async Fundamentals — reference solution.
//!
//! `Delay` is a hand-written `Future` that arms a helper thread when first
//! polled; `block_on` is a tiny executor that polls a future to completion,
//! parking the calling thread between polls. Together they demonstrate
//! everything `async`/`.await` desugars into — before any runtime exists.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};

/// A future that completes once its deadline has passed.
pub struct Delay {
    deadline: Instant,
    state: Arc<DelayState>,
}

/// Shared state between the `Delay` and its helper thread.
struct DelayState {
    /// The waker registered by the most recent poll.
    waker: Mutex<Option<Waker>>,
    /// Whether the helper thread has already been spawned.
    started: AtomicBool,
}

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
        // Fast path: the deadline has already passed, the future is ready.
        if Instant::now() >= self.deadline {
            return Poll::Ready(());
        }

        // First poll only: arm the helper thread that will wake us.
        if !self.state.started.swap(true, Ordering::SeqCst) {
            let deadline = self.deadline;
            let state = self.state.clone();
            thread::spawn(move || {
                let now = Instant::now();
                if deadline > now {
                    thread::sleep(deadline - now);
                }
                if let Some(waker) = state.waker.lock().unwrap().take() {
                    waker.wake();
                }
            });
        }

        // Remember who to wake, and admit we are not done yet.
        *self.state.waker.lock().unwrap() = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// A waker that wakes the thread it was created on.
struct ThreadWaker(thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// A tiny executor: poll, park, repeat.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = Box::pin(fut);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
        thread::park();
    }
}

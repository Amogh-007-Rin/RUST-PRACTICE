# Module 033: Concurrency III — Channels (`mpsc`)

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 60–90 min
**Prerequisites:** Module 031 (threads), Module 032 (`Arc` for shared senders)

## Learning Objectives

- Create a channel with `mpsc::channel()` and explain the **m**ulti-**p**roducer, **s**ingle-**c**onsumer contract.
- Send values with `Sender::send` and receive with blocking `Receiver::recv`, handling both error paths (receiver gone, senders gone).
- Clone a `Sender` so several threads can produce into one channel (fan-in).
- Explain channel closing: when every `Sender` is dropped, the receiver's iteration ends and `recv` returns `Err`.
- Build a fan-in aggregation: worker threads each compute a partial result and the main thread sums the incoming messages.

## Why This Matters

Channels are the backbone of actor-style and pipeline architectures. Every message queue you'll meet later — `tokio::sync::mpsc`, crossbeam, redis pub/sub, Kafka in miniature — is the same conceptual shape: producers push, a consumer pulls, and the channel handles the synchronization for you. Rust's standard channels are famously well-designed: `send` moves the value, the channel owns it in between, and the type system guarantees no two threads ever touch the same message. If you can reason about `mpsc` closing and blocking, you can reason about the design of most real-world concurrent systems.

## Concept

### The channel model

A **channel** is a pipe between threads with two ends: `Sender<T>` and `Receiver<T>`. A sender pushes values in; a receiver pulls them out. The channel itself owns every value in transit — once you `send`, your copy is gone (moved), and nobody but the receiver will ever see it again. The name `mpsc` spells out the contract: **m**ultiple **p**roducers (any number of `Sender` clones) but **s**ingle **c**onsumer (exactly one `Receiver`). That asymmetry is what keeps the design simple and deadlock-free by construction.

Here is the flow, time flowing downward:

```
 producer thread A            channel                  consumer (main)
──────────────────      ─────────────────────      ──────────────────
  tx.send(msg1) ───────► [ msg1 |  msg2 |  ] ───────► rx.recv() → msg1
  tx.send(msg2) ───────► [  msg2 | msg3 |  ] ───────► rx.recv() → msg2
  tx.clone()/send...    │  ...queue...
                        │
 producer thread B                                  rx.recv() → msg3
  tx.send(msg3) ───────► [  msg4 |       ] ───────►  rx.recv() → msg4
                        │
   every tx dropped ───► channel CLOSED             rx.recv() → Err(Disconnected)
                        │                            rx.iter() ends
```

Notice what the channel does for you: it **buffers** (sends don't block waiting for a receiver), it **orders** (FIFO within the channel), and it **synchronizes** (the consumer never races the producers — it can only see fully-sent values).

### Sending and receiving

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
tx.send(42).unwrap();

let value = rx.recv().unwrap();
assert_eq!(value, 42);
```

- `mpsc::channel()` returns the pair. `tx` moves values in, `rx` pulls them out.
- `send(value)` *moves* `value` into the channel; you can't use it afterwards. It returns `Result` — if the receiver has been dropped (the other end is gone), you get `Err(RecvError)` and your value comes back to you inside the error. That's the "nobody is listening" case.
- `recv()` **blocks** the current thread until a value arrives or the channel closes. The close case returns `Err(RecvError)`. This blocking is exactly like `join()` in Module 031 — the thread sleeps, it doesn't spin.
- There's also `try_recv()` which never blocks and returns `Err(Empty)` when nothing is queued — useful for polling loops.

### The worker-thread pattern

The canonical use: hand each worker its own slice of work, have workers send partial results back, and have the main thread aggregate:

```rust
use std::sync::mpsc;
use std::thread;

fn sum_chunks(chunks: Vec<Vec<u32>>) -> u64 {
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    for chunk in chunks {
        // One Sender per thread — Sender is cheap to clone.
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let sum: u64 = chunk.iter().map(|&x| x as u64).sum();
            tx.send(sum).unwrap();
        }));
    }

    // The original sender must be dropped too, or the receiver
    // will wait forever once the workers finish.
    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    // rx.iter() ends exactly when every sender (including our clone)
    // has been dropped — i.e. when all workers are done.
    rx.iter().sum()
}

assert_eq!(sum_chunks(vec![vec![1, 2], vec![3, 4]]), 10);
```

Three details make this pattern work, and they're the heart of this module:

1. **`Sender` is `Clone`.** Each worker gets its own `Sender` referring to the same channel. This is the "multiple producers" part of `mpsc`.
2. **`drop(tx)` is not optional.** The receiver cannot distinguish "no messages yet" from "no more messages ever" — the only signal is the count of live senders. If the main thread keeps its original `tx` alive, the channel never closes and `rx.iter()` blocks forever (deadlock). Dropping your own copy tells the channel: only the workers remain, so the channel closes when they finish.
3. **Messages are the result channel.** The workers don't return values through `join()` (though they could); they push into the channel, and the consumer aggregates in arrival order. `join()` still matters — it converts worker panics into main-thread panics instead of silently-short channels.

### Error handling and disconnection

The channel has two error cases, both worth internalizing because they're how you detect a broken pipeline at runtime:

- `send` fails when the **receiver** is gone — e.g. the consumer panicked. Your value comes back in the `Err` (`send` returns `Result<T, SendError<T>>`).
- `recv` fails when **all senders** are gone — the channel is closed and no more values will ever come.

Both are *normal control flow*, not bugs: `rx.iter()` literally uses the second case to terminate the loop. `unwrap()` is fine in exercises and examples; in production code the choice of `?` vs. `unwrap` depends on whether a dropped peer is part of the protocol (it usually is — shutdown is a real protocol step).

### Producers and consumers at scale

The pattern generalizes: producers can push work *into* a channel for consumers to pick up (a work queue), or consumers push results *out* (fan-in). You can even chain channels into pipelines. In every case the same rules apply: clone senders for producers, keep exactly one receiver, and let sender-drop signal completion.

## Common Pitfalls

- **Forgetting to drop your own `Sender`.** The single most common mpsc deadlock: the main thread holds `tx` while calling `rx.recv()`/`rx.iter()`, so the channel never closes and everyone waits forever. If a worker loop ever seems to hang, check every `tx` in scope.
- **Sending from two threads without cloning.** `Sender` is not `Copy`; the second thread's `move` closure can't use the already-moved `tx` — compile error. Clone once per producer.
- **`recv()` where `try_recv()` belongs.** Blocking in a loop that should be polling (e.g. a UI or I/O loop) stalls everything until a message arrives. Use `try_recv()` when the thread has other work to do.
- **Ignoring the value in `SendError`.** When `send` fails you get the value *back*; if you `unwrap()` you lose it. If you're implementing retry/queue logic, `?` or match on the error and keep the value.
- **Assuming message order across producers.** Within a single sender, order is FIFO; across multiple cloned senders, the global order is whatever the scheduler interleaving produced. Order only matters if you designed the system around one producer per channel.

## Key Terms

- **channel:** a typed pipe between threads; `Sender<T>` pushes, `Receiver<T>` pulls.
- **mpsc:** multiple producer, single consumer — the channel's concurrency contract.
- **send:** move a value into the channel; returns `Err(SendError<T>)` if the receiver is gone.
- **recv:** block until a value is available or the channel closes.
- **try_recv:** non-blocking receive; returns `Err(Empty)` when nothing is queued.
- **disconnected:** the channel state when the peer end has been dropped; senders/receivers see this as an error.
- **fan-in:** many producers feeding one consumer; the standard aggregation shape.
- **sender drop semantics:** the channel closes exactly when all `Sender`s are dropped — the receiver's signal that no more messages will come.

## Exercise

Open `exercises/` and fill in the `// TODO(module-033)` comments in `src/lib.rs`:

1. Implement `roundtrip(value: u32) -> u32`: spawn a worker thread, send `value` to it over one channel, have the worker double it and send the result back over a second channel, and return what the main thread receives.
2. Implement `sum_chunks_via_channel(chunks: Vec<Vec<u32>>) -> u64`: spawn one worker per chunk, each sends its chunk's sum to a shared channel (clone the sender!), then join all workers, drop the main thread's sender, and aggregate everything the receiver iterates over.

The tests in `tests/module_033.rs` check the round trip, single and multi-chunk sums, and the empty-input edge case (zero chunks must return 0 without hanging — if your sender handling is wrong, this test times out).

```bash
cargo test -p module-033-exercises
```

When you're done, compare with `solutions/`.

## Further Reading

- The Rust Book, [Chapter 16.2: Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [`std::sync::mpsc` API reference](https://doc.rust-lang.org/std/sync/mpsc/index.html)
- [Rust Atomics and Locks (free online book), Chapter 5 — "Channels"](https://marabos.nl/atomics/)

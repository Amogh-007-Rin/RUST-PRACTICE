# Capstone 08: Distributed Key-Value Store

A TCP-based distributed key-value store with leader/follower replication, a CLI client
built with clap, and JSON-serialized wire protocol.

## Overview

This capstone covers Modules 071—080 (CLI, Networking & Distributed Systems).
You'll implement:

- A core KV store library with operations (get, set, delete, keys)
- A simple leader election model (highest term wins)
- Replication via TCP with JSON-over-newline serialization
- A CLI interface using clap for interacting with nodes

## Architecture

```
┌──────────┐  TCP/JSON   ┌───────────┐
│  Client  │ ──────────> │  Leader   │
│ (kv cli) │             │  :9000    │
└──────────┘             └───────────┘
                              │ replicate
                              ▼
                         ┌───────────┐
                         │ Follower  │
                         │  :9001    │
                         └───────────┘
```

## Wire Protocol

Messages are JSON objects terminated by a newline (`\n`).

### Commands (client → server)

```json
{"Get":{"key":"mykey"}}
{"Set":{"key":"mykey","value":"myval"}}
{"Delete":{"key":"mykey"}}
"Keys"
{"Replicate":{"key":"k","value":"v","term":1}}
{"Heartbeat":{"term":1,"leader_id":"node-9000"}}
```

### Responses (server → client)

```json
"Ok"
{"Value":{"value":"myval"}}
{"Value":{"value":null}}
{"Keys":{"keys":["a","b"]}}
{"Error":{"message":"key not found: x"}}
```

## Usage

### Building

```bash
cargo build -p capstone-08-starter
cargo build -p capstone-08-solution
```

### Running the leader

```bash
# Start leader on port 9000
cargo run -p capstone-08-starter -- --role leader --port 9000
```

### Running a follower

```bash
# Start follower on port 9001
cargo run -p capstone-08-starter -- --role follower --port 9001
```

### CLI operations

```bash
# Point at a running server
cargo run -p capstone-08-starter -- --port 9002 --leader-addr 127.0.0.1:9000 set hello world
cargo run -p capstone-08-starter -- --port 9002 --leader-addr 127.0.0.1:9000 get hello
cargo run -p capstone-08-starter -- --port 9002 --leader-addr 127.0.0.1:9000 keys
cargo run -p capstone-08-starter -- --port 9002 --leader-addr 127.0.0.1:9000 delete hello
cargo run -p capstone-08-starter -- --port 9002 info
```

### Testing

```bash
cargo test -p capstone-08-solution
```

## Key Concepts

- **Leader Election**: Simplified model — the node with the highest term becomes leader.
  A node calls `become_leader()` to increment its term and take leadership.
- **Replication**: The leader sends `Replicate` commands to followers. Followers apply
  writes only if the term is >= their current term (stale terms are rejected).
- **Heartbeats**: Leaders periodically send `Heartbeat` messages so followers know
  the current leader and term.

## Starter vs Solution

- `starter/` — Full scaffold with `todo!()` placeholders. Implement the KV store
  library methods to make all tests pass.
- `solution/` — Complete working implementation with passing tests.

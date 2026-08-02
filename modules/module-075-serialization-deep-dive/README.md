# Module 075: Serialization Deep Dive

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 60–90 min
**Prerequisites:** Module 016 (Traits I), Module 017 (Traits II)

## Learning Objectives
- Serialize and deserialize Rust types to/from JSON using `serde` and `serde_json`
- Understand the difference between text-based and binary serialization formats
- Convert JSON data to bincode — a compact binary format
- Use `#[serde(rename)]` to decouple Rust field names from wire format names
- Recognise when different serialization formats are appropriate (human-readable vs. machine-efficient)

## Why This Matters
`serde` is a cornerstone of the Rust ecosystem. Every web backend (JSON bodies in `axum`/`actix-web`), every configuration file, every database ORM result, every gRPC message, and every CLI tool that reads settings touches serde. Understanding the deeper serde API — not just `#[derive(Serialize, Deserialize)]` — lets you handle non-standard wire formats, customise field names for legacy APIs, and choose the right serialization format for the job. binary encoding with bincode is 5-10x smaller and faster than JSON, which matters when you're designing the replication protocol in a distributed system (the exact thing you'll build in Block H's capstone).

## Concept

Serialization converts in-memory data structures into a byte representation for storage or transmission. Deserialisation reverses that. Rust's `serde` framework is the standard: it's a data-model abstraction layer that lets you pair any data format library with any Rust type by implementing the `Serialize` and `Deserialize` traits once.

### JSON: the universal text format

`serde_json` is the most common serde back-end. It converts Rust values to and from JSON strings:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
}

let user = User { name: "Alice".into(), age: 30 };

// Serialize to JSON string
let json = serde_json::to_string(&user).unwrap();
// => {"name":"Alice","age":30}

// Deserialize from JSON
let restored: User = serde_json::from_str(&json).unwrap();
assert_eq!(restored, user);
```

JSON is human-readable and interoperable across languages, but it's verbose. For performance-sensitive network protocols (like a replicated key-value store sending millions of log entries), binary formats are preferred.

### Bincode: compact binary encoding

`bincode` is a binary serde format. It encodes Rust types directly — no human-readable keys, no string interpolation of numbers — just raw field values:

```rust
let bytes = bincode::serialize(&user).unwrap();
// bincode output is typically 5-10x smaller than JSON

let restored: User = bincode::deserialize(&bytes).unwrap();
assert_eq!(restored, user);
```

The trade-off: bincode data isn't readable by non-Rust programs (unless they implement the same wire format). It's ideal for internal Rust-to-Rust communication — service-to-service messaging, replication protocols, cache layers.

### The serde data model

serde doesn't directly map Rust to JSON or bincode. It defines an abstract **data model** — a set of types that any format must support:

| serde model type | Rust examples          | JSON form      | Bincode form  |
|-----------------|------------------------|----------------|---------------|
| bool            | `bool`                 | `true`/`false` | 1 byte (0/1)  |
| i8/i16/i32/i64  | `i32`, `usize`          | number         | varint        |
| f32/f64         | `f32`                  | number         | 4/8 bytes     |
| char            | `char`                 | string         | 4 bytes       |
| string          | `String`, `&str`       | `"..."`        | length+bytes  |
| seq             | `Vec<T>`, `[T]`        | `[...]`        | length+items  |
| map             | `HashMap<K,V>`         | `{...}`        | length+pairs  |
| struct          | custom structs         | `{...}`        | field values  |
| enum            | Rust enums             | various        | variant index |

When you `#[derive(Serialize)]`, serde generates code that maps your struct to this abstract model. A format like JSON then encodes the model as text; bincode encodes it as bytes. You can write one `Serialize` impl and use it with any format.

### Bridging JSON and bincode

A common pattern: parse JSON (maybe from a config file or an HTTP request), validate it, then store it in a compact binary cache:

```rust
// 1. Parse JSON string into a serde_json::Value (validates syntax)
let value: serde_json::Value = serde_json::from_str(json_string)?;

// 2. Re-serialize the raw JSON text with bincode for compact storage
let binary = bincode::serialize(&json_string)?;

// Later: read the binary cache, get the JSON back, re-parse
let json_back: String = bincode::deserialize(&binary)?;
let value: serde_json::Value = serde_json::from_str(&json_back)?;
```

### Renaming fields: decoupling Rust from the wire

APIs evolve. A Rust field named `user_id` might need to appear as `userId` in JSON (camelCase convention for REST APIs) or as `user_id` (snake_case convention for gRPC). `#[serde(rename)]` handles this:

```rust
#[derive(Serialize, Deserialize)]
struct ApiModel {
    #[serde(rename = "userId")]
    user_id: u64,

    #[serde(rename = "createdAt")]
    created_at: String,
}

let m = ApiModel { user_id: 1, created_at: "2024-01-01".into() };
let json = serde_json::to_string(&m).unwrap();
// => {"userId":1,"createdAt":"2024-01-01"}
```

You can also rename the entire struct with `#[serde(rename_all = "camelCase")]` at the struct level, or use `#[serde(alias = "old_field_name")]` to accept both old and new names during deserialization.

### `Serialize` and `Deserialize` — two independent traits

You don't need both. A struct might be `Serialize` only (outgoing data) or `Deserialize` only (incoming config). The derive macro can generate either individually:

```rust
#[derive(Serialize)]       // can be written out
struct ServerConfig { ... }

#[derive(Deserialize)]     // can be read from a file
struct ClientConfig { ... }
```

The type parameter on our helper functions reflects this independence:

```rust
fn serialize_to_json<T: Serialize>(value: &T) -> String;
fn deserialize_from_json<T: Deserialize>(json: &str) -> Result<T, _>;
```

### Format limitations

Not every serde format supports every data model feature. Bincode does not support `deserialize_any()`, which `serde_json::Value`'s deserialization implementation relies on. This means you can't `bincode::deserialize::<serde_json::Value>(&bytes)` directly — you'll get a `DeserializeAnyNotSupported` error. The workarounds: use a concrete type (e.g., `HashMap<String, String>`), or serialize/deserialize the raw JSON text string instead of the parsed Value.

## Common Pitfalls
- **Using JSON when bincode would be better**: JSON is 5-10x larger. For inter-service RPC, use a binary format. For config files and debug logging, use JSON.
- **Deserializing into the wrong type**: `let v: Vec<u8> = serde_json::from_str("[1,2,3]")?` works because JSON arrays map to vecs. But `let v: BTreeMap<String, usize> = serde_json::from_str("[1,2,3]")?` panics — the JSON shape must match the Rust type.
- **Forgetting to handle `DeserializeAny` limitations**: bincode and many binary formats don't support it. Use concrete types.
- **`#[serde(rename)]` on one field but forgetting corresponding client code**: if you rename a field, your HTTP client, gRPC client, or config file must use the new name.

## Key Terms
- **serde**: Rust's serialization/deserialization framework (trait-based data model abstraction)
- **serde_json**: the standard JSON back-end for serde
- **bincode**: a compact binary serde format, ideal for Rust-to-Rust communication
- **`#[serde(rename)]`**: attribute that changes the wire name of a field
- **serde data model**: the abstract type system (bool, integer, string, seq, map, etc.) that format back-ends and Rust types both implement against

## Exercise

In `exercises/`, fill in the `TODO(module-075)` markers to:

1. **`serialize_to_json`** — use `serde_json::to_string`
2. **`deserialize_from_json`** — use `serde_json::from_str`
3. **`json_to_bincode`** — validate the JSON is parseable, then encode the raw string with `bincode::serialize`

Study the `Person` struct (fully wired up with `#[serde(rename)]`) to understand field renaming. The integration tests verify both the helper functions and the rename behaviour.

Run `cargo test -p module-075-exercises` to verify.

## Further Reading
- [serde documentation](https://serde.rs/)
- [serde attributes reference](https://serde.rs/attributes.html)
- [bincode crate](https://docs.rs/bincode/1)
- [serde data model](https://serde.rs/data-model.html)

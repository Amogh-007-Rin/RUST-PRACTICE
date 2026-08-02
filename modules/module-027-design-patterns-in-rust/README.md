# Module 027: Design Patterns in Rust

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Modules 016–017 (traits, bounds), 024 (patterns), 026 (trait objects), 021 (closures)

## Learning Objectives

- Build chainable configuration APIs with the builder pattern.
- Encode state machines in the type system with the typestate pattern.
- Explain RAII and use `Drop` for automatic cleanup.
- Separate data from operations with the visitor pattern.

## Why This Matters

"Design patterns" in Rust are less about mimicking the Gang of Four catalog and more about leveraging what the language gives you. The builder is how `tokio::net::TcpListener`, `Command`, and every CLI framework configure themselves; typestate is how `mio` and HTTP frameworks make illegal states unrepresentable; RAII is how the borrow checker's sibling, the *drop checker*, guarantees file and lock cleanup (and how `MutexGuard` in Module 032 will work); and the visitor is how `syn` (procedural macros) and `serde`'s deserializers walk trees. These four patterns are the shared vocabulary of every serious Rust codebase.

## Concept

### 1. Builder: configuration as a chain

The builder pattern solves "a constructor with too many optional parameters." Instead of `new(size, topping_a, topping_b, ..., cheese)`, you accumulate state on a helper type and finish with `build`:

```rust
#[derive(Debug, PartialEq)]
struct Command {
    program: String,
    args: Vec<String>,
    env: Vec<(String, String)>,
}

#[derive(Default)]
struct CommandBuilder {
    program: String,
    args: Vec<String>,
    env: Vec<(String, String)>,
}

impl CommandBuilder {
    fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            ..Self::default()
        }
    }

    fn arg(mut self, value: &str) -> Self {
        self.args.push(value.to_string());
        self
    }

    fn env(mut self, key: &str, value: &str) -> Self {
        self.env.push((key.to_string(), value.to_string()));
        self
    }

    fn build(self) -> Command {
        Command {
            program: self.program,
            args: self.args,
            env: self.env,
        }
    }
}

fn main() {
    let cmd = CommandBuilder::new("cargo").arg("build").env("RUST_LOG", "info").build();
    assert_eq!(cmd.program, "cargo");
    assert_eq!(cmd.args, vec!["build"]);
    assert_eq!(cmd.env, vec![("RUST_LOG", "info")]);
}
```

The pattern's mechanics, step by step:

```
  CommandBuilder::new("cargo")
        |
        |  consumes the builder (self), pushes "build", returns Self
        v
  CommandBuilder { program: "cargo", args: ["build"], env: [] }
        |
        |  same again with ("RUST_LOG", "info")
        v
  CommandBuilder { program: "cargo", args: ["build"], env: [("RUST_LOG","info")] }
        |
        |  build(self) moves the fields out into a Command
        v
  Command { program: "cargo", args: ["build"], env: [("RUST_LOG", "info")] }
```

Each `mut self` method eats the old builder and hands back a new one; because it's `self`, not `&mut self`, you can't accidentally keep using a half-configured builder afterwards. Consumers get named methods instead of positional arguments, and `build()` is the single point where "configuration" becomes "object."

### 2. Typestate: states as types

Typestate pushes the pattern further: the *type* of the value encodes which operations are legal. A method that makes no sense for the current state simply doesn't exist — illegal states become *unrepresentable*, caught at compile time:

```rust
use std::marker::PhantomData;

struct Locked;
struct Unlocked;

struct Door<S> {
    _state: PhantomData<S>,
}

impl Door<Locked> {
    fn new() -> Self {
        Door { _state: PhantomData }
    }

    fn unlock(self) -> Door<Unlocked> {
        Door { _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        Door { _state: PhantomData }
    }
}

fn main() {
    let _door = Door::new().unlock().lock();
}
```

The state is a *marker type*; `PhantomData<S>` tells the compiler the type parameter is used without actually storing data (the door's state lives only in its type). The transitions form a compile-time state machine:

```
              unlock(self) -> Door<Unlocked>
  Door<Locked> ──────────────────────────────> Door<Unlocked>
       ^                                            │
       │                                            │ lock(self) -> Door<Locked>
       └────────────────────────────────────────────┘
    (each transition consumes the door and returns the
     door in its new state; only the methods of that
     state's impl block exist)
```

This will not compile — `Door<Locked>` has no `lock` method (only `Door<Unlocked>` does):

```rust,ignore
let door = Door::new();          // Door<Locked>
door.lock();                     // error: no method named `lock` found
                                 // for struct `Door<Locked>`
```

The error is the feature: state transitions validated by the compiler, before your program runs.

### 3. RAII: cleanup on scope exit

**RAII** (Resource Acquisition Is Initialization) means: acquiring a resource happens in a constructor, and releasing it happens automatically in `Drop`. Rust's `Drop` runs when the value goes out of scope — no `close()`, no `defer`, no forgotten cleanup paths:

```rust
struct FileLease {
    path: String,
}

impl FileLease {
    fn open(path: &str) -> Self {
        println!("opened {path}");
        FileLease { path: path.to_string() }
    }
}

impl Drop for FileLease {
    fn drop(&mut self) {
        println!("closed {}", self.path);
    }
}

fn main() {
    {
        let _lease = FileLease::open("/tmp/data.txt");
        println!("working...");
    }
    println!("lease is gone");
}
```

The output order — `opened /tmp/data.txt`, `working...`, `closed /tmp/data.txt`, `lease is gone` — is deterministic: the moment the block ends, `_lease` is dropped and cleanup runs, *even if a panic unwinds the scope*. This is why every `Mutex` lock (Module 032) and every owned file handle is safe: the guard type's `Drop` releases the resource exactly once, and the compiler guarantees `drop` runs.

### 4. Visitor: data vs. operations

The visitor pattern keeps the data structure (the tree) unchanged while letting you add operations to it from outside — you *visit* nodes rather than adding a method per operation to every node:

```rust
trait Visitor {
    fn visit(&mut self, n: i32);
}

struct SumVisitor {
    total: i64,
}

impl Visitor for SumVisitor {
    fn visit(&mut self, n: i32) {
        self.total += n as i64;
    }
}

enum Node {
    Leaf(i32),
    Branch(Vec<Node>),
}

fn walk(node: &Node, visitor: &mut dyn Visitor) {
    match node {
        Node::Leaf(n) => visitor.visit(*n),
        Node::Branch(children) => {
            for child in children {
                walk(child, visitor);
            }
        }
    }
}

fn main() {
    let tree = Node::Branch(vec![Node::Leaf(1), Node::Branch(vec![Node::Leaf(2)])]);
    let mut sum = SumVisitor { total: 0 };
    walk(&tree, &mut sum);
    assert_eq!(sum.total, 3);
}
```

Why `&mut dyn Visitor` (Module 026)? The visitor is stateful (it accumulates results), so it needs `&mut`; and `dyn` lets `walk` accept *any* visitor implementation — tomorrow's `CountVisitor` needs zero changes to the tree. The tree stays dumb; the operations multiply.

## Common Pitfalls

- **Builder methods taking `&mut self` instead of `self`.** With `self`-by-value chains the intermediate states are unreachable and borrow errors surface early; `&mut` builders let you keep using stale stages and encourage bugs.
- **Forgetting `#[must_use]` on chained builder methods.** Clippy's `return_self_not_must_use` will remind you; a discarded intermediate configuration is almost always a mistake.
- **Typestate with a hidden fallible transition.** If `connect()` can fail, returning `Result<TcpConnection<Connected>, E>` is the honest signature — never silently "succeed" into the connected state.
- **Doing cleanup in a `close()` method AND `Drop`.** Double-close bugs; make `Drop` the single release point, and only keep manual methods when the resource needs reuse semantics.
- **Visitor methods that recurse by calling themselves on the wrong side.** The traversal (`walk`) owns the recursion; visitor methods should handle one node each, or you get duplicated/subtle visit orders.

## Key Terms

- **builder:** a configurable constructor-object that accumulates state and produces the final value with `build()`.
- **typestate:** encoding a state machine in types (marker types + `PhantomData`), so illegal transitions are compile errors.
- **RAII:** resource acquisition in constructors, guaranteed release in `Drop`.
- **`Drop`:** the trait whose `drop(&mut self)` runs automatically when a value leaves scope.
- **visitor:** an object with per-node callbacks, walked over a data structure to add operations without modifying it.
- **marker type:** an empty type used purely for its name, carried via `PhantomData`.

## Exercise

In `exercises/`, four patterns are scaffolded. Fill in each `TODO(module-027)`:

1. `PizzaBuilder` — implement `add_topping`, `extra_cheese`, and `build` (builder).
2. `TcpConnection` — implement `connect`, `send`, and `disconnect` (typestate; note that only `TcpConnection<Connected>` gets `send`).
3. `LogTimer` — implement `elapsed`, `stop`, and the `Drop` body (RAII).
4. `walk` — implement the recursive traversal (visitor).

Run `cargo test -p module-027-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 15.3: Running Code on Cleanup with the Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)
- [Rust by Example: Builder pattern](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
- [std docs: `std::marker::PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)

# Module 006: Slices & Strings

**Block:** Block A — Foundations I
**Estimated time:** 60–120 min
**Prerequisites:** Module 005 (borrowing, `&T` vs `&mut T`)

## Learning Objectives

- You will be able to explain what a slice is and what `&str` borrows.
- You will be able to explain the relationship between `String` and `&str` and convert between them (`&s[..]`, `s.to_string()`).
- You will be able to slice ranges of text (`&s[0..5]`) and know when a slice is a string *boundary* and when it panics.
- You will be able to use common string methods (`len`, `chars`, `find`, `split_whitespace`, `to_uppercase`, `contains`).
- You will be able to explain UTF-8: why `len()` counts bytes, not characters, and what a "code point" is.

## Why This Matters

`&str` is arguably the most common type in the entire Rust ecosystem — every HTTP handler parameter, every CLI argument, every log line you parse is a `&str` borrowed from something. Knowing exactly what it is (a view into text you don't own) makes you fluent in the single most frequent API decision in Rust: `String` (owned) vs `&str` (borrowed). And UTF-8 byte-vs-character confusion is a classic interview gotcha and a real-world bug source (truncating text at a byte offset can panic or split a character).

## Concept

### Slices: borrowed views into contiguous data

A **slice** is a view into a contiguous range of elements — an array or a string — *without owning them*. The type `&[T]` is a slice of `T`s; `&str` is a slice of `char`s worth of bytes. Think of a slice as a pair: a pointer to the first element and a length.

```text
                 ┌─────────────┐
   phrase:       │  "hello"    │   the full String, owned by `phrase`
                 └─────────────┘
   ┌───────────────────────────────────────────────┐
   │ &phrase[0..2]   ptr ─┐  len = 2               │  a slice is a
   │                      │                        │  pointer + a length
   └──────────────────────┼────────────────────────┘
                          ▼
                 ┌──────┬──────────┐
                 │ "he" │  "llo"   │  the same heap data; the slice
                 └──────┴──────────┘  just points into a part of it
```

Slicing is zero-cost: no copying, no allocation — you're just recording *where* and *how much*. This is why functions take `&str` instead of `String` when they only need to read: the caller keeps ownership, and the callee gets a free, precise view.

### `String` vs `&str`

The relationship is easy to remember:

- **`String`** owns its text — a growable, heap-allocated buffer you can push into and extend. It's the "mutable owner".
- **`&str`** borrows text — a view into a `String`, a string literal, or some other buffer. It's the "lender's view", immutable and fixed-length.

Conversions are cheap and explicit:

```rust
fn main() {
    let owned = String::from("hello");       // &str -> String
    let borrowed: &str = &owned[..];         // String -> &str (borrow all of it)
    let literal: &str = "hello";             // string literals are &str already
    let owned2 = literal.to_string();        // &str -> String, another spelling
    let owned3 = format!("{literal}!");      // build a new String from parts
}
```

A string literal like `"hello"` is itself a `&str` baked into the program's memory. That's why this course's functions take `&str` — they work with literals, `String` borrows, and slices alike.

### How to choose: `String` vs `&str` in a signature

The rule of thumb used across this course (and most of the ecosystem): take `&str` when the function only needs to *read* text; take `String` when it must *own* it (store it, mutate it, hand it on). `&str` accepts everything — literals, borrowed `String`s, and slices — while `String` demands that the caller own a heap buffer:

```rust
fn looks_at(s: &str) -> usize {
    s.len()
}

fn owns_it(s: String) -> String {
    format!("{s} and more")
}

fn main() {
    looks_at("literal");          // fine: &str
    looks_at(&String::from("x")); // fine: &String coerces to &str
    // owns_it("literal");        // compile error: expected String
    owns_it(String::from("x"));   // fine: owned
}
```

When you do hold a `String` and want a borrowed view of it, `&s[..]` (or just `&s` where the type is known) is the conversion — zero cost. Notice the borrow rule from Module 005 never goes away: a `&str` into a function keeps the caller's `String` alive.

### Slicing ranges

You can slice a string by byte range with square brackets. `&s[0..5]` takes bytes 0 through 4:

```rust
fn main() {
    let s = String::from("hello world");
    let first = &s[0..5];
    let rest = &s[6..];
    println!("{first} {rest}"); // "hello world"
}
```

`0..5` is *end-exclusive*: it includes index 0, 1, 2, 3, 4 but not 5. `&s[6..]` means "from byte 6 to the end". This is the mechanism behind this module's `first_word` — find the space, slice up to it:

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}
```

`first_word` borrows `s` and returns a *part of that borrow* — perfectly legal, because the output lives inside the input. This is the classic "return a slice of a parameter" pattern; the compiler checks (via lifetimes, Module 018) that the result can't outlive the input.

### UTF-8: bytes vs characters

Here's the part that surprises everyone coming from Python or Java. Rust strings are **UTF-8** — and `s.len()` returns the number of *bytes*, not characters:

```rust
fn main() {
    let s = "hello";
    println!("{} bytes", s.len());      // 5 — one byte per ASCII char

    let czech = "žluťoučký";
    println!("{} bytes", czech.len());  // 11 bytes for 9 characters!
}
```

ASCII characters take 1 byte; other scripts take 2–4 bytes per character (é is 2 bytes, most CJK characters are 3). The consequences:

- `s[0]` is **not valid syntax** for strings — indexing a string by byte would risk landing mid-character. Use `.chars()` instead:

```rust
fn main() {
    let s = "žluť";
    let first = s.chars().next();     // Some('ž') — the first *character*
    for ch in s.chars() {
        println!("{ch}");
    }
}
```

- Slicing at an arbitrary byte offset can split a character, and Rust *panics* rather than produce a half character. `&s[0..1]` on a string starting with a 2-byte character is a runtime panic — that's a deliberate design decision: you get a loud crash instead of a silently corrupted string. The safe method is `s.get(0..n)`, which returns `Option<&str>`.

Where does a `&str` slice *borrow* from? Whichever buffer it was cut from: a `String`'s heap block, a literal's static memory, or another slice. The borrow rules from Module 005 apply unchanged — a slice keeps its source alive, and returning a slice of a parameter (like this module's `first_word`) is fine precisely because the parameter outlives the call. That linkage is what the compiler verifies via lifetimes, which you'll write by hand in Module 018.

### A gallery of useful string methods

The four you'll need for this module and the capstone:

```rust
fn main() {
    let s = "hello world";

    println!("{}", s.len());                          // 11 (bytes)
    println!("{}", s.find(' ').unwrap_or(11));        // 5 (byte index of first space)
    println!("{}", s.split_whitespace().count());     // 2 (words)
    println!("{}", s.to_uppercase());                 // HELLO WORLD (owned String)
    println!("{}", s.contains("world"));              // true
    println!("{}", &s[6..11]);                        // world (sliced view)
}
```

Note the family resemblance: `to_uppercase`, `split_whitespace`, `trim` — anything that *transforms* text returns a fresh owned `String`, while anything that *looks* (`.chars()`, `.find()`, `.contains()`) borrows or returns indices.

Three more methods you'll use constantly once you hit the capstone: `trim` strips surrounding whitespace and returns a *borrowed* `&str` (perfect for cleaning up user input), `split(' ')` iterates over the parts between separators, and `contains` tests for a substring. All three borrow — they never copy your text. And when you need to *build* text from pieces, `format!` and `String::push_str` are the tools:

```rust
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}");
}
```

`push_str` borrows its argument — you can append a literal, a slice, or a borrowed `String` without losing it, which is exactly the `&str` design you just read about.

### The exercise in a sentence each

- `first_word(s: &str) -> &str` — your first slice return: find the space, slice up to it.
- `slice_range(s, start, end) -> &str` — direct range slicing.
- `word_count(s: &str) -> usize` — `split_whitespace().count()`.
- `shout(s: &str) -> String` — `to_uppercase()`, the borrow-to-owned round trip.

The tests make one thing explicit: after `first_word(&phrase)`, `phrase` is still fully usable — because borrowing and slicing never steal ownership. That's the whole point of the module.

## Common Pitfalls

- **Indexing a string with `s[0]`.** Strings don't support direct indexing; use `s.chars().next()`.
- **Trusting `len()` for characters.** `"é".len()` is 2, not 1 — it's bytes. Use `.chars().count()` for character counts.
- **Slicing mid-character.** `&s[0..1]` on a non-ASCII string panics at runtime. Use `s.get(0..1)` for a safe `Option`.
- **Returning a slice of a temporary.** `fn f() -> &str { let s = String::from("x"); &s }` is a dangling reference — rejected at compile time. Borrow from your parameters, not your locals.
- **`to_uppercase()` not being in-place.** It returns a new `String`; strings are not mutable through `&str` — which is exactly why `shout` returns an owned value.

## Key Terms

- **slice:** a borrowed view (pointer + length) into contiguous data; `&[T]` or `&str`.
- **`String`:** an owned, growable, heap-allocated UTF-8 string.
- **`&str`:** a borrowed view of UTF-8 text — the "lender's view".
- **byte index:** a position in a string measured in bytes, not characters.
- **UTF-8:** the variable-width encoding Rust strings use; ASCII is 1 byte, others 2–4.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the four `// TODO(module-006)` comments.
2. Implement `first_word(s)` — `s.find(' ')`, then slice `&s[..index]` or return `s`.
3. Implement `slice_range(s, start, end)` — return `&s[start..end]`.
4. Implement `word_count(s)` — `s.split_whitespace().count()`.
5. Implement `shout(s)` — `s.to_uppercase()`.
6. Run `cargo test -p module-006-exercises` until all 13 tests pass.
7. Compare with `solutions/` afterwards.

## Further Reading

- [The Rust Book, Chapter 4: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html) — the canonical slice chapter, including `first_word`.
- [The Rust Book, Chapter 8: Storing UTF-8 Encoded Text](https://doc.rust-lang.org/book/ch08-02-strings.html) — `String` methods and UTF-8 in depth.
- [std: `str`](https://doc.rust-lang.org/std/primitive.str.html) — the full reference for `&str` methods.
- [The Rust Reference: String literal tokens](https://doc.rust-lang.org/reference/tokens.html#string-literals) — how literals become `&str` and why.

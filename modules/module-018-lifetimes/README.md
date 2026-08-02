# Module 018: Lifetimes

**Block:** Block B — Foundations II
**Estimated time:** 60–90 min
**Prerequisites:** Module 004 (ownership), Module 005 (borrowing & references), Module 007 (structs)

## Learning Objectives

- Read and write lifetime annotations (`<'a>`, `&'a str`) and explain what they assert.
- Apply the three lifetime elision rules so you can read most real signatures without annotations.
- Annotate structs that hold references (`struct Book<'a>`) and their `impl` blocks.
- Explain why `longest(x, y) -> &str` needs `'a` while `first_word(s) -> &str` doesn't.
- Diagnose "lifetime may not live long enough" errors and know the standard fixes.

## Why This Matters

Lifetimes are the part of Rust that makes dangling references *impossible to write*. The borrow checker (Modules 004–005) uses them to prove that every reference you return or store still points at valid data. Real code is full of lifetimes: every `&str` you slice, every `&self` method, every `sqlx` row borrowed from a query result, every `serde` deserialized value borrowing from its input buffer. You won't write `'a` every day — elision hides most of it — but when the compiler rejects your code with a lifetime error, understanding annotations is the only way out. Interviewers also love the "why does `longest` need `'a` but `first_word` doesn't" question.

## Concept

### What a lifetime is

A lifetime is a label for *how long a borrow stays valid* — a region of code in which a reference may be used. It is not about *when* a value is created; it's about *how long* you're allowed to hold a reference to it. Every reference in Rust has a lifetime; most are inferred (elided). The annotation `'a` is just a name for one of these regions.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
```

Read it aloud: "for *some* lifetime `'a`, this function takes two string references that are both valid for `'a`, and returns a reference that is valid for `'a`." Because the return might be `x` *or* `y`, the only safe guarantee is the *overlap* — the time during which both inputs exist:

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

                |<-------- 'a (overlap) -------->|
  string1:      |  "hello"          |  lives until here
                |        x -------->|
  string2:      |  "hello world"    |
                |        y --------------->|
  returned      |  points at either x or y |
  reference:    |  must not outlive 'a      |
                +---------------------------+

  'a is the INTERSECTION of the two borrows: the only time window in
  which the return value is guaranteed to point at live data.
```

The signature doesn't name a specific code region — it states a *relationship*: the output lives no longer than the inputs. Every call site instantiates `'a` with the actual overlap of the arguments' borrows, and the compiler checks that the caller uses the result within that window.

Without the annotation, this function doesn't compile:

```rust,ignore
fn longest_broken(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
    // ERROR: missing lifetime specifier — the compiler cannot tell
    // whether the returned reference borrows from x or from y
}
```

The compiler *needs* the annotation because there are two candidate input lifetimes and no rule to pick between them. The elision rules below are exactly the compiler's "obvious choices" — and this case isn't obvious, so it's not elided.

### The three elision rules

Most signatures in real code carry no annotations because of three rules:

1. **Every elided lifetime in a parameter position gets its own fresh lifetime.** `fn f(x: &str, y: &str)` becomes `fn f<'a, 'b>(x: &'a str, y: &'b str)`.
2. **If there is exactly one input lifetime, it is assigned to every elided output lifetime.** `fn first_word(s: &str) -> &str` becomes `fn first_word<'a>(s: &'a str) -> &'a str`.
3. **If there are multiple input lifetimes but one of them is `&self` or `&mut self` (a method), the self lifetime is assigned to all elided output lifetimes.**

Rule 2 explains `first_word`:

```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

One input reference → the output borrows from it. The compiler writes the `'a`s for you. Rule 3 explains methods:

```rust
impl<'a> Book<'a> {
    fn title(&self) -> &str {        // elided: &'self str — output tied to self
        self.title
    }
}
```

The returned `&str` is tied to `&self`, so calling `book.title()` can't outlive the `book` borrow. Rule 1 + Rule 2 is why `longest` needs its annotation: two input lifetimes (rule 1), so no single candidate for the output (rule 2 doesn't apply).

### Lifetimes in structs: `Book<'a>`

A struct holding references must name their lifetime:

```rust
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}
```

Without `<'a>` this is a compile error — the struct can't own a reference it can't scope. The annotation says: "a `Book` is only valid while the strings it borrows are valid." The lifetime is part of the *type*: `Book<'static>` is a distinct type from `Book<'a>` for a short `'a`. The `impl` block must repeat it:

```rust
impl<'a> Book<'a> {
    fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }

    fn citation(&self) -> String {
        format!("{} — {}", self.author, self.title)
    }
}
```

Note `citation` returns an *owned* `String` — no lifetime needed, it doesn't borrow. Returning `&str` would tie it to `self` via rule 3.

### Why the borrow checker is never wrong about this

The classic mistake: returning a reference to data that dies when the function returns.

```rust,ignore
fn bad<'a>(x: &'a str, y: &'a str) -> &'a str {
    let local = String::from("temporary");
    &local   // ERROR: `local` does not live long enough
}
```

`local` is dropped at the end of the function, so no `'a` can cover it. The compiler refuses — a reference that outlives its data is a dangling pointer, and lifetimes exist precisely to rule those out. The fix is always structural: create the data before the function, return an owned value, or store it somewhere with a matching lifetime.

The other classic: struct outliving its borrow.

```rust,ignore
let title = String::from("Rust");
let book;
{
    let borrowed = String::from("temp");
    book = Book::new(&title, &borrowed); // borrows `borrowed`
}                                        // `borrowed` dies here
println!("{}", book.title());            // ERROR: `borrowed` does not live long enough
```

The compiler sees `Book<'a>` where `'a` is bounded by `borrowed`'s region, and the `book` binding outlives it. Moving `book`'s use inside the block, or owning the strings, fixes it.

## Common Pitfalls

- **Annotating when elision already decides.** `fn first_word<'a>(s: &'a str) -> &'a str` works but clippy calls it `needless_lifetimes` — the annotation is redundant, not wrong. Fix: prefer the elided form in production code.
- **Missing `'a` on a struct holding references.** `struct Book { title: &str }` is a compile error. Fix: `struct Book<'a> { title: &'a str }`, and remember `impl<'a> Book<'a>`.
- **Two input lifetimes, one output.** `fn pick(x: &str, y: &str) -> &str` needs an annotation. Fix: `fn pick<'a>(x: &'a str, y: &'a str) -> &'a str` — and note `'a` becomes the intersection.
- **"does not live long enough".** You're returning or storing a reference to data that dies too early. Fix: own the data (`String` instead of `&str`), restructure, or shorten the borrow.
- **Lifetime parameters that never appear.** `fn f<'a>(x: &i32) -> &i32` — unused `'a` triggers a warning. Fix: elide it; a lifetime that appears in no signature is pointless.

## Key Terms

- **Lifetime (`'a`):** a named region of code during which a borrow is valid.
- **Annotation:** explicit syntax (`<'a>`, `&'a str`) naming lifetimes in signatures and types.
- **Elision:** the three compiler rules that fill in obvious lifetimes for you.
- **Intersection of borrows:** the overlap window that `longest`-style signatures guarantee.
- **`'static`:** the lifetime of the whole program — string literals (`"hi"`) and other program-lifetime data.
- **Borrow region:** the span of code where a particular reference may be used.
- **Dangling reference:** a reference to freed memory; lifetimes make these unrepresentable.

## Exercise

Open `exercises/src/lib.rs`. The annotated signatures are fixed — read them, understand why each annotation is there, and fill in the `TODO(module-018)` bodies:

1. `first_word` / `last` — `split_whitespace` + `next`/`next_back` (elidable `'a`, written explicitly for learning).
2. `longest` — returns the longer input; this is the case that genuinely requires `'a`.
3. `longest_line` — borrow from a slice of owned `String`s, return `&str` tied to `'a`.
4. `Book<'a>` — `new`, `title`, `citation`; see rule 3 working in `title`.
5. `first_and_last` — a tuple of two `'a` references.

The tests in `tests/module_018.rs` define "done":

```bash
cargo test -p module-018-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading

- [The Rust Book, Chapter 10.3 — Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [The Rust Book, Chapter 4 — Borrowing, reviewed through the lifetime lens](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Reference — lifetime elision rules, precisely](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [The Rustonomicon — Lifetimes, a deep dive](https://doc.rust-lang.org/nomicon/lifetimes.html)

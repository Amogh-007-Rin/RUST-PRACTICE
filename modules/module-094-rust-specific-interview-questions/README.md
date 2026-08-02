# Module 094: Rust-Specific Interview Questions — Ownership & Borrowing Gotchas

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 60–90 min
**Prerequisites:** Module 004–005 (ownership & borrowing), Module 018 (lifetimes), Module 029 (`Rc`/`RefCell`), Module 092 (data structures)

## Learning Objectives

- You will be able to recognize the seven most-asked Rust gotchas on sight and name the compiler error behind each one (E0505, E0506, E0499, E0515, E0308…).
- You will be able to fix each gotcha with the idiomatic pattern (`.take()`, `remove`, split borrows, owning instead of borrowing, `retain`, copy-the-reference).
- You will be able to answer whiteboard-style "what's wrong with this code?" prompts with a three-part answer: *what the compiler says, what rule it's enforcing, how you fix it*.
- You will be able to use lifetimes in structs fluently, including the `&'a str`-returning method signature.

## Why This Matters

The Rust-specific interview loop is short and predictable: the interviewer pastes a six-line function, asks "why doesn't this compile?", and listens for the *rule* — not for the exact error number. Ownership, borrowing, and lifetimes are the one topic where Rust interviews actually differ from generic software interviews, and they're the filter for most Rust roles. Every function in this module is a real question that has been asked; internalize the seven patterns here and the whiteboard becomes a recall exercise instead of a gamble.

## Concept

### The three rules behind every gotcha

Everything in this module reduces to three rules you already know from Modules 004–005:

1. **Every value has exactly one owner** — moving a value out of a place you only hold a reference to is forbidden, because that would leave the owner with nothing.
2. **You can borrow immutably any number of times, or mutably exactly once** — two overlapping `&mut`s are an error even if they "would be fine" in practice.
3. **A reference must never outlive the value it points at** — neither a local variable (E0515) nor an internal borrow that blocks the owner.

The skill is *recognizing which rule is being violated in code you're told is broken*. The exercise functions are the seven most common shapes. Let's walk each one, because the walk is the interview.

**1. Moving out of an `Option` through `&mut`.** "Implement a function that takes the value out of `Option<T>` by reference." Naive attempt: `opt.unwrap()` — that's a *borrow* (`&self`), and you can't move out of a borrow. The fix is `Option::take()`, which swaps `None` in and returns the value:

```rust
fn pop_option<T>(opt: &mut Option<T>) -> T {
    opt.take().expect("pop_option called on None")
}
```

`take()` is the general answer to "move out of something I only have `&mut` to" — you met it in Module 091 as the `push_front` move, and it's the same trick everywhere.

**2. Removing the head of a `Vec`.** "Remove the first element without knowing its index." `v.pop()` takes the *last* one; `v[0]` can't move. `v.remove(0)` is the answer, with its O(n) shift acknowledged. Small point, but it distinguishes people who've written Rust from people who've read about it.

**3. Two mutable borrows of one value.** "You have a struct with two fields; write a function returning mutable references to both." `let a = &mut c; let b = &mut c;` is E0499. But the compiler tracks borrows *at field granularity* — split borrows are legal because the checker can see `left` and `right` are disjoint paths:

```rust
struct Counter {
    left: u32,
    right: u32,
}

fn both_mut(c: &mut Counter) -> (&mut u32, &mut u32) {
    (&mut c.left, &mut c.right) // disjoint paths → fine
}
```

This is how `HashMap::get_mut` and iterator adapters stay sound, and why `split_at_mut` exists for slices: the compiler can prove two regions don't overlap.

**4. Lifetimes in structs.** "Why does `struct Searcher { haystack: &str }` not compile?" Because the struct has no idea how long the reference lives — the lifetime must be a parameter, `Searcher<'a>`, and every `impl` and method must repeat it. The subtle half: a method returning `&'a str` (the *input's* lifetime) rather than `&self`-tied `&str`. In the exercise, `first_match` returns the matched slice of the original haystack — the signature says the result outlives the searcher itself:

```rust
struct Searcher<'a> {
    haystack: &'a str,
    needle: &'a str,
}

impl<'a> Searcher<'a> {
    fn new(haystack: &'a str, needle: &'a str) -> Self {
        Self { haystack, needle }
    }

    fn first_match(&self) -> Option<&'a str> {
        let offset = self.haystack.find(self.needle)?;
        Some(&self.haystack[offset..offset + self.needle.len()])
    }
}
```

**5. Returning a reference to a local.** The oldest trick in the book, in any language: "write `fn shout(s: &str) -> &str` that uppercases." The fix — `-> String` — is the moment to say the sentence interviewers want: *"Rust won't let me return a reference to a value that dies when the function returns; so the function returns ownership."*

```rust,ignore
// This will NOT compile — E0515: cannot return reference to local variable
// `s`. The uppercased String is owned by the function and dropped at exit.
fn shout_broken(s: &str) -> &str {
    let uppercased = s.to_uppercase();
    &uppercased
}
```

**6. Mutating while iterating.** "Remove all even numbers from a `Vec`." `for x in &mut v { if *x % 2 == 0 { v.remove(...) } }` borrows `v` inside the loop body while the iterator holds a borrow — E0496-ish, and unsound in every language. The Rust answer is `retain`, which exists exactly for "keep the elements that satisfy this predicate," and it's O(n) with no index bookkeeping:

```rust
fn remove_evens(v: &mut Vec<i32>) -> usize {
    let before = v.len();
    v.retain(|x| x % 2 != 0);
    before - v.len()
}
```

**7. The `&mut &str` reborrow.** "Write a function that pops one line off a `&mut &str`." The trap: `split_once` borrows `*s`, then you assign to `*s` while that borrow is alive — E0505. The elegant fix is that `&str` is `Copy` — read everything through a copy first:

```rust
fn pop_line<'a>(s: &mut &'a str) -> Option<&'a str> {
    let rest = *s; // copy the reference out — no borrow held
    let (line, after) = rest.split_once('\n')?;
    *s = after; // free to assign now
    Some(line)
}
```

### Whiteboard Q&A

The exercise README answers these in the "ask out loud" style: state the error, state the rule, state the fix.

**Q: What does "cannot move out of borrowed content" mean?** *The value behind the reference has an owner; taking it would leave the owner holding nothing. Either use a method designed to move out through `&mut` (`Option::take`, `Vec::remove`, `mem::replace`), or change the API to take ownership.*

**Q: Why does `let y = &x; x += 1;` fail while the same code passes if you print `y` between the lines?** *Rule 2: a mutable borrow and a live immutable borrow overlap. If `y` is never used after the `&x`, the borrow's last use is earlier (NLL — non-lexical lifetimes) and the mutable borrow no longer overlaps. The compiler isn't judging by blocks, it's judging by live ranges.*

**Q: What's wrong with a `struct` holding `&str` without a lifetime?** *The struct would need to store a reference forever; the lifetime parameter names the scope the reference is valid for, and the borrow checker then enforces it. Adding `<'a>` is not ceremony — it's the compiler asking "for how long do you promise these references are valid?"*

**Q: When can two `&mut` borrows coexist?** *When the borrow checker can prove they point at disjoint memory: different fields of the same struct, different elements via `split_at_mut`, or references that come from separate allocations. The rule isn't "one mutable borrow per function" — it's "no two live mutable borrows of the same path."*

**Q: Why does my recursive tree function fight the borrow checker?** *Because a helper taking `&Node` and mutating `node.left` needs `&mut`. Recurse with `&mut Option<Box<Node<T>>>` (Module 092) and each call owns exactly one disjoint mutable path — the same reason both_mut works.*

### Interview technique

Three habits make gotcha questions go well. First, **name the error category** before the fix — "E0505, this is the classic move-out-of-borrowed-content case" earns more than the fix alone. Second, **state the invariant you're protecting** — "the value must keep exactly one owner" — because interviewers want to hear that you understand *why* the rule exists, not that you memorized an error. Third, **offer the safer alternative** — "you *could* `unwrap()` here, but `take()` lets the caller distinguish 'empty' from 'bug'". That last one is what a senior Rust developer sounds like.

## Common Pitfalls

- **Answering with the fix before the rule.** "Use `.take()`" without "because you can't move out of a borrowed value" reads as memorization. Rule first, then fix.
- **Confusing E0505 (value outlives its borrow) with E0499 (overlapping `&mut`).** Both are "borrow problems" but the fixing patterns differ completely: shorten the borrow vs. split it.
- **Adding lifetimes to data when the borrow should be owned.** `shout` returning `String` is simpler than `shout<'a>(...) -> &'a str` — which is impossible anyway. Owned-first is the idiomatic default.
- **Hand-indexing a `Vec` while filtering.** `retain` exists; if you find yourself with `i` and `len` counters and `remove`, stop — the std method is both correct and clearer.
- **Forgetting `&str: Copy`.** The `pop_line` trick looks like magic until you remember references are `Copy`; the whole "copy first, mutate after" pattern depends on it.

## Key Terms

- **Split borrow:** two mutable borrows of disjoint paths (fields, slices) that the checker accepts.
- **NLL:** non-lexical lifetimes — borrows end at their *last use*, not at the end of the block.
- **E0515:** the error for returning a reference to a local value.
- **`retain`:** `Vec`'s in-place filter; the idiom for removing elements while iterating.
- **`take()`/`mem::replace`:** moving a value out of a place you only hold `&mut` to.

## Exercise

In `exercises/`, each public function is one of the seven gotchas above, with the signature that makes the *right* answer possible. Implement `pop_option`, `remove_first`, `both_mut`, `Searcher::first_match`, `shout`, `remove_evens`, and `pop_line` so `cargo test -p module-094-exercises` passes. Try to state, out loud, which rule each one violates before you write the fix — that's the interview drill. Then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 4 — Ownership, and Chapter 10.3 — Lifetimes](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Reference — lifetime elision rules, the prose behind every `&str` signature you write](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [The Rust RFC for NLL — "non-lexical lifetimes," why the borrow checker feels smart](https://rust-lang.github.io/rfcs/2094-nll.html)
- [`std::mem::replace` and `Option::take` — the two move-out primitives](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)

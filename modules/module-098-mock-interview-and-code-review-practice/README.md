# Module 098: Mock Interview & Code Review Practice

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 60–90 min
**Prerequisites:** Module 097 (Building Your Portfolio)

## Learning Objectives
- You will be able to apply a structured code review checklist to unfamiliar Rust code.
- You will be able to identify common logic bug categories: off-by-one errors, incorrect arithmetic, missing edge-case guards, and semantic naming mismatches.
- You will be able to practice an interview-style "debug and fix" flow: reproduce the bug, trace the root cause, write a minimal fix, verify with tests.
- You will be able to communicate findings professionally in the style of a PR review comment — specific, non-judgmental, and actionable.

## Why This Matters

Code review is one of the most common interview formats for senior and mid-level Rust roles. You are given a diff or a short program with intentional bugs and asked to find and explain them. This tests three things simultaneously: your Rust fluency, your ability to read code you didn't write, and your communication skills. Every production Rust codebase — from `rustc` itself to $5B fintech services — runs on PR reviews. The habits you build here transfer directly to your first week on the job.

## Concept

### Why Code Review Exists

Code review is not about catching typos. Its purpose is to answer four questions before code reaches production:

1. **Does this do what it claims to?** Read the comments, the function names, the test names — does the implementation match the intent?
2. **Are there edge cases the author didn't consider?** Empty collections, zero values, maximum values, concurrent access, cancellation.
3. **Is this maintainable?** Will the next person understand it? Are the names helpful? Are the abstractions at the right level?
4. **Does this introduce a regression?** Does it break existing tests? Does it change existing behavior in a way that surprises callers?

A bug that reaches production costs 10–100x more to fix than one caught in review. This is why companies invest in review culture — not to slow developers down, but to prevent the slow-down that comes from shipping bugs.

### A Structured Review Checklist

When you sit down to review code — whether in an interview or on the job — follow this order:

```
1. Read the test file first.
   └─ Tests tell you the spec. What are the expected behaviors? What are the edge cases?
2. Skim the public API.
   └─ What types are exposed? What guarantees do the docs promise?
3. Read the implementation top-to-bottom.
   └─ For each function: does the code match the doc comment?
4. For each function, mentally trace one happy path and one error path.
   └─ What happens with empty input? With max values? With invalid state?
5. Note every discrepancy between (1) what the tests expect, (2) what the docs say, and (3) what the code does.
6. Categorize each finding: logic bug, edge-case gap, semantic mismatch, or style concern.
```

In an interview context, steps 1–3 should take no more than 5 minutes. Step 4 is where you slow down and think aloud — this is what the interviewer wants to hear. Step 5 produces your "findings list." Step 6 prioritizes them.

### Common Bug Categories

#### Off-by-One Errors

The classic: a loop runs one iteration too many or too few. In Rust, this often appears in range expressions. `0..len` is correct; `0..=len` panics on an empty collection. `1..len` skips the first element. When reviewing, check that every index access, range, and counter-increment aligns with the data structure's actual length.

```rust
// Bug: removes ALL matching items, not just the first
fn remove_first_match(items: &mut Vec<Item>, name: &str) {
    items.retain(|item| item.name != name);  // removes every match
}

// Fix: find the position and remove exactly one
fn remove_first_match(items: &mut Vec<Item>, name: &str) -> bool {
    if let Some(pos) = items.iter().position(|item| item.name == name) {
        items.remove(pos);
        true
    } else {
        false
    }
}
```

#### Incorrect Arithmetic

Does the calculation match the doc comment's stated formula? Common variants: forgetting to multiply by quantity (summing only unit prices), integer division truncation in percentage calculations, using the wrong operator (e.g. `*` instead of `+`). In `total = subtotal - (subtotal * discount_percent / 100)`, the division is last — and integer division truncates, which is usually correct for cents (you can't charge fractional cents), but only if the order of operations is right.

```rust
// Bug: doesn't multiply by quantity
let subtotal: u64 = order.items.iter().map(|item| item.price_cents).sum();

// Fix: include quantity
let subtotal: u64 = order.items.iter()
    .map(|item| item.price_cents * item.quantity as u64)
    .sum();
```

#### Missing Edge-Case Guards

What happens when the input is empty? What happens when the discount is 100%? What if an order has no items? A function that works for "normal" inputs but panics or produces nonsense on edge cases is a bug. Every `if let` that doesn't have an `else` is a candidate for a missing guard.

```rust
// Bug: item_count returns distinct item count, not total quantity
fn item_count(&self, order_id: u64) -> Option<usize> {
    self.orders.iter()
        .find(|o| o.id == order_id)
        .map(|order| order.items.len())  // counts entries, not quantities
}

// Fix: sum the quantities
fn item_count(&self, order_id: u64) -> Option<usize> {
    self.orders.iter()
        .find(|o| o.id == order_id)
        .map(|order| order.items.iter().map(|i| i.quantity as usize).sum())
}
```

#### Semantic Mismatch

The function name or doc comment promises one thing, but the implementation does something else. `remove_item` that removes all matching items instead of "the first" is a semantic bug. `item_count` that returns the number of distinct item types rather than the total quantity is a semantic bug. These are the hardest to spot because the code looks correct in isolation — the bug is in the *contract*, not the logic.

### How to Write a Good PR Review Comment

A review comment has three parts: **observation**, **impact**, and **suggestion**. Keep it specific and non-judgmental. Never say "this is wrong" — say "this doesn't match the documented behavior."

**Bad:**
```
This is broken. Fix it.
```

**Good:**
```
`remove_item` uses `retain`, which removes every item matching
`item_name`. The doc comment says it removes "the first item" —
this will delete all matching items instead. Suggestion: use
`position()` to find the first match, then `remove(pos)`.

This is covered by the test `remove_item_removes_only_first_occurrence`,
which expects exactly one Widget to survive when two are added.
```

The good comment tells the author:
- What the bug is (retain removes all, not first).
- Why it matters (violates the documented contract).
- How to fix it (use position + remove).
- Which test catches it (so the author can verify the fix).

### The Interview "Debug and Fix" Flow

When an interviewer hands you buggy code, they are not looking for you to spot every bug in 30 seconds. They are evaluating your process:

1. **Read the tests aloud.** "I see a test called `total_calculates_correctly_with_quantities` that expects 7000. Let me trace that..."
2. **Pick one failing test and reproduce the failure mentally.** Walk through the code line-by-line with the test's input values.
3. **Identify the root cause, not the symptom.** "The total is 3000 instead of 7000 because the `total` method sums `price_cents` but never multiplies by `quantity`."
4. **Propose a fix and verify it against the test.** "If we change the map to `item.price_cents * item.quantity as u64`, the total becomes 3*1000 + 2*2000 = 7000. That matches the test expectation."
5. **Check if the fix breaks anything else.** "This change doesn't affect the empty-order test because an empty iterator sums to zero either way."

This five-step flow is what separates a "found three bugs" candidate from a "demonstrated systematic debugging" candidate. The latter gets the offer.

### Reading the Exercise Code

Open `exercises/src/lib.rs`. The code implements an `OrderProcessor` — a simple order management system with create, add-item, remove-item, total-calculation, discount, finalize, and cancel operations. It compiles and passes clippy clean. But four tests fail. Look for `// BUG:` comments — they mark the intentional bugs in lines 88–90, 101, and 161. However, not every bug is labeled. Part of the exercise is verifying that the unlabeled code is genuinely correct.

The bugs in this scaffold fall into three of the four categories above: incorrect arithmetic (total doesn't multiply by quantity), semantic mismatch (remove_item removes all, not first; item_count counts distinct types, not quantities), and the effects compound — the discount test fails because the total is wrong even before the discount is applied.

## Common Pitfalls
- **Fixing symptoms instead of root causes.** If `total_applies_discount` fails, check whether `total` returns the right undiscounted subtotal first. If the subtotal is wrong, the discount will be too. Fix the arithmetic, not the discount formula.
- **Not testing edge cases.** After fixing, run the full suite, not just the 1–2 tests you think relate to your fix. A fix to `remove_item` might break `finalize_prevents_modifications` if you introduce a borrow-split.
- **Focusing on style over substance.** Renaming variables or adding whitespace is not code review. Find the logic bugs first. Style feedback is secondary and only worth mentioning if it affects readability in a way that hides bugs.

## Key Terms
- **Code review:** a systematic examination of source code by someone other than the author, intended to find bugs and improve quality before merging.
- **Logic bug:** code that compiles and runs without panicking, but produces an incorrect result (wrong sum, wrong count, wrong behavior).
- **Edge case:** an input or state at the extreme of what's valid — empty collection, zero value, maximum value, already-canceled order.
- **Regression:** a change that breaks previously working behavior. A fix that introduces a regression is not a complete fix.
- **PR feedback:** a comment on a pull request that describes an observation, its impact, and a suggested change, written to be actionable and non-judgmental.

## Exercise

Open `exercises/src/lib.rs`. The file contains a working-but-flawed `OrderProcessor`. The code compiles and is clippy-clean. Your task:

1. Run `cargo test -p module-098-exercises` and note the four failing tests.
2. Read `exercises/tests/module_098.rs` to understand the expected behavior — the tests are your specification.
3. Read the implementation in `exercises/src/lib.rs`. Look for `// BUG:` comments marking intentional bugs (lines 88, 101, 161). Verify the unlabeled code too — bugs may hide where you don't expect them.
4. For each bug, write a mental PR review comment (observation, impact, suggestion) before fixing the code.
5. Fix the bugs. The tests that currently fail should pass, and no previously-passing test should break.
6. Run the full suite to confirm:
   ```bash
   cargo test -p module-098-exercises
   ```
   All 18 tests should pass.

There are no `todo!()` macros or `panic!()` calls in this module — the code compiles. The challenge is finding logic errors. Compare with `solutions/` only after you have made a genuine attempt to find and fix every bug yourself.

## Further Reading
- [How to Do Code Reviews Like a Human (blog)](https://mtlynch.io/human-code-reviews-1/) — a widely-cited series on review communication.
- [Google's Code Review Developer Guide](https://google.github.io/eng-practices/review/) — the industry standard for review practices.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — what well-designed Rust APIs look like, useful for evaluating "is this the right abstraction?" during review.
- [The Rust Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html) — reading tests before code is a review skill; reinforce it here.

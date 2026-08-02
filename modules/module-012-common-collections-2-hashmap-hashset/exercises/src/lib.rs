//! Module 012: Common Collections II — `HashMap<K, V>` and `HashSet<T>`.
//!
//! Fill in the `TODO(module-012)` bodies below so the integration tests in
//! `tests/module_012.rs` pass.

use std::collections::{HashMap, HashSet};

/// Builds a map from each player (in `players`) to their score (in `scores`).
///
/// If the slices have different lengths, pairs only as far as the shorter
/// slice allows.
pub fn build_scoreboard<'a>(players: &[&'a str], scores: &[u32]) -> HashMap<&'a str, u32> {
    // TODO(module-012): zip `players.iter()` with `scores.iter()` and
    // `.collect()` the pairs into a HashMap. Dereference with `.copied()` so
    // the map stores `u32`, not `&u32`.
    let _count = players.len().min(scores.len());
    HashMap::new()
}

/// Counts how often each whitespace-separated word appears in `text`.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    // TODO(module-012): iterate `text.split_whitespace()`. For each word use
    // the entry API: `map.entry(word.to_string()).or_insert(0)` and then add
    // `1` to the value. `or_insert` returns a `&mut usize` you can increment.
    let _words = text.split_whitespace().count();
    HashMap::new()
}

/// Returns the name of the player with the highest score, or `None` if the
/// map is empty.
pub fn top_scorer<'a>(scoreboard: &HashMap<&'a str, u32>) -> Option<&'a str> {
    // TODO(module-012): use `scoreboard.iter().max_by_key(|(_, &score)| score)`
    // to find the winning entry, then return its key.
    let _count = scoreboard.len();
    None
}

/// Collects the set of every word that appears in any of `texts`.
pub fn unique_words(texts: &[&str]) -> HashSet<String> {
    // TODO(module-012): iterate the texts, `split_whitespace()` each one, and
    // `.insert(word.to_string())` into a HashSet. You can insert from inside
    // nested loops, or use `.flat_map(...).collect()`.
    let _count = texts.len();
    HashSet::new()
}

/// Returns the elements present in *both* `a` and `b`.
pub fn intersection(a: &HashSet<i32>, b: &HashSet<i32>) -> HashSet<i32> {
    // TODO(module-012): build the result set by keeping only elements of `a`
    // that are also in `b` — `a.iter().filter(|n| b.contains(n))`, or use the
    // `&` operator between two sets (it produces an owned intersection).
    let _counts = (a.len(), b.len());
    HashSet::new()
}

/// Counts how many entries in `scoreboard` scored at least `minimum`.
pub fn count_above(scoreboard: &HashMap<&str, u32>, minimum: u32) -> usize {
    // TODO(module-012): count entries whose value is `>= minimum`.
    let _count = scoreboard.len();
    let _minimum = minimum;
    0
}

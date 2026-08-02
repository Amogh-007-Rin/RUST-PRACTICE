//! Module 012: Common Collections II — `HashMap<K, V>` and `HashSet<T>`
//! (reference solution).

use std::collections::{HashMap, HashSet};

/// Builds a map from each player (in `players`) to their score (in `scores`).
///
/// If the slices have different lengths, pairs only as far as the shorter
/// slice allows.
pub fn build_scoreboard<'a>(players: &[&'a str], scores: &[u32]) -> HashMap<&'a str, u32> {
    players
        .iter()
        .zip(scores.iter())
        .map(|(&p, &s)| (p, s))
        .collect()
}

/// Counts how often each whitespace-separated word appears in `text`.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut frequencies = HashMap::new();
    for word in text.split_whitespace() {
        *frequencies.entry(word.to_string()).or_insert(0) += 1;
    }
    frequencies
}

/// Returns the name of the player with the highest score, or `None` if the
/// map is empty.
pub fn top_scorer<'a>(scoreboard: &HashMap<&'a str, u32>) -> Option<&'a str> {
    scoreboard
        .iter()
        .max_by_key(|(_, &score)| score)
        .map(|(&name, _)| name)
}

/// Collects the set of every word that appears in any of `texts`.
pub fn unique_words(texts: &[&str]) -> HashSet<String> {
    texts
        .iter()
        .flat_map(|text| text.split_whitespace())
        .map(str::to_string)
        .collect()
}

/// Returns the elements present in *both* `a` and `b`.
pub fn intersection(a: &HashSet<i32>, b: &HashSet<i32>) -> HashSet<i32> {
    a.iter().filter(|n| b.contains(n)).copied().collect()
}

/// Counts how many entries in `scoreboard` scored at least `minimum`.
pub fn count_above(scoreboard: &HashMap<&str, u32>, minimum: u32) -> usize {
    scoreboard
        .values()
        .filter(|&&score| score >= minimum)
        .count()
}

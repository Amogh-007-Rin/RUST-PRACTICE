//! Module 023: Iterators II — reference solution.

/// Returns a new vector with the squares of the even numbers in `v`,
/// in the original relative order.
pub fn squares_of_evens(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).collect()
}

/// Returns the sum of the squares of all values in `v`.
pub fn sum_of_squares(v: &[i32]) -> i64 {
    v.iter().map(|&x| x as i64 * x as i64).sum()
}

/// Counts the whitespace-separated words in `s`.
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Counts the words in `s` whose length is at most `max_len`.
pub fn count_short_words(s: &str, max_len: usize) -> usize {
    s.split_whitespace().filter(|w| w.len() <= max_len).count()
}

/// Returns the longest word in `s`, or `None` if `s` has no words.
pub fn longest_word(s: &str) -> Option<&str> {
    s.split_whitespace().max_by_key(|w| w.len())
}

/// Counts the positive values in `v` using `fold`.
pub fn count_positive(v: &[i32]) -> usize {
    v.iter()
        .fold(0, |acc, &x| if x > 0 { acc + 1 } else { acc })
}

/// Computes the dot product of `a` and `b`, or `None` if their lengths
/// differ.
pub fn dot_product(a: &[i32], b: &[i32]) -> Option<i64> {
    if a.len() != b.len() {
        return None;
    }
    Some(a.iter().zip(b).map(|(&x, &y)| x as i64 * y as i64).sum())
}

/// Returns `true` if `haystack` contains any of the `needles`.
pub fn contains_any(needles: &[&str], haystack: &str) -> bool {
    needles.iter().any(|n| haystack.contains(n))
}

/// Returns `true` if every value in `v` is even.
pub fn is_all_even(v: &[i32]) -> bool {
    v.iter().all(|&x| x % 2 == 0)
}

//! Module 011: Common Collections I — `Vec<T>`.
//!
//! Fill in the `TODO(module-011)` bodies below so the integration tests in
//! `tests/module_011.rs` pass.

/// Returns the sum of the even numbers in `numbers`.
pub fn sum_even(numbers: &[i32]) -> i32 {
    // TODO(module-011): iterate over `numbers` and add up only the even
    // values. `numbers.iter()` gives you a `&i32` per element; check
    // evenness with `n.is_multiple_of(2)`.
    let _len = numbers.len();
    0
}

/// Appends every value in `additions` to `items`, in order.
pub fn push_many(items: &mut Vec<i32>, additions: &[i32]) {
    // TODO(module-011): loop over `additions` and push each value onto
    // `items` with `items.push(value)`.
    for _ in additions {
        items.push(0);
    }
}

/// Returns the median (middle value) of `numbers`, sorting it in place.
///
/// For an odd-length slice, the median is the middle element. For an
/// even-length slice it is the average of the two middle elements.
pub fn median(numbers: &mut [i32]) -> f64 {
    // TODO(module-011): sort the slice with `numbers.sort_unstable()`, then
    // index into it. Remember that the two middle indices of an even-length
    // slice are `len / 2 - 1` and `len / 2`.
    let _len = numbers.len();
    0.0
}

/// Removes the first occurrence of `value` from `items`.
///
/// Returns `true` if a value was removed, `false` otherwise.
pub fn remove_value(items: &mut Vec<i32>, value: i32) -> bool {
    // TODO(module-011): use `items.iter().position(...)` to find the index of
    // the first element equal to `value`. If found, `items.remove(index)` and
    // return `true`; otherwise return `false`.
    let _value = value;
    items.pop();
    false
}

/// Maps each word to the number of characters it contains.
pub fn word_lengths(words: &[String]) -> Vec<usize> {
    // TODO(module-011): build a `Vec<usize>` where element *i* is
    // `words[i].len()`. `words.iter().map(...).collect()` is the idiomatic
    // way; collect() knows to build a `Vec` from the return type.
    let _count = words.len();
    Vec::new()
}

/// Returns the arithmetic mean of `numbers`, or `None` if it is empty.
pub fn mean(numbers: &[f64]) -> Option<f64> {
    // TODO(module-011): return `None` for an empty slice. Otherwise sum the
    // values with `numbers.iter().sum::<f64>()` and divide by
    // `numbers.len() as f64`.
    let _len = numbers.len();
    None
}

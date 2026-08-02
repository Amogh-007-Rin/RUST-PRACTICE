//! Module 011: Common Collections I — `Vec<T>` (reference solution).

/// Returns the sum of the even numbers in `numbers`.
pub fn sum_even(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&&n| n % 2 == 0).sum()
}

/// Appends every value in `additions` to `items`, in order.
pub fn push_many(items: &mut Vec<i32>, additions: &[i32]) {
    for &value in additions {
        items.push(value);
    }
}

/// Returns the median (middle value) of `numbers`, sorting it in place.
///
/// For an odd-length slice, the median is the middle element. For an
/// even-length slice it is the average of the two middle elements.
pub fn median(numbers: &mut [i32]) -> f64 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    if numbers.len().is_multiple_of(2) {
        (numbers[mid - 1] + numbers[mid]) as f64 / 2.0
    } else {
        numbers[mid] as f64
    }
}

/// Removes the first occurrence of `value` from `items`.
///
/// Returns `true` if a value was removed, `false` otherwise.
pub fn remove_value(items: &mut Vec<i32>, value: i32) -> bool {
    match items.iter().position(|&item| item == value) {
        Some(index) => {
            items.remove(index);
            true
        }
        None => false,
    }
}

/// Maps each word to the number of characters it contains.
pub fn word_lengths(words: &[String]) -> Vec<usize> {
    words.iter().map(String::len).collect()
}

/// Returns the arithmetic mean of `numbers`, or `None` if it is empty.
pub fn mean(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        Some(numbers.iter().sum::<f64>() / numbers.len() as f64)
    }
}

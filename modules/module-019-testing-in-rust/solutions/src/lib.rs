//! Module 019: Testing in Rust (reference solution).

/// Converts Fahrenheit to Celsius: `(f - 32) * 5 / 9`.
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Counts the whitespace-separated words in `text`.
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Returns `true` if `s` reads the same forwards and backwards.
pub fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

/// Returns the `n`-th Fibonacci number: 0, 1, 1, 2, 3, 5, 8, ...
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Returns `true` if `score` is a valid grade: between 50 and 100 inclusive.
pub fn valid_grade(score: u8) -> bool {
    (50..=100).contains(&score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing_point_is_zero_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    }

    #[test]
    fn boiling_point_is_100_celsius() {
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn empty_text_has_zero_words() {
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn racecar_is_a_palindrome() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn abca_is_not_a_palindrome() {
        assert!(!is_palindrome("abca"));
    }

    #[test]
    fn fibonacci_small_sequence() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
    }

    #[test]
    fn grade_boundaries_are_inclusive() {
        assert!(valid_grade(50));
        assert!(valid_grade(100));
        assert!(!valid_grade(49));
        assert!(!valid_grade(101));
    }
}

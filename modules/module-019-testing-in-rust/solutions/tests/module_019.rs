use module_019_solutions::{
    fahrenheit_to_celsius, fibonacci, is_palindrome, valid_grade, word_count,
};

#[test]
fn fahrenheit_freezing_point_is_zero_celsius() {
    assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
}

#[test]
fn fahrenheit_boiling_point_is_100_celsius() {
    assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
}

#[test]
fn fahrenheit_room_temperature_is_20_celsius() {
    assert_eq!(fahrenheit_to_celsius(68.0), 20.0);
}

#[test]
fn word_count_counts_whitespace_separated_words() {
    assert_eq!(word_count("hello world"), 2);
}

#[test]
fn word_count_of_empty_text_is_zero() {
    assert_eq!(word_count(""), 0);
    assert_eq!(word_count("   "), 0);
}

#[test]
fn word_count_ignores_punctuation_but_not_spaces() {
    assert_eq!(word_count("a, b. c!"), 3);
}

#[test]
fn is_palindrome_accepts_palindromes() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("a"));
}

#[test]
fn is_palindrome_rejects_non_palindromes() {
    assert!(!is_palindrome("hello"));
    assert!(!is_palindrome("abca"));
}

#[test]
fn fibonacci_known_values() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
}

#[test]
fn valid_grade_accepts_the_pass_range() {
    assert!(valid_grade(50));
    assert!(valid_grade(75));
    assert!(valid_grade(100));
}

#[test]
fn valid_grade_rejects_scores_below_50() {
    assert!(!valid_grade(0));
    assert!(!valid_grade(49));
}

#[test]
fn valid_grade_rejects_scores_above_100() {
    assert!(!valid_grade(101));
    assert!(!valid_grade(200));
}

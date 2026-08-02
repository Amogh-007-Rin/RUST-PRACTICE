use module_023_exercises::{
    contains_any, count_positive, count_short_words, count_words, dot_product, is_all_even,
    longest_word, squares_of_evens, sum_of_squares,
};

#[test]
fn squares_of_evens_filters_then_maps() {
    assert_eq!(squares_of_evens(&[1, 2, 3, 4, 5]), vec![4, 16]);
    assert_eq!(squares_of_evens(&[1, 3, 5]), Vec::<i32>::new());
    assert_eq!(squares_of_evens(&[2, 4]), vec![4, 16]);
}

#[test]
fn sum_of_squares_adds_the_squares() {
    assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
    assert_eq!(sum_of_squares(&[]), 0);
}

#[test]
fn count_words_counts_whitespace_separated_words() {
    assert_eq!(count_words("the quick brown fox"), 4);
    assert_eq!(count_words("  a  b "), 2);
    assert_eq!(count_words(""), 0);
}

#[test]
fn count_short_words_filters_by_length() {
    assert_eq!(count_short_words("a bb ccc dddd", 2), 2);
    assert_eq!(count_short_words("a bb ccc", 0), 0);
}

#[test]
fn longest_word_finds_the_maximum_by_length() {
    assert_eq!(longest_word("a bb ccc d"), Some("ccc"));
    assert_eq!(longest_word(""), None);
}

#[test]
fn count_positive_uses_fold() {
    assert_eq!(count_positive(&[1, -2, 3, -4, 5]), 3);
    assert_eq!(count_positive(&[-1, -2]), 0);
    assert_eq!(count_positive(&[]), 0);
}

#[test]
fn dot_product_works_on_equal_lengths() {
    assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), Some(32));
    assert_eq!(dot_product(&[1, 2], &[1, 2]), Some(5));
}

#[test]
fn dot_product_rejects_mismatched_lengths() {
    assert_eq!(dot_product(&[1, 2, 3], &[4, 5]), None);
    assert_eq!(dot_product(&[], &[1]), None);
}

#[test]
fn contains_any_uses_any() {
    assert!(contains_any(&["rust", "python"], "i love rust"));
    assert!(!contains_any(&["java", "go"], "i love rust"));
    assert!(contains_any(&["a"], "a"));
}

#[test]
fn is_all_even_uses_all() {
    assert!(is_all_even(&[2, 4, 6]));
    assert!(!is_all_even(&[2, 3, 4]));
    assert!(is_all_even(&[]));
}

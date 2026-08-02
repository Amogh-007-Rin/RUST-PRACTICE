use module_011_solutions::{mean, median, push_many, remove_value, sum_even, word_lengths};

#[test]
fn sum_even_filters_odd_numbers() {
    assert_eq!(sum_even(&[1, 2, 3, 4, 5, 6]), 12);
}

#[test]
fn sum_even_is_zero_without_evens() {
    assert_eq!(sum_even(&[1, 3, 5]), 0);
}

#[test]
fn sum_even_of_empty_slice_is_zero() {
    assert_eq!(sum_even(&[]), 0);
}

#[test]
fn push_many_appends_in_order() {
    let mut items = vec![1, 2];
    push_many(&mut items, &[3, 4, 5]);
    assert_eq!(items, vec![1, 2, 3, 4, 5]);
}

#[test]
fn push_many_with_empty_additions_is_a_noop() {
    let mut items = vec![10];
    push_many(&mut items, &[]);
    assert_eq!(items, vec![10]);
}

#[test]
fn median_of_odd_length_is_the_middle_element() {
    let mut numbers = vec![3, 1, 2];
    assert_eq!(median(&mut numbers), 2.0);
}

#[test]
fn median_of_even_length_averages_the_two_middles() {
    let mut numbers = vec![4, 1, 3, 2];
    assert_eq!(median(&mut numbers), 2.5);
}

#[test]
fn remove_value_removes_only_the_first_occurrence() {
    let mut items = vec![1, 7, 2, 7, 3];
    assert!(remove_value(&mut items, 7));
    assert_eq!(items, vec![1, 2, 7, 3]);
}

#[test]
fn remove_value_reports_missing_values() {
    let mut items = vec![1, 2, 3];
    assert!(!remove_value(&mut items, 99));
    assert_eq!(items, vec![1, 2, 3]);
}

#[test]
fn word_lengths_maps_words_to_character_counts() {
    let words = vec!["hi".to_string(), "rust".to_string()];
    assert_eq!(word_lengths(&words), vec![2, 4]);
}

#[test]
fn word_lengths_of_empty_list_is_empty() {
    assert_eq!(word_lengths(&[]), Vec::<usize>::new());
}

#[test]
fn mean_averages_numbers() {
    assert_eq!(mean(&[2.0, 4.0, 6.0]), Some(4.0));
}

#[test]
fn mean_of_empty_slice_is_none() {
    assert_eq!(mean(&[]), None);
}

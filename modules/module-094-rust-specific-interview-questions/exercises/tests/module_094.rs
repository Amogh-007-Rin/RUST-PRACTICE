//! Integration tests for Module 094 — ownership & borrowing gotchas.
//!
//! Run with: `cargo test -p module-094-exercises`

use module_094_exercises::{
    both_mut, pop_line, pop_option, remove_evens, remove_first, shout, Counter, Searcher,
};

#[test]
fn pop_option_moves_value_out_and_leaves_none() {
    let mut opt = Some(String::from("hello"));
    let value = pop_option(&mut opt);
    assert_eq!(value, "hello");
    assert!(opt.is_none(), "take() must leave None behind");
}

#[test]
fn pop_option_with_copy_values() {
    let mut opt = Some(42_i32);
    assert_eq!(pop_option(&mut opt), 42);
    assert!(opt.is_none());
}

#[test]
fn remove_first_takes_head_and_shifts() {
    let mut v = vec![1, 2, 3];
    assert_eq!(remove_first(&mut v), 1);
    assert_eq!(v, vec![2, 3]);
    assert_eq!(remove_first(&mut v), 2);
    assert_eq!(remove_first(&mut v), 3);
    assert!(v.is_empty());
}

#[test]
fn remove_first_leaves_rest_ordered() {
    let mut v = vec!["a", "b", "c", "d"];
    assert_eq!(remove_first(&mut v), "a");
    assert_eq!(v, vec!["b", "c", "d"]);
}

#[test]
fn both_mut_returns_two_disjoint_borrows() {
    let mut c = Counter { left: 1, right: 2 };
    {
        let (left, right) = both_mut(&mut c);
        // These are disjoint borrows: mutating one can't affect the other.
        *left += 10;
        *right += 100;
        assert_eq!(*left, 11);
        assert_eq!(*right, 102);
    }
    assert_eq!((c.left, c.right), (11, 102));
}

#[test]
fn searcher_finds_first_match_with_input_lifetime() {
    let haystack = String::from("the quick brown fox jumps over the lazy dog");
    let needle = String::from("quick");
    let searcher = Searcher::new(&haystack, &needle);
    assert_eq!(searcher.first_match(), Some("quick"));
}

#[test]
fn searcher_missing_needle_is_none() {
    let haystack = String::from("the quick brown fox");
    let needle = String::from("zzz");
    let searcher = Searcher::new(&haystack, &needle);
    assert_eq!(searcher.first_match(), None);
}

#[test]
fn searcher_multiple_occurrences_returns_first() {
    let haystack = String::from("banana");
    let needle = String::from("na");
    let searcher = Searcher::new(&haystack, &needle);
    assert_eq!(searcher.first_match(), Some("na"));
}

#[test]
fn shout_returns_owned_uppercase() {
    assert_eq!(shout("rust"), "RUST");
    assert_eq!(shout("Hello, World!"), "HELLO, WORLD!");
    assert_eq!(shout(""), "");
    assert_eq!(shout("mixed CASE 123"), "MIXED CASE 123");
}

#[test]
fn remove_evens_keeps_odds_and_counts() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let removed = remove_evens(&mut v);
    assert_eq!(removed, 3);
    assert_eq!(v, vec![1, 3, 5]);
}

#[test]
fn remove_evens_all_or_none() {
    let mut all_even = vec![2, 4, 6];
    assert_eq!(remove_evens(&mut all_even), 3);
    assert!(all_even.is_empty());

    let mut none_even = vec![1, 3, 5];
    assert_eq!(remove_evens(&mut none_even), 0);
    assert_eq!(none_even, vec![1, 3, 5]);
}

#[test]
fn remove_evens_empty_vec() {
    let mut v: Vec<i32> = vec![];
    assert_eq!(remove_evens(&mut v), 0);
    assert!(v.is_empty());
}

#[test]
fn pop_line_splits_one_line_at_a_time() {
    let mut input = "first line\nsecond line\nthird";
    assert_eq!(pop_line(&mut input), Some("first line"));
    assert_eq!(pop_line(&mut input), Some("second line"));
    assert_eq!(pop_line(&mut input), Some("third"));
    assert_eq!(pop_line(&mut input), None);
}

#[test]
fn pop_line_trailing_newline_ends_with_empty_input() {
    let mut input = "a\nb\n";
    assert_eq!(pop_line(&mut input), Some("a"));
    assert_eq!(pop_line(&mut input), Some("b"));
    assert_eq!(pop_line(&mut input), None);
}

#[test]
fn pop_line_single_line_no_newline() {
    let mut input = "only line";
    assert_eq!(pop_line(&mut input), Some("only line"));
    assert_eq!(pop_line(&mut input), None);
}

#[test]
fn pop_line_empty_input() {
    let mut input = "";
    assert_eq!(pop_line(&mut input), None);
}

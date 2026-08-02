use module_006_solutions::{first_word, shout, slice_range, word_count};

#[test]
fn first_word_simple() {
    assert_eq!(first_word("hello world"), "hello");
}

#[test]
fn first_word_single() {
    assert_eq!(first_word("only"), "only");
}

#[test]
fn first_word_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn first_word_does_not_steal_ownership() {
    let phrase = String::from("ownership slices");
    assert_eq!(first_word(&phrase), "ownership");
    assert_eq!(phrase, "ownership slices");
}

#[test]
fn slice_range_mid() {
    assert_eq!(slice_range("abcdef", 1, 4), "bcd");
}

#[test]
fn slice_range_whole() {
    assert_eq!(slice_range("rust", 0, 4), "rust");
}

#[test]
fn slice_range_empty() {
    assert_eq!(slice_range("rust", 2, 2), "");
}

#[test]
fn word_count_basic() {
    assert_eq!(word_count("one two three"), 3);
}

#[test]
fn word_count_whitespace_variety() {
    assert_eq!(word_count("  spaced \n out  "), 2);
}

#[test]
fn word_count_empty() {
    assert_eq!(word_count(""), 0);
}

#[test]
fn shout_uppercases() {
    assert_eq!(shout("hello"), "HELLO");
}

#[test]
fn shout_keeps_borrower_usable() {
    let s = String::from("hi");
    assert_eq!(shout(&s), "HI");
    assert_eq!(s, "hi");
}

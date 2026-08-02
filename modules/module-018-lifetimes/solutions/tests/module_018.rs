use module_018_solutions::{first_and_last, first_word, last, longest, longest_line, Book};

#[test]
fn first_word_returns_up_to_whitespace() {
    assert_eq!(first_word("hello world"), "hello");
}

#[test]
fn first_word_of_a_single_word_is_the_word() {
    assert_eq!(first_word("rust"), "rust");
}

#[test]
fn first_word_of_empty_input_is_empty() {
    assert_eq!(first_word(""), "");
    assert_eq!(first_word("   "), "");
}

#[test]
fn longest_returns_the_longer_string() {
    assert_eq!(longest("ab", "cdef"), "cdef");
    assert_eq!(longest("abcd", "ef"), "abcd");
}

#[test]
fn last_returns_after_the_last_whitespace() {
    assert_eq!(last("hello world"), "world");
    assert_eq!(last("a b c"), "c");
}

#[test]
fn longest_line_returns_the_longest_string() {
    let lines = vec![
        "short".to_string(),
        "a much longer line".to_string(),
        "tiny".to_string(),
    ];
    assert_eq!(longest_line(&lines), "a much longer line");
}

#[test]
fn book_returns_its_title() {
    let title = String::from("The Rust Book");
    let author = String::from("Community");
    let book = Book::new(&title, &author);
    assert_eq!(book.title(), "The Rust Book");
}

#[test]
fn book_citation_puts_author_first() {
    let title = String::from("The Rust Book");
    let author = String::from("Community");
    let book = Book::new(&title, &author);
    assert_eq!(book.citation(), "Community — The Rust Book");
}

#[test]
fn first_and_last_splits_the_ends() {
    assert_eq!(first_and_last("hello world"), ("hello", "world"));
    assert_eq!(first_and_last("rust"), ("rust", "rust"));
}

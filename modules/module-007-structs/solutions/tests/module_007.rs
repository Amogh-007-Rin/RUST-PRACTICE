use module_007_solutions::{Book, Point};

#[test]
fn book_new_creates_book() {
    let b = Book::new("The Rust Programming Language", "Steve Klabnik", 500);
    assert_eq!(b.title, "The Rust Programming Language");
    assert_eq!(b.author, "Steve Klabnik");
    assert_eq!(b.pages, 500);
}

#[test]
fn book_summary_format() {
    let b = Book::new("Foundations", "Ada", 250);
    assert_eq!(b.summary(), "\"Foundations\" by Ada (250 pages)");
}

#[test]
fn book_is_long_threshold() {
    assert!(Book::new("Tome", "A", 401).is_long());
    assert!(!Book::new("Pamphlet", "B", 400).is_long());
}

#[test]
fn point_distance_basic() {
    let a = Point(0.0, 0.0);
    let b = Point(3.0, 4.0);
    assert_eq!(a.distance(&b), 5.0);
}

#[test]
fn point_distance_reversed() {
    let a = Point(1.0, 1.0);
    let b = Point(4.0, 5.0);
    assert_eq!(b.distance(&a), 5.0);
}

#[test]
fn point_distance_same_point() {
    let a = Point(1.5, 2.5);
    assert_eq!(a.distance(&a), 0.0);
}

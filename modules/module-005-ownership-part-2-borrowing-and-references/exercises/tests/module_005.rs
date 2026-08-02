use module_005_exercises::{add_one, first_char, swap, total_length};

#[test]
fn first_char_returns_first() {
    assert_eq!(first_char("rust"), Some('r'));
}

#[test]
fn first_char_empty_is_none() {
    assert_eq!(first_char(""), None);
}

#[test]
fn add_one_mutates_in_place() {
    let mut x = 41;
    add_one(&mut x);
    assert_eq!(x, 42);
}

#[test]
fn add_one_multiple_calls() {
    let mut x = 0;
    add_one(&mut x);
    add_one(&mut x);
    assert_eq!(x, 2);
}

#[test]
fn total_length_adds_borrowed_strings() {
    let a = String::from("foo");
    let b = String::from("bar");
    assert_eq!(total_length(&a, &b), 6);
    assert_eq!(a, "foo");
    assert_eq!(b, "bar");
}

#[test]
fn swap_exchanges_values() {
    let mut a = 1;
    let mut b = 2;
    swap(&mut a, &mut b);
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}

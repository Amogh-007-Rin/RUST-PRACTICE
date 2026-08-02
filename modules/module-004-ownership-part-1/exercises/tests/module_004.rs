use module_004_exercises::{byte_len, concat, copy_of};

#[test]
fn byte_len_measures_string() {
    assert_eq!(byte_len(String::from("hello")), 5);
}

#[test]
fn byte_len_empty() {
    assert_eq!(byte_len(String::new()), 0);
}

#[test]
fn byte_len_utf8_bytes() {
    assert_eq!(byte_len(String::from("héllo")), 6);
}

#[test]
fn copy_of_returns_both() {
    let (copy, original) = copy_of(String::from("Ada"));
    assert_eq!(copy, "Ada");
    assert_eq!(original, "Ada");
}

#[test]
fn copy_of_does_not_alias() {
    let (mut copy, _original) = copy_of(String::from("Ada"));
    copy.push('!');
    assert_eq!(copy, "Ada!");
}

#[test]
fn concat_joins_two_strings() {
    let joined = concat(String::from("foo"), String::from("bar"));
    assert_eq!(joined, "foobar");
}

#[test]
fn concat_with_empty() {
    assert_eq!(concat(String::new(), String::from("x")), "x");
}

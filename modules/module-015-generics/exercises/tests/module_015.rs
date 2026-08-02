use module_015_exercises::{combine, first_or, largest, Maybe, Pair};

#[test]
fn largest_finds_the_maximum() {
    assert_eq!(largest(&[3, 7, 1, 9, 2]), Some(&9));
}

#[test]
fn largest_works_with_strings() {
    assert_eq!(largest(&["apple", "kiwi"]), Some(&"kiwi"));
}

#[test]
fn largest_of_empty_slice_is_none() {
    assert_eq!(largest(&[] as &[i32]), None);
}

#[test]
fn first_or_returns_first_element_when_present() {
    let numbers = [5, 6];
    assert_eq!(first_or(&numbers, &0), &5);
}

#[test]
fn first_or_falls_back_when_empty() {
    assert_eq!(first_or(&[] as &[i32], &0), &0);
    let names: [String; 0] = [];
    assert_eq!(first_or(&names, &"default".to_string()), "default");
}

#[test]
fn pair_exposes_both_fields() {
    let pair = Pair {
        first: "x",
        second: 42,
    };
    assert_eq!(pair.first(), &"x");
    assert_eq!(pair.second(), &42);
}

#[test]
fn pair_swap_reverses_the_types() {
    let pair = Pair {
        first: "x",
        second: 42,
    };
    let swapped = pair.swap();
    assert_eq!(swapped.first(), &42);
    assert_eq!(swapped.second(), &"x");
}

#[test]
fn maybe_is_just_detects_both_cases() {
    assert!(Maybe::Just(1).is_just());
    assert!(!Maybe::<i32>::Nothing.is_just());
}

#[test]
fn maybe_unwrap_or_returns_just_value() {
    assert_eq!(Maybe::Just(7).unwrap_or(0), 7);
}

#[test]
fn maybe_unwrap_or_returns_default_for_nothing() {
    assert_eq!(Maybe::<i32>::Nothing.unwrap_or(9), 9);
}

#[test]
fn combine_concatenates_vectors() {
    assert_eq!(combine(vec![1, 2], vec![3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn combine_works_with_any_element_type() {
    let strings: Vec<String> = vec!["a".to_string()];
    assert_eq!(combine(strings, Vec::new()), vec!["a".to_string()]);
}

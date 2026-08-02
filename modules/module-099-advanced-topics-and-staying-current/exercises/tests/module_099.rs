use module_099_exercises::*;

#[test]
fn test_fixed_size_array_sum() {
    assert_eq!(fixed_size_array_sum([1, 2, 3, 4, 5]), 15);
    assert_eq!(
        fixed_size_array_sum::<0>([]),
        0,
        "zero-length array sums to 0"
    );
    assert_eq!(fixed_size_array_sum([42]), 42);
    assert_eq!(fixed_size_array_sum([10, 20, 30]), 60);
}

#[test]
fn test_gat_container_for_vec() {
    let v: Vec<i32> = vec![10, 20, 30];
    assert_eq!(Container::get(&v, 0), Some(&10));
    assert_eq!(Container::get(&v, 1), Some(&20));
    assert_eq!(Container::get(&v, 2), Some(&30));
    assert_eq!(Container::get(&v, 3), None);
    assert_eq!(Container::get(&v, 100), None);
}

#[test]
fn test_const_evaluation() {
    assert_eq!(demonstrate_const_evaluation(), 120);
}

#[test]
fn test_static_assertion() {
    let msg = demonstrate_static_assertion();
    assert_eq!(msg, "static assertion passed");
}

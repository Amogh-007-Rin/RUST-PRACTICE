use module_009_solutions::{math, shout_sum, utils};

#[test]
fn math_add() {
    assert_eq!(math::add(2, 3), 5);
}

#[test]
fn math_sub() {
    assert_eq!(math::sub(5, 2), 3);
}

#[test]
fn math_mul() {
    assert_eq!(math::mul(4, 3), 12);
}

#[test]
fn utils_shout() {
    assert_eq!(utils::shout("hi"), "HI!");
}

#[test]
fn utils_is_blank() {
    assert!(utils::is_blank(""));
    assert!(utils::is_blank("   "));
    assert!(!utils::is_blank("  x"));
}

#[test]
fn shout_sum_combines_modules() {
    assert_eq!(shout_sum(2, 3), "5!");
}

#[test]
fn shout_sum_negative() {
    assert_eq!(shout_sum(-2, -3), "-5!");
}

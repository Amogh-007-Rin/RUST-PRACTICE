use module_051_solutions::{
    align_of_messy, offset_of_y, optimized_size, size_of_messy, size_of_repr_c, Messy, Optimized,
    ReprC, Sample,
};
use std::mem::{align_of, size_of};

#[test]
fn messy_has_expected_size() {
    let s = size_of_messy();
    assert!(s >= 16, "Messy must be at least 16 bytes, got {s}");
    assert_eq!(s, size_of::<Messy>());
}

#[test]
fn messy_alignment_is_u64_alignment() {
    assert_eq!(align_of_messy(), align_of::<u64>());
    assert_eq!(align_of_messy(), align_of::<Messy>());
}

#[test]
fn sample_y_offset_is_after_x_and_padding() {
    let off = offset_of_y();
    assert_eq!(off, 8);
}

#[test]
fn repr_c_has_c_layout_size() {
    let s = size_of_repr_c();
    assert_eq!(s, size_of::<ReprC>());
    assert_eq!(s, 12);
}

#[test]
fn optimized_struct_is_smaller_than_messy() {
    let opt = optimized_size();
    let messy = size_of::<Messy>();
    assert!(
        opt < messy,
        "Optimized ({opt}) should be smaller than Messy ({messy})"
    );
    assert_eq!(opt, size_of::<Optimized>());
}

#[test]
fn optimized_struct_is_tight() {
    let opt = optimized_size();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(opt, 16);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(opt, 8);
}

#[test]
fn sample_size_is_correct() {
    let s = size_of::<Sample>();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(s, 24);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(s, 16);
}

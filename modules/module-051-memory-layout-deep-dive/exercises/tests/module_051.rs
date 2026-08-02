use module_051_exercises::{
    align_of_messy, offset_of_y, optimized_size, size_of_messy, size_of_repr_c, Messy, Optimized,
    ReprC, Sample,
};
use std::mem::{align_of, size_of};

#[test]
fn messy_has_expected_size() {
    // On any reasonable target, a {u8, u64, u8} struct is at least 16 bytes.
    // On 64-bit it's 24 (7 + 8 + 7 padding).
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
    // y is a u64 in a #[repr(C)] struct { u8, u64, u32 }.
    // x takes 1 byte, then 7 bytes of padding to align y to 8.
    assert_eq!(off, 8);
}

#[test]
fn repr_c_has_c_layout_size() {
    // #[repr(C)] { u8, u32, u16 } on 64-bit: 1 + 3pad + 4 + 2 + 2pad = 12
    // on 32-bit: same layout (align of u32 is 4 on both)
    let s = size_of_repr_c();
    assert_eq!(s, size_of::<ReprC>());
    // The exact value is target-dependent only on pointer-width; on both
    // 32- and 64-bit, align_of::<u32>() == 4, so the layout is 12.
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
    // The best possible layout for {u8, u8, u64} on 64-bit:
    //   u64 (8) + u8 (1) + u8 (1) + 6 trailing pad = 16 bytes.
    // The test is cfg-guarded so it passes on 32-bit too (with the
    // appropriate smaller size).
    let opt = optimized_size();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(opt, 16);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(opt, 8);
}

#[test]
fn sample_size_is_correct() {
    // Sanity check: the Sample struct is repr(C) with {u8, u64, u32}.
    // 1 + 7pad + 8 + 4 + 4pad = 24 on 64-bit; 1+3pad+8+4+0pad = 16 on 32-bit.
    let s = size_of::<Sample>();
    #[cfg(target_pointer_width = "64")]
    assert_eq!(s, 24);
    #[cfg(target_pointer_width = "32")]
    assert_eq!(s, 16);
}

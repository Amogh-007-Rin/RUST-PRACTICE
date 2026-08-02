use module_035_exercises::{
    read_via_raw, sum_slice_via_raw, swap_via_raw, unsafe_double, write_via_raw,
};

#[test]
fn read_via_raw_reads_memory() {
    let value = 42u32;
    let ptr: *const u32 = &value;
    assert_eq!(unsafe { read_via_raw(ptr) }, 42);
}

#[test]
fn write_via_raw_writes_memory() {
    let mut value = 1u32;
    let ptr: *mut u32 = &mut value;
    unsafe { write_via_raw(ptr, 99) };
    assert_eq!(value, 99);
}

#[test]
fn write_then_read_via_raw() {
    let mut value = 0u32;
    let mut_ptr: *mut u32 = &mut value;
    unsafe {
        write_via_raw(mut_ptr, 7);
    }
    assert_eq!(unsafe { read_via_raw(mut_ptr) }, 7);
}

#[test]
fn sum_via_raw_matches_iterator_sum() {
    let data = vec![10, 20, 30, 40];
    assert_eq!(sum_slice_via_raw(&data), 100);
    assert_eq!(sum_slice_via_raw(&[]), 0);
    assert_eq!(sum_slice_via_raw(&[5]), 5);
}

#[test]
fn sum_via_raw_handles_larger_slices() {
    let data: Vec<u32> = (0..1000).collect();
    let expected: u32 = (0..1000).sum();
    assert_eq!(sum_slice_via_raw(&data), expected);
}

#[test]
fn unsafe_double_dereferences_and_doubles() {
    let value = 21u32;
    let ptr: *const u32 = &value;
    assert_eq!(unsafe { unsafe_double(ptr) }, 42);
}

#[test]
fn swap_via_raw_swaps_in_place() {
    let mut a = 10i32;
    let mut b = 20i32;
    swap_via_raw(&mut a, &mut b);
    assert_eq!((a, b), (20, 10));
}

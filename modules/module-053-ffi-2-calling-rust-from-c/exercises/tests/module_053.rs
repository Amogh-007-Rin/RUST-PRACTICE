use module_053_exercises::Rectangle;

extern "C" {
    fn rust_add(a: i32, b: i32) -> i32;
    fn rust_multiply(a: f64, b: f64) -> f64;
    fn rust_strlen(s: *const std::os::raw::c_char) -> usize;
    fn rust_rectangle_area(r: *const Rectangle) -> f64;
    fn rust_sum_array(data: *const i32, len: usize) -> i32;
}

#[test]
fn add_works() {
    assert_eq!(unsafe { rust_add(3, 4) }, 7);
    assert_eq!(unsafe { rust_add(-10, 5) }, -5);
    assert_eq!(unsafe { rust_add(0, 0) }, 0);
}

#[test]
fn multiply_works() {
    assert_eq!(unsafe { rust_multiply(2.0, 3.0) }, 6.0);
    assert_eq!(unsafe { rust_multiply(-2.5, 4.0) }, -10.0);
}

#[test]
fn strlen_works() {
    let s = b"hello\0";
    assert_eq!(unsafe { rust_strlen(s.as_ptr() as *const _) }, 5);

    let empty = b"\0";
    assert_eq!(unsafe { rust_strlen(empty.as_ptr() as *const _) }, 0);

    assert_eq!(unsafe { rust_strlen(std::ptr::null()) }, 0);
}

#[test]
fn rectangle_area_works() {
    let r = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    assert_eq!(unsafe { rust_rectangle_area(&r) }, 15.0);

    assert_eq!(unsafe { rust_rectangle_area(std::ptr::null()) }, 0.0);
}

#[test]
fn sum_array_works() {
    let data = [1_i32, 2, 3, 4, 5];
    assert_eq!(unsafe { rust_sum_array(data.as_ptr(), data.len()) }, 15);

    let empty: [i32; 0] = [];
    assert_eq!(unsafe { rust_sum_array(empty.as_ptr(), 0) }, 0);

    assert_eq!(unsafe { rust_sum_array(std::ptr::null(), 5) }, 0);
}

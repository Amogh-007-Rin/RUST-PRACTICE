use std::f64::consts::PI;

use module_017_exercises::{
    biggest, describe_shapes, largest_area, total_area, total_area_mixed, Area, Circle, Rectangle,
    Triangle,
};

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn circle_area_uses_pi() {
    let circle = Circle { radius: 2.0 };
    assert!(approx(circle.area(), 4.0 * PI));
}

#[test]
fn rectangle_area_multiplies_sides() {
    let rect = Rectangle {
        width: 4.0,
        height: 2.0,
    };
    assert_eq!(rect.area(), 8.0);
}

#[test]
fn triangle_area_halves_the_product() {
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    assert_eq!(triangle.area(), 6.0);
}

#[test]
fn largest_area_finds_the_biggest_shape() {
    let shapes = [
        Rectangle {
            width: 1.0,
            height: 1.0,
        },
        Rectangle {
            width: 4.0,
            height: 2.0,
        },
        Rectangle {
            width: 2.0,
            height: 3.0,
        },
    ];
    assert!(approx(largest_area(&shapes), 8.0));
}

#[test]
fn largest_area_of_empty_slice_is_zero() {
    let shapes: [Rectangle; 0] = [];
    assert_eq!(largest_area(&shapes), 0.0);
}

#[test]
fn total_area_sums_all_shapes() {
    let shapes = [
        Rectangle {
            width: 1.0,
            height: 1.0,
        },
        Rectangle {
            width: 2.0,
            height: 3.0,
        },
    ];
    assert_eq!(total_area(&shapes), 7.0);
}

#[test]
fn biggest_returns_a_reference_to_the_largest_shape() {
    let small = Rectangle {
        width: 1.0,
        height: 1.0,
    };
    let big = Rectangle {
        width: 4.0,
        height: 2.0,
    };
    let shapes = [small, big];
    assert!(approx(biggest(&shapes).unwrap().area(), 8.0));
}

#[test]
fn biggest_of_empty_slice_is_none() {
    let shapes: [Circle; 0] = [];
    assert!(biggest(&shapes).is_none());
}

#[test]
fn total_area_mixed_handles_heterogeneous_shapes() {
    let circle = Circle { radius: 1.0 };
    let rect = Rectangle {
        width: 4.0,
        height: 2.0,
    };
    let shapes: Vec<&dyn Area> = vec![&circle, &rect];
    assert!(approx(total_area_mixed(&shapes), PI + 8.0));
}

#[test]
fn describe_shapes_calls_the_default_method_through_dyn() {
    let rect = Rectangle {
        width: 4.0,
        height: 2.0,
    };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let shapes: Vec<&dyn Area> = vec![&rect, &triangle];
    assert_eq!(describe_shapes(&shapes), vec!["area = 8.00", "area = 6.00"]);
}

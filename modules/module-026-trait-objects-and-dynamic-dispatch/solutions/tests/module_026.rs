use module_026_solutions::{largest_shape, total_area, total_area_generic, Circle, Shape, Square};

#[test]
fn area_and_name_of_individual_shapes() {
    let circle = Circle { radius: 1.0 };
    let square = Square { side: 2.0 };
    assert!((circle.area() - std::f64::consts::PI).abs() < 1e-9);
    assert_eq!(square.area(), 4.0);
    assert_eq!(circle.name(), "circle");
    assert_eq!(square.name(), "square");
}

#[test]
fn total_area_uses_dynamic_dispatch_over_mixed_shapes() {
    let circle = Circle { radius: 1.0 };
    let square = Square { side: 2.0 };
    let shapes: Vec<&dyn Shape> = vec![&circle, &square];
    assert!((total_area(&shapes) - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
}

#[test]
fn total_area_generic_uses_static_dispatch() {
    let squares = [Square { side: 2.0 }, Square { side: 1.0 }];
    assert_eq!(total_area_generic(&squares), 5.0);
}

#[test]
fn largest_shape_returns_the_biggest() {
    let circle = Circle { radius: 10.0 };
    let square = Square { side: 2.0 };
    let shapes: Vec<&dyn Shape> = vec![&circle, &square];
    let largest = largest_shape(&shapes).unwrap();
    assert_eq!(largest.name(), "circle");
}

#[test]
fn largest_shape_is_none_for_empty_input() {
    let shapes: Vec<&dyn Shape> = Vec::new();
    assert!(largest_shape(&shapes).is_none());
}

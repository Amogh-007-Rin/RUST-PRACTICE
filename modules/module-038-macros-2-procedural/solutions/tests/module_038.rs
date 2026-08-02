use module_038_solutions::{describe_all, Book, Describe, Point, Shape};

#[test]
fn point_describes_with_fields() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(p.describe(), "Point { x: 1, y: 2 }");
}

#[test]
fn point_handles_negative_values() {
    let p = Point { x: -5, y: 10 };
    assert_eq!(p.describe(), "Point { x: -5, y: 10 }");
}

#[test]
fn book_describes_with_fields() {
    let book = Book {
        title: String::from("The Rust Book"),
        pages: 400,
    };
    assert_eq!(book.describe(), "Book { title: The Rust Book, pages: 400 }");
}

#[test]
fn shape_circle_describes() {
    let circle = Shape::Circle { radius: 1.5 };
    assert_eq!(circle.describe(), "Shape::Circle { radius: 1.5 }");
}

#[test]
fn shape_rectangle_describes() {
    let rect = Shape::Rectangle {
        width: 3,
        height: 4,
    };
    assert_eq!(rect.describe(), "Shape::Rectangle { width: 3, height: 4 }");
}

#[test]
fn describe_all_maps_over_slices() {
    let points = vec![Point { x: 0, y: 0 }, Point { x: 10, y: 20 }];
    assert_eq!(
        describe_all(&points),
        vec!["Point { x: 0, y: 0 }", "Point { x: 10, y: 20 }"]
    );
}

#[test]
fn all_types_implement_describe() {
    fn assert_describable<T: Describe>() {}
    assert_describable::<Point>();
    assert_describable::<Book>();
    assert_describable::<Shape>();
}

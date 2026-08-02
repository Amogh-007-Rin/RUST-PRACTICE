use module_025_exercises::{
    first, print_summary, Book, Celsius, Container, Fahrenheit, Summarizable, Vector,
};

#[test]
fn container_works_for_vec_and_string() {
    let v = vec![10, 20, 30];
    assert_eq!(first(&v), Some(&10));
    assert_eq!(v.len(), 3);
    assert!(!v.is_empty());
    assert_eq!(v.get(5), None);

    let s = String::from("ab");
    assert_eq!(first(&s), Some(&b'a'));
    assert_eq!(s.len(), 2);
}

#[test]
fn vector_adds_component_wise() {
    let sum = Vector(1.0, 2.0) + Vector(3.0, 4.0);
    assert_eq!(sum, Vector(4.0, 6.0));
}

#[test]
fn vector_scales_by_scalar() {
    let scaled = Vector(2.0, 3.0) * 2.0;
    assert_eq!(scaled, Vector(4.0, 6.0));
}

#[test]
fn temperature_newtypes_convert() {
    let celsius = Celsius(100.0);
    assert!((celsius.to_fahrenheit() - 212.0).abs() < 1e-9);

    let fahrenheit: Fahrenheit = celsius.into();
    assert!((fahrenheit.0 - 212.0).abs() < 1e-9);
    assert!((fahrenheit.to_celsius() - 100.0).abs() < 1e-9);
}

#[test]
fn supertrait_lets_print_summary_use_display() {
    let book = Book {
        title: "Rust in Action".into(),
        author: "Tim McNamara".into(),
        pages: 382,
    };
    assert_eq!(
        print_summary(&book),
        "Rust in Action by Tim McNamara (382 pages) | Rust in Action by Tim McNamara"
    );
}

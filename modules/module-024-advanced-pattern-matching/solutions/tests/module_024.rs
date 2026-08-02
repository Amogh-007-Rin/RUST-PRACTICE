use module_024_solutions::{
    describe_point, describe_shape, greeting, parse_i32_pair, Role, Shape, User,
};

#[test]
fn describe_point_origin_and_axes() {
    assert_eq!(describe_point((0, 0)), "origin");
    assert_eq!(describe_point((5, 0)), "positive x-axis");
    assert_eq!(describe_point((-3, 0)), "negative x-axis");
    assert_eq!(describe_point((0, 7)), "positive y-axis");
    assert_eq!(describe_point((0, -2)), "negative y-axis");
}

#[test]
fn describe_point_quadrants() {
    assert_eq!(describe_point((1, 2)), "quadrant I");
    assert_eq!(describe_point((-1, 2)), "quadrant II");
    assert_eq!(describe_point((-1, -2)), "quadrant III");
    assert_eq!(describe_point((1, -2)), "quadrant IV");
}

#[test]
fn describe_shape_circles_and_squares() {
    assert_eq!(
        describe_shape(&Shape::Circle { radius: 2.0 }),
        "circle of radius 2"
    );
    assert_eq!(
        describe_shape(&Shape::Circle { radius: -1.0 }),
        "invalid circle with radius -1"
    );
    assert_eq!(
        describe_shape(&Shape::Rectangle {
            width: 1.0,
            height: 1.0
        }),
        "square of side 1"
    );
    assert_eq!(
        describe_shape(&Shape::Rectangle {
            width: 2.0,
            height: 3.0
        }),
        "rectangle 2 x 3"
    );
}

#[test]
fn greeting_nests_struct_and_enum_patterns() {
    let admin = User {
        name: "alice".into(),
        role: Role::Admin,
    };
    assert_eq!(greeting(&admin), "Welcome back, admin alice");

    let newcomer = User {
        name: "bob".into(),
        role: Role::Member { joined_year: 2024 },
    };
    assert_eq!(greeting(&newcomer), "bob, welcome aboard");

    let veteran = User {
        name: "carol".into(),
        role: Role::Member { joined_year: 2019 },
    };
    assert_eq!(greeting(&veteran), "Hi carol");
}

#[test]
fn parse_i32_pair_accepts_well_formed_input() {
    assert_eq!(parse_i32_pair("3, 4"), Some((3, 4)));
    assert_eq!(parse_i32_pair("-1, 42"), Some((-1, 42)));
}

#[test]
fn parse_i32_pair_rejects_bad_input() {
    assert_eq!(parse_i32_pair("5"), None);
    assert_eq!(parse_i32_pair("1,2,3"), None);
    assert_eq!(parse_i32_pair("a, b"), None);
    assert_eq!(parse_i32_pair("1, "), None);
    assert_eq!(parse_i32_pair(""), None);
}

use module_037_exercises::{count_tt, def_struct, min, my_vec, sum};

#[test]
fn my_vec_builds_vectors() {
    let three: Vec<i32> = my_vec![1, 2, 3];
    assert_eq!(three, vec![1, 2, 3]);
    let trailing: Vec<i32> = my_vec![1, 2, 3,];
    assert_eq!(trailing, vec![1, 2, 3]);
    let single: Vec<i32> = my_vec![42];
    assert_eq!(single, vec![42]);
    let empty: Vec<u32> = my_vec![];
    assert!(empty.is_empty());
    let strings: Vec<&str> = my_vec!["a", "b"];
    assert_eq!(strings.len(), 2);
}

#[test]
fn min_finds_the_smallest() {
    assert_eq!(min!(7), 7);
    assert_eq!(min!(5, 4), 4);
    assert_eq!(min!(3, 1, 2), 1);
    assert_eq!(min!(10, 20, 5, 30), 5);
}

#[test]
fn sum_sums_any_number_of_arguments() {
    assert_eq!(sum!(), 0);
    assert_eq!(sum!(1), 1);
    assert_eq!(sum!(1, 2, 3), 6);
    assert_eq!(sum!(1, 2, 3, 4, 5), 15);
}

#[test]
fn count_tt_counts_token_trees() {
    assert_eq!(count_tt!(), 0);
    assert_eq!(count_tt!(a), 1);
    assert_eq!(count_tt!(a b c), 3);
    assert_eq!(count_tt!(a b c d e), 5);
}

#[test]
fn count_tt_counts_a_group_as_one() {
    assert_eq!(count_tt!((a, b, c)), 1);
    assert_eq!(count_tt!(a (b, c) d), 3);
}

def_struct!(Point { x: i32, y: i32 });

#[test]
fn def_struct_generates_struct_and_constructor() {
    let p = Point::new(1, 2);
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 2);
    let q = Point::new(-5, 10);
    assert_eq!((q.x, q.y), (-5, 10));
}

#[test]
fn def_struct_fields_are_public() {
    let p = Point { x: 7, y: 8 };
    assert_eq!(p.x + p.y, 15);
}

def_struct!(Player {
    name: String,
    health: u32
});

#[test]
fn def_struct_works_with_any_field_types() {
    let player = Player::new(String::from("hero"), 100);
    assert_eq!(player.name, "hero");
    assert_eq!(player.health, 100);
}

use std::path::PathBuf;

use module_013_exercises::{
    check_grade, nth_item, parse_stock_quantity, read_first_line, safe_divide,
};

fn write_temp(contents: &str, tag: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("module_013_{}_{}.txt", std::process::id(), tag));
    std::fs::write(&path, contents).unwrap();
    path
}

#[test]
fn check_grade_passes_at_fifty() {
    assert_eq!(check_grade(50), Ok("pass"));
    assert_eq!(check_grade(87), Ok("pass"));
}

#[test]
fn check_grade_fails_below_fifty() {
    assert_eq!(check_grade(49), Err("fail"));
    assert_eq!(check_grade(0), Err("fail"));
}

#[test]
fn safe_divide_divides() {
    assert_eq!(safe_divide(10, 2), Ok(5));
}

#[test]
fn safe_divide_rejects_zero_divisor() {
    assert_eq!(safe_divide(10, 0), Err("division by zero".to_string()));
}

#[test]
fn parse_stock_quantity_parses_valid_numbers() {
    assert_eq!(parse_stock_quantity("42"), Ok(42));
}

#[test]
fn parse_stock_quantity_rejects_garbage() {
    assert!(parse_stock_quantity("forty-two").is_err());
}

#[test]
fn parse_stock_quantity_rejects_negatives() {
    assert!(parse_stock_quantity("-5").is_err());
}

#[test]
fn nth_item_returns_the_element() {
    let items = [10, 20, 30];
    assert_eq!(nth_item(&items, 1), Ok(&20));
}

#[test]
fn nth_item_errors_out_of_bounds() {
    let items = [10, 20, 30];
    assert_eq!(nth_item(&items, 99), Err("index out of bounds"));
}

#[test]
fn read_first_line_returns_first_line() {
    let path = write_temp("hello\nworld\n", "hello");
    let result = read_first_line(&path);
    let _ = std::fs::remove_file(&path);
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn read_first_line_of_empty_file_is_empty_string() {
    let path = write_temp("", "empty");
    let result = read_first_line(&path);
    let _ = std::fs::remove_file(&path);
    assert_eq!(result.unwrap(), "");
}

#[test]
fn read_first_line_reports_missing_files() {
    let missing = std::env::temp_dir().join("module_013_missing_xyz.txt");
    let _ = std::fs::remove_file(&missing);
    assert!(read_first_line(&missing).is_err());
}

use std::fs;

use capstone_02_solution::{Inventory, InventoryError, Item};

#[test]
fn new_inventory_is_empty() {
    let inventory = Inventory::new();
    assert!(inventory.items.is_empty());
    assert_eq!(inventory.total_units(), 0);
}

#[test]
fn add_item_then_get() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 3, 1).unwrap();
    let item = inventory.get_item("laptop").unwrap();
    assert_eq!(item.name, "laptop");
    assert_eq!(item.category, "electronics");
    assert_eq!(item.quantity, 3);
    assert_eq!(item.low_stock_threshold, 1);
}

#[test]
fn add_duplicate_item_errors() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 3, 1).unwrap();
    let result = inventory.add_item("laptop", "electronics", 5, 1);
    assert!(matches!(
        result,
        Err(InventoryError::DuplicateItem(name)) if name == "laptop"
    ));
    assert_eq!(inventory.get_item("laptop").unwrap().quantity, 3);
}

#[test]
fn update_quantity_changes_the_value() {
    let mut inventory = Inventory::new();
    inventory.add_item("apple", "food", 10, 5).unwrap();
    inventory.update_quantity("apple", 30).unwrap();
    assert_eq!(inventory.get_item("apple").unwrap().quantity, 30);
}

#[test]
fn update_quantity_on_missing_item_errors() {
    let mut inventory = Inventory::new();
    let result = inventory.update_quantity("nope", 5);
    assert!(matches!(result, Err(InventoryError::ItemNotFound(_))));
}

#[test]
fn adjust_quantity_adds_and_subtracts() {
    let mut inventory = Inventory::new();
    inventory.add_item("apple", "food", 10, 5).unwrap();
    inventory.adjust_quantity("apple", 5).unwrap();
    assert_eq!(inventory.get_item("apple").unwrap().quantity, 15);
    inventory.adjust_quantity("apple", -7).unwrap();
    assert_eq!(inventory.get_item("apple").unwrap().quantity, 8);
}

#[test]
fn adjust_quantity_below_zero_errors_and_leaves_value_unchanged() {
    let mut inventory = Inventory::new();
    inventory.add_item("apple", "food", 2, 5).unwrap();
    let result = inventory.adjust_quantity("apple", -3);
    assert!(matches!(result, Err(InventoryError::InvalidQuantity(_))));
    assert_eq!(inventory.get_item("apple").unwrap().quantity, 2);
}

#[test]
fn remove_item_removes_and_errors_on_missing() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 3, 1).unwrap();
    inventory.remove_item("laptop").unwrap();
    assert!(inventory.get_item("laptop").is_none());
    let result = inventory.remove_item("laptop");
    assert!(matches!(result, Err(InventoryError::ItemNotFound(_))));
}

#[test]
fn item_is_low_stock_flag() {
    let at_threshold = Item {
        name: "a".to_string(),
        category: "c".to_string(),
        quantity: 3,
        low_stock_threshold: 3,
    };
    assert!(at_threshold.is_low_stock());

    let above_threshold = Item {
        name: "b".to_string(),
        category: "c".to_string(),
        quantity: 4,
        low_stock_threshold: 3,
    };
    assert!(!above_threshold.is_low_stock());
}

#[test]
fn low_stock_detection() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 1, 1).unwrap();
    inventory.add_item("apple", "food", 20, 5).unwrap();
    inventory.add_item("keyboard", "electronics", 0, 2).unwrap();

    let low = inventory.low_stock_items();
    assert_eq!(low.len(), 2);
    assert!(low.iter().all(|item| item.is_low_stock()));

    let names: Vec<&str> = low.iter().map(|item| item.name.as_str()).collect();
    assert!(names.contains(&"laptop"));
    assert!(names.contains(&"keyboard"));
    assert!(!names.contains(&"apple"));
}

#[test]
fn categories_are_unique_and_sorted() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 1, 1).unwrap();
    inventory.add_item("keyboard", "electronics", 1, 1).unwrap();
    inventory.add_item("apple", "food", 1, 1).unwrap();
    assert_eq!(inventory.categories(), vec!["electronics", "food"]);
}

#[test]
fn items_in_category_filters() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 1, 1).unwrap();
    inventory.add_item("apple", "food", 1, 1).unwrap();
    inventory.add_item("keyboard", "electronics", 1, 1).unwrap();

    let electronics = inventory.items_in_category("electronics");
    assert_eq!(electronics.len(), 2);
    assert!(electronics
        .iter()
        .all(|item| item.category == "electronics"));

    assert!(inventory.items_in_category("nonexistent").is_empty());
}

#[test]
fn total_units_sums_all_quantities() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 3, 1).unwrap();
    inventory.add_item("apple", "food", 20, 5).unwrap();
    inventory.add_item("keyboard", "electronics", 0, 2).unwrap();
    assert_eq!(inventory.total_units(), 23);
}

#[test]
fn total_by_category_groups_quantities() {
    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 3, 1).unwrap();
    inventory.add_item("keyboard", "electronics", 1, 2).unwrap();
    inventory.add_item("apple", "food", 20, 5).unwrap();

    let totals = inventory.total_by_category();
    assert_eq!(totals.get("electronics"), Some(&4));
    assert_eq!(totals.get("food"), Some(&20));
}

#[test]
fn save_then_load_round_trips() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("inventory.json");

    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 3, 1).unwrap();
    inventory.add_item("apple", "food", 20, 5).unwrap();
    inventory.save(&path).unwrap();

    let loaded = Inventory::load(&path).unwrap();
    assert_eq!(loaded, inventory);
}

#[test]
fn saved_file_contains_item_names_as_json() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("inventory.json");

    let mut inventory = Inventory::new();
    inventory.add_item("laptop", "electronics", 3, 1).unwrap();
    inventory.save(&path).unwrap();

    let raw = fs::read_to_string(&path).unwrap();
    assert!(raw.contains("\"laptop\""));
    assert!(raw.contains("\"electronics\""));
}

#[test]
fn load_missing_file_errors() {
    let tmp = tempfile::tempdir().unwrap();
    let result = Inventory::load(&tmp.path().join("does_not_exist.json"));
    assert!(matches!(result, Err(InventoryError::Io(_))));
}

#[test]
fn load_corrupt_file_errors() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("corrupt.json");
    fs::write(&path, "this is not json").unwrap();
    let result = Inventory::load(&path);
    assert!(matches!(result, Err(InventoryError::Json(_))));
}

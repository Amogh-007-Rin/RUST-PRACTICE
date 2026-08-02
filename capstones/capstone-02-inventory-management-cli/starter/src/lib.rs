//! Capstone 02: Inventory Management CLI — library.
//!
//! The types (`Item`, `Inventory`, `InventoryError`) are complete. Fill in
//! the `TODO(capstone-02)` method bodies so the integration tests in
//! `tests/capstone_02.rs` pass.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// A single tracked stock item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub category: String,
    pub quantity: u32,
    pub low_stock_threshold: u32,
}

impl Item {
    /// Returns `true` when the quantity is at or below the low-stock
    /// threshold (the definition of "needs restocking").
    pub fn is_low_stock(&self) -> bool {
        // TODO(capstone-02): `self.quantity <= self.low_stock_threshold`.
        todo!()
    }
}

/// An inventory: an owned list of items.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
}

/// The single error type of this crate.
///
/// `#[from]` lets `?` convert `std::io::Error` and `serde_json::Error`
/// automatically (Module 014).
#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    #[error("item already exists: {0}")]
    DuplicateItem(String),
    #[error("item not found: {0}")]
    ItemNotFound(String),
    #[error("quantity would become negative: {0}")]
    InvalidQuantity(i64),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl Inventory {
    /// Creates an empty inventory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new item, rejecting duplicate names.
    pub fn add_item(
        &mut self,
        name: &str,
        category: &str,
        quantity: u32,
        low_stock_threshold: u32,
    ) -> Result<(), InventoryError> {
        // TODO(capstone-02): reject duplicates by checking
        // `self.get_item(name)` first. Otherwise push an `Item` built from
        // the arguments (note: `name` and `category` are `&str` — build
        // `String`s from them).
        let _name = name;
        let _ = (category, quantity, low_stock_threshold);
        todo!()
    }

    /// Sets an item's quantity to an absolute value.
    pub fn update_quantity(&mut self, name: &str, quantity: u32) -> Result<(), InventoryError> {
        // TODO(capstone-02): find the item by name (an item is missing when
        // `self.get_item(name)` is `None`), then set its quantity.
        let _name = name;
        let _quantity = quantity;
        todo!()
    }

    /// Adjusts an item's quantity by `delta` (negative allowed), rejecting
    /// any result below zero.
    pub fn adjust_quantity(&mut self, name: &str, delta: i64) -> Result<(), InventoryError> {
        // TODO(capstone-02): compute the new quantity as
        // `item.quantity as i64 + delta`; if it is negative, return
        // `Err(InventoryError::InvalidQuantity(new_quantity))`. Otherwise
        // update the item.
        let _name = name;
        let _delta = delta;
        todo!()
    }

    /// Removes an item by name.
    pub fn remove_item(&mut self, name: &str) -> Result<(), InventoryError> {
        // TODO(capstone-02): use `self.items.iter().position(...)` to find
        // the index of the item, then `self.items.remove(index)`. A missing
        // item is `InventoryError::ItemNotFound`.
        let _name = name;
        todo!()
    }

    /// Returns the item with the given name, if present.
    pub fn get_item(&self, name: &str) -> Option<&Item> {
        // TODO(capstone-02): `self.items.iter().find(...)` matching on the
        // item's name.
        let _name = name;
        None
    }

    /// Returns the distinct categories, sorted alphabetically.
    pub fn categories(&self) -> Vec<String> {
        // TODO(capstone-02): collect the categories into a `HashSet` (or
        // sort + dedup a `Vec`), then return them sorted. `Vec::sort` works
        // on any `Ord` type.
        Vec::new()
    }

    /// Returns references to all items in `category`.
    pub fn items_in_category(&self, category: &str) -> Vec<&Item> {
        // TODO(capstone-02): filter `self.items.iter()` on the category.
        let _category = category;
        Vec::new()
    }

    /// Returns references to all items at or below their threshold.
    pub fn low_stock_items(&self) -> Vec<&Item> {
        // TODO(capstone-02): keep the items where `item.is_low_stock()`.
        Vec::new()
    }

    /// Returns the total number of units across all items.
    pub fn total_units(&self) -> u32 {
        // TODO(capstone-02): sum the `quantity` fields.
        0
    }

    /// Returns the total units per category.
    pub fn total_by_category(&self) -> HashMap<String, u32> {
        // TODO(capstone-02): for each item, add its quantity into a map
        // under its category — `map.entry(category).or_insert(0)` plus `+=`
        // is the Module 012 pattern.
        HashMap::new()
    }

    /// Saves the inventory to `path` as pretty JSON.
    pub fn save(&self, path: &Path) -> Result<(), InventoryError> {
        // TODO(capstone-02): `serde_json::to_string_pretty(self)` then
        // `std::fs::write(path, ...)` — both `?`s convert automatically via
        // the `#[from]` attributes.
        let _path = path;
        todo!()
    }

    /// Loads an inventory from `path`, replacing the contents.
    pub fn load(path: &Path) -> Result<Self, InventoryError> {
        // TODO(capstone-02): `std::fs::read_to_string(path)?`, then
        // `serde_json::from_str(&contents)` — both errors convert with `?`.
        let _path = path;
        todo!()
    }
}

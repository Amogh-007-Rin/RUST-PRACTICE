//! Capstone 02: Inventory Management CLI — library (reference solution).

use std::collections::{HashMap, HashSet};
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
        self.quantity <= self.low_stock_threshold
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
        if self.get_item(name).is_some() {
            return Err(InventoryError::DuplicateItem(name.to_string()));
        }
        self.items.push(Item {
            name: name.to_string(),
            category: category.to_string(),
            quantity,
            low_stock_threshold,
        });
        Ok(())
    }

    /// Sets an item's quantity to an absolute value.
    pub fn update_quantity(&mut self, name: &str, quantity: u32) -> Result<(), InventoryError> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.name == name)
            .ok_or_else(|| InventoryError::ItemNotFound(name.to_string()))?;
        item.quantity = quantity;
        Ok(())
    }

    /// Adjusts an item's quantity by `delta` (negative allowed), rejecting
    /// any result below zero.
    pub fn adjust_quantity(&mut self, name: &str, delta: i64) -> Result<(), InventoryError> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.name == name)
            .ok_or_else(|| InventoryError::ItemNotFound(name.to_string()))?;
        let new_quantity = item.quantity as i64 + delta;
        if new_quantity < 0 {
            return Err(InventoryError::InvalidQuantity(new_quantity));
        }
        item.quantity = new_quantity as u32;
        Ok(())
    }

    /// Removes an item by name.
    pub fn remove_item(&mut self, name: &str) -> Result<(), InventoryError> {
        let index = self
            .items
            .iter()
            .position(|item| item.name == name)
            .ok_or_else(|| InventoryError::ItemNotFound(name.to_string()))?;
        self.items.remove(index);
        Ok(())
    }

    /// Returns the item with the given name, if present.
    pub fn get_item(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.name == name)
    }

    /// Returns the distinct categories, sorted alphabetically.
    pub fn categories(&self) -> Vec<String> {
        let mut categories: HashSet<&str> = self
            .items
            .iter()
            .map(|item| item.category.as_str())
            .collect();
        let mut sorted: Vec<String> = categories.drain().map(str::to_string).collect();
        sorted.sort();
        sorted
    }

    /// Returns references to all items in `category`.
    pub fn items_in_category(&self, category: &str) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.category == category)
            .collect()
    }

    /// Returns references to all items at or below their threshold.
    pub fn low_stock_items(&self) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.is_low_stock())
            .collect()
    }

    /// Returns the total number of units across all items.
    pub fn total_units(&self) -> u32 {
        self.items.iter().map(|item| item.quantity).sum()
    }

    /// Returns the total units per category.
    pub fn total_by_category(&self) -> HashMap<String, u32> {
        let mut totals = HashMap::new();
        for item in &self.items {
            *totals.entry(item.category.clone()).or_insert(0) += item.quantity;
        }
        totals
    }

    /// Saves the inventory to `path` as pretty JSON.
    pub fn save(&self, path: &Path) -> Result<(), InventoryError> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Loads an inventory from `path`, replacing the contents.
    pub fn load(path: &Path) -> Result<Self, InventoryError> {
        let contents = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

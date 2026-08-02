//! Module 098 — reference solution (bugs fixed).

/// An item in an order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub price_cents: u64,
    pub quantity: u32,
}

/// The status of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Finalized,
    Canceled,
}

/// An order with items and a status.
#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub items: Vec<Item>,
    pub status: OrderStatus,
    pub discount_percent: u32,
}

/// Processes orders: create, add items, calculate totals, finalize.
pub struct OrderProcessor {
    orders: Vec<Order>,
    next_id: u64,
}

impl OrderProcessor {
    /// Creates a new, empty order processor.
    pub fn new() -> Self {
        Self {
            orders: Vec::new(),
            next_id: 1,
        }
    }

    /// Creates a new pending order and returns its id.
    pub fn create_order(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.orders.push(Order {
            id,
            items: Vec::new(),
            status: OrderStatus::Pending,
            discount_percent: 0,
        });
        id
    }

    /// Adds an item to the order. Returns `false` if the order doesn't exist
    /// or is not pending.
    pub fn add_item(&mut self, order_id: u64, name: &str, price_cents: u64, quantity: u32) -> bool {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            if order.status != OrderStatus::Pending {
                return false;
            }
            order.items.push(Item {
                name: name.to_string(),
                price_cents,
                quantity,
            });
            true
        } else {
            false
        }
    }

    /// Removes the first item with the given name from the order. Returns
    /// `true` if an item was removed, `false` otherwise.
    pub fn remove_item(&mut self, order_id: u64, item_name: &str) -> bool {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            if order.status != OrderStatus::Pending {
                return false;
            }
            if let Some(pos) = order.items.iter().position(|item| item.name == item_name) {
                order.items.remove(pos);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Calculates the total price of the order in cents, after applying any
    /// discount. Returns `None` if the order doesn't exist.
    pub fn total(&self, order_id: u64) -> Option<u64> {
        self.orders.iter().find(|o| o.id == order_id).map(|order| {
            let subtotal: u64 = order
                .items
                .iter()
                .map(|item| item.price_cents * item.quantity as u64)
                .sum();
            let discount = subtotal * order.discount_percent as u64 / 100;
            subtotal - discount
        })
    }

    /// Applies a percentage discount (0–100) to the order. Returns `false`
    /// if the order doesn't exist or is not pending.
    pub fn apply_discount(&mut self, order_id: u64, percent: u32) -> bool {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            if order.status != OrderStatus::Pending {
                return false;
            }
            if percent > 100 {
                return false;
            }
            order.discount_percent = percent;
            true
        } else {
            false
        }
    }

    /// Finalizes the order. Returns `false` if the order doesn't exist or
    /// is not pending.
    pub fn finalize(&mut self, order_id: u64) -> bool {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            if order.status != OrderStatus::Pending {
                return false;
            }
            order.status = OrderStatus::Finalized;
            true
        } else {
            false
        }
    }

    /// Cancels the order. Returns `false` if the order doesn't exist or is
    /// already finalized.
    pub fn cancel(&mut self, order_id: u64) -> bool {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            if order.status == OrderStatus::Finalized {
                return false;
            }
            order.status = OrderStatus::Canceled;
            true
        } else {
            false
        }
    }

    /// Returns a reference to the order, or `None` if it doesn't exist.
    pub fn get_order(&self, order_id: u64) -> Option<&Order> {
        self.orders.iter().find(|o| o.id == order_id)
    }

    /// Returns the total quantity of items in the order, or `None` if the
    /// order doesn't exist.
    pub fn item_count(&self, order_id: u64) -> Option<usize> {
        self.orders
            .iter()
            .find(|o| o.id == order_id)
            .map(|order| order.items.iter().map(|item| item.quantity as usize).sum())
    }
}

impl Default for OrderProcessor {
    fn default() -> Self {
        Self::new()
    }
}

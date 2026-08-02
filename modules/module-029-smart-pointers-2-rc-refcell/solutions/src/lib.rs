//! Module 029: Smart Pointers II — reference solution.

use std::cell::RefCell;
use std::rc::Rc;

/// A chat room. `Member`s all hold an `Rc<Chat>` so the room is shared,
/// not copied, and lives as long as the last member holds it.
pub struct Chat {
    pub name: String,
}

/// A chat member sharing a reference-counted `Chat`.
pub struct Member {
    pub chat: Rc<Chat>,
    pub nickname: String,
}

impl Member {
    /// The name of the chat this member belongs to.
    pub fn chat_name(&self) -> &str {
        &self.chat.name
    }
}

/// Creates one `Member` per nickname, all sharing the same `chat`.
pub fn shared_members(chat: Rc<Chat>, nicknames: &[&str]) -> Vec<Member> {
    nicknames
        .iter()
        .map(|nickname| Member {
            chat: Rc::clone(&chat),
            nickname: nickname.to_string(),
        })
        .collect()
}

/// A counter that can be incremented through an immutable reference,
/// thanks to interior mutability via `RefCell`.
pub struct Counter {
    value: RefCell<u32>,
}

impl Counter {
    /// Creates a counter starting at zero.
    pub fn new() -> Self {
        Self {
            value: RefCell::new(0),
        }
    }

    /// Increments the counter by one.
    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    /// The current value of the counter.
    pub fn value(&self) -> u32 {
        *self.value.borrow()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// A shared wallet: every `share()` creates a new handle onto the same
/// `Rc<RefCell<i64>>` balance, so two handles observe each other's
/// deposits and withdrawals.
pub struct Wallet {
    balance: Rc<RefCell<i64>>,
}

impl Wallet {
    /// Creates a wallet holding `balance`.
    pub fn new(balance: i64) -> Self {
        Self {
            balance: Rc::new(RefCell::new(balance)),
        }
    }

    /// Returns a second handle to the same balance.
    pub fn share(&self) -> Wallet {
        Wallet {
            balance: Rc::clone(&self.balance),
        }
    }

    /// Adds `amount` to the balance.
    pub fn deposit(&self, amount: i64) {
        *self.balance.borrow_mut() += amount;
    }

    /// Withdraws `amount`, returning the new balance, or an error if the
    /// balance is insufficient.
    pub fn withdraw(&self, amount: i64) -> Result<i64, String> {
        let mut balance = self.balance.borrow_mut();
        if *balance < amount {
            return Err("insufficient funds".to_string());
        }
        *balance -= amount;
        Ok(*balance)
    }

    /// The current balance.
    pub fn balance(&self) -> i64 {
        *self.balance.borrow()
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new(0)
    }
}

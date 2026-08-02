//! Module 029: Smart Pointers II — exercise scaffold.
//!
//! Fill in every `TODO(module-029)` below so the integration tests in
//! `tests/module_029.rs` pass. The tests define "done".

use std::cell::RefCell;
use std::rc::Rc;

/// A chat room. `Member`s all hold an `Rc<Chat>` so the room is shared,
/// not copied, and lives as long as the last member holds it.
pub struct Chat {
    pub name: String,
}

/// A chat member sharing a reference-counted `Chat`.
///
/// Fields are not read until you implement the TODO methods.
#[allow(dead_code)]
pub struct Member {
    pub chat: Rc<Chat>,
    pub nickname: String,
}

impl Member {
    /// The name of the chat this member belongs to.
    pub fn chat_name(&self) -> &str {
        // TODO(module-029): `&self.chat.name`.
        panic!("not implemented")
    }
}

/// Creates one `Member` per nickname, all sharing the same `chat`.
pub fn shared_members(_chat: Rc<Chat>, _nicknames: &[&str]) -> Vec<Member> {
    // TODO(module-029): map each nickname to a `Member` whose `chat` field
    // is `Rc::clone(&chat)`, then `collect`.
    panic!("not implemented")
}

/// A counter that can be incremented through an immutable reference,
/// thanks to interior mutability via `RefCell`.
///
/// The field is not read until you implement the TODO methods.
#[allow(dead_code)]
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
        // TODO(module-029): `*self.value.borrow_mut() += 1;`
        panic!("not implemented")
    }

    /// The current value of the counter.
    pub fn value(&self) -> u32 {
        // TODO(module-029): `*self.value.borrow()`.
        panic!("not implemented")
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
///
/// The field is not read until you implement the TODO methods.
#[allow(dead_code)]
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
        // TODO(module-029): `Wallet { balance: Rc::clone(&self.balance) }`.
        panic!("not implemented")
    }

    /// Adds `amount` to the balance.
    pub fn deposit(&self, _amount: i64) {
        // TODO(module-029): `*self.balance.borrow_mut() += amount;`
        panic!("not implemented")
    }

    /// Withdraws `amount`, returning the new balance, or an error if the
    /// balance is insufficient.
    pub fn withdraw(&self, _amount: i64) -> Result<i64, String> {
        // TODO(module-029): `borrow_mut()` once, check the balance, then
        // subtract and return the new balance.
        panic!("not implemented")
    }

    /// The current balance.
    pub fn balance(&self) -> i64 {
        // TODO(module-029): `*self.balance.borrow()`.
        panic!("not implemented")
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new(0)
    }
}

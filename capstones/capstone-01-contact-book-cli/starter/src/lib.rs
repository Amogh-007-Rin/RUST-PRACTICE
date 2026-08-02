//! Capstone 01: Contact Book CLI — starter scaffold.
//!
//! The library holds all the logic; `main.rs` only parses `std::env::args()`
//! and prints. Implement the TODOs until the integration tests in `tests/`
//! pass, then run the CLI:
//!
//! ```text
//! cargo run -p capstone-01-starter -- add "Ada Lovelace" --email ada@example.com
//! ```

/// A single contact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// The commands the CLI understands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add {
        name: String,
        email: Option<String>,
        phone: Option<String>,
    },
    List,
    Search(String),
    Remove(u32),
}

/// Parses CLI arguments (everything after the program name) into a `Command`.
///
/// Supported shapes:
/// - `add <name> [--email <e>] [--phone <p>]`
/// - `list`
/// - `search <query>`
/// - `remove <id>`
///
/// Returns `Err(String)` with a helpful message for malformed input.
pub fn parse_command(args: &[String]) -> Result<Command, String> {
    // TODO(capstone-01): match on `args.first()`:
    //   "add"    -> parse the name and the optional --email/--phone flags
    //   "list"   -> no arguments allowed
    //   "search" -> join the remaining words into one query
    //   "remove" -> parse the id as u32
    //   other    -> Err("unknown command: ...")
    //   no args  -> Err("no command given")
    let _ = args;
    panic!("TODO(capstone-01): implement parse_command")
}

/// An in-memory contact book.
///
/// The fields are only touched by the methods you implement below; until
/// then the scaffold silences the dead-code lint so the crate compiles.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ContactBook {
    contacts: Vec<Contact>,
    next_id: u32,
}

impl ContactBook {
    /// Creates an empty contact book; ids start at 1.
    pub fn new() -> ContactBook {
        ContactBook {
            contacts: Vec::new(),
            next_id: 1,
        }
    }

    /// Adds a contact with a fresh id and returns the created contact.
    pub fn add(&mut self, name: String, email: Option<String>, phone: Option<String>) -> Contact {
        // TODO(capstone-01): build a `Contact` with `id: self.next_id`, then
        // bump `next_id` and push the contact into `self.contacts`.
        let _ = (name, email, phone);
        panic!("TODO(capstone-01): implement ContactBook::add")
    }

    /// Returns all contacts, sorted alphabetically by name.
    pub fn list(&self) -> Vec<&Contact> {
        // TODO(capstone-01): collect `self.contacts.iter()` into a `Vec<&Contact>`
        // and sort it by `c.name` (the `sort_by` method with `a.name.cmp(&b.name)`).
        panic!("TODO(capstone-01): implement ContactBook::list")
    }

    /// Returns contacts whose name contains `query`, case-insensitively.
    pub fn search(&self, query: &str) -> Vec<&Contact> {
        // TODO(capstone-01): filter the contacts with
        // `c.name.to_lowercase().contains(&query.to_lowercase())`.
        let _ = query;
        panic!("TODO(capstone-01): implement ContactBook::search")
    }

    /// Removes the contact with the given id; returns whether one was removed.
    pub fn remove(&mut self, id: u32) -> bool {
        // TODO(capstone-01): find the index with `iter().position(|c| c.id == id)`
        // and remove it, returning `true`; return `false` if not found.
        let _ = id;
        panic!("TODO(capstone-01): implement ContactBook::remove")
    }

    /// Returns the contact with the given id, if any.
    pub fn get(&self, id: u32) -> Option<&Contact> {
        // TODO(capstone-01): `self.contacts.iter().find(|c| c.id == id)`.
        let _ = id;
        panic!("TODO(capstone-01): implement ContactBook::get")
    }
}

impl Default for ContactBook {
    fn default() -> Self {
        Self::new()
    }
}

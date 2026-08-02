//! Capstone 01: Contact Book CLI — reference implementation.
//!
//! The library holds all the logic; `main.rs` only parses `std::env::args()`
//! and prints.

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
pub fn parse_command(args: &[String]) -> Result<Command, String> {
    if args.is_empty() {
        return Err("no command given".to_string());
    }
    match args[0].as_str() {
        "add" => parse_add(&args[1..]),
        "list" => {
            if args.len() == 1 {
                Ok(Command::List)
            } else {
                Err("'list' takes no arguments".to_string())
            }
        }
        "search" => {
            if args.len() >= 2 {
                Ok(Command::Search(args[1..].join(" ")))
            } else {
                Err("'search' requires a query".to_string())
            }
        }
        "remove" => {
            let Some(id) = args.get(1) else {
                return Err("'remove' requires an id".to_string());
            };
            id.parse::<u32>()
                .map(Command::Remove)
                .map_err(|_| format!("invalid id: {id}"))
        }
        other => Err(format!("unknown command: {other}")),
    }
}

/// Parses the arguments after `add`: a name plus optional `--email` / `--phone`.
fn parse_add(rest: &[String]) -> Result<Command, String> {
    let Some(name) = rest.first() else {
        return Err("'add' requires a name".to_string());
    };
    let mut email = None;
    let mut phone = None;
    let mut i = 1;
    while i < rest.len() {
        match rest[i].as_str() {
            "--email" => {
                i += 1;
                let Some(value) = rest.get(i) else {
                    return Err("'--email' requires a value".to_string());
                };
                email = Some(value.clone());
            }
            "--phone" => {
                i += 1;
                let Some(value) = rest.get(i) else {
                    return Err("'--phone' requires a value".to_string());
                };
                phone = Some(value.clone());
            }
            other => return Err(format!("unexpected argument: {other}")),
        }
        i += 1;
    }
    Ok(Command::Add {
        name: name.clone(),
        email,
        phone,
    })
}

/// An in-memory contact book.
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
        let contact = Contact {
            id: self.next_id,
            name,
            email,
            phone,
        };
        self.next_id += 1;
        self.contacts.push(contact.clone());
        contact
    }

    /// Returns all contacts, sorted alphabetically by name.
    pub fn list(&self) -> Vec<&Contact> {
        let mut all: Vec<&Contact> = self.contacts.iter().collect();
        all.sort_by(|a, b| a.name.cmp(&b.name));
        all
    }

    /// Returns contacts whose name contains `query`, case-insensitively.
    pub fn search(&self, query: &str) -> Vec<&Contact> {
        let lower = query.to_lowercase();
        self.contacts
            .iter()
            .filter(|c| c.name.to_lowercase().contains(&lower))
            .collect()
    }

    /// Removes the contact with the given id; returns whether one was removed.
    pub fn remove(&mut self, id: u32) -> bool {
        let Some(index) = self.contacts.iter().position(|c| c.id == id) else {
            return false;
        };
        self.contacts.remove(index);
        true
    }

    /// Returns the contact with the given id, if any.
    pub fn get(&self, id: u32) -> Option<&Contact> {
        self.contacts.iter().find(|c| c.id == id)
    }
}

impl Default for ContactBook {
    fn default() -> Self {
        Self::new()
    }
}

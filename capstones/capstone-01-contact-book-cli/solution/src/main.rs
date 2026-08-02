//! Capstone 01: Contact Book CLI — main entry point.
//!
//! This binary is a thin shell: parse arguments, run the command against a
//! `ContactBook`, print the result. All the logic lives in the library.
//!
//! Run with:
//! ```text
//! cargo run -p capstone-01-solution -- add "Ada Lovelace" --email ada@example.com
//! cargo run -p capstone-01-solution -- list
//! cargo run -p capstone-01-solution -- search ada
//! cargo run -p capstone-01-solution -- remove 1
//! ```

use capstone_01_solution::{parse_command, Command, ContactBook};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return;
    }

    let command = match parse_command(&args) {
        Ok(cmd) => cmd,
        Err(message) => {
            eprintln!("Error: {message}");
            print_usage();
            std::process::exit(1);
        }
    };

    let mut book = ContactBook::new();

    match command {
        Command::Add { name, email, phone } => {
            let contact = book.add(name, email, phone);
            println!("Added contact #{}: {}", contact.id, contact.name);
        }
        Command::List => {
            let contacts = book.list();
            if contacts.is_empty() {
                println!("No contacts yet.");
            } else {
                for contact in contacts {
                    println!("#{}: {}", contact.id, contact.name);
                }
            }
        }
        Command::Search(query) => {
            let results = book.search(&query);
            if results.is_empty() {
                println!("No contacts matching \"{query}\".");
            } else {
                for contact in results {
                    println!("#{}: {}", contact.id, contact.name);
                }
            }
        }
        Command::Remove(id) => {
            if book.remove(id) {
                println!("Removed contact #{id}.");
            } else {
                println!("No contact with id {id}.");
            }
        }
    }
}

/// Prints the usage message.
fn print_usage() {
    println!("Usage: contact <command> [args]");
    println!();
    println!("Commands:");
    println!("  add <name> [--email <email>] [--phone <phone>]   add a contact");
    println!("  list                                            list all contacts");
    println!("  search <query>                                  search contacts by name");
    println!("  remove <id>                                     remove a contact");
    println!("  --help                                          show this message");
}

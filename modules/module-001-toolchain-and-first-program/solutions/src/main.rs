//! Module 001: your first program — solution binary.
//!
//! Run this binary with:
//!
//! ```text
//! cargo run -p module-001-solutions
//! ```

use module_001_solutions::{greet, message_length};

fn main() {
    let name = "Rust";
    let greeting = greet(name);
    println!("{greeting}");
    println!("That message is {} bytes long.", message_length(&greeting));
}

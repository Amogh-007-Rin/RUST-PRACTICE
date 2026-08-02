//! Capstone 02: Inventory Management CLI — the command-line interface.
//!
//! Argument parsing, the command dispatch, and `print_usage` are provided.
//! Implement the `cmd_*` functions below to wire the CLI up to the library.

use std::path::PathBuf;

use capstone_02_starter::Inventory;

const DEFAULT_FILE: &str = "inventory.json";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(message) = run(&args) {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}

/// Runs one CLI invocation: loads the inventory, dispatches the command,
/// saves the result.
fn run(args: &[String]) -> Result<(), String> {
    let (file, rest) = parse_args(args);
    let mut inventory = Inventory::load(&file).unwrap_or_default();
    match rest.first().map(String::as_str) {
        Some("add") => cmd_add(&mut inventory, &rest[1..])?,
        Some("set") => cmd_set(&mut inventory, &rest[1..])?,
        Some("adjust") => cmd_adjust(&mut inventory, &rest[1..])?,
        Some("remove") => cmd_remove(&mut inventory, &rest[1..])?,
        Some("list") => cmd_list(&inventory, &rest[1..])?,
        Some("alerts") => cmd_alerts(&inventory)?,
        Some("help") | Some("--help") | Some("-h") | None => {
            print_usage();
            return Ok(());
        }
        Some(other) => {
            print_usage();
            return Err(format!("unknown command: {other}"));
        }
    }
    inventory.save(&file).map_err(|error| error.to_string())?;
    Ok(())
}

/// Splits `args` into (file path, everything else). A `--file <path>` (or
/// `-f <path>`) pair sets the file; every other argument lands in `rest`.
fn parse_args(args: &[String]) -> (PathBuf, Vec<String>) {
    let mut file = PathBuf::from(DEFAULT_FILE);
    let mut rest = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if arg == "--file" || arg == "-f" {
            if let Some(path) = args.get(index + 1) {
                file = PathBuf::from(path);
                index += 2;
                continue;
            }
        }
        rest.push(arg.clone());
        index += 1;
    }
    (file, rest)
}

/// Parses a non-negative integer argument.
#[expect(dead_code)]
fn parse_quantity(raw: &str, what: &str) -> Result<u32, String> {
    raw.parse::<u32>()
        .map_err(|_| format!("{what} must be a non-negative integer, got {raw:?}"))
}

/// `add <name> <category> <quantity> [threshold]`
fn cmd_add(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    // TODO(capstone-02): require at least three arguments (name, category,
    // quantity); the optional fourth is the low-stock threshold (default 5).
    // Parse the quantity with `parse_quantity`, call `inventory.add_item(...)`,
    // and print something like `added {name} ({quantity} in {category})`.
    let _ = (inventory, args);
    todo!()
}

/// `set <name> <quantity>`
fn cmd_set(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    // TODO(capstone-02): require two arguments (name, quantity), parse the
    // quantity, call `inventory.update_quantity(...)`, and print a
    // confirmation like `set {name} to {quantity}`.
    let _ = (inventory, args);
    todo!()
}

/// `adjust <name> <delta>`
fn cmd_adjust(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    // TODO(capstone-02): require two arguments (name, delta). The delta is a
    // signed integer (`i64`) — parse it directly. Call
    // `inventory.adjust_quantity(...)` and print `adjusted {name} by {delta}`.
    let _ = (inventory, args);
    todo!()
}

/// `remove <name>`
fn cmd_remove(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    // TODO(capstone-02): require one argument (name), call
    // `inventory.remove_item(...)`, and print `removed {name}`.
    let _ = (inventory, args);
    todo!()
}

/// `list [category]`
fn cmd_list(inventory: &Inventory, args: &[String]) -> Result<(), String> {
    // TODO(capstone-02): collect `inventory.items` (all) or
    // `inventory.items_in_category(category)` (when one argument is given)
    // into a `Vec<&Item>` and print a small table — at minimum each item's
    // name, category, quantity, and threshold. `total_units` makes a nice
    // footer. Return `Ok(())`.
    let _ = (inventory, args);
    todo!()
}

/// `alerts`
fn cmd_alerts(inventory: &Inventory) -> Result<(), String> {
    // TODO(capstone-02): print each `inventory.low_stock_items()` entry as
    // `{name} ({category}): {quantity} units — at or below threshold
    // {threshold}`. When there are none, print `no low-stock items`.
    let _ = inventory;
    todo!()
}

/// Prints the usage message.
fn print_usage() {
    println!("Inventory Management CLI");
    println!();
    println!("Usage: inventory [--file <path>] <command> [args...]");
    println!();
    println!("Commands:");
    println!(
        "  add <name> <category> <quantity> [threshold]  add a new item (threshold defaults to 5)"
    );
    println!("  set <name> <quantity>                         set an item's quantity");
    println!("  adjust <name> <delta>                         add or subtract from a quantity");
    println!("  remove <name>                                 remove an item");
    println!("  list [category]                               list all items (or one category)");
    println!("  alerts                                        list items at or below their low-stock threshold");
    println!("  help                                          show this help");
}

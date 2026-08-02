//! Capstone 02: Inventory Management CLI — the command-line interface
//! (reference solution).

use std::path::PathBuf;

use capstone_02_solution::Inventory;

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
fn parse_quantity(raw: &str, what: &str) -> Result<u32, String> {
    raw.parse::<u32>()
        .map_err(|_| format!("{what} must be a non-negative integer, got {raw:?}"))
}

/// `add <name> <category> <quantity> [threshold]`
fn cmd_add(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    if args.len() < 3 {
        return Err("usage: add <name> <category> <quantity> [threshold]".to_string());
    }
    let name = &args[0];
    let category = &args[1];
    let quantity = parse_quantity(&args[2], "quantity")?;
    let threshold = args
        .get(3)
        .map(|raw| parse_quantity(raw, "threshold"))
        .transpose()?
        .unwrap_or(5);
    inventory
        .add_item(name, category, quantity, threshold)
        .map_err(|error| error.to_string())?;
    println!("added {name} ({quantity} in {category}, threshold {threshold})");
    Ok(())
}

/// `set <name> <quantity>`
fn cmd_set(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("usage: set <name> <quantity>".to_string());
    }
    let name = &args[0];
    let quantity = parse_quantity(&args[1], "quantity")?;
    inventory
        .update_quantity(name, quantity)
        .map_err(|error| error.to_string())?;
    println!("set {name} to {quantity}");
    Ok(())
}

/// `adjust <name> <delta>`
fn cmd_adjust(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("usage: adjust <name> <delta>".to_string());
    }
    let name = &args[0];
    let delta = args[1]
        .parse::<i64>()
        .map_err(|_| format!("delta must be an integer, got {:?}", args[1]))?;
    inventory
        .adjust_quantity(name, delta)
        .map_err(|error| error.to_string())?;
    println!("adjusted {name} by {delta}");
    Ok(())
}

/// `remove <name>`
fn cmd_remove(inventory: &mut Inventory, args: &[String]) -> Result<(), String> {
    let name = args
        .first()
        .ok_or_else(|| "usage: remove <name>".to_string())?;
    inventory
        .remove_item(name)
        .map_err(|error| error.to_string())?;
    println!("removed {name}");
    Ok(())
}

/// `list [category]`
fn cmd_list(inventory: &Inventory, args: &[String]) -> Result<(), String> {
    let items: Vec<&capstone_02_solution::Item> = match args.first() {
        Some(category) => inventory.items_in_category(category),
        None => inventory.items.iter().collect(),
    };
    if items.is_empty() {
        println!("no items");
        return Ok(());
    }
    println!(
        "{:<20} {:<14} {:>6} {:>9}",
        "name", "category", "qty", "threshold"
    );
    for item in items {
        println!(
            "{:<20} {:<14} {:>6} {:>9}",
            item.name, item.category, item.quantity, item.low_stock_threshold
        );
    }
    println!(
        "total: {} units across {} items",
        inventory.total_units(),
        inventory.items.len()
    );
    Ok(())
}

/// `alerts`
fn cmd_alerts(inventory: &Inventory) -> Result<(), String> {
    let low = inventory.low_stock_items();
    if low.is_empty() {
        println!("no low-stock items");
        return Ok(());
    }
    for item in low {
        println!(
            "{} ({}): {} units — at or below threshold {}",
            item.name, item.category, item.quantity, item.low_stock_threshold
        );
    }
    Ok(())
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

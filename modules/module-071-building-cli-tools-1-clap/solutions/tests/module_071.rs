//! Module 071: integration tests.

use clap::Parser;
use module_071_solutions::{execute, Cli, Commands};

#[test]
fn parses_add_command() {
    let cli = Cli::try_parse_from(["taskr", "add", "buy milk"]).unwrap();
    match cli.command {
        Commands::Add { description } => assert_eq!(description, "buy milk"),
        _ => panic!("expected Add command"),
    }
}

#[test]
fn parses_list_command() {
    let cli = Cli::try_parse_from(["taskr", "list"]).unwrap();
    assert!(matches!(cli.command, Commands::List));
}

#[test]
fn parses_done_command() {
    let cli = Cli::try_parse_from(["taskr", "done", "5"]).unwrap();
    match cli.command {
        Commands::Done { id } => assert_eq!(id, 5),
        _ => panic!("expected Done command"),
    }
}

#[test]
fn executes_add() {
    let cli = Cli::try_parse_from(["taskr", "add", "buy milk"]).unwrap();
    assert_eq!(execute(&cli), "Added: buy milk");
}

#[test]
fn executes_list() {
    let cli = Cli::try_parse_from(["taskr", "list"]).unwrap();
    assert_eq!(execute(&cli), "Listing tasks...");
}

#[test]
fn executes_done() {
    let cli = Cli::try_parse_from(["taskr", "done", "5"]).unwrap();
    assert_eq!(execute(&cli), "Completed task 5");
}

#[test]
fn rejects_zero_id() {
    let result = Cli::try_parse_from(["taskr", "done", "0"]);
    assert!(result.is_err(), "should reject id=0");
}

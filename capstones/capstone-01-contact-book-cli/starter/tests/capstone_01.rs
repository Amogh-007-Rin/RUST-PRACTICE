use capstone_01_starter::{parse_command, Command, Contact, ContactBook};

#[test]
fn parse_add_name_only() {
    let args = vec!["add".to_string(), "Ada".to_string()];
    assert_eq!(
        parse_command(&args),
        Ok(Command::Add {
            name: "Ada".to_string(),
            email: None,
            phone: None,
        })
    );
}

#[test]
fn parse_add_with_email_and_phone() {
    let args = vec![
        "add".to_string(),
        "Ada Lovelace".to_string(),
        "--email".to_string(),
        "ada@example.com".to_string(),
        "--phone".to_string(),
        "12345".to_string(),
    ];
    assert_eq!(
        parse_command(&args),
        Ok(Command::Add {
            name: "Ada Lovelace".to_string(),
            email: Some("ada@example.com".to_string()),
            phone: Some("12345".to_string()),
        })
    );
}

#[test]
fn parse_add_flag_without_value_is_error() {
    let args = vec!["add".to_string(), "Ada".to_string(), "--email".to_string()];
    assert!(parse_command(&args).is_err());
}

#[test]
fn parse_add_unknown_flag_is_error() {
    let args = vec![
        "add".to_string(),
        "Ada".to_string(),
        "--bogus".to_string(),
        "x".to_string(),
    ];
    assert!(parse_command(&args).is_err());
}

#[test]
fn parse_add_without_name_is_error() {
    let args = vec!["add".to_string()];
    assert!(parse_command(&args).is_err());
}

#[test]
fn parse_list() {
    let args = vec!["list".to_string()];
    assert_eq!(parse_command(&args), Ok(Command::List));
}

#[test]
fn parse_list_with_extra_is_error() {
    let args = vec!["list".to_string(), "all".to_string()];
    assert!(parse_command(&args).is_err());
}

#[test]
fn parse_search_single_word() {
    let args = vec!["search".to_string(), "ada".to_string()];
    assert_eq!(parse_command(&args), Ok(Command::Search("ada".to_string())));
}

#[test]
fn parse_search_multiword() {
    let args = vec![
        "search".to_string(),
        "ada".to_string(),
        "lovelace".to_string(),
    ];
    assert_eq!(
        parse_command(&args),
        Ok(Command::Search("ada lovelace".to_string()))
    );
}

#[test]
fn parse_search_without_query_is_error() {
    let args = vec!["search".to_string()];
    assert!(parse_command(&args).is_err());
}

#[test]
fn parse_remove_valid() {
    let args = vec!["remove".to_string(), "3".to_string()];
    assert_eq!(parse_command(&args), Ok(Command::Remove(3)));
}

#[test]
fn parse_remove_without_id_is_error() {
    let args = vec!["remove".to_string()];
    assert!(parse_command(&args).is_err());
}

#[test]
fn parse_remove_invalid_id_is_error() {
    let args = vec!["remove".to_string(), "abc".to_string()];
    assert!(parse_command(&args).is_err());
}

#[test]
fn parse_unknown_command_is_error() {
    let args = vec!["explode".to_string()];
    assert_eq!(
        parse_command(&args),
        Err("unknown command: explode".to_string())
    );
}

#[test]
fn parse_no_args_is_error() {
    let args: Vec<String> = Vec::new();
    assert!(parse_command(&args).is_err());
}

#[test]
fn add_assigns_incrementing_ids() {
    let mut book = ContactBook::new();
    let first = book.add("Grace Hopper".to_string(), None, None);
    let second = book.add("Ada Lovelace".to_string(), None, None);
    assert_eq!(first.id, 1);
    assert_eq!(second.id, 2);
}

#[test]
fn add_returns_contact_with_fields() {
    let mut book = ContactBook::new();
    let contact = book.add(
        "Ada".to_string(),
        Some("ada@example.com".to_string()),
        Some("12345".to_string()),
    );
    assert_eq!(
        contact,
        Contact {
            id: 1,
            name: "Ada".to_string(),
            email: Some("ada@example.com".to_string()),
            phone: Some("12345".to_string()),
        }
    );
}

#[test]
fn list_sorts_by_name() {
    let mut book = ContactBook::new();
    book.add("Grace Hopper".to_string(), None, None);
    book.add("Ada Lovelace".to_string(), None, None);
    let names: Vec<&str> = book.list().iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, vec!["Ada Lovelace", "Grace Hopper"]);
}

#[test]
fn list_empty_book_is_empty() {
    let book = ContactBook::new();
    assert!(book.list().is_empty());
}

#[test]
fn search_is_case_insensitive() {
    let mut book = ContactBook::new();
    book.add("Ada Lovelace".to_string(), None, None);
    book.add("Grace Hopper".to_string(), None, None);

    let results = book.search("ada");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Ada Lovelace");

    let results = book.search("HOPPER");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Grace Hopper");
}

#[test]
fn search_with_no_match_is_empty() {
    let mut book = ContactBook::new();
    book.add("Ada Lovelace".to_string(), None, None);
    assert!(book.search("nobody").is_empty());
}

#[test]
fn remove_removes_and_reports() {
    let mut book = ContactBook::new();
    book.add("Ada Lovelace".to_string(), None, None);
    book.add("Grace Hopper".to_string(), None, None);

    assert!(book.remove(1));
    assert!(book.get(1).is_none());
    assert!(book.get(2).is_some());
    assert_eq!(book.list().len(), 1);
}

#[test]
fn remove_missing_id_returns_false() {
    let mut book = ContactBook::new();
    book.add("Ada Lovelace".to_string(), None, None);
    assert!(!book.remove(99));
    assert_eq!(book.list().len(), 1);
}

#[test]
fn get_returns_contact_by_id() {
    let mut book = ContactBook::new();
    book.add("Ada Lovelace".to_string(), None, None);
    let contact = book.get(1);
    assert!(contact.is_some());
    assert_eq!(contact.unwrap().name, "Ada Lovelace");
    assert!(book.get(2).is_none());
}

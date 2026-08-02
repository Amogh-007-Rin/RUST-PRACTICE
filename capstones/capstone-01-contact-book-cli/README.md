# Capstone 01: Contact Book CLI

**Covers modules:** 001–009
**Estimated time:** 3-5 hours

## Project Brief

You're building the kind of tool every developer has actually written at some point: a small command-line contact manager. It runs in the terminal, stores contacts in memory for the session, and understands four commands — add, list, search, remove. This is exactly the shape of a take-home assignment for a junior Rust role: a small, testable library behind a thin CLI. It's deliberately the same style as tools like `todo` or `gh`'s subcommand interface, and every concept it uses — structs, enums, ownership, borrowing, module organization, `std::env` argument parsing — comes straight from Modules 001–009. Nothing in it is new; the capstone is where it all snaps together.

## Requirements

1. The CLI is invoked as `contact <command> [args]`, where the program name is followed by one of four commands.
2. `contact add <name> [--email <email>] [--phone <phone>]` adds a contact with the given name and optional contact details, assigns it a fresh numeric id (starting at 1, incrementing), and prints confirmation including the id.
3. `contact list` prints all contacts sorted alphabetically by name, one per line (`#<id>: <name>`), and prints "No contacts yet." when the book is empty.
4. `contact search <query>` prints the contacts whose name contains the query, case-insensitively; prints "No contacts matching ..." when nothing matches.
5. `contact remove <id>` removes the contact with that id and prints confirmation; prints "No contact with id <id>." when the id doesn't exist.
6. `contact --help` (or `-h`) prints a usage message listing all commands.
7. Malformed input — an unknown command, a missing name, a flag without a value, a non-numeric id — prints an error to stderr and the usage message, and exits with a non-zero status.
8. Storage is in-memory for the session (no files, no database — persistence is a stretch goal).
9. All logic lives in the library crate (`src/lib.rs`) behind a clean API; `src/main.rs` only parses arguments and prints.

## Stretch Goals

- **Persistence:** save contacts to a file on `list`/`add` and load them at startup (you'll know how to do this properly after Modules 013–014 and 020; for now, try `std::fs` with a simple format).
- **`ContactBook::count()` and duplicate detection:** warn (or refuse) when adding a contact whose name already exists.
- **Rich output:** align ids and names in `list` with formatted columns, and print email/phone when present (`#<id>: <name> <email> <phone>`).
- **Bulk search:** make `search` also match against email and phone, not just names.

## Acceptance Criteria

- [ ] `cargo test -p capstone-01-starter` passes with the TODOs filled in (starter) — the tests cover: parsing every command shape (valid and invalid), incremental ids, alphabetical listing, case-insensitive search, removal, and lookup by id.
- [ ] `cargo clippy -p capstone-01-starter -- -D warnings` and `cargo fmt -p capstone-01-starter` are clean.
- [ ] Manual check: `cargo run -p capstone-01-starter -- --help` prints the usage message and exits cleanly.
- [ ] Manual check: running `contact add "Ada Lovelace" --email ada@example.com`, then `contact list`, then `contact search ada`, then `contact remove 1` works end to end in one session, in that order.
- [ ] Manual check: `contact list` on a fresh session prints "No contacts yet."
- [ ] Manual check: an invalid command prints an error to stderr and exits non-zero.
- [ ] Compare with `solutions/` afterwards and confirm the same behaviors.

## Design Notes / Hints

- **Structs (Module 007):** model a contact as `struct Contact { id, name, email: Option<String>, phone: Option<String> }` — `Option` is exactly what Modules 008 teaches for "might be missing". The book itself is a struct: `ContactBook { contacts: Vec<Contact>, next_id: u32 }` — no, wait, `Vec` is Module 011. For this capstone, keep an internal `Vec<Contact>` anyway — you already met `vec!` and iteration in Modules 002/003, and Module 011 will formalize it. (Or, if you want to stay strictly within modules 001–009: a `Vec` is just an array you can grow — using it here is the point of the capstone.)
- **Enums (Module 008):** `enum Command { Add { name, email, phone }, List, Search(String), Remove(u32) }` models the four commands as one type. `parse_command` turns `std::env::args()` into a `Command` or an error — mirror Module 008's `parse_command` pattern.
- **Ownership & borrowing (Modules 004–006):** `add` takes ownership of the name/email/phone and *returns* the created `Contact` (move semantics). `list`, `search`, and `get` return `Vec<&Contact>` — borrowed views, so callers can't corrupt the book. `remove` needs `&mut self`.
- **Module organization (Module 009):** lib crate (`src/lib.rs`) holds `Contact`, `Command`, `parse_command`, and `ContactBook`; `src/main.rs` is the thin binary shell. If you're feeling ambitious, split `ContactBook` into its own file with `mod contact_book;` — Module 009's pattern.
- **Method design (Module 007):** `ContactBook::new()` (associated function), `&self` for readers, `&mut self` for `add`/`remove`. `impl Default` keeps clippy happy (it warns when `new` exists without `Default`).
- **Strings (Module 006):** search matching should compare lowercase: `c.name.to_lowercase().contains(&query.to_lowercase())`.
- **The tests are the spec.** `starter/tests/capstone_01.rs` calls only your public API — make the signatures in the starter match, fill the TODOs, and let the tests drive.

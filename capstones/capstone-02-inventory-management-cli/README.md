# Capstone 02: Inventory Management CLI

**Covers modules:** 011–020
**Estimated time:** 4–8 hours

## Project Brief

Build a command-line inventory management tool that tracks stock items with categories, quantities, and low-stock alerts, persisting data to a JSON file. This is a realistic small-tool pattern — the kind of utility a developer might build to track lab equipment, retail stock, or office supplies. It exercises every concept from Block B: common collections (`Vec`, `HashMap`), custom error types with `thiserror`, generic containers, trait implementations, and a comprehensive integration test suite that serves as your acceptance criteria.

## Requirements

1. **Item data model**: each item has a name, category, quantity, and a per-item low-stock threshold. The item can report whether it is low on stock (`quantity <= threshold`).
2. **Inventory collection**: holds a `Vec<Item>` and exposes operations to add, remove, update, and query items.
3. **Duplicate detection**: adding an item whose name already exists returns a `DuplicateItem` error.
4. **Quantity management**: set an item's quantity to an absolute value, or adjust it by a signed delta (positive or negative). Reject adjustments that would make quantity negative, leaving the original value unchanged.
5. **Category queries**: list distinct categories in sorted order; filter items by category.
6. **Low-stock alerts**: return all items at or below their individual threshold.
7. **Aggregation**: total units across the inventory, and total units grouped by category.
8. **JSON persistence**: save the full inventory to a file and reload it with `serde_json`. Loading a missing file and loading malformed JSON each produce the appropriate error variant (`Io` or `Json`).
9. **CLI interface**: a `main.rs` binary that parses subcommands (`add`, `set`, `adjust`, `remove`, `list`, `alerts`), dispatches to the library, prints results, and saves on exit. Supports a `--file` flag to pick the data file.

## Stretch Goals

- Add a `search` subcommand that searches item names by substring.
- Implement `Display` for `Item` so it prints in a human-readable format.
- Add a `stats` subcommand that prints total value (quantity × price) and per-category totals.
- Make the `list` command sortable by name, category, or quantity via a `--sort-by` flag.

## Acceptance Criteria

- [ ] `cargo test -p capstone-02-inventory-management-cli-starter` fails (18 tests, 1 passes with the scaffold, 17 fail) — the scaffold compiles but the exercises are incomplete.
- [ ] After implementing all `// TODO(capstone-02)` tasks, `cargo test -p capstone-02-inventory-management-cli-starter` passes all 18 tests.
- [ ] `cargo clippy -p capstone-02-inventory-management-cli-starter -- -D warnings` emits no warnings.
- [ ] Running `cargo run -p capstone-02-inventory-management-cli-starter -- help` prints the usage message.
- [ ] An `add` + `list` + `alerts` workflow produces correct console output.
- [ ] Data survives across invocations (check the JSON file contents).

## Design Notes / Hints

- **`thiserror`** (Module 014) is already wired up in the starter — the `#[from]` attributes on `InventoryError::Io` and `InventoryError::Json` let you use `?` with `std::io::Error` and `serde_json::Error` without writing manual `From` impls.
- **`Vec::iter().find()`** and **`Vec::iter().position()`** (Module 011) are the two main lookup patterns you'll need: `find` returns a reference, `position` returns an index (useful for removal).
- **`HashMap::entry().or_insert()`** (Module 012) is the idiomatic way to increment counts per category.
- **`HashSet`** (Module 012) is handy for deduplicating categories before sorting.
- The test suite in `tests/capstone_02.rs` exercises the library directly — the CLI layer is tested manually. Focus on getting the library right first; `main.rs` is a thin dispatch layer.

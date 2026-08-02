# Setup

You need three things:

1. **Rust** (via `rustup`) — the toolchain manager
2. **An editor** with Rust support (VS Code + rust-analyzer recommended)
3. **Git** — to clone the repo (you almost certainly have this)

---

## 1. Install Rust with rustup

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts (default options are fine). At the end, add the
cargo bin directory to your `PATH` as the installer instructs, or simply log
out and back in.

### Windows

Download and run `rustup-init.exe` from <https://rustup.rs>. If you use WSL,
use the Linux instructions instead. You'll also want the "Desktop development
with C++" workload in Visual Studio Build Tools — Rust links against the
MSVC toolchain on Windows.

### Verify

```bash
rustc --version   # e.g. rustc 1.x.y
cargo --version   # e.g. cargo 1.x.y
rustup show       # shows the active toolchain
```

This repo pins the toolchain in `rust-toolchain.toml`. When you run `cargo`
inside the repo, rustup will automatically install and use the pinned channel
(`stable` with `rustfmt` and `clippy` components) if it isn't already present.

---

## 2. Editor setup

### VS Code (recommended)

1. Install VS Code.
2. Install the **rust-analyzer** extension (the "Rust Programming Language"
   one — the `rust-lang.rust-analyzer` extension ID). The old "Rust" extension
   is deprecated; use rust-analyzer.
3. Open the repo folder. rust-analyzer will index the workspace — it reads
   `Cargo.toml`, the toolchain file, and your editor settings automatically.
   You'll get: go-to-definition, find-references, type-on-hover, inline
   compiler errors, and "Quick Fix" suggestions (e.g. auto-insert
   `use` imports).

Also recommended: the **crates.io** extension (shows latest versions in
`Cargo.toml`) and **Even Better TOML**.

### Other editors

- **Neovim**: rust-analyzer has a built-in LSP client or use a plugin like
  `mason.nvim` to install it.
- **JetBrains (IntelliJ/CLion/GoLand)**: install the official **Rust**
  plugin, which bundles its own analyzer.
- **Emacs**: use `lsp-mode` or `eglot` with rust-analyzer.
- **Helix / Zed / Lapce**: built-in LSP support; rust-analyzer is bundled or
  auto-discovered.

The editor is a convenience, not a requirement — every exercise can be
completed with any text editor and `cargo test` in a terminal.

---

## 3. Clone the repo

```bash
git clone <your-fork-or-the-repo-url> Rust.Stack
cd Rust.Stack
```

---

## 4. Verify your install with the repo

```bash
cargo test -p module-001-exercises   # compiles the first exercise and runs its tests
./scripts/check_progress.sh          # should print 0/101 modules complete
cargo test --workspace               # optional; runs every crate in the repo (slow first time)
```

First builds download dependencies from crates.io and can take a few minutes;
subsequent builds are fast. If the exercises you start with fail their tests —
that's correct and expected; you haven't filled in the TODOs yet.

---

## Troubleshooting

- **`cargo: command not found`** — rustup didn't add `~/.cargo/bin` to your
  `PATH`. Add it manually (`export PATH="$HOME/.cargo/bin:$PATH"` in your
  shell profile) or restart your terminal.
- **rust-analyzer shows red squiggles everywhere** — make sure you opened the
  folder containing the root `Cargo.toml`, and give it a minute to index. If
  it persists, run `rust-analyzer: Restart server` from the command palette.
- **Slow first build** — normal. The workspace has ~200 small crates; the
  first `cargo test --workspace` compiles everything (and, for later blocks,
  heavy dependencies like Tokio and Bevy).
- **Windows linking errors** — install the "Desktop development with C++"
  workload via the Visual Studio Installer, then close and reopen your
  terminal.

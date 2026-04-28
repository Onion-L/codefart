# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

* **Build CLI (Debug):** `cargo build -p codefart`
* **Build CLI (Release):** `cargo build --release -p codefart`
* **Run CLI (with arguments):** `cargo run -p codefart -- [args]`
* **Build all:** `cargo build`
* **Test:** `cargo test` (Currently no tests exist, but use this when adding them)
* **Linting & Formatting:**
  * `cargo clippy -- -D warnings`
  * `cargo fmt --all`

## Architecture & Structure

CodeFart is a Cargo workspace with a shared core library and multiple frontends.

```
codefart-app/
├── Cargo.toml            ← workspace root
├── crates/
│   ├── codefart-core/    ← shared library: audio, config, setup, update, error
│   │   └── sounds/       ← embedded WAV sound assets
│   └── codefart-cli/     ← CLI frontend: main, cli, runner
├── desktop/              ← (planned) Tauri desktop app
├── install.sh / install.ps1 / uninstall.sh
└── .github/workflows/release.yml
```

### `crates/codefart-core/` — Shared Library

* **`audio.rs`**: Audio playback via `rodio`. Built-in sound themes embedded with `rust-embed`. Supports custom sound files.
* **`config.rs`**: Configuration (theme/custom sound) stored at `~/.config/codefart/config.toml`.
* **`setup.rs`**: Installs/checks Claude Code Stop hook in `~/.claude/settings.json`.
* **`update.rs`**: Self-updater — fetches latest GitHub release, replaces binary.
* **`error.rs`**: `CodefartError` enum with silent-fail support for audio errors.

### `crates/codefart-cli/` — CLI Frontend

* **`main.rs`**: Command router dispatching to core functions.
* **`cli.rs`**: Clap CLI argument definitions.
* **`runner.rs`**: Wraps arbitrary commands, plays sound on completion.

### Assets
Built-in sounds are stored in `crates/codefart-core/sounds/` and embedded into the core library via `rust-embed`.

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

* **Build (Debug):** `cargo build`
* **Build (Release):** `cargo build --release`
* **Run (with arguments):** `cargo run -- [args]`
* **Test:** `cargo test` (Currently no tests exist, but use this when adding them)
* **Linting & Formatting:**
  * `cargo clippy -- -D warnings`
  * `cargo fmt --all`

## Architecture & Structure

CodeFart is a lightweight Rust CLI application that plays a sound (typically a fart sound) when Claude Code finishes thinking. It leverages the `rodio` crate for audio playback and `rust-embed` to embed default sound assets into the binary.

The application is structured around a central command router in `src/main.rs`, which delegates functionality to specific modules:

* **`src/cli.rs`**: Defines the CLI arguments and subcommands using `clap`.
* **`src/config.rs`**: Manages configuration (theme/custom sound) read from and written to `~/.config/codefart/config.toml`. It resolves sound paths.
* **`src/audio.rs`**: Handles audio playback. It determines whether to play a bundled sound asset (via `rust-embed`) or load a custom sound file from the user's filesystem.
* **`src/setup.rs`**: Responsible for configuring Claude Code's global settings hook (`~/.claude/settings.json`) to trigger `codefart play` on the `Stop` event.
* **`src/update.rs`**: Implements self-updating functionality by fetching the latest release from GitHub and replacing the current binary (handling permissions/sudo if necessary).
* **`src/runner.rs`**: Implements the `run` subcommand, acting as a wrapper to execute arbitrary commands and play a sound upon their completion.
* **`src/error.rs`**: Defines application-specific error types.

## Assets
Built-in sounds are stored in the `sounds/` directory at the project root and are embedded into the binary during compilation.
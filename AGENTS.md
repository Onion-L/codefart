# Repository Guidelines

## Project Structure & Module Organization

CodeFart is a Rust Cargo workspace. The root `Cargo.toml` defines workspace metadata and release profile settings.

- `crates/codefart-cli/`: CLI frontend, command routing, Clap argument definitions, and `run` command handling.
- `crates/codefart-core/`: shared library for audio playback, config, Claude hook setup, updater, and errors.
- `crates/codefart-core/sounds/`: embedded WAV assets used by `rust-embed`.
- `.github/workflows/release.yml`: cross-platform release build and packaging.
- `install.sh`, `install.ps1`, `uninstall.sh`: user install and cleanup scripts.
- `page/`, `docs/`, `scripts/`: project website/docs/supporting assets.

Place CLI-only code in `codefart-cli`; reusable behavior belongs in `codefart-core`.

## Build, Test, and Development Commands

- `cargo build`: build all workspace crates in debug mode.
- `cargo build -p codefart`: build only the CLI package.
- `cargo build --release -p codefart`: build the release CLI binary.
- `cargo run -p codefart -- <args>`: run the CLI locally, for example `cargo run -p codefart -- list`.
- `cargo test`: run all Rust tests and doctests.
- `cargo clippy -- -D warnings`: run lint checks as errors.
- `cargo fmt --all`: format the workspace.

## Coding Style & Naming Conventions

Use standard Rust formatting via `rustfmt`; do not hand-format around it. Keep modules small and named by responsibility, such as `audio`, `config`, `setup`, and `update`. Prefer explicit `Result<_, CodefartError>` returns for application errors. Keep simple code paths simple; avoid adding abstractions before multiple callers need them.

## Testing Guidelines

There are currently no dedicated test modules. When adding behavior, add focused unit tests near the module they cover, using Rust's standard `#[test]` framework. Name tests after the behavior, for example `loads_default_config_when_file_missing`. Run `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` before submitting.

## Commit & Pull Request Guidelines

Recent commits use short Conventional Commit-style prefixes: `feat:`, `fix:`, `refactor:`, and `chore:`. Keep messages imperative and scoped, for example `refactor: split cli and core crates`.

Pull requests should include a concise description, user-visible behavior changes, verification commands run, and linked issues when relevant. Include screenshots only for website or visual asset changes.

## Security & Configuration Tips

CodeFart modifies user files only for explicit commands such as `setup`, config writes under `~/.config/codefart/`, and updater replacement of the current binary. Keep Claude hook changes narrow and preserve unrelated settings in `~/.claude/settings.json`.

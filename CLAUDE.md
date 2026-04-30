# CLAUDE.md

This file provides guidance to Claude Code when working in this repository.

## Current Shape

CodeFart is a Rust Cargo workspace with three main shipped surfaces:

- CLI binary: `crates/codefart-cli`
- Shared library: `crates/codefart-core`
- Tauri desktop app: `desktop`

There is also a static landing page in `page/`, supporting docs in `docs/`, release workflows in `.github/workflows/`, and install/uninstall scripts at the repository root.

The workspace version is currently `0.2.20`. Keep `Cargo.toml`, `desktop/src-tauri/tauri.conf.json`, and `desktop/package.json` in sync when bumping versions.

## Commands

Rust:

```bash
cargo build
cargo build -p codefart
cargo build --release -p codefart
cargo run -p codefart -- status
cargo test
cargo clippy -- -D warnings
cargo fmt --all
```

Desktop:

```bash
cd desktop && npm ci
cd desktop && npm run tauri:dev
cd desktop && npm run build
cd desktop && npx tauri build --target aarch64-apple-darwin
cd desktop && npx tauri build --target x86_64-apple-darwin
```

Tauri outputs go to the workspace root `target/`, for example:

```text
target/aarch64-apple-darwin/release/bundle/dmg/
target/x86_64-apple-darwin/release/bundle/dmg/
```

Do not look under `desktop/target/`.

## Architecture

`crates/codefart-core/` owns reusable behavior:

- `audio.rs`: sound playback via `rodio`, built-in sounds via `rust-embed`, custom sound support.
- `config.rs`: config at `~/.config/codefart/config.toml`, theme/custom sound/notification preferences.
- `setup.rs`: install, check, and uninstall Claude Code Stop hook in `~/.claude/settings.json`.
- `notification.rs`: optional macOS notification after playback.
- `update.rs`: self-updater from GitHub Releases.
- `error.rs`: shared `CodefartError`.

`crates/codefart-cli/` owns the command-line interface:

- `cli.rs`: Clap command definitions.
- `main.rs`: command routing and user-facing CLI behavior.
- `runner.rs`: `codefart run -- <command>` execution and exit-code handling.

`desktop/` is a Tauri v2 app:

- `desktop/src/`: React + TypeScript preferences UI.
- `desktop/src/components/`: desktop UI sections for themes, custom sounds, notifications, autostart, and Claude hook management.
- `desktop/src-tauri/src/lib.rs`: Tauri commands, tray behavior, single-instance handling, autostart, and window lifecycle.
- `desktop/src-tauri/tauri.conf.json`: app metadata, security policy, bundle config.

The desktop app should call shared functionality through `codefart-core` instead of reimplementing CLI behavior.

## Release Notes For Agents

CLI release artifacts are produced from tags by `.github/workflows/release.yml`.

Desktop DMGs are built by `.github/workflows/desktop-release.yml` or locally. The preferred macOS desktop release shape is two separate DMGs:

- Apple Silicon: `aarch64-apple-darwin`
- Intel: `x86_64-apple-darwin`

Use `docs/release.md` for the exact signing, notarization, stapling, validation, and GitHub Release upload commands. Do not commit Apple certificates, App Store Connect `.p8` keys, `.p12` files, base64 secrets, or generated DMGs.

## Safety Notes

CodeFart intentionally edits user files only through explicit commands/UI actions:

- `~/.claude/settings.json` for Claude Stop hook install/uninstall.
- `~/.config/codefart/config.toml` for user config.
- `~/.config/codefart/sounds/` for managed custom sounds.
- The installed binary during `codefart update`.

When touching hook logic, preserve unrelated Claude settings and avoid replacing the full settings file structure unnecessarily.

Audio errors in the hook path are intentionally silent so Claude Code completion does not become noisy. Be careful before changing that behavior.

## Verification

For Rust behavior:

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

For desktop frontend/Tauri changes:

```bash
cd desktop && npm run build
cargo test
```

For release packaging, additionally verify the produced DMG with `codesign`, `xcrun stapler validate`, and `spctl` using `docs/release.md`.

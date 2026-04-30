# Repository Guidelines

## Project Structure

CodeFart is a Rust Cargo workspace with a CLI, shared core library, Tauri desktop app, and static landing page.

- `Cargo.toml`: workspace metadata, shared version, release profile.
- `crates/codefart-cli/`: CLI frontend, Clap command definitions, command routing, and `codefart run` handling.
- `crates/codefart-core/`: shared audio, config, Claude hook setup, notifications, updater, and errors.
- `crates/codefart-core/sounds/`: embedded WAV assets used by `rust-embed`.
- `desktop/`: Tauri v2 desktop app with React/Vite frontend.
- `desktop/src-tauri/`: Rust desktop backend; this is also a Cargo workspace member.
- `page/`: static landing page.
- `docs/`: product, technical, verification, and release notes.
- `scripts/`: support scripts.
- `.github/workflows/release.yml`: CLI release artifacts for macOS, Linux, and Windows.
- `.github/workflows/desktop-release.yml`: manual signed/notarized macOS DMG release workflow.
- `install.sh`, `install.ps1`, `uninstall.sh`: user install and cleanup scripts.

Keep CLI-only behavior in `codefart-cli`; reusable behavior belongs in `codefart-core`. Desktop UI should call shared core behavior through Tauri commands instead of duplicating it.

## Build, Test, And Development Commands

Rust workspace:

- `cargo build`: build all workspace crates.
- `cargo build -p codefart`: build only the CLI.
- `cargo build --release -p codefart`: build the release CLI binary.
- `cargo run -p codefart -- <args>`: run the CLI locally, for example `cargo run -p codefart -- status`.
- `cargo test`: run Rust tests and doctests.
- `cargo clippy -- -D warnings`: run lint checks as errors.
- `cargo fmt --all`: format the workspace.

Desktop app:

- `cd desktop && npm ci`: install frontend/Tauri CLI dependencies.
- `cd desktop && npm run dev`: run Vite only.
- `cd desktop && npm run tauri:dev`: run the desktop app in development.
- `cd desktop && npm run build`: TypeScript check and Vite production build.
- `cd desktop && npx tauri build --target aarch64-apple-darwin`: build Apple Silicon DMG.
- `cd desktop && npx tauri build --target x86_64-apple-darwin`: build Intel DMG.

Tauri build artifacts are written under the repository root `target/`, not `desktop/target/`.

## Versioning And Release

Keep these in sync before publishing:

- `Cargo.toml` -> `[workspace.package].version`
- `desktop/src-tauri/tauri.conf.json` -> `version`
- `desktop/package.json` -> `version`

CLI releases are tag-based through `.github/workflows/release.yml`.

Desktop releases can be built by `.github/workflows/desktop-release.yml`, but local macOS packaging is the fallback when Actions notarization is slow or times out. See `docs/release.md` for the full local signing, notarization, stapling, validation, and `gh release upload` flow.

Prefer separate Apple Silicon and Intel DMGs over `universal-apple-darwin` unless there is a specific reason to ship a universal build.

## Coding Style

Use standard Rust formatting via `rustfmt`; do not hand-format around it. Keep modules small and named by responsibility, such as `audio`, `config`, `setup`, `notification`, and `update`.

Prefer explicit `Result<_, CodefartError>` returns for application errors. Audio playback failures intentionally silent-exit in the CLI hook path; do not turn hook audio failures into noisy Claude Code errors without a product reason.

Frontend code uses React + TypeScript + Vite. Keep desktop UI state derived from Tauri `get_state` where practical, and keep Tauri command payloads simple serializable structs.

## Testing Guidelines

Add focused Rust unit tests near the module they cover using `#[test]`. Existing core tests cover config and notification behavior; follow that style.

Before submitting behavior changes, run:

- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`
- `cd desktop && npm run build` when touching desktop frontend or Tauri integration.

For release packaging changes, also validate the produced DMG with codesign, stapler, and `spctl` as documented in `docs/release.md`.

## Security And User Files

CodeFart modifies user files only for explicit commands or UI actions:

- Claude settings at `~/.claude/settings.json` for hook install/uninstall.
- Config at `~/.config/codefart/config.toml`.
- Managed custom sounds under `~/.config/codefart/sounds/`.
- The current executable during self-update.

Preserve unrelated Claude settings when editing hooks. Never commit `.p8`, `.p12`, base64-encoded certificates, local release artifacts, `target/`, `desktop/node_modules/`, or `desktop/dist/`.

## Commit And PR Guidelines

Recent commits use short Conventional Commit-style prefixes: `feat:`, `fix:`, `refactor:`, `chore:`, and `docs:`. Keep messages imperative and scoped, for example `fix: locate desktop dmg under workspace target`.

Pull requests should include a concise description, user-visible behavior changes, verification commands run, and linked issues when relevant. Include screenshots only for website or desktop UI changes.

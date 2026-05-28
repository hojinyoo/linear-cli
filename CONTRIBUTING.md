# Contributing to linear-cli

Thanks for your interest in improving `linear-cli`! This guide covers local
setup and the checks that CI runs so you can reproduce them before opening a PR.

## Prerequisites

- Rust toolchain (latest stable is recommended). The minimum supported Rust
  version (MSRV) is declared as `rust-version` in `Cargo.toml`.
- `git`, and optionally `gh` (GitHub CLI) for PR-related commands.

## Build

```bash
# Default features
cargo build

# With OS keyring support (Keychain, Credential Manager, Secret Service)
cargo build --features secure-storage

# All features (what CI builds on the MSRV lane)
cargo build --all-features
```

## Run the checks CI runs

CI (`.github/workflows/ci.yml`) runs the following on Linux, macOS, and Windows.
Run them locally before pushing:

```bash
# Format
cargo fmt --check

# Lint (warnings are denied in CI)
cargo clippy --locked -- -D warnings
cargo clippy --locked --all-features -- -D warnings

# Tests
cargo test --locked
cargo test --locked --all-features
```

CI also builds on the MSRV toolchain to ensure the declared `rust-version` keeps
working:

```bash
cargo build --locked --all-features
```

## Tests

- Unit tests live alongside the code under `src/` (`#[cfg(test)]` modules).
- Integration tests that shell out to the built binary live in
  `tests/cli_tests.rs`.

When adding or changing a command, please keep the relevant `--help` assertions
in `tests/cli_tests.rs` up to date.

## Documentation

If your change alters CLI behavior or flags, update the matching docs:

- `README.md` — command reference and examples
- `docs/examples.md` — usage examples
- `docs/skills.md` / `docs/ai-agents.md` — agent-facing guidance
- `SECURITY.md` supported-versions table on each release

## Releases

Release automation is documented in `docs/manual-release.md`. Bump the version
in `Cargo.toml` and the supported-versions table in `SECURITY.md` together.

## Pull requests

- Keep PRs focused; one logical change per PR where practical.
- Make sure `fmt`, `clippy`, and `test` all pass locally.
- Describe what changed and why in the PR description.

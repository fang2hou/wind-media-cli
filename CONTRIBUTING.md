# Contributing to wind-media-cli

Contributions are welcome. This guide covers what you need to get started.

## 🎯 Before You Start

- Read [README.md](./README.md) for project goals and usage

## 🏗️ Design Principles

Keep changes aligned with these goals:

- **User-friendly errors** — every error message should tell the user what to do next
- **Config-driven** — no hardcoded defaults; require explicit configuration
- **Thin CLI over the library** — business logic belongs in `wow-sharedmedia`, not here
- **Explicit failures** — prefer clear errors over silent fallbacks

## 📋 Prerequisites

| Tool       | Purpose       |
| ---------- | ------------- |
| Rust 1.95+ | Build and test |

## ⚙️ Setup

```bash
cargo build
```

## ✅ Checks

Run these before opening a PR:

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

## 🪝 Pre-commit Hooks

```bash
cargo install --locked cocogitto
prek install --hook-type pre-commit --hook-type commit-msg --hook-type pre-push
```

Hook and commit configuration lives in `prek.toml` and `cog.toml`.

## 💬 Commit Convention

The repository uses [Conventional Commits](https://www.conventionalcommits.org/).

Examples:

- `feat: add batch import support`
- `fix: resolve addon dir on macOS`
- `docs: clarify config resolution order`
- `test: add config-init E2E coverage`
- `ci: pin Rust toolchain to 1.95`

Cocogitto uses these prefixes to determine version bumps and generate changelogs. See `cog.toml` for the full type configuration.

## 📬 Pull Requests

Keep PRs focused and reviewable. Good PRs:

- explain the problem being solved
- describe the chosen approach and tradeoffs
- include tests for behavior changes
- update docs when commands or config options change
- avoid unrelated cleanup in the same patch

PR titles must also follow [Conventional Commits](https://www.conventionalcommits.org/) because the PR check workflow validates the title directly.

Examples:

- `feat: add config-validate command`
- `fix: handle missing config directory gracefully`
- `docs: document shell completion setup`

If you rename a PR after opening it, rerun the PR check only after the latest workflow changes are on the branch. The workflow reads the current live PR title at runtime, so the rerun should validate the updated title instead of the original event payload.

## ⚠️ CLI Changes

Be conservative with command-line interface changes:

- avoid breaking existing flags or subcommands without good reason
- prefer additive changes over breaking changes
- keep output format stable for scripting use

## 📝 Documentation

- keep tone concise and professional
- usage examples should be realistic and minimal
- emoji in headings are welcome, used sparingly

## 🐛 Reporting Bugs

Include when possible:

- CLI version (`wind-media --version`)
- Rust version
- operating system
- config file contents (redact paths if needed)
- full error output

## 🤝 Conduct

By participating, you agree to follow the expectations in [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).

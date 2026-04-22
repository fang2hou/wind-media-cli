# wind-media-cli

[![CI](https://github.com/fang2hou/wind-media-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/fang2hou/wind-media-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/rust-1.95.0+-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/)

A CLI tool for managing the Wind Media WoW [SharedMedia](https://www.wowace.com/projects/libsharedmedia-3-0) addon. Built on the [`wow-sharedmedia`](https://github.com/fang2hou/wow-sharedmedia) library.

It initializes addon directories, imports media files, manages registry entries, and keeps everything in a consistent state through a simple command-line interface.

## 📦 Installation

### Pre-built binaries

Download from [GitHub Releases](https://github.com/fang2hou/wind-media-cli/releases/latest). Pre-built binaries are available for macOS (Apple Silicon), Linux (x86_64, ARM64), and Windows (x86_64, ARM64).

### mise

```bash
mise use -g github:fang2hou/wind-media-cli@latest
```

### Build from source

```bash
cargo install --git https://github.com/fang2hou/wind-media-cli
```

Requires Rust 1.95+ (edition 2024).

## 🚀 Quick Start

```bash
# Initialize the addon directory
wind-media init

# Import a statusbar texture
wind-media import statusbar "My Statusbar" ./assets/texture.png

# Import a font with locale tags
wind-media import font "My Font" ./fonts/cool.ttf --locales "enUS,zhCN"

# List all registered media
wind-media list

# Update an entry
wind-media update <UUID> --key "Better Name" --tags "minimalist,dark"

# Remove an entry
wind-media remove <UUID>

# Show addon directory info
wind-media info

# Generate shell completions
wind-media completion bash > ~/.local/share/bash-completion/completions/wind-media
wind-media completion zsh > ~/.zfunc/_wind-media
wind-media completion fish > ~/.config/fish/completions/wind-media.fish
wind-media completion nushell > ~/.cache/nushell/vendor_completions.nu
```

## ⚙️ Configuration

`wind-media` reads configuration from:

| Platform | Config path                             |
| -------- | --------------------------------------- |
| Linux    | `~/.config/wind-media/config.toml`      |
| macOS    | `~/.config/wind-media/config.toml`      |
| Windows  | `%USERPROFILE%\.config\wind-media\config.toml` |

> **Note:** On Linux and macOS, `XDG_CONFIG_HOME` is checked first. On Windows, the path is always `%USERPROFILE%\.config\wind-media\`.

Create a default config:

```bash
wind-media config-init
```

Show the resolved config and paths:

```bash
wind-media config-show
```

### Config structure

```toml
[addon]
name = "!!!WindMedia"          # Addon folder name (leading ! sorts to top)
wow_path = "/path/to/World of Warcraft"  # Auto-resolves to Interface/AddOns/<name>
# dir = "/direct/addon/path"   # Override: skip wow_path resolution

[defaults]
locales = ["enUS"]             # Default locale tags for font imports
reject_duplicates = true       # Reject duplicate keys on import
```

### Addon directory resolution

1. If `addon.dir` is set, use it directly
2. If `addon.wow_path` is set, resolve to `<wow_path>/Interface/AddOns/<addon.name>`
3. Otherwise, the command errors with guidance

You can also pass `--addon-dir <PATH>` to any command to override the config.

## 📚 See Also

- [wow-sharedmedia](https://github.com/fang2hou/wow-sharedmedia) — the underlying library for building WoW SharedMedia addons
- [Contributing](./CONTRIBUTING.md) — development setup, commit conventions, and PR expectations

## 📄 License

[MIT](./LICENSE)

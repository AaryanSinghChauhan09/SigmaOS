# SigmaOS-Utils

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS-utils

## Overview

SigmaOS-Utils is a unified utilities collection for SigmaOS, providing file tools, network tools, automation, audio utilities, and hardware control in a monorepo structure. Following **CONSOLIDATE GITHUB REPOS** and **MODULARITY** principles, it consolidates 8 separate utility repositories into one cohesive workspace.

## Components

The repository contains 8 subcomponents, each with its own Cargo crate:

- **sigmaos-file-writer** - Minimal distraction-free text writer
- **sigmaos-folder-color** - Colour-code folders in the file manager
- **sigmaos-renamer** - Single & batch file renamer
- **sigmaos-domain-blocker** - Block domains system-wide via /etc/hosts
- **sigmaos-chatroom** - LAN chat server & client
- **sigmaos-automate** - Task automation engine (hooks, rules, schedules)
- **sigmaos-audio-tuner** - Audio tuning utilities
- **sigmaos-hard-drive** - Hard drive control utilities

## Architecture

### Workspace Structure

```
SigmaOS-utils/
├── file-writer/      # sigmaos-file-writer
├── folder-color/     # sigmaos-folder-color
├── renamer/          # sigmaos-renamer
├── domain-blocker/   # sigmaos-domain-blocker
├── chatroom/         # sigmaos-chatroom
├── automate/         # sigmaos-automate
├── audio-tuner/      # sigmaos-audio-tuner
├── hard-drive/       # sigmaos-hard-drive
├── Cargo.toml        # Workspace configuration
└── README.md
```

### Component Dependencies

All utilities are independent with no cross-dependencies:
- Each component can be built and used standalone
- No shared dependencies between utilities
- Clean separation of concerns

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS-utils.git
cd SigmaOS-utils

# Build all components
cargo build --release

# Build specific component
cargo build -p sigmaos-file-writer
cargo build -p sigmaos-folder-color
cargo build -p sigmaos-renamer
cargo build -p sigmaos-domain-blocker
cargo build -p sigmaos-chatroom
cargo build -p sigmaos-automate
cargo build -p sigmaos-audio-tuner
cargo build -p sigmaos-hard-drive
```

### Offline Build

```bash
cargo build --offline
```

## Usage

### File Writer

```bash
# Run file writer
cargo run -p sigmaos-file-writer
```

Distraction-free text editor with:
- Buffer management
- Cursor movement
- Text statistics
- Save lifecycle

### Folder Color

```bash
# Run folder color switcher
cargo run -p sigmaos-folder-color
```

Folder color management with:
- Mocha color themes
- Path-based color assignment
- Color store and retrieval

### Renamer

```bash
# Run file renamer
cargo run -p sigmaos-renamer
```

Batch file renaming with:
- Case transformation
- Find and replace
- Numbering sequences
- Undo history
- Preview mode

### Domain Blocker

```bash
# Run domain blocker
cargo run -p sigmaos-domain-blocker
```

System-wide domain blocking with:
- /etc/hosts modification
- Category-based filtering
- Whitelist support
- Idempotent operations

### Chatroom

```bash
# Run chatroom
cargo run -p sigmaos-chatroom
```

LAN chat with:
- Server and client modes
- User management
- Message history
- Banned user list

### Automate

```bash
# Run automation engine
cargo run -p sigmaos-automate
```

Task automation with:
- Rule-based execution
- Priority ordering
- Enable/disable rules
- Hook support

### Audio Tuner

```bash
# Run audio tuner
cargo run -p sigmaos-audio-tuner
```

Audio management with:
- EQ band configuration
- Profile presets
- Device management
- Brightness/contrast control

### Hard Drive

```bash
# Run hard drive control
cargo run -p sigmaos-hard-drive
```

Hardware control with:
- Display settings
- Snapshot management
- Rollback support
- Controller registration

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific component
cargo test -p sigmaos-file-writer
cargo test -p sigmaos-folder-color
cargo test -p sigmaos-renamer
cargo test -p sigmaos-domain-blocker
cargo test -p sigmaos-chatroom
cargo test -p sigmaos-automate
cargo test -p sigmaos-audio-tuner
cargo test -p sigmaos-hard-drive
```

### Test Coverage

- **sigmaos-file-writer**: 6 tests (buffer, cursor, statistics)
- **sigmaos-folder-color**: 6 tests (colors, store, validation)
- **sigmaos-renamer**: 14 tests (rename, undo, preview)
- **sigmaos-domain-blocker**: 15 tests (blocking, categories, hosts)
- **sigmaos-chatroom**: 6 tests (chat, users, messages)
- **sigmaos-automate**: 6 tests (rules, priority, execution)
- **sigmaos-audio-tuner**: 6 tests (profiles, EQ, devices)
- **sigmaos-hard-drive**: 7 tests (display, snapshots, rollback)

Total: 67 tests, all passing

## Code Style

Following **KISS**, **DRY**, and **SOLID** principles:
- Each utility has clear single responsibility
- Common patterns extracted where appropriate
- Minimal dependencies between components
- Extensive error handling with `anyhow` and `thiserror`

## Configuration

Each component has its own configuration:
- File writer: Buffer size, cursor bounds
- Folder color: Color presets, path mappings
- Renamer: Pattern rules, undo depth
- Domain blocker: Block lists, categories
- Chatroom: Port, user limits
- Automate: Rule priority, hooks
- Audio tuner: EQ bands, profiles
- Hard drive: Display settings, snapshot path

## Integration with SigmaOS

The SigmaOS-utils repository is registered in the main SigmaOS component registry (`COMPONENTS-FOCUSED.toml`):

```toml
[[component]]
name = "SigmaOS-utils"
description = "Unified utilities: file tools, network tools, automation, audio, hardware control"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-utils"
engine = "utils_collection"
category = "Utilities"
language = "rust"
inspirations = "SigmaOS"
visibility = "public"
depends_on = []
```

## Contributing

Following **TALK IS CHEAP, SHOW ME THE CODE** and **TEST CODE USING CI/CD**:

1. All changes must build: `cargo build --offline`
2. All tests must pass: `cargo test --offline`
3. Follow existing code patterns
4. Update COMPONENT.toml if adding new utilities
5. Use semantic versioning for releases

## License

MIT OR Apache-2.0

## Related Components

- [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) - Main SigmaOS repository
- [SigmaOS-themes](https://github.com/AaryanSinghChauhan09/SigmaOS-themes) - Unified theming system

## Changelog

### Version 1.0.0 (2026-11-19)

- Initial consolidation of 8 utility repositories
- All 67 tests passing
- External inspirations removed (normalized to SigmaOS)
- Workspace structure with 8 subcomponents

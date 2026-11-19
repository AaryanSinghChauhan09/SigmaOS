# SigmaOS-Dev

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS-dev

## Overview

SigmaOS-Dev is a unified development tools collection for SigmaOS, providing toolchain helpers, shell configurations, agent CLI, quick shell runner, and workspace management in a monorepo structure. Following **CONSOLIDATE GITHUB REPOS** and **MODULARITY** principles, it consolidates 5 separate development tool repositories into one cohesive workspace.

## Components

The repository contains 5 subcomponents, each with its own Cargo crate:

- **sigmaos-toolchain** - Developer toolchain helpers
- **sigmaos-fish** - Fish config & plugins
- **sigmaos-agent-cli** - AI agent CLI interface
- **sigmaos-quickshell** - Quick shell command runner
- **sigmaos-workspace** - Workspace management

## Architecture

### Workspace Structure

```
SigmaOS-dev/
├── toolchain/      # sigmaos-toolchain
├── fish/            # sigmaos-fish
├── agent-cli/       # sigmaos-agent-cli
├── quickshell/      # sigmaos-quickshell
├── workspace/       # sigmaos-workspace
├── Cargo.toml       # Workspace configuration
└── README.md
```

### Component Dependencies

All dev tools are independent with no cross-dependencies:
- Each component can be built and used standalone
- No shared dependencies between tools
- Clean separation of concerns

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS-dev.git
cd SigmaOS-dev

# Build all components
cargo build --release

# Build specific component
cargo build -p sigmaos-toolchain
cargo build -p sigmaos-fish
cargo build -p sigmaos-agent-cli
cargo build -p sigmaos-quickshell
cargo build -p sigmaos-workspace
```

### Offline Build

```bash
cargo build --offline
```

## Usage

### Toolchain

```bash
# Run toolchain helpers
cargo run -p sigmaos-toolchain
```

Developer toolchain with:
- Build system configuration (CMake, Meson, etc.)
- Project template scaffolding
- Tool requirement checking
- Development environment management

### Fish

```bash
# Run fish config manager
cargo run -p sigmaos-fish
```

Fish shell configuration with:
- Plugin installation and management
- Theme customization
- Path management
- Abbreviation expansion

### Agent CLI

```bash
# Run agent CLI
cargo run -p sigmaos-agent-cli
```

AI agent CLI interface with:
- Configuration management
- Initialization handling
- Runtime control

### Quickshell

```bash
# Run quick shell
cargo run -p sigmaos-quickshell
```

Quick shell command runner with:
- Configurable key bindings
- Monitor setup
- Shell command execution
- Omarchy-style config rendering

### Workspace

```bash
# Run workspace manager
cargo run -p sigmaos-workspace
```

Workspace management with:
- Workspace creation and naming
- Path management
- Development environment setup

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for specific component
cargo test -p sigmaos-toolchain
cargo test -p sigmaos-fish
cargo test -p sigmaos-agent-cli
cargo test -p sigmaos-quickshell
cargo test -p sigmaos-workspace
```

### Test Coverage

- **sigmaos-toolchain**: 6 tests (build systems, environment, templates)
- **sigmaos-fish**: 15 tests (plugins, themes, paths, abbreviations)
- **sigmaos-agent-cli**: 3 tests (configuration, initialization)
- **sigmaos-quickshell**: 5 tests (bindings, monitors, execution)
- **sigmaos-workspace**: 1 test (workspace creation)

Total: 30 tests, all passing

## Code Style

Following **KISS**, **DRY**, and **SOLID** principles:
- Each dev tool has clear single responsibility
- Common patterns extracted where appropriate
- Minimal dependencies between components
- Extensive error handling with `anyhow` and `thiserror`

## Configuration

Each component has its own configuration:
- Toolchain: Build system mappings, tool requirements
- Fish: Plugin paths, theme settings, abbreviation rules
- Agent CLI: Configuration file, initialization settings
- Quickshell: Key bindings, monitor configuration
- Workspace: Workspace paths, naming conventions

## Integration with SigmaOS

The SigmaOS-dev repository is registered in the main SigmaOS component registry (`COMPONENTS-FOCUSED.toml`):

```toml
[[component]]
name = "SigmaOS-dev"
description = "Unified development tools: toolchain, shell configs, agent CLI, workspace management"
repo = "https://github.com/AaryanSinghChauhan09/SigmaOS-dev"
engine = "dev_tools_collection"
category = "Developer Tools"
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
4. Update COMPONENT.toml if adding new dev tools
5. Use semantic versioning for releases

## License

MIT OR Apache-2.0

## Related Components

- [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) - Main SigmaOS repository
- [SigmaOS-themes](https://github.com/AaryanSinghChauhan09/SigmaOS-themes) - Unified theming system
- [SigmaOS-utils](https://github.com/AaryanSinghChauhan09/SigmaOS-utils) - Unified utilities

## Changelog

### Version 1.0.0 (2026-11-19)

- Initial consolidation of 5 dev tool repositories
- All 30 tests passing
- External inspirations removed (normalized to SigmaOS)
- Workspace structure with 5 subcomponents

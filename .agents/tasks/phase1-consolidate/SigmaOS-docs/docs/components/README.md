# Component Registry

Central registry of SigmaOS components with their responsibilities, interfaces, and interdependencies.

## Overview

The component registry provides:
- **COMPONENTS.md**: Auto-generated component matrix with status, owner, and dependencies
- **component-registry-snapshot.toml**: Component metadata snapshot (synced from main repo)

## Component Categories

- **Core**: Fundamental building blocks (Scheduler, VFS, LSM)
- **Network**: Networking subsystem components
- **Desktop**: Desktop environment and UI
- **Package Manager**: Package management and distribution
- **Utilities**: Helper libraries and tools

## Matrix Format

The COMPONENTS.md matrix is auto-generated from the main repository's COMPONENTS.toml on each commit.

See [COMPONENTS.md](./COMPONENTS.md) for the current component matrix.

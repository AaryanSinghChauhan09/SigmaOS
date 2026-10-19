# SigPkg Universal Package Manager Components

## Overview
Universal package manager providing multi-format foreign package translation (APT, Pacman, RPM, Flatpak), SAT dependency resolution, and atomic rolling releases.

## Features
- Universal translation adapters for `.deb`, `.pkg.tar.zst`, and `.rpm`
- Fast SAT solver for complete conflict and dependency resolution
- Content-addressed CAS store preventing package file duplication
- Cryptographic signature verification (Ed25519) with atomic A/B updates

## Structures
- **UniversalPackage** - Package metadata and dependencies
- **SatSolver** - SAT solver for conflict resolution
- **PackageRepository** - Package database management
- **PackageManager** - Package management operations

## Status
- **Implementation**: Planned
- **Dependencies**: Repository infrastructure, cryptographic verification
- **Roadmap Phase**: Package manager (P4)

## Related Files
- `src/sigpkg/models.rs` - Package metadata
- `src/sigpkg/repository.rs` - Repository management
- `wiki/SigPkg-Universal-Package-Manager-Components.md` - Wiki documentation

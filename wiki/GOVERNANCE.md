# SigmaOS Multi-Repository Governance

## Overview
SigmaOS is organized as a multi-repository architecture inspired by Linux Mint and Omarchy Linux. This document defines the governance structure linking all repositories.

## Repository Catalog

### Core Foundation (Tier 0) - No Dependencies
- **SigmaOS-kernel**: https://github.com/AaryanSinghChauhan09/SigmaOS-kernel
  - CPU architecture support, memory management, scheduler, syscall dispatch, signal handling
  - Foundation for all other repositories

- **SigmaOS-klib**: https://github.com/AaryanSinghChauhan09/SigmaOS-klib
  - Kernel library with common utilities and abstractions

- **SigmaOS-bootloader**: https://github.com/AaryanSinghChauhan09/SigmaOS-bootloader
  - UEFI 2.10 ESP bootloader with PCIe NVMe 1.4 storage driver

### Core Services (Tier 1) - Depend on Tier 0
- **SigmaOS-iso**: https://github.com/AaryanSinghChauhan09/SigmaOS-ISO
  - ISO image builder with multi-architecture support
  - Status: ✅ Extracted

- **SigmaOS-package**: https://github.com/AaryanSinghChauhan09/SigmaOS-package
  - Universal package manager (APT, pacman, Flatpak, Snap)
  - Status: ✅ Extracted

- **SigmaOS-drivers**: https://github.com/AaryanSinghChauhan09/SigmaOS-drivers
  - Driver management with hardware detection
  - Status: ✅ Extracted

- **SigmaOS-security**: https://github.com/AaryanSinghChauhan09/SigmaOS-security
  - Security framework (SELinux, AppArmor, Landlock, seccomp, pledge, Capsicum)
  - Status: ✅ Extracted

- **SigmaOS-network**: https://github.com/AaryanSinghChauhan09/SigmaOS-network
  - Network interface management
  - Status: ✅ Extracted

- **SigmaOS-filesystem**: https://github.com/AaryanSinghChauhan09/SigmaOS-filesystem
  - VFS abstraction layer with Ext4, FAT32, NTFS drivers
  - Status: ✅ Extracted

### Desktop Platform (Tier 2) - Depend on Tier 0-1
- **SigmaOS-desktop-environment**: https://github.com/AaryanSinghChauhan09/SigmaOS-desktop-environment
  - Main desktop environment (Zenith/Cinnamon-based)
  - Status: ✅ Extracted

- **SigmaOS-window-management-library**: https://github.com/AaryanSinghChauhan09/SigmaOS-window-management-library
  - Window management library with tiling/floating support

- **SigmaOS-theme-desktop-environment**: https://github.com/AaryanSinghChauhan09/SigmaOS-theme-desktop-environment
  - Desktop themes with Mint-Y color palette

### User Applications (Tier 3) - Depend on Tier 0-2
- **SigmaOS-software-manager**: https://github.com/AaryanSinghChauhan09/SigmaOS-software-manager
  - GUI software manager with plugin backend architecture
  - Status: ✅ Extracted

- **SigmaOS-file-manager-system**: https://github.com/AaryanSinghChauhan09/SigmaOS-file-manager-system
  - File manager with dual-pane, preview, batch operations

- **SigmaOS-terminal**: https://github.com/AaryanSinghChauhan09/SigmaOS-terminal
  - Terminal emulator with modern features, tabs, splits

- **SigmaOS-screen-recorder**: https://github.com/AaryanSinghChauhan09/SigmaOS-screen-recorder
  - Screen recording with multiple formats
  - Status: ✅ Extracted

- **SigmaOS-screen-shot**: https://github.com/AaryanSinghChauhan09/SigmaOS-screen-shot
  - Screenshot tool with multiple modes
  - Status: ✅ Extracted

- **SigmaOS-player**: https://github.com/AaryanSinghChauhan09/SigmaOS-player
  - Media player with multiple codec support

- **SigmaOS-photo-viewer**: https://github.com/AaryanSinghChauhan09/SigmaOS-photo-viewer
  - Image viewer with basic editing

- **SigmaOS-draw**: https://github.com/AaryanSinghChauhan09/SigmaOS-draw
  - Drawing application

### System Utilities (Tier 4) - Depend on Tier 0-2
- **SigmaOS-update**: https://github.com/AaryanSinghChauhan09/SigmaOS-update
  - System update manager with kernel selection

- **SigmaOS-backup-tool**: https://github.com/AaryanSinghChauhan09/SigmaOS-backup-tool
  - System backup and restore with snapshot support

- **SigmaOS-stick**: https://github.com/AaryanSinghChauhan09/SigmaOS-stick
  - USB image writer and formatter

- **SigmaOS-timeshift**: https://github.com/AaryanSinghChauhan09/SigmaOS-timeshift
  - System snapshot and restore tool

### Additional Repositories
See `wiki/REPOSITORY_MIGRATION_MATRIX.md` for complete catalog of 195 repositories.

## Dependency Graph

```
SigmaOS-kernel (foundation)
├── SigmaOS-iso ✅
├── SigmaOS-package
├── SigmaOS-drivers
├── SigmaOS-security
├── SigmaOS-network
└── SigmaOS-klib

SigmaOS-klib
├── SigmaOS-desktop-environment
├── SigmaOS-terminal
└── SigmaOS-filesystem

SigmaOS-desktop-environment
├── SigmaOS-window-management-library
├── SigmaOS-theme-desktop-environment
├── SigmaOS-file-manager-system
├── SigmaOS-software-manager ✅
├── SigmaOS-screen-recorder ✅
└── SigmaOS-screen-shot ✅

SigmaOS-package
├── SigmaOS-software-manager ✅
└── SigmaOS-update
```

## Release Coordination

### Version Bumping Strategy
- Tier 0 repos: Monthly LTS releases
- Tier 1 repos: Bi-weekly releases
- Tier 2 repos: Weekly releases
- Tier 3+ repos: Weekly or as needed

### Breaking Change Process
1. Deprecate old API (with warning) for 1 release cycle
2. Introduce new API alongside old API
3. Remove old API in next major version
4. Update dependent repositories
5. Coordinate release across all affected repos

## Principles Applied

### MODULARITY
- Each repository has single, well-defined responsibility
- Clear boundaries between components

### LOOSE COUPLING
- Minimal dependencies between repositories
- Communication via well-defined trait interfaces

### INTERFACE SEGREGATION
- Minimal, focused public APIs
- No repository depends on internal implementation details

### DEPENDENCY INVERSION
- High-level repositories depend on abstractions (traits)
- Kernel provides trait definitions that userland implements

### DETERMINISTIC TESTING
- Each repository has standalone tests
- Cross-repo integration tests in main SigmaOS repo

### ZERO-COPY
- Design APIs to avoid unnecessary data copying
- Use shared memory where appropriate

### FAIL-FAST
- Validate dependencies at build time
- Detect version conflicts early

## CI/CD Strategy

### Per-Repository CI
- Each repository has its own CI pipeline
- Runs unit tests, linting, formatting checks

### Cross-Repository Integration Tests
- Main SigmaOS repository runs integration tests
- Tests actual usage of repository combinations

### Release Pipeline
- Automated release for each repository
- Coordinated releases for dependent repositories

## Governance

### Repository Ownership
- Each repository has designated maintainers
- Maintainers approve PRs to their repository
- Cross-repo changes require approval from all affected maintainers

### API Review Process
1. Proposed API change documented in RFC
2. Community review (2 weeks minimum)
3. SIG (Special Interest Group) approval
4. Implementation
5. Testing and validation
6. Release

### Backward Compatibility Policy
- Public APIs cannot break without major version bump
- Internal APIs may break at any time
- Document deprecated APIs with removal timeline

## Documentation

### Wiki Repository
- Centralized documentation in SigmaOS.wiki
- Architecture documentation, development plans, user guides
- API documentation

### Per-Repository Documentation
- README.md with overview and usage
- API documentation in code comments
- Contributing guidelines

## Success Metrics

### Code Quality
- Each repository has > 80% test coverage
- Zero high-severity security vulnerabilities
- Code complexity metrics within thresholds

### Build Performance
- Full build of all repositories < 30 minutes
- Incremental build < 2 minutes
- CI/CD pipeline < 10 minutes per PR

### Developer Experience
- Clear documentation for all public APIs
- Easy to add new repositories
- Minimal boilerplate for new components

## Migration Status

### Completed ✅
- Phase 1: Repository migration matrix
- Phase 2: Component boundaries and dependency contracts
- Phase 3: GitHub organization structure (144/195 repos initialized)
- Phase 4: SigmaOS-kernel extraction
- Phase 5: Desktop environment extraction
- Phase 6: Screen subsystem extraction (recorder, screenshot)
- Phase 7: Software manager extraction
- Phase 8: ISO builder extraction
- Phase 9: Governance document (this file)
- Phase 10: Wiki consolidation
- Phase 11: Documentation cleanup in main repo
- Phase 12: Cross-repo CI/CD setup

### Pending ⏳
- Remaining 180+ repository extractions (Tier 1-6 repos)

## References

- Linux Mint: https://github.com/linuxmint
- Omarchy: https://github.com/omacom
- Repository Migration Matrix: wiki/REPOSITORY_MIGRATION_MATRIX.md
- Component Boundaries: wiki/COMPONENT_BOUNDARIES_AND_DEPENDENCY_CONTRACTS.md

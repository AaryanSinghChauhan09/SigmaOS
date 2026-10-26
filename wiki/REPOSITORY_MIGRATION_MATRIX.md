# SigmaOS Multi-Repository Migration Matrix

## Overview
This document defines the transformation of SigmaOS from a monorepo to a multi-repository architecture inspired by Linux Mint and Omarchy Linux. Following the principles of **MODULARITY**, **MINIMAL COUPLING**, **MAXIMISE COHESION**, and **SEPARATION OF CONCERNS**.

## Phase 1: Core Infrastructure Repositories (Priority 1)

### 1. SigmaOS-kernel
- **Source**: `src/kernel/`, `src/arch/`, `src/boot/`, `src/memory/`, `src/scheduler/`, `src/signal/`, `src/syscall/`, `src/interrupt/`, `src/thread/`, `src/process/`
- **Dependencies**: None (kernel is the foundation)
- **Description**: Core kernel including CPU architecture support, memory management, scheduler, syscall dispatch, signal handling
- **Inspiration**: https://github.com/omacom/linux-aurora
- **Status**: 🔄 Planned (Phase 4)

### 2. SigmaOS-iso
- **Source**: `src/iso/`, `src/installer/`, `src/iso_root/`
- **Dependencies**: SigmaOS-kernel
- **Description**: ISO image builder with multi-architecture support, bootloader integration, live session creation
- **Inspiration**: https://github.com/omacom/omarchy-iso
- **Status**: 🔄 Planned (Phase 8)

### 3. SigmaOS-package
- **Source**: `src/package/`, `src/sigpkg/`, `src/pacman/`
- **Dependencies**: SigmaOS-kernel (for kernel package format)
- **Description**: Universal package manager supporting APT, pacman, Flatpak, Snap backends
- **Inspiration**: https://github.com/linuxmint/aptkit, https://github.com/linuxmint/packagekit
- **Status**: 🔄 Planned (Phase 9)

## Phase 2: Desktop Environment Repositories (Priority 2)

### 4. SigmaOS-desktop-environment
- **Source**: `src/desktop/`, `src/compositor/`, `src/panel/`, `src/dock/`, `src/shell/`, `src/cinnamon/`
- **Dependencies**: SigmaOS-kernel, SigmaOS-klib
- **Description**: Main desktop environment (Zenith/Cinnamon-based) with window management, panels, docks
- **Inspiration**: https://github.com/linuxmint/cinnamon
- **Status**: 🔄 Planned (Phase 5)

### 5. SigmaOS-window-management-library
- **Source**: `src/compositor/` (window manager specific code)
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Window management library with tiling/floating support, Hyprland integration
- **Inspiration**: https://github.com/linuxmint/muffin, https://github.com/omacom/omarchy-hyprland-titlebars
- **Status**: 🔄 Planned (Phase 5)

### 6. SigmaOS-theme-desktop-environment
- **Source**: `src/theming/`, `src/desktop/theme.rs`
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Desktop themes, Mint-Y color palette, dark/light mode support
- **Inspiration**: https://github.com/linuxmint/cinnamon-themes
- **Status**: 🔄 Planned (Phase 5)

## Phase 3: User Applications (Priority 3)

### 7. SigmaOS-software-manager
- **Source**: `src/software_manager/`, `src/mintinstall/`
- **Dependencies**: SigmaOS-package
- **Description**: GUI software manager with plugin backend architecture
- **Inspiration**: https://github.com/linuxmint/mintinstall
- **Status**: ✅ Implemented (unit-tested), needs extraction

### 8. SigmaOS-file-manager-system
- **Source**: `src/nemo/`, `src/file-manager/`
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: File manager with dual-pane, preview, batch operations
- **Inspiration**: https://github.com/linuxmint/nemo
- **Status**: 🔄 Planned

### 9. SigmaOS-terminal
- **Source**: `src/shell/`, `src/userland/terminal_emulator.rs`
- **Dependencies**: SigmaOS-kernel (pty support)
- **Description**: Terminal emulator with modern features, tabs, splits
- **Inspiration**: https://github.com/linuxmint/ptyxis-minted, https://github.com/omacom/omaterm
- **Status**: 🔄 Planned

### 10. SigmaOS-screen-recorder
- **Source**: `src/screen/recorder.rs`
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Screen recording with multiple formats, state machine
- **Inspiration**: OBS Studio, SimpleScreenRecorder
- **Status**: ✅ Implemented (unit-tested), needs extraction

### 11. SigmaOS-screen-shot
- **Source**: `src/screen/screenshot.rs`
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Screenshot tool with multiple modes (fullscreen, window, region, timed)
- **Inspiration**: https://github.com/omacom/omasnap, GNOME Screenshot, Flameshot
- **Status**: ✅ Implemented (unit-tested), needs extraction

## Phase 4: System Utilities (Priority 4)

### 12. SigmaOS-update
- **Source**: `src/update/`, `src/mintupdate/`
- **Dependencies**: SigmaOS-package
- **Description**: System update manager with kernel selection, security updates
- **Inspiration**: https://github.com/linuxmint/mintupdate
- **Status**: 🔄 Planned

### 13. SigmaOS-backup-tool
- **Source**: `src/backup/`, `src/timeshift/`
- **Dependencies**: SigmaOS-filesystem
- **Description**: System backup and restore with snapshot support
- **Inspiration**: https://github.com/linuxmint/mintbackup, https://github.com/linuxmint/timeshift
- **Status**: 🔄 Planned

### 14. SigmaOS-drivers
- **Source**: `src/drivers/`, `src/driver/`, `src/driver_mgr/`
- **Dependencies**: SigmaOS-kernel
- **Description**: Driver management with hardware detection, proprietary driver support
- **Inspiration**: https://github.com/linuxmint/mintdrivers
- **Status**: 🔄 Planned

### 15. SigmaOS-stick
- **Source**: `src/mintstick/`, `src/usb_formatter/`
- **Dependencies**: SigmaOS-filesystem
- **Description**: USB image writer and formatter
- **Inspiration**: https://github.com/linuxmint/mintstick, https://github.com/balena-io/etcher
- **Status**: 🔄 Planned

## Phase 5: Graphics & Multimedia (Priority 5)

### 16. SigmaOS-player
- **Source**: `src/media/`, `src/video/`
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Media player with multiple codec support
- **Inspiration**: https://github.com/linuxmint/xplayer
- **Status**: 🔄 Planned

### 17. SigmaOS-photo-viewer
- **Source**: `src/media/` (photo specific)
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Image viewer with basic editing
- **Inspiration**: https://github.com/linuxmint/pix
- **Status**: 🔄 Planned

### 18. SigmaOS-draw
- **Source**: `src/creative/` (drawing tools)
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Drawing application
- **Inspiration**: https://github.com/linuxmint/drawing
- **Status**: 🔄 Planned

## Phase 6: Security & Access Control (Priority 6)

### 19. SigmaOS-security
- **Source**: `src/security/`, `src/secure/`, `src/sigma_sandbox/`
- **Dependencies**: SigmaOS-kernel
- **Description**: Security framework (SELinux, AppArmor, Landlock, seccomp, pledge, Capsicum)
- **Inspiration**: https://github.com/omacom/omarchy/security
- **Status**: 🔄 Planned

### 20. SigmaOS-accessibility
- **Source**: `src/accessibility/`
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Accessibility tools (screen reader, magnifier, high contrast)
- **Inspiration**: https://github.com/GNOME/accerciser, https://github.com/linuxmint/orca
- **Status**: ✅ Repository exists, needs population

## Phase 7: Office Suite (Priority 7)

### 21. SigmaOS-office
- **Source**: Office-related components (offline Chromium-based)
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Office suite (Writer, Calc, Impress alternatives)
- **Inspiration**: https://github.com/linuxmint/libreoffice-style-mint
- **Status**: ✅ Repository exists, needs population

## Phase 8: Networking & Connectivity (Priority 8)

### 22. SigmaOS-network-discovery
- **Source**: `src/network/`, `src/net/`
- **Dependencies**: SigmaOS-kernel
- **Description**: Network discovery and management
- **Inspiration**: https://github.com/linuxmint/network-discovery
- **Status**: 🔄 Planned

### 23. SigmaOS-bluetooth
- **Source**: `src/bluetooth/`
- **Dependencies**: SigmaOS-kernel
- **Description**: Bluetooth management GUI
- **Inspiration**: https://github.com/linuxmint/cinnamon-bluetooth
- **Status**: 🔄 Planned

### 24. SigmaOS-LAN-fileshare
- **Source**: `src/warpinator/`
- **Dependencies**: SigmaOS-network
- **Description**: Local network file sharing
- **Inspiration**: https://github.com/linuxmint/warpinator
- **Status**: 🔄 Planned

## Phase 9: Theming & Customization (Priority 9)

### 25. SigmaOS-theme-icon
- **Source**: Icon sets
- **Dependencies**: None
- **Description**: Icon themes (Mint-X, Mint-Y, etc.)
- **Inspiration**: https://github.com/linuxmint/mint-y-icons
- **Status**: 🔄 Planned

### 26. SigmaOS-theme-cursor
- **Source**: Cursor themes
- **Dependencies**: None
- **Description**: Cursor themes
- **Inspiration**: https://github.com/linuxmint/mint-cursor-themes
- **Status**: 🔄 Planned

### 27. SigmaOS-wallpaper-engine
- **Source**: `src/wallpaper_palette/`
- **Dependencies**: SigmaOS-desktop-environment
- **Description**: Dynamic wallpaper engine
- **Inspiration**: https://github.com/omacom/owe
- **Status**: 🔄 Planned

## Phase 10: Developer Tools (Priority 10)

### 28. SigmaOS-dev-tools
- **Source**: `src/development/`, `src/compiler/`, `src/toolchain/`
- **Dependencies**: SigmaOS-kernel
- **Description**: Development tools and toolchain
- **Inspiration**: https://github.com/linuxmint/mint-dev-tools
- **Status**: 🔄 Planned

### 29. SigmaOS-term
- **Source**: `src/shell/` (terminal-specific)
- **Dependencies**: SigmaOS-kernel
- **Description**: Advanced terminal with text effects
- **Inspiration**: https://github.com/omacom/omaterm, https://github.com/omacom/ttfx
- **Status**: 🔄 Planned

## Governance Repository

### 30. SigmaOS (Main Repository)
- **Purpose**: Governance, CI/CD coordination, documentation hub, dependency management
- **Contents**:
  - Repository manifest (all sub-repositories listed)
  - CI/CD pipelines for cross-repo integration
  - Governance and contribution guidelines
  - API contracts between repositories
  - Release coordination
- **Dependencies**: None (orchestrates all others)
- **Status**: 🔄 To be refactored from current monorepo

## Documentation Repository

### 31. SigmaOS-wiki
- **Source**: `wiki/`, `docs/`, `WIKI/`
- **Purpose**: Centralized documentation hub
- **Contents**:
  - Architecture documentation
  - Development plans
  - User guides
  - API documentation
  - Component status tracking
- **Dependencies**: None
- **Status**: 🔄 Planned (Phase 10)

## Migration Principles

### 1. **BOY SCOUT RULE**
- Leave each repository cleaner than you found it
- Apply consistent code standards across all repos

### 2. **MINIMAL KERNEL PHILOSOPHY**
- Keep kernel repository focused on core OS functionality
- Move all userland to separate repositories

### 3. **COMPOSITION OVER INHERITANCE**
- Design repository interfaces as composable modules
- Avoid deep inheritance hierarchies between repos

### 4. **SEPARATION OF MECHANISM & POLICY**
- Mechanism (kernel, libraries) in core repos
- Policy (desktop configuration, themes) in app repos

### 5. **DETERMINISTIC TESTING**
- Each repository must have standalone tests
- Cross-repo integration tests in main SigmaOS repo

### 6. **INTERFACE SEGREGATION**
- Define clear contracts between repositories
- Use trait-based interfaces for loose coupling

### 7. **ZERO-COPY**
- Design APIs to avoid unnecessary data copying between repos
- Use shared memory where appropriate

### 8. **FAIL-FAST**
- Validate dependencies at build time
- Detect version conflicts early

## Dependency Graph

```
SigmaOS-kernel (foundation)
├── SigmaOS-iso
├── SigmaOS-package
├── SigmaOS-drivers
├── SigmaOS-security
├── SigmaOS-network
└── SigmaOS-klib

SigmaOS-klib (kernel library)
├── SigmaOS-desktop-environment
├── SigmaOS-terminal
└── SigmaOS-filesystem

SigmaOS-desktop-environment
├── SigmaOS-window-management-library
├── SigmaOS-theme-desktop-environment
├── SigmaOS-file-manager-system
├── SigmaOS-software-manager
├── SigmaOS-screen-recorder
├── SigmaOS-screen-shot
├── SigmaOS-player
├── SigmaOS-photo-viewer
├── SigmaOS-draw
└── SigmaOS-accessibility

SigmaOS-package
├── SigmaOS-software-manager
├── SigmaOS-update
└── SigmaOS-drivers

SigmaOS-filesystem
├── SigmaOS-backup-tool
└── SigmaOS-stick

SigmaOS (main governance repo)
└── All repositories (orchestration)
```

## Migration Timeline

### Week 1-2: Infrastructure Setup
- Create SigmaOS-wiki repository
- Set up GitHub organization structure
- Define CI/CD pipeline for cross-repo testing

### Week 3-4: Core Repositories
- Extract SigmaOS-kernel
- Extract SigmaOS-iso
- Extract SigmaOS-package

### Week 5-6: Desktop Environment
- Extract SigmaOS-desktop-environment
- Extract SigmaOS-window-management-library
- Extract SigmaOS-theme-desktop-environment

### Week 7-8: User Applications
- Extract SigmaOS-software-manager
- Extract SigmaOS-screen-recorder
- Extract SigmaOS-screen-shot
- Extract SigmaOS-file-manager-system

### Week 9-10: System Utilities
- Extract SigmaOS-update
- Extract SigmaOS-backup-tool
- Extract SigmaOS-drivers
- Extract SigmaOS-stick

### Week 11-12: Graphics & Multimedia
- Extract SigmaOS-player
- Extract SigmaOS-photo-viewer
- Extract SigmaOS-draw

### Week 13-14: Security & Networking
- Extract SigmaOS-security
- Extract SigmaOS-accessibility
- Extract SigmaOS-network-discovery
- Extract SigmaOS-bluetooth
- Extract SigmaOS-LAN-fileshare

### Week 15-16: Theming & Customization
- Extract SigmaOS-theme-icon
- Extract SigmaOS-theme-cursor
- Extract SigmaOS-wallpaper-engine

### Week 17-18: Developer Tools
- Extract SigmaOS-dev-tools
- Extract SigmaOS-term

### Week 19-20: Final Cleanup
- Remove documentation from main SigmaOS repo
- Update cross-repo dependencies
- Consolidate wiki into SigmaOS-wiki
- Finalize governance documentation

## Success Criteria

1. **Compilation**: Each repository compiles independently
2. **Testing**: Each repository has passing unit tests
3. **Integration**: Cross-repo integration tests pass
4. **Documentation**: All components documented in SigmaOS-wiki
5. **Governance**: Clear dependency contracts defined
6. **CI/CD**: Automated testing across all repositories
7. **Release**: Coordinated release process established

## Open Questions

1. **Binary Distribution**: How to distribute binary packages across repos?
2. **Version Coordination**: How to synchronize version bumps across dependent repos?
3. **Breaking Changes**: How to handle API breaking changes between repos?
4. **Mono-repo Benefits**: How to preserve mono-repo benefits (atomic commits, refactoring)?
5. **CI/CD Performance**: How to avoid excessive CI/CD time with 100+ repos?

## References

- Linux Mint Repository Structure: https://github.com/linuxmint
- Omarchy Repository Structure: https://github.com/omacom
- GNU/Linux Distros Repository Patterns: Debian, Fedora, Arch
- Microsoft Repository Strategy: https://github.com/microsoft
- Google Repository Strategy: https://github.com/google

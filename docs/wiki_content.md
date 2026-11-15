# SigmaOS GitHub Wiki Content

## Home

SigmaOS is a sovereign operating system built on Arch Linux and Omarchy Linux, following the multi-repository architecture pattern.

**Version**: 1.0.0-alpha.1
**Status**: In Development
**License**: MIT OR GPL-2.0

### Quick Start

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git && cd SigmaOS && ./run.sh
```

### Documentation

- [Getting Started](#getting-started)
- [Architecture](#architecture)
- [Components](#components)
- [Development](#development)
- [Principles](#principles)
- [Roadmap](#roadmap)

## Getting Started

### Prerequisites

- Rust toolchain (stable)
- Python 3.x
- GCC (for kernel compilation)
- Docker (optional)
- QEMU (optional, for testing)

### Installation

```bash
# Clone repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install dependencies
sudo apt install libseccomp-dev mysql-client parallel nasm xorriso

# Build and run
./run.sh
```

### First Run

```bash
./run.sh --check  # Verify system readiness
./run.sh          # Launch interactive workstation
./run.sh --omarchy # Launch Omarchy persona
./run.sh --arch   # Launch Arch persona
```

## Architecture

### Overview

SigmaOS follows a modular microkernel architecture with component-based design:

```
SigmaOS
├── Kernel (Microkernel)
├── Filesystem (VFS)
├── Network (TCP/IP)
├── Security (LSM)
├── System (Init/Services)
├── Desktop (Wayland)
├── Packages (Package Management)
└── Craft Suite (20 modules)
```

### Core Components

- **Kernel**: Microkernel with scheduler, memory management, IPC
- **Filesystem**: VFS layer with FAT32/EXT4/BTRFS support
- **Network**: TCP/IP stack with zero-copy optimization
- **Security**: LSM hooks, capability isolation
- **System**: Init, service manager, system orchestration
- **Desktop**: Wayland compositor, window management
- **Packages**: Package catalog, dependency resolution

### Multi-Persona Support

- **Arch Linux**: Rolling release, pacman, AUR
- **Omarchy**: Hyprland, Waybar, aesthetic theming
- **Linux Mint**: Cinnamon, XApps, ergonomic design
- **CachyOS**: BORE scheduler, PGO profiles
- **Debian**: Multiarch, apt compatibility
- **BSD Variants**: FreeBSD, OpenBSD, DragonFlyBSD

## Components

### Component Registry

SigmaOS uses a multi-repository architecture with 2,495 component declarations.

**Categories**:
- Core System (kernel, filesystem, network, security, system)
- Desktop Environment (compositor, window manager, workspace)
- Package Management (package catalog, package manager)
- Craft Suite (20 creative modules)
- Tools (utilities, applications)

### New Components Added

- **SigmaOS-aur-helper**: AUR helper for Arch Linux package management
- **SigmaOS-hyprland-config**: Hyprland Wayland compositor configuration

### Missing Components (Planned)

- SigmaOS-wallust-themes: Wallpaper-driven theming
- SigmaOS-quickshell: Aesthetic bar
- SigmaOS-waybar-themes: Waybar themes
- SigmaOS-dpkg-compat: Debian compatibility
- SigmaOS-portage: Gentoo package manager
- And 7 more...

See [Component Registry](#component-registry) for full list.

## Development

### Getting Started

```bash
# Clone repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install dependencies
sudo apt install libseccomp-dev mysql-client parallel nasm xorriso

# Build
cargo build --release

# Test
cargo test
```

### Build Commands

```bash
make check          # Cargo check
make test-fast      # Unit tests (memory-safe)
make fmt            # Verify formatting
make iso-stub       # Build ISO
make run-stub       # Boot in QEMU
make registry-audit # Component audit
```

### Run Commands

```bash
./run.sh            # Interactive REPL
./run.sh --omarchy  # Omarchy persona
./run.sh --arch     # Arch persona
./run.sh --mint     # Mint persona
./run.sh --check    # Launch readiness
./run.sh --doctor   # Diagnostic checks
```

### CI/CD

- **Status**: Active
- **Jobs**: 6 automated jobs (check, fmt, clippy, audit, kernel, ISO)
- **Trigger**: Push to main/master/develop, PRs
- **Caching**: Cargo registry, index, build artifacts

### Testing

```bash
# Run tests
cargo test

# Memory-safe testing
cargo test --lib kernel -- --test-threads=4
cargo test --lib filesystem -- --test-threads=4
cargo test --lib network -- --test-threads=4
cargo test --lib security -- --test-threads=4
```

## Principles

SigmaOS follows 100+ software engineering principles:

### Core Principles
- **KISS**: Keep It Simple, Stupid
- **DRY**: Don't Repeat Yourself
- **SOLID**: Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion
- **YAGNI**: You Aren't Gonna Need It
- **Gall's Law**: Start simple, work up to complex systems
- **Zero-Copy**: Avoid unnecessary memory copies
- **Fail-Fast**: Fail early and explicitly
- **Defensive Programming**: Assume errors will happen

### Architectural Principles
- **Modular Kernel Design**: Component-based architecture
- **Separation of Mechanism & Policy**: Kernel/userspace split
- **Single Responsibility**: One repo per component
- **Composition Over Inheritance**: Rust trait-based design
- **Mechanical Sympathy**: Performance optimization

### Process Principles
- **Upstream First**: Contribute to upstream projects
- **Spec-Driven Development**: Formal specifications
- **Test-Driven Development**: Tests before code
- **Continuous Integration**: Automated testing
- **Semantic Versioning**: MAJOR.MINOR.PATCH

### Security Principles
- **Least Privilege**: Minimal permissions
- **Secure Defaults**: Safe by default
- **Fail-Fast**: Fail securely
- **Memory Safety**: Rust ownership system

See [Principles](#principles) for full list.

## Roadmap

### Phase 1: Foundation (Current)
- [x] Fix hardcoded paths
- [x] Set up CI/CD pipeline
- [x] Implement semantic versioning
- [x] Create component creation infrastructure
- [x] Fix test suite OOM issue
- [ ] Migrate documentation to Wiki
- [ ] Create missing component repositories

### Phase 2: Quality
- [ ] Code quality improvements
- [ ] Spec-driven development
- [ ] Comprehensive test coverage
- [ ] Linting & formatting enforcement

### Phase 3: Expansion
- [ ] Create all missing components
- [ ] Cross-platform support
- [ ] Web integration
- [ ] Component consolidation

### Phase 4: Advanced
- [ ] Instrumentation
- [ ] Rolling release model
- [ ] Upstream-first policy
- [ ] Community building

## Component Registry

### Core Components

- **SigmaOS-kernel**: Microkernel core
- **SigmaOS-filesystem**: Virtual filesystem layer
- **SigmaOS-network**: Networking stack
- **SigmaOS-security**: Security module
- **SigmaOS-system**: Init and service manager

### Desktop Components

- **SigmaOS-desktop**: Wayland compositor
- **SigmaOS-window-manager**: Window management
- **SigmaOS-workspace**: Workspace management
- **SigmaOS-hyprland-config**: Hyprland configuration (NEW)

### Package Management

- **SigmaOS-packages**: Package catalog
- **SigmaOS-aur-helper**: AUR helper (NEW)
- **SigPkg**: Package manager interface

### Craft Suite

- **3DCraft**: 3D modeling and rendering
- **AniCraft**: 2D animation
- **ArtCraft**: Digital painting
- **CADCraft**: Parametric CAD
- **FilmCraft**: Video editing
- **SoundCraft**: Audio production
- **DesignCraft**: UI/UX design
- **VectorCraft**: SVG illustration
- **WordCraft**: Document processing
- **PDFCraft**: PDF engine
- **PhotoCraft**: Image processing
- **PrintCraft**: Desktop publishing
- **FontCraft**: Typography
- **GridCraft**: Spreadsheets
- **LightCraft**: Lighting simulation
- **VoiceCraft**: Speech recognition
- **FigCraft**: Technical diagrams
- **EffectCraft**: VFX compositing
- **DeckCraft**: Presentations
- **CraftLibs**: Creative core

### Tools

- **SigmaOS-tools-***: Various utility tools
- **SigmaOS-zoxide**: Directory jumper
- **SigmaOS-zellij**: Terminal multiplexer
- **SigmaOS-yazi**: File manager

## Personas

### Arch Linux Persona
- Rolling release model
- Pacman package manager
- AUR integration
- Minimalist design

### Omarchy Persona
- Hyprland Wayland compositor
- Wallust theming
- Waybar/Quickshell bar
- Aesthetic design
- Keyboard-driven

### Linux Mint Persona
- Cinnamon desktop
- XApps integration
- Ergonomic design
- Driver autodetect

### CachyOS Persona
- BORE scheduler
- PGO profiles
- zram-zstd
- Performance focused

### Debian Persona
- APT compatibility
- Multiarch support
- Stability focus
- Policy compliance

### BSD Personas
- **FreeBSD**: Capsicum, Jails, ZFS
- **OpenBSD**: pledge/unveil, PF firewall
- **DragonFlyBSD**: HAMMER2 filesystem

## Reference

### Comparison with Reference Distros

| Feature | Arch | Omarchy | Debian | Gentoo | SigmaOS |
|---------|------|---------|--------|--------|---------|
| Package Manager | pacman | pacman | apt | portage | sigpkg + alpm |
| AUR Support | yay/paru | - | - | - | aur-helper |
| Wayland | Hyprland | Hyprland | GNOME | GNOME | Hyprland |
| Theming | Manual | Wallust | Manual | Manual | Wallust (planned) |
| Kernel | Linux | Linux | Linux | Linux | Custom Rust |

### Build Profiles

- **dev**: Fast iteration, minimal optimization
- **test**: Test configuration, panic=abort
- **release**: Maximum optimization
- **release-fast**: Thin LTO for fast builds

### Dependencies

- Rust toolchain (stable)
- Python 3.x
- GCC
- libseccomp-dev
- Docker (optional)
- QEMU (optional)

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

### Component Creation

```bash
./scripts/create_component_repo.sh \
  SigmaOS-component-name \
  "Component description" \
  "Category" \
  "rust"
```

## License

MIT OR GPL-2.0 — Free and Open Source Software.
© SigmaOS Contributors

## Links

- **GitHub**: https://github.com/AaryanSinghChauhan09/SigmaOS
- **Issues**: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- **Releases**: https://github.com/AaryanSinghChauhan09/SigmaOS/releases
- **Component Registry**: COMPONENTS.toml

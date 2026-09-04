# Contributing to SigmaOS

Thank you for your interest in contributing to SigmaOS! This document provides guidelines and instructions for contributing to the core operating system, hardware drivers, application compatibility layers, and declarative app shards.

## Code of Conduct

- Be respectful and inclusive
- Focus on what is best for the community and open sovereign computing
- Show empathy towards other community members

## Special Interest Groups (SIGs)

To foster specialized collaboration, SigmaOS organizes community work into Special Interest Groups (SIGs):

- **SIG-Kernel**: Low-level kernel scheduling, virtual memory (VMM), IPC, eBPF, and syscall gates.
- **SIG-Drivers**: Hardware abstraction layers (HAL), PCIe, NVMe, e1000e NICs, xHCI USB, Intel HDA, and net80211/iwlwifi.
- **SIG-Apps & Shards**: Declarative app manifests (`.sigma-app`), immutable SquashFS/OverlayFS layers, and Shards Marketplace ecosystem.
- **SIG-Security**: OpenBSD-style `pledge`/`unveil`, SELinux MAC policies, PQC cryptographic enclaves, and binary hardening.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Git
- QEMU / KVM (for OS testing)
- Make & GCC/Clang

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the project
cargo build

# Run core library unit tests
cargo test --lib

# Run the interactive REPL shell
cargo run --bin sigma_userspace
```

## Declarative App Manifests & Shards Packaging

Developers are encouraged to package applications as declarative SigmaOS Shards using single-file `.sigma-app` specs:

```toml
# Example Declarative App Manifest
name = "my-sovereign-app"
version = "1.0.0"
entrypoint = "/bin/myapp"
description = "High-performance modular app"
allow_gpu = "true"
allow_audio = "true"
allow_network = "false"
depends = "sigma-libc"
env.APP_MODE = "production"
```

App shards run in immutable, read-only layers with atomic zero-downtime slot updates and capability permission enforcement.

## Development Workflow

### Branching Strategy

- `main` - The main development branch
- All changes should be made through pull requests
- Feature branches should be named `feature/description`
- Bugfix branches should be named `fix/description`
- Shard / package updates should be named `shard/app-name`

### Commit Guidelines

- Use clear, descriptive commit messages
- Follow conventional commit format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `shard`

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Use clippy for linting: `cargo clippy`
- Write unit tests for new functionality
- Document public APIs with rustdoc

## Testing & Verification

```bash
# Run all unit tests
cargo test --lib

# Run binary executable target checks
cargo check --bins

# Run specific driver test
cargo test --lib drivers::modern_nvme
```

## Hackathons, Community Sprints & Roadmap

- **Developer Roadmap**: Check `ROADMAP.md` and `3-YEAR-STRATEGIC-VISION.md` to align your contributions with current milestones.
- **Community Hackathons & Sprints**: We host quarterly virtual hackathons and monthly bug-hunting sprints. Announcements and sign-ups are posted in GitHub Discussions.

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes and add unit tests
4. Update documentation or manifest specs as needed
5. Submit a pull request
6. Address review feedback from SIG maintainers
7. Obtain approval and merge

## Questions & Discussions

- Join discussions in **GitHub Discussions**
- File bug reports and feature proposals via **GitHub Issues**
- Reach out to SIG leads in relevant subproject channels

Thank you for building the future of sovereign, AI-native operating systems with SigmaOS!

# Package Management (`sigpkg`)

`sigpkg` is the universal package management system for SigmaOS. It handles dependency resolution, atomic upgrades, and source-based compilation.

## Package Format (`.spkg`)
The `.spkg` format is a compressed tarball containing:
- Compiled binaries / shared libraries
- System configuration defaults
- A manifest file (JSON/TOML) defining dependencies, capabilities required, and metadata.

## Dependency Resolution
`sigpkg` uses a SAT solver-based approach (similar to DNF and Zypper) to ensure dependency graphs are correctly evaluated without conflicts.

## USE Flags Support
Inspired by Gentoo's Portage, `sigpkg` allows users to compile packages from source with specific `USE` flags to enable or disable features (e.g., `+wayland`, `-x11`, `+alsa`).

## OCI Container Integration
`sigpkg` can pull and extract standard OCI container images directly into SigmaOS namespaces, functioning both as a system package manager and a container image manager.

## Comparison to Legacy Tools
- **APT / DPKG (Debian)**: `sigpkg` is faster and supports atomic rollbacks.
- **Pacman (Arch)**: `sigpkg` borrows the simplicity and speed of pacman, while adding source-compilation capabilities.
- **Nix**: While not fully declarative like Nix, `sigpkg` supports isolated package profiles to prevent dependency hell.

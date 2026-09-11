# Package Management (sigpkg & UniversalPackageManager)

`UniversalPackageManager` (`src/package/universal.rs`) is the native, zero-dependency universal package manager for SigmaOS, providing secure dependency resolution, transactional checkpoints, and isolated capability execution across 18 major distribution formats.

## Universal Format Support
Supports 18 major package formats natively without external tools:
- **Debian / Ubuntu / Mint**: `.deb`
- **Fedora / RedHat / CentOS / Zypper**: `.rpm`
- **Arch Linux / Manjaro / CachyOS**: `.pkg.tar.zst` / `Pacman`
- **Gentoo**: `Ebuild`
- **Alpine**: `.apk`
- **NixOS**: `Nix Flakes`
- **Flatpak**: Desktop containers
- **Canonical**: `.snap`
- **AppImage**: Portable binaries
- **Void Linux**: `.xbps`
- **Slackware**: `.txz`
- **Solus**: `.eopkg`
- **openSUSE**: `.zypper`
- **GNU Guix**: `Guix`
- **CachyOS**: `CachyOS`
- **Intel Swupd**: `Swupd`
- **Post-Quantum**: `Starling`
- **Native**: `SigmaPkg` (`.sigpkg`)

## OOP Architectural Patterns
- **Strategy Pattern**: Per-format install, verify, and remove logic (`DebInstallStrategy`, `RpmInstallStrategy`, etc.).
- **Adapter Pattern**: Metadata extraction and parsing (`DebMetadataAdapter`, `PacmanMetadataAdapter`, etc.).
- **State Pattern**: Explicit lifecycle tracking (`Uninstalled` ➔ `Downloading` ➔ `Installing` ➔ `Installed`).
- **Decorator Pattern**: Dynamic sandboxing (`SandboxDecorator`) and network enforcement (`NetworkRestrictionDecorator`).
- **Factory Pattern**: Central strategy and metadata adapter creation (`PackageFactory`).
- **Observer Pattern & UDFs**: Event notification hooks and user-defined closure triggers (`PackageTriggerRegistry`).

## Transactional Checkpoints & Rollbacks
System state changes are recorded into checkpoint entries, allowing instant $O(1)$ rollbacks to prior checkpoints.

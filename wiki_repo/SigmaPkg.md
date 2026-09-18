# SigmaPkg - Universal Package Manager

SigmaPkg is SigmaOS's universal package management system, designed to work with multiple Linux and BSD package formats while providing content-addressed storage and cryptographic verification.

## Features

### Multi-Format Support
SigmaPkg supports package formats from multiple distributions:
- **Arch Linux**: `.pkg.tar.zst` with PKGBUILD files
- **Debian/Ubuntu**: `.deb` with APT repositories
- **Fedora/openSUSE**: `.rpm` with DNF repositories
- **FreeBSD**: `.pkg` with pkg repository
- **Gentoo**: `.ebuild` with Portage
- **Alpine**: `.apk` packages
- **Void Linux**: `.xbps` packages
- **Nix/Guix**: Content-addressed Nix/Guix store
- **Slackware**: `.txz` packages

### Content-Addressed Storage
All packages are stored in a content-addressed store using SHA256 hashes:
- Immutable package storage prevents accidental modification
- Deduplication reduces storage usage
- Cryptographic verification ensures package integrity
- Atomic updates guarantee system stability

### Rollback Engine
SovereignPackageRollbackEngine provides:
- Sub-1ms state restoration
- Automatic rollback on package installation failure
- Generation-based system snapshots
- Differential rollback to minimize data transfer

### Package Hooks
Automatic execution of pre/post-install hooks:
- Database schema migrations
- System service restarts
- Configuration file updates
- Desktop integration scripts

## Usage

### Installing Packages
```bash
sigpkg install firefox
sigpkg install --from-aur chromium
sigpkg install --from-debian vlc
```

### Updating Packages
```bash
sigpkg update
sigpkg upgrade firefox
sigpkg upgrade --all
```

### Removing Packages
```bash
sigpkg remove firefox
sigpkg remove --purge firefox
```

### Searching Packages
```bash
sigpkg search firefox
sigpkg search --all-repos python
```

### Package Information
```bash
sigpkg info firefox
sigpkg info --files firefox
sigpkg info --dependencies firefox
```

## Configuration

Repository configuration in `/etc/sigpkg/config.toml`:

```toml
[repositories]
arch = ["https://archlinux.org/packages/core"]
debian = ["http://deb.debian.org/debian bullseye main"]
fedora = ["https://download.fedoraproject.org/pub/fedora/linux/releases/38/Everything/x86_64/os/"]

[storage]
store_path = "/var/lib/sigpkg/store"
cache_size = "10G"

[security]
signature_verification = true
trusted_keys = ["/etc/sigpkg/keys/arch.gpg"]
```

## Architecture

SigmaPkg is built with:
- Zero external dependencies
- Pure Rust implementation
- Cross-OS compatibility (Linux/BSD modes)
- Integration with Landlock, Capsicum, and pledge/unveil

### Components
- **Package Repository Manager**: Handles repository metadata and package downloads
- **Content-Addressed Store**: Immutable storage with SHA256-based addressing
- **Rollback Engine**: Atomic package operations with rollback capability
- **Signature Verification**: Ed25519 signature verification for package integrity
- **Hook System**: Pre/post transaction hooks for system integration

## Security

### Cryptographic Verification
- Ed25519 signatures for package authenticity
- SHA256 hashes for package integrity
- GPG key management for trusted package sources
- Secure key distribution and revocation

### Sandboxing
- Package operations run in sandboxed environments
- Landlock v5 restricts file system access
- Capsicum capabilities limit system calls
- Pledge/unveil enforce process restrictions

### Rollback Safety
- Atomic transactions prevent partial installations
- Automatic rollback on failure
- Generation-based snapshots allow complete system rollback
- Differential rollback minimizes downtime

## Troubleshooting

### Package Not Found
If a package isn't found:
1. Check repository configuration: `sigpkg list-repos`
2. Update repository metadata: `sigpkg update`
3. Search in all repositories: `sigpkg search --all-repos <package>`

### Signature Verification Failed
If signature verification fails:
1. Update trusted keys: `sigpkg update-keys`
2. Check key expiration: `sigpkg list-keys`
3. Verify repository integrity: `sigpkg verify-repo`

### Installation Failure
If installation fails:
1. Check logs: `journalctl -u sigpkg -f`
2. Rollback to previous state: `sigpkg rollback`
3. Verify package integrity: `sigpkg verify <package>`

---

**[Package Management](Category-Package-Management)** | **[Repository Management](Repository-Management)** | **[Package Hooks](Package-Hooks)**

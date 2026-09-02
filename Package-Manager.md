# sigpkg — SigmaOS Package Manager

sigpkg is SigmaOS's universal, multi-format package manager combining the best of pacman, apt, dnf, nix, and portage.

---

## Quick Reference

```bash
sigpkg install nginx          # Install
sigpkg remove nginx           # Remove
sigpkg upgrade                # Upgrade all
sigpkg search "web server"    # Search
sigpkg info nginx             # Package info
sigpkg rollback               # Rollback to previous generation
sigpkg aur install brave-bin  # Install from AUR
```

---

## Supported Package Formats

| Format | Extension | Distro |
|--------|-----------|--------|
| Sigma native | `.spkg` | SigmaOS |
| Arch pacman | `.pkg.tar.zst` | Arch Linux |
| Debian | `.deb` | Debian/Ubuntu |
| RPM | `.rpm` | Fedora/RHEL |
| Alpine | `.apk` | Alpine Linux |
| Gentoo ebuild | `ebuild` | Gentoo |
| Nix expression | `.nix` | NixOS |
| FreeBSD ports | `Makefile` | FreeBSD |

---

## Content-Addressed Store

All packages install to `/sigma/store/<content-hash>/`:
- **Immutable** — never modified after installation
- **Atomic** — all-or-nothing installation
- **Deduplicated** — identical files share storage
- **Multi-version** — old and new versions coexist

## Atomic Upgrades & Rollback

```bash
# Upgrade the system (two-phase commit)
sigpkg upgrade

# If something breaks:
sigpkg rollback           # Instant rollback

# List all generations
sigpkg generations list

# Rollback to specific generation
sigpkg rollback --generation 42
```

## Dependency Resolution (SAT Solver)

sigpkg uses a SAT solver for complete, conflict-free dependency resolution:
- Guarantees all dependencies are satisfied before installation
- Detects conflicts before making any changes
- Finds the minimal set of packages needed

## PKGBUILD Support (Arch AUR Parity)

```bash
# Build from PKGBUILD
sigpkg makepkg PKGBUILD

# Install from AUR
sigpkg aur install package-name

# Update AUR packages
sigpkg aur upgrade
```

## Configuration

`/etc/sigma/sigpkg.toml`:
```toml
[general]
store = "/sigma/store"
parallel_downloads = 4
signature_check = "always"
keep_generations = 5

[[repositories]]
name = "sigma-main"
url = "https://pkg.sigmaos.dev/main"
type = "sigma"
```

## Full Documentation

See [docs/PACKAGE_MANAGER.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/PACKAGE_MANAGER.md) for the complete package manager guide.

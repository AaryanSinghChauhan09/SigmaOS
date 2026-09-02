# Linux & BSD Compatibility

SigmaOS implements comprehensive compatibility with all major Linux and BSD distributions.

---

## Compatibility Overview

| Distribution | Parity Features | Source File |
|-------------|----------------|-------------|
| Arch Linux | Rolling release, AUR, PKGBUILD, pacman | `src/distro/arch_inspirations.rs` |
| NixOS | Declarative config, atomic upgrades, store | `src/distro/nixos_inspirations.rs` |
| Gentoo | USE flags, Portage, ebuilds | `src/distro/gentoo_inspirations.rs` |
| Fedora | Cockpit, PipeWire, FreeIPA, Anitya | `src/compatibility/fedora.rs` |
| CachyOS | BORE, PGO, x86-64-v3 | `src/compatibility/cachy_os.rs` |
| Debian | apt, dpkg, stable | `src/compatibility/` |
| Alpine | musl, apk, minimal | `src/distro/` |
| FreeBSD | Capsicum, Jails, ZFS | `src/security/` |
| OpenBSD | pledge, unveil, KARL | `src/security/` |
| DragonFly BSD | HAMMER2 | `src/filesystem/` |
| Garuda | Zen kernel, ZRAM | `src/distro/` |
| openSUSE | Snapper, zypper | `src/distro/` |

---

## Arch Linux Parity

### Rolling Release
```toml
# /etc/sigma/release.toml
[release]
channel = "stable"    # edge | stable | lts
auto_update = true
```

### AUR Support
```bash
sigpkg aur search yay
sigpkg aur install yay
sigpkg aur upgrade
```

### PKGBUILD Builds
```bash
sigpkg makepkg PKGBUILD
```

---

## NixOS Parity

### Declarative Configuration
```nix
# /etc/sigma/configuration.sigma
{
  environment.systemPackages = with pkgs; [ vim git firefox ];
  services.nginx.enable = true;
  networking.hostName = "my-sigma";
}
```

```bash
sigma-apply /etc/sigma/configuration.sigma
```

### Generations
```bash
sigma-gen list           # All generations
sigma-gen rollback 42    # Roll back
```

---

## Gentoo Parity

### USE Flags
```bash
echo 'USE="ssl http2 -perl"' >> /etc/sigma/make.conf
echo 'www-servers/nginx ssl http2' >> /etc/sigma/package.use
sigpkg install --use "ssl http2" nginx
```

---

## OpenBSD Security Parity

### pledge()
```rust
process.pledge(&[PledgeClass::Stdio, PledgeClass::Inet])?;
```

### unveil()
```rust
process.unveil("/etc/nginx", UnveilMode::Read)?;
process.unveil_lock()?;
```

---

## FreeBSD Parity

### Capsicum
```rust
cap_enter()?;  // Enter capability mode
fd.cap_rights_limit(&[CapRight::Read, CapRight::Seek])?;
```

### Jails
```bash
sigma-jail create --path /jails/web --hostname web.example.com
sigma-jail exec web nginx -g "daemon off;"
```

---

## Full Documentation

See [docs/DISTRO_COMPAT.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/DISTRO_COMPAT.md) for the complete compatibility guide.

# SigmaOS Linux & BSD Compatibility

SigmaOS implements comprehensive parity with major Linux and BSD distributions, drawing the best features from each into a unified sovereign OS.

***

## Supported Distributions

| Distribution | Category | Parity Status |
|-------------|----------|--------------|
| Arch Linux | Rolling release | ✅ Full |
| NixOS | Declarative | ✅ Full |
| Gentoo | Source-based | ✅ Full |
| Fedora | RPM-based | ✅ Full |
| CachyOS | Performance | ✅ Full |
| Alpine Linux | Minimal | ✅ Full |
| Debian/Ubuntu | Stable | ✅ Full |
| Linux Mint | Desktop | ✅ Full |
| openSUSE | Enterprise | ✅ Full |
| Void Linux | Independent | ✅ Full |
| Garuda Linux | Gaming | ✅ Full |
| FreeBSD | BSD | ✅ Full |
| OpenBSD | Security | ✅ Full |
| DragonFly BSD | Storage | ✅ Full |
| NetBSD | Portable | ✅ Partial |

***

## Arch Linux (`src/distro/arch_inspirations.rs`)

### Rolling Release Model

SigmaOS implements Arch-style rolling releases with three channels:

```rust
pub enum RollingChannel {
    Edge,   // Latest upstream, updates daily
    Stable, // Promoted from Edge after ~1 week
    Lts,    // Security fixes only
}
```

Configure in `/etc/sigma/release.toml`:

```toml
[release]
channel = "stable"  # edge | stable | lts
auto_update = true
update_interval_hours = 24
```

### AUR / PKGBUILD Support

Full PKGBUILD recipe support:

```bash
# sigpkg builds exactly like makepkg
sigpkg aur install yay
sigpkg aur install brave-bin
```

`SigmaMakePkg` implements:

*   PKGBUILD parsing (bash variable extraction)
*   Source download with checksum verification
*   Sandboxed build environment
*   `.pkg.tar.zst` packaging
*   Signature with local keyring

### Pacman Compatibility

```bash
# Arch packages install natively
sigpkg install package.pkg.tar.zst

# Pacman-style database sync
sigpkg sync  # equivalent to pacman -Sy
sigpkg sync --full  # equivalent to pacman -Syuu
```

***

## NixOS (`src/distro/nixos_inspirations.rs`)

### Declarative System Configuration

Define the entire system state in `/etc/sigma/configuration.sigma`:

```nix
# /etc/sigma/configuration.sigma
{ config, pkgs, ... }:
{
  # System packages
  environment.systemPackages = with pkgs; [
    vim git curl wget firefox
  ];

  # Services
  services.nginx.enable = true;
  services.postgresql = {
    enable = true;
    package = pkgs.postgresql_16;
  };

  # Network
  networking.hostName = "my-sigma-machine";
  networking.interfaces.eth0.ipv4.addresses = [{
    address = "192.168.1.100";
    prefixLength = 24;
  }];

  # Kernel modules
  boot.kernelModules = [ "kvm-intel" "usb_storage" ];
}
```

Apply configuration:

```bash
sigma-apply /etc/sigma/configuration.sigma
```

### Content-Addressed Store

All packages live in `/sigma/store/<hash>/`:

    /sigma/store/
    ├── sha256-abc123.../   ← nginx 1.25.3
    ├── sha256-def456.../   ← openssl 3.1.0
    └── sha256-789ghi.../   ← curl 8.2.0

The store is immutable — packages are never modified after installation. Multiple versions coexist without conflict.

### Atomic Upgrades

The **active generation** is a symlink:

    /sigma/current → /sigma/store/generation-47-abc123.../

Switching generations (upgrade or rollback) is a single atomic `rename(2)` call. The system can never be caught in a half-upgraded state.

### Generations & Rollback

```bash
# List all system generations
sigma-gen list
# → Gen 45 (2026-08-01): nginx 1.25.1, kernel 6.5
# → Gen 46 (2026-08-15): nginx 1.25.2, kernel 6.6  ← active
# → Gen 47 (2026-09-01): nginx 1.25.3, kernel 6.7

# Instantly roll back to gen 45
sigma-gen rollback 45

# Boot menu also shows all generations (GRUB integration)
```

***

## Gentoo (`src/distro/gentoo_inspirations.rs`)

### USE Flags

Compile packages with exactly the features you need:

```bash
# Show available USE flags for a package
sigpkg use-flags nginx

# Set USE flags globally
echo "USE=\"ssl http2 -perl -ruby\"" >> /etc/sigma/make.conf

# Set per-package USE flags
echo "www-servers/nginx ssl http2 gzip" >> /etc/sigma/package.use

# Install with custom flags
sigpkg install --use "ssl http2" nginx
```

### Portage-Style Source Compilation

`SigmaPortage` compiles packages from source:

```bash
# Emerge-style installation
sigma-emerge install nginx

# World update (all installed packages)
sigma-emerge upgrade @world

# Dependency check
sigma-emerge check-deps nginx
```

### Ebuilds

Write ebuilds to build custom packages:

```bash
# Example SigmaOS ebuild
EAPI=8
DESCRIPTION="My application"
HOMEPAGE="https://example.com"
SRC_URI="https://example.com/myapp-${PV}.tar.gz"
LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64"
IUSE="ssl +http2"

DEPEND="ssl? ( dev-libs/openssl )"
RDEPEND="${DEPEND}"

src_configure() {
    econf $(use_enable ssl) $(use_enable http2)
}

src_install() {
    default
    dobin myapp
}
```

***

## Fedora (`src/compatibility/fedora.rs`)

### Fedora Services

**Cockpit Web Console** (`src/remote/`):

```bash
# Enable and start Cockpit
sigpkg install sigma-cockpit
sigma-service start cockpit
# Access at https://localhost:9090
```

**PipeWire Audio**:

```bash
# PipeWire replaces PulseAudio/JACK
sigpkg install sigma-pipewire
sigma-service start pipewire pipewire-pulse
```

**FreeIPA / Kerberos**:

```bash
# Join a FreeIPA domain
sigma-ipa join ipa.example.com --user admin

# Kerberos ticket
kinit user@EXAMPLE.COM
```

### Fedora Package Ecosystem

```bash
# Install from Fedora repositories
sigpkg --repo fedora install dnf-utils

# RPM package compatibility
sigpkg install package.rpm
rpm -qa  # List installed RPM packages (compat)
```

### Anitya Release Monitoring

Automatically monitors upstream versions and alerts when packages are outdated:

```bash
# Check if installed packages have upstream updates
sigpkg check-upstream

# Configure monitoring
sigma-anitya configure --backend github --project sigmaos/sigmaos
```

***

## CachyOS (`src/compatibility/cachy_os.rs`)

### BORE Scheduler

Burst-Oriented Response Enhancer for improved desktop responsiveness:

```bash
# Enable BORE scheduler
sigma-sysctl kernel.sched_bore=1

# Tune burst penalty
sigma-sysctl kernel.sched_burst_penalty_scale=1280
```

### Performance Optimisations

**LLVM PGO (Profile-Guided Optimisation)**:

```bash
# Build PGO-optimised kernel
sigma-build --pgo

# Apply collected profile
sigma-build --pgo-use=/var/sigma/kernel.profdata
```

**x86-64-v3 Microarchitecture**:

```bash
# Build packages for x86-64-v3 (AVX2, BMI2, FMA)
sigma-emerge build --march=x86-64-v3 nginx

# Check if CPU supports x86-64-v3
/lib/sigma/haswell-check.sh
```

### CachyOS Kernel Feature Matrix

`CachyosKernelFeatureMatrix` unifies:

*   BORE scheduler
*   LLVM ThinLTO
*   Clang CFI
*   ZSTD kernel compression
*   NVIDIA DKMS headers
*   TCP BBR2 congestion control

***

## OpenBSD (`src/security/`)

### pledge()

```c
// C compat: restrict process syscall set
pledge("stdio rpath inet", NULL);

// Rust native API
process.pledge(&[
    PledgeClass::Stdio,
    PledgeClass::Rpath,
    PledgeClass::Inet,
])?;
```

### unveil()

```c
// C compat: expose only specific filesystem paths
unveil("/etc/nginx", "r");
unveil("/var/www", "rwc");
unveil(NULL, NULL);  // lock down

// Rust native API
process.unveil("/etc/nginx", UnveilMode::Read)?;
process.unveil("/var/www", UnveilMode::ReadWriteCreate)?;
process.unveil_lock()?;
```

### KARL (Kernel Address Randomised Link)

At each boot, the kernel binary is relinked with randomised section order:

*   Makes ROP gadget exploitation unreliable
*   Different offset each boot (not just at install time)
*   Enabled by default

***

## FreeBSD (`src/security/`)

### Capsicum Capability Mode

```rust
// Enter capability mode — no new syscalls can gain ambient authority
cap_enter()?;

// Limit a file descriptor's rights
let fd = open("/etc/config")?;
let limited = fd.cap_rights_limit(&[
    CapRight::Read,
    CapRight::Seek,
])?;
// Now `limited` cannot write or execute
```

### Jails

```bash
# Create a jail
sigma-jail create \
    --path /jails/web-server \
    --hostname web.example.com \
    --ip 10.0.0.1

# Run a command in a jail
sigma-jail exec web-server nginx -g "daemon off;"

# List active jails
sigma-jail list
```

### ZFS Compatibility

```bash
# ZFS pool operations (FreeBSD parity)
zpool create tank mirror /dev/sdb /dev/sdc
zpool status

# ZFS datasets
zfs create tank/data
zfs snapshot tank/data@backup-2026
zfs rollback tank/data@backup-2026
```

***

## DragonFly BSD — HAMMER2

SigmaOS's native filesystem draws from DragonFly BSD's HAMMER2:

    Features:
    - Multi-master clustering
    - On-the-fly compression (LZ4, ZSTD)
    - Deduplication
    - Snapshots (CoW, near-instant)
    - Multiple PFS (pseudo-filesystems) per volume

```bash
# Create HAMMER2 volume
sigma-fs create hammer2 /dev/sda1 my-volume

# Create PFS
sigma-hammer2 pfs-create my-volume/data

# Create snapshot
sigma-hammer2 snapshot my-volume/data snap-2026-09
```

***

## Alpine Linux

### Musl libc Compatibility

SigmaOS supports musl libc alongside glibc:

```bash
# Build against musl
CC=musl-gcc cargo build

# Install musl-linked packages
sigpkg --libc musl install nginx
```

### Minimal Footprint Mode

```bash
# Install minimal edition
sigpkg install sigmaos-minimal

# Uses BusyBox-compatible sigma-toolbox
sigma-toolbox ls
sigma-toolbox grep pattern /etc/sigma
```

***

## openSUSE

### Snapper Integration

CoW filesystem snapshots before/after package operations:

```bash
# List snapshots
snapper list

# Compare changes between snapshots
snapper diff 41..42

# Rollback to before last upgrade
snapper rollback 41
```

### Zypper Compatibility

```bash
# openSUSE zypper commands work via compatibility layer
zypper install nginx
zypper search "web server"
zypper update
```

***

## Void Linux

### runit Init System (`src/distro/`)

SigmaOS supports runit as an alternative init:

```bash
# Service management (runit-style)
sigma-runit enable nginx
sigma-runit start nginx
sigma-runit status nginx
sigma-runit stop nginx
```

### XBPS Package Format

```bash
# Install from Void xbps repository
sigpkg --repo void install xbps-utils

# XBPS package file compatibility
sigpkg install package.xbps
```

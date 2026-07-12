# SigmaOS — Linux-as-Application Roadmap

## Making Every Linux Distro Run as a sigma-pod App on SigmaOS

**Vision:** Ubuntu, Fedora, Debian, Arch, Kali, NixOS, and every other
Linux distribution becomes an installable application on SigmaOS —
exactly the way WSL2 made Linux an app on Windows, but done better:
sovereign, post-quantum secure, and with full hardware access.

---

## Strategic Context

```
Today's world:
  Linux distros ARE the OS  →  users are locked into distro choices
  Windows adds WSL2         →  Linux becomes an app, but:
                               - Microsoft controls the hypervisor
                               - No PQC, no India Stack, no DID
                               - Telemetry, vendor lock-in

SigmaOS vision:
  SigmaOS IS the OS        →  every Linux distro runs as a sigma-pod app
  sigma-linux exec ubuntu  →  Ubuntu 24.04 shell in < 2 seconds
  sigma-linux exec fedora  →  Fedora 41 shell in < 2 seconds
  sigma-linux exec kali    →  Kali Linux (for ethical security work)
  sigma-linux exec nixos   →  NixOS reproducible environment
  sigma-linux exec arch    →  Arch Linux rolling release app
  All with:
    - No real VM overhead (sigma-linux-compat + lightweight namespace)
    - PQC-TLS for all network traffic out of the Linux app
    - DID identity passed into the Linux environment
    - India Stack APIs accessible from inside Linux apps
    - sigma-audit tamper-evident log of everything inside Linux app
    - sigma-pod resource limits enforced (CPU/RAM/IO)
```

---

## Architecture: Linux Distro as sigma-pod

```
SigmaOS kernel (Ring-0)
  ├── sigma-sched (MLFQ + MCS)
  ├── sigma-mm (buddy + slab + VMM)
  └── sigma-bus (capability IPC)

  sigma-pod "ubuntu-24.04" (namespaced container)
  ├── PID namespace   → Ubuntu sees its own PID 1 (init/systemd-stub)
  ├── NET namespace   → Ubuntu has its own network stack via veth
  ├── MNT namespace   → Ubuntu rootfs from .spkg image (dm-verity)
  ├── IPC namespace   → Ubuntu D-Bus isolated from SigmaOS sigma-bus
  ├── UTS namespace   → hostname = "ubuntu-24.04"
  ├── USER namespace  → root inside maps to non-root on SigmaOS
  └── cgroup v2       → CPU/RAM/IO limits enforced by kernel

  sigma-linux-compat (Ring-3 translator)
  ├── ELF64 loader (already built)
  ├── Linux syscall → sigma-syscall translator (15 calls done, expand to 300)
  ├── /proc/self, /sys, /dev stubs (sigma-procfs provides)
  ├── vDSO shim (clock_gettime, gettimeofday)
  └── Linux signal handling → sigma-signal

  Ubuntu 24.04 rootfs (compressed .spkg image, dm-verity verified)
  ├── /bin/bash (from Ubuntu apt packages)
  ├── /lib/x86_64-linux-gnu/libc.so.6 (Ubuntu glibc)
  ├── /usr/bin/* (any Ubuntu package)
  └── All Ubuntu software runs natively — no recompilation
```

---

## Implementation Phases

### Phase L0 — Foundation (Months 1-3, parallel to kernel boot work)

**Goal:** sigma-linux namespace infrastructure ready; static binaries run.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Expand syscall translator to 100 calls | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Add: stat/fstat/lseek/pipe/dup2/clone/wait4/execve/mprotect/getcwd/socket/connect/send/recv/futex + 85 more |
| `/proc/self/maps` via sigma-procfs | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | glibc reads this at startup |
| `/proc/cpuinfo` stub | `kernel/vfs/sigma_procfs.cpp` | `kernel-exp` | Expose physical CPU info to Linux app |
| `/sys/` minimal tree | `kernel/vfs/sigma_sysfs.cpp` | `kernel-exp` | `/sys/class/net/`, `/sys/block/` |
| `/dev/` device nodes | `kernel/vfs/sigma_devfs.cpp` | `kernel-exp` | `/dev/null`, `/dev/zero`, `/dev/urandom`, `/dev/tty` |
| Linux signal routing | `runtime/containers/sigma_linux_compat.cpp` | `kernel-exp` | SIGTERM/SIGKILL/SIGINT → sigma kill |
| `sigma-linux exec <static-binary>` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | Run static ELF: `sigma-linux exec /bin/ls` |

**Exit gate:** `sigma-linux exec /bin/ls` (static musl) lists `/` directory.

### Phase L1 — Dynamic Binaries (Months 3-6)

**Goal:** glibc-linked Ubuntu binaries run inside sigma-pod namespace.

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-ldso (ELF dynamic linker) | `userland/ldso/sigma_ldso.cpp` | `tools-dev` | Load ld-linux.so.2 from the distro rootfs |
| veth pair for Linux network namespace | `kernel/net/sigma_veth.cpp` | `drivers-dev` | Virtual ethernet: sigma-net ↔ Linux net namespace |
| Linux rootfs .spkg image builder | `tools/sigma_rootfs_builder.sh` | `tools-dev` | `debootstrap` → `.spkg` (dm-verity + ML-DSA-87 signed) |
| Minimal Ubuntu 24.04 rootfs | `sigma_pkg_registry/linux-images/ubuntu-24.04.spkg` | `tools-dev` | 200 MB minimal Ubuntu rootfs as .spkg |
| `/etc/resolv.conf` → sigma-dns | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Route DNS from Linux app through sigma-dns-cache |
| Terminal emulator in Zenith | `zenith_desktop/ui/sigma_terminal.cpp` | `release/standalone` | GPU-accelerated terminal (sigma-term) |
| `sigma-linux exec ubuntu bash` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | Full Ubuntu shell in < 2 seconds |

**Exit gate:** `sigma-linux exec ubuntu bash` opens Ubuntu 24.04 bash prompt.

### Phase L2 — Full Distro Support (Months 6-12)

**Goal:** Every major Linux distro installable and usable via sigma-linux.

```bash

# Install any distro as an app:

sigma-linux install ubuntu       # Ubuntu 24.04 LTS

sigma-linux install fedora       # Fedora 41

sigma-linux install debian       # Debian 12 Bookworm

sigma-linux install arch         # Arch Linux (rolling)

sigma-linux install kali         # Kali Linux (forensic profile)

sigma-linux install nixos        # NixOS (declarative)

sigma-linux install alpine       # Alpine (musl, minimal)

sigma-linux install pop-os       # Pop!_OS (NVIDIA-friendly)

sigma-linux install manjaro      # Manjaro (user-friendly Arch)

sigma-linux install opensuse     # openSUSE Tumbleweed

# Run:

sigma-linux exec ubuntu          # bash inside Ubuntu

sigma-linux exec fedora -- dnf install vim
sigma-linux exec arch -- pacman -S firefox
sigma-linux exec kali -- nmap -sV 192.168.1.1  # requires forensic cap

# GUI apps from Linux distros in Zenith:

sigma-linux exec ubuntu -- firefox              # X11/Wayland app in Zenith

sigma-linux exec fedora -- gnome-text-editor
sigma-linux exec ubuntu -- code .              # VSCode from Ubuntu

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Expand syscall translator to 300 calls | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Full Linux x86-64 ABI coverage |
| X11/Wayland socket bridge | `zenith_desktop/compat/sigma_xwayland_bridge.cpp` | `release/standalone` | Linux X11 apps render in Zenith windows |
| Vulkan ICD passthrough | `runtime/containers/sigma_linux_compat.cpp` | `release/standalone` | Linux apps use SigmaOS GPU via Vulkan |
| Audio bridge (PipeWire compat) | `userland/audio/sigma_pipewire_bridge.cpp` | `release/standalone` | Linux audio → sigma-audio HDA |
| Distro rootfs auto-download | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | `sigma-linux install ubuntu` → fetch + verify + install |
| Rootfs update command | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | `sigma-linux update ubuntu` → `apt upgrade` inside |
| Multi-distro storage isolation | `kernel/core/process/sigma_namespace.cpp` | `release/cloud` | Each distro has its own `/home`, `/var`, `/etc` namespace |
| `sigma-linux list` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | Show installed distros + disk usage |
| `sigma-linux remove ubuntu` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | Wipe rootfs + config cleanly |
| `sigma-linux backup ubuntu` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | Export distro rootfs as `.spkg` |
| `sigma-linux restore ubuntu.spkg` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | Restore from backup |
| Distro config file | `/sigma/etc/sigma-linux/ubuntu.conf` | `tools-dev` | TOML: rootfs path, resource limits, default shell |

### Phase L3 — GUI & Desktop Integration (Months 9-18)

**Goal:** Linux GUI apps appear as first-class Zenith windows.

```bash

# Linux GUI app in Zenith window:

sigma-linux app ubuntu firefox         # Firefox from Ubuntu in Zenith

sigma-linux app fedora libreoffice     # LibreOffice from Fedora

sigma-linux app ubuntu --display sigma -- gimp  # GIMP in Zenith

# Linux app appears in:

#  - Zenith app launcher (Super key search)

#  - Taskbar with correct icon

#  - Alt+Tab window switcher

#  - Proper window title and close button

```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-xserver (X11 compatibility server) | `zenith_desktop/compat/sigma_xserver.cpp` | `release/standalone` | Minimal X11 server: maps X11 windows → Zenith surfaces |
| XWayland bridge (reuse XWayland approach) | `zenith_desktop/compat/sigma_xwayland_bridge.cpp` | `release/standalone` | X11 → Wayland → sigma-display protocol |
| Linux app icon extraction | `userland/tools/sigma_linux_cli.cpp` | `release/standalone` | Extract `.desktop` file icon → Zenith launcher |
| Linux app in Zenith launcher | `zenith_desktop/launcher/sigma_launcher.cpp` | `release/standalone` | Show `[Ubuntu] Firefox` in app list |
| Clipboard sharing | `zenith_desktop/clipboard/sigma_clipboard.cpp` | `release/standalone` | Copy in Zenith, paste in Linux app |
| Drag-and-drop between worlds | `zenith_desktop/compositor/sigma_compositor.cpp` | `release/standalone` | Drag file from Zenith file manager to Ubuntu Firefox |
| Font sharing | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Noto Sans Devanagari available inside Linux apps |
| IME passthrough | `userland/ime/sigma_ime_cli.cpp` | `release/standalone` | Hindi input works in Linux apps |

### Phase L4 — Security Integration (Months 12-18)

**Goal:** Linux apps inside sigma-pod are MORE secure than on native Linux.

```
Security advantages over native Linux:

  1. dm-verity rootfs  → distro rootfs tamper-detected on every read
  2. sigma-mac policy  → Linux app's syscalls checked against .sigma-policy
  3. sigma-audit       → every syscall inside Linux app logged with DID
  4. PQC-TLS wrapper   → all outbound traffic from Linux app uses ML-KEM
  5. cgroup limits     → Ubuntu can't OOM the whole system
  6. Network policy    → sigma-pod network rules: Kali can't reach GSTN
  7. Capability tokens → Firefox in Ubuntu can't access sigma-health data
  8. No root escape    → USER namespace: root inside = unprivileged outside
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-mac policy for Linux apps | `kernel/security/sigma_mac.cpp` | `release/standalone` | Apply `.sigma-policy` to Linux syscall boundary |
| dm-verity on distro rootfs | `kernel/fs/sigma_dmverity.cpp` | `release/standalone` | Verify rootfs on every mount read |
| PQC-TLS wrap all Linux outbound | `net/tls/sigma_tls.cpp` | `drivers-dev` | Transparent TLS upgrade for Linux app HTTP |
| sigma-audit per Linux syscall | `kernel/security/sigma_immutable_audit_trail.cpp` | `release/standalone` | Log every Linux syscall with DID |
| Network policy per distro | `kernel/core/process/sigma_namespace.cpp` | `release/cloud` | Kali: only forensic network allowed; Ubuntu: full |
| Capability grant for Linux app | `include/sigma_caps.h` | `kernel-exp` | `sigma-linux exec --cap net.tx,fs.read ubuntu bash` |
| Linux app sandbox profile | `/sigma/etc/sigma-linux/profiles/` | `tools-dev` | TOML: per-distro security policy templates |

---

## 3. sigma-linux CLI — Complete Command Set

```bash

# Installation

sigma-linux install <distro> [--version x.y]   # download + verify + install

sigma-linux install --url <spkg-url>            # install from custom URL

sigma-linux import <rootfs.tar.gz>              # import custom rootfs

# Execution

sigma-linux exec <distro>                       # interactive shell

sigma-linux exec <distro> -- <command>          # run single command

sigma-linux exec <distro> --root -- <cmd>       # run as root inside

sigma-linux exec <distro> --cpu 500 --mem 1024  # with resource limits

sigma-linux exec <distro> --cap forensic        # with forensic profile

sigma-linux exec <distro> --no-network          # network-isolated

sigma-linux exec <distro> --bind /sigma/data/ca:/mnt/ca  # mount path

# GUI apps

sigma-linux app <distro> <app-name>             # launch GUI app in Zenith

sigma-linux app <distro> --list                 # list installed GUI apps

sigma-linux app <distro> install <pkg>          # install GUI app from distro pkg

# Management

sigma-linux list                                # installed distros + disk usage

sigma-linux status <distro>                     # running instances + resources

sigma-linux stop <distro>                       # stop all instances

sigma-linux update <distro>                     # run distro's package updater

sigma-linux remove <distro> [--purge]           # uninstall

sigma-linux backup <distro> <file.spkg>         # export snapshot

sigma-linux restore <file.spkg>                 # restore from snapshot

sigma-linux rename <old> <new>                  # rename distro instance

# Configuration

sigma-linux config <distro> shell /usr/bin/zsh  # set default shell

sigma-linux config <distro> memory 4096         # set RAM limit (MB)

sigma-linux config <distro> cpu 1000            # set CPU limit (milliCPU)

sigma-linux config <distro> dns sigma           # use sigma-dns-cache

sigma-linux config <distro> gpu enable          # GPU passthrough

sigma-linux config <distro> audio enable        # audio bridge

# Security

sigma-linux policy <distro> show                # current sigma-mac policy

sigma-linux policy <distro> set forensic        # apply forensic profile

sigma-linux audit <distro> log --last 50        # audit trail inside distro

sigma-linux audit <distro> verify               # verify audit chain

```

| Task | File | Branch | Priority |
|------|------|--------|---------|
| `sigma-linux install` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | 🔴 Core |
| `sigma-linux exec` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | 🔴 Core |
| `sigma-linux list/status/stop` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | 🔴 Core |
| `sigma-linux app` (GUI) | `userland/tools/sigma_linux_cli.cpp` | `release/standalone` | 🟠 Desktop |
| `sigma-linux policy/audit` | `userland/tools/sigma_linux_cli.cpp` | `release/standalone` | 🟠 Security |
| `sigma-linux config` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | 🟠 Config |
| `sigma-linux backup/restore` | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | 🟡 Data |

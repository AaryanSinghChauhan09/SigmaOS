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

### Phase L2 — Full Distro Library (Months 6-12)

**Goal:** 10 major Linux distributions available as one-command installs.

```bash
# Install any distro:
sigma-linux install ubuntu:24.04
sigma-linux install fedora:41
sigma-linux install debian:12
sigma-linux install arch:latest
sigma-linux install kali:2026.1
sigma-linux install nixos:24.05
sigma-linux install alpine:3.20
sigma-linux install opensuse:tumbleweed
sigma-linux install centos-stream:9
sigma-linux install manjaro:latest

# List installed:
sigma-linux list
# ubuntu:24.04    200 MB   ✓ running  (PID ns 1247)
# kali:2026.1     450 MB   ✓ stopped
# fedora:41       350 MB   ✓ stopped

# Launch a distro:
sigma-linux exec ubuntu:24.04
sigma-linux exec ubuntu:24.04 -- /bin/bash -c "apt install vim"
sigma-linux exec kali:2026.1 -- nmap -sV 10.0.0.1  # inside Kali

# Run X11/Wayland app from Linux distro on Zenith:
sigma-linux exec ubuntu:24.04 -- firefox
# Firefox window appears in Zenith compositor
# sigma-display protocol bridges Linux X11 → Zenith surface

# Share files between SigmaOS and Linux distro:
sigma-linux mount ubuntu:24.04 /sigma/data /mnt/sigma
# Inside Ubuntu: ls /mnt/sigma shows SigmaOS files

# sigma-linux is the sigma equivalent of WSL:
# WSL2  = Linux under Windows via Hyper-V (VM overhead ~128MB RAM)
# sigma-linux = Linux under SigmaOS via namespaces (< 5MB overhead)
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| Ubuntu 24.04 rootfs .spkg | `sigma_pkg_registry/linux-images/ubuntu-24.04.spkg` | `tools-dev` | `debootstrap --variant=minbase` → 200 MB |
| Fedora 41 rootfs .spkg | `sigma_pkg_registry/linux-images/fedora-41.spkg` | `tools-dev` | `dnf install --installroot` minimal |
| Kali Linux 2026.1 rootfs | `sigma_pkg_registry/linux-images/kali-2026.spkg` | `tools-dev` | Kali tools for ethical hacking (sigma-mac forensic profile) |
| Alpine Linux 3.20 rootfs | `sigma_pkg_registry/linux-images/alpine-3.20.spkg` | `tools-dev` | Tiny (5 MB) — musl + busybox |
| NixOS 24.05 rootfs | `sigma_pkg_registry/linux-images/nixos-24.05.spkg` | `tools-dev` | Nix package manager inside sigma-pod |
| Arch Linux rootfs | `sigma_pkg_registry/linux-images/arch-latest.spkg` | `tools-dev` | Arch + pacman inside sigma-pod |
| X11 bridge → Zenith | `zenith_desktop/compat/sigma_x11_bridge.cpp` | `release/standalone` | XWayland-style: Linux X11 apps render in Zenith |
| `/mnt/sigma` file sharing | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | Bind-mount SigmaOS VFS path into Linux namespace |
| sigma-linux CLI full | `userland/tools/sigma_linux_cli.cpp` | `tools-dev` | install/remove/list/exec/mount/shell/stop/kill |

**Exit gate:** `sigma-linux exec ubuntu:24.04 -- apt install neovim && nvim` works.

### Phase L3 — Graphical Linux Apps in Zenith (Months 12-18)

**Goal:** Any Linux GUI app (Electron, GTK, Qt) renders natively in Zenith.

```bash
# Install and run Linux GUI apps:
sigma-linux exec ubuntu:24.04 -- apt install libreoffice -y
sigma-linux exec ubuntu:24.04 -- libreoffice --writer
# → LibreOffice Writer window opens in Zenith, fully integrated

sigma-linux exec ubuntu:24.04 -- apt install code -y
sigma-linux exec ubuntu:24.04 -- code
# → VSCode window in Zenith, with sigma-bus file system access

sigma-linux exec kali:2026.1 -- apt install burpsuite -y
sigma-linux exec kali:2026.1 -- burpsuite
# → Burp Suite in Zenith, network goes through sigma-firewall
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| X11 socket via sigma-display bridge | `zenith_desktop/compat/sigma_x11_bridge.cpp` | `release/standalone` | Linux `DISPLAY=:0` connects to Zenith compositor surface |
| Wayland protocol bridge | `zenith_desktop/compat/sigma_wayland_bridge.cpp` | `release/standalone` | Linux Wayland apps via XDG surfaces in Zenith |
| DRI3/DRM passthrough | `drivers/graphics/sigma_kms.cpp` | `drivers-dev` | Linux apps get GPU-accelerated rendering |
| Clipboard sync (Linux ↔ SigmaOS) | `zenith_desktop/clipboard/sigma_clipboard.cpp` | `release/standalone` | Copy in Ubuntu, paste in sigma-accounts |
| Font sharing | `zenith_desktop/compositor/sigma_font.cpp` | `release/standalone` | Linux apps use same Noto fonts as Zenith |
| Sound: Linux ALSA → sigma-audio | `drivers/audio/sigma_hda.cpp` | `drivers-dev` | Linux audio routed through sigma-audio mixer |
| Input: sigma-input → Linux app | `userland/daemons/sigma_inputd.cpp` | `release/standalone` | Keyboard/pointer events forwarded to X11/Wayland |
| Window decorations in Zenith | `zenith_desktop/wm/sigma_tiling_wm.cpp` | `release/standalone` | Linux windows tiled by sigma-tiling-wm |

**Exit gate:** `LibreOffice Writer` opens from Ubuntu namespace, renders in Zenith, prints via sigma-audio, saves to `/sigma/data/docs/`.

---

## sigma-linux vs WSL2 — Technical Comparison

| Dimension | WSL2 (Microsoft) | sigma-linux (SigmaOS) |
|-----------|-----------------|----------------------|
| **Architecture** | Lightweight VM (Hyper-V Type-1) | namespace isolation (no VM) |
| **RAM overhead** | ~128 MB for VM + Linux kernel | < 5 MB for namespace |
| **Boot time** | 2-4 s (VM boot) | < 1 s (namespace spawn) |
| **Kernel** | Full Linux kernel in VM | sigma-linux-compat translator |
| **Security** | VM boundary + Windows NTLM | sigma-mac + ML-DSA-87 signed images |
| **PQC** | ❌ None | ✅ All traffic ML-KEM encrypted |
| **Telemetry** | ✅ Microsoft collects | ❌ Zero telemetry |
| **Vendor lock-in** | Microsoft controls Hyper-V | sigma-linux is fully open |
| **India Stack** | ❌ Not available | ✅ Full GSTN/ABDM access from inside Linux app |
| **DID identity** | ❌ Microsoft account | ✅ DID passed into Linux environment |
| **Audit trail** | Windows Event Log | sigma-audit ML-DSA-87 per entry |
| **File system** | Plan 9 (9P) — slow | sigma-vfs bind-mount — same speed |
| **GUI apps** | WSLg (RDP + X11 over RDP) | X11/Wayland → Zenith direct |
| **GPU** | D3D12 passthrough via Mesa | DRI3/DRM passthrough native |
| **GPU for AI** | DirectML | Vulkan compute → sigma-ai |
| **Distro images** | MS Store only | sigma-pkg: any distro |
| **Reproducibility** | No guarantees | dm-verity + ML-DSA-87 signed |
| **Open source** | WSL kernel open, host closed | Entirely open source |

---

## sigma-SL (Sovereign Linux Subsystem) — Full Architecture

```
sigma-SL architecture (sigma equivalent of WSL):

┌────────────────────────────────────────────────────────────┐
│                    SigmaOS Host                            │
│                                                            │
│  sigma-bus ────────────────────────────────────────────   │
│      │                                                     │
│      ├── sigma-SL manager daemon (sigmad/sl/main.go)      │
│      │     manages: image registry, running instances      │
│      │     provides: sigma-linux CLI                       │
│      │                                                     │
│      └── sigma-pod "ubuntu-24.04"                         │
│           ├── PID/NET/MNT/IPC/UTS/USER namespaces         │
│           ├── cgroup v2: CPU=50% RAM=4GB IO=200MB/s       │
│           │                                                 │
│           │   sigma-linux-compat layer                     │
│           │   (Ring-3 syscall translator)                  │
│           │   Linux syscall → sigma-syscall                │
│           │                                                 │
│           │   Ubuntu 24.04 rootfs (.spkg image)           │
│           │   /bin/bash /usr/bin/* /lib/x86_64-linux-gnu/ │
│           │   All Ubuntu packages work natively            │
│           │                                                 │
│           │   sigma-display bridge                         │
│           │   X11 DISPLAY=:sigma-0 → Zenith surface       │
│           │                                                 │
│           │   sigma-audio bridge                           │
│           │   ALSA /dev/snd → sigma-audio PCM             │
│           │                                                 │
│           │   sigma-net bridge                             │
│           │   veth0 → sigma-net TCP stack                 │
│           │   All traffic: PQC-TLS 1.3 enforced           │
│           │                                                 │
│           │   sigma-india bridge                           │
│           │   From inside Ubuntu:                          │
│           │   sigma-ca gst compute → works!               │
│           │   sigma-health prescribe → works!             │
│           └────────────────────────────────────────────── │
└────────────────────────────────────────────────────────────┘
```

---

## Linux Syscall Translator — Completion Roadmap

**Current:** 15 syscalls mapped. Need ≥ 300 for full distro compatibility.

### Priority 1 — glibc startup (needed for ANY dynamic binary)

| Syscall | Linux NR | Maps to | Status |
|---------|---------|---------|--------|
| `read` | 0 | `sigma_sys_read` | ✅ |
| `write` | 1 | `sigma_sys_write` | ✅ |
| `open` | 2 | `sigma_sys_open` | ✅ |
| `close` | 3 | `sigma_sys_close` | ✅ |
| `fstat` | 5 | `sigma_sys_stat` | ⚠️ stub |
| `lseek` | 8 | `sigma_sys_lseek` | ❌ |
| `mmap` | 9 | `sigma_sys_mmap` | ✅ |
| `mprotect` | 10 | `sigma_sys_mprotect` | ✅ |
| `munmap` | 11 | `sigma_sys_munmap` | ✅ |
| `brk` | 12 | `sigma_sys_brk` | ✅ |
| `rt_sigaction` | 13 | sigma signal | ❌ |
| `rt_sigprocmask` | 14 | sigma signal | ❌ |
| `ioctl` | 16 | sigma device | ❌ |
| `pread64` | 17 | `sigma_sys_pread` | ❌ |
| `pwrite64` | 18 | `sigma_sys_pwrite` | ❌ |
| `readv` | 19 | sigma scatter-gather | ❌ |
| `writev` | 20 | sigma scatter-gather | ❌ |
| `access` | 21 | sigma_sys_access | ❌ |
| `pipe` | 22 | sigma_sys_pipe | ❌ |
| `dup` | 32 | sigma_sys_dup | ❌ |
| `dup2` | 33 | sigma_sys_dup2 | ❌ |
| `pause` | 34 | sigma_sys_pause | ❌ |
| `nanosleep` | 35 | `sigma_sys_nanosleep` | ✅ |
| `getpid` | 39 | `sigma_sys_getpid` | ✅ |
| `socket` | 41 | sigma_sys_socket | ❌ |
| `connect` | 42 | sigma_sys_connect | ❌ |
| `accept` | 43 | sigma_sys_accept | ❌ |
| `sendto` | 44 | sigma_sys_sendto | ❌ |
| `recvfrom` | 45 | sigma_sys_recvfrom | ❌ |
| `clone` | 56 | sigma_sys_clone | ❌ |
| `fork` | 57 | sigma_sys_fork | ❌ |
| `execve` | 59 | sigma_sys_execve | ❌ |
| `exit` | 60 | `sigma_sys_exit` | ✅ |
| `wait4` | 61 | sigma_sys_wait4 | ❌ |
| `uname` | 63 | sigma_uname stub | ✅ |
| `getdents64` | 217 | sigma_sys_getdents64 | ❌ |
| `futex` | 202 | sigma_futex | ❌ |
| `set_tid_address` | 218 | stub | ✅ |
| `clock_gettime` | 228 | sigma_tsc_read | ✅ |
| `exit_group` | 231 | `sigma_sys_exit` | ✅ |
| `arch_prctl` | 158 | sigma_arch_prctl | ✅ |

### Priority 2 — Networking & I/O (needed for apt, curl, wget)

```
socket(2), bind, listen, accept, connect, sendto, recvfrom
getsockopt, setsockopt, shutdown
epoll_create1, epoll_ctl, epoll_wait
select, poll
read(2), write(2) on network fds
getaddrinfo (via /etc/resolv.conf → sigma-dns)
```

### Priority 3 — Process management (needed for shells, daemons)

```
clone(2) with CLONE_VM, CLONE_FS, CLONE_FILES, CLONE_SIGHAND
fork(2), vfork(2), execve(2)
wait4(2), waitpid(2)
kill(2), tkill(2), tgkill(2)
getpid, getppid, getuid, getgid, geteuid, getegid
setuid, setgid (mapped to sigma capability drop)
prctl(2) — PR_SET_NAME, PR_GET_DUMPABLE
```

### Priority 4 — File system (needed for package managers)

```
openat(2), fstatat(2), unlinkat(2), mkdirat(2)
rename(2), link(2), symlink(2)
chmod(2), chown(2), chdir(2), getcwd(2)
statx(2), stat64 compat
flock(2), fcntl(2)
sendfile(2), splice(2), tee(2)
inotify_init1, inotify_add_watch (needed by systemd + apt)
```

| Task | File | Branch | Count |
|------|------|--------|-------|
| Priority 1 syscalls (glibc startup) | `runtime/containers/sigma_linux_compat.cpp` | `tools-dev` | 40 syscalls |
| Priority 2 syscalls (networking) | `runtime/containers/sigma_linux_compat.cpp` | `drivers-dev` | 30 syscalls |
| Priority 3 syscalls (process mgmt) | `runtime/containers/sigma_linux_compat.cpp` | `kernel-exp` | 40 syscalls |
| Priority 4 syscalls (filesystem) | `runtime/containers/sigma_linux_compat.cpp` | `fs-dev` | 50 syscalls |
| Remaining 200+ syscalls | `runtime/containers/sigma_linux_compat.cpp` | all | Phase L2 |

---

## sigma-linux CLI — Complete Interface

```bash
# Image management:
sigma-linux images list              # available distros in sigma-pkg
sigma-linux install ubuntu:24.04    # download + verify + install
sigma-linux install --size-only fedora:41  # show download size first
sigma-linux remove ubuntu:24.04     # remove rootfs image
sigma-linux update ubuntu:24.04     # update rootfs packages

# Instance management:
sigma-linux list                     # running + stopped instances
sigma-linux start ubuntu:24.04      # start instance
sigma-linux stop ubuntu:24.04       # stop (SIGTERM, clean shutdown)
sigma-linux kill ubuntu:24.04       # force stop (SIGKILL)
sigma-linux status ubuntu:24.04     # CPU/RAM/IO stats

# Execution:
sigma-linux exec ubuntu:24.04             # interactive shell
sigma-linux exec ubuntu:24.04 -- <cmd>   # run single command
sigma-linux exec ubuntu:24.04 --as root -- apt update
sigma-linux exec ubuntu:24.04 --env HOME=/root -- vim

# File system:
sigma-linux mount ubuntu:24.04 /sigma/data /mnt/sigma
sigma-linux cp ubuntu:24.04 /etc/hosts ./ubuntu-hosts
sigma-linux cp ./myfile.py ubuntu:24.04 /home/user/myfile.py

# Networking:
sigma-linux exec ubuntu:24.04 --network none -- ...  # air-gap
sigma-linux exec ubuntu:24.04 --network host -- ...  # share sigma-net
sigma-linux exec ubuntu:24.04 --network bridge -- ...  # private veth

# Resource limits:
sigma-linux exec ubuntu:24.04 --cpus 2 --memory 4g -- make -j8
sigma-linux exec ubuntu:24.04 --disk-quota 20g -- apt install llvm

# India Stack integration:
sigma-linux exec ubuntu:24.04 --india-stack -- python3 gst_script.py
# Inside Ubuntu: sigma-ca, sigma-gst, sigma-upi all accessible

# Security profiles:
sigma-linux exec kali:2026.1 --profile forensic -- nmap -sV target
sigma-linux exec ubuntu:24.04 --profile developer -- code .
sigma-linux exec ubuntu:24.04 --profile sandbox -- ./untrusted-app

# Export/import:
sigma-linux export ubuntu:24.04 > ubuntu-custom.sigma-img
sigma-linux import ubuntu-custom.sigma-img as my-ubuntu
```

---

## Security Model for Linux Apps

```
Every Linux distro runs inside sigma-mac policy:

Policy for ubuntu:24.04 (developer profile):
  sigma.cap.fs.read          ✓  can read SigmaOS VFS (explicit paths)
  sigma.cap.fs.write         ✓  can write to ~/sigma-linux/ only
  sigma.cap.net.tcp          ✓  can open TCP connections
  sigma.cap.display          ✓  can render windows in Zenith
  sigma.cap.audio            ✓  can play audio via sigma-audio
  sigma.cap.india.gstn       ✗  NO — Linux app cannot call GSTN directly
  sigma.cap.india.abdm       ✗  NO — sensitive APIs blocked by default
  sigma.cap.crypto.keys      ✗  NO — cannot access sigma-trustd DID keys

Policy for kali:2026.1 (forensic profile):
  sigma.cap.net.capture      ✓  can capture packets (forensic cap)
  sigma.cap.fs.read          ✓  read-only mount of target filesystem
  sigma.cap.fs.write         ✗  NO writes (forensic integrity)
  sigma.cap.india.*          ✗  NO India Stack (not needed in forensic)

Policy for ubuntu:24.04 (sandbox profile):
  sigma.cap.net.*            ✗  NO network (air-gapped sandbox)
  sigma.cap.fs.write         ✗  NO writes outside /tmp
  sigma.cap.display          ✓  can show windows
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| sigma-linux policy templates | `kernel/security/sigma_linux_policies/` | `tools-dev` | developer/forensic/sandbox/india-stack profiles |
| PQC-TLS enforcement for Linux net | `net/tls/sigma_tls.cpp` | `drivers-dev` | All TCP from Linux namespace tunnelled via sigma-tls |
| India Stack bridge (opt-in) | `userland/tools/sigma_linux_cli.cpp` | `release/standalone` | `--india-stack` flag exposes sigma-bus India topics |
| DID identity forwarding | `security/SovereignDID.cpp` | `release/standalone` | `$SIGMA_DID` env var inside Linux namespace |
| Audit log for Linux syscalls | `runtime/containers/sigma_linux_compat.cpp` | all | Every translated syscall → sigma-audit |
| dm-verity on all rootfs images | `userland/sigma-pkg/sigma_pkg_cli.cpp` | `tools-dev` | Verify rootfs hash before mount |

---

## Compatible Software Matrix

After Phase L2 (Month 12), these should all work via sigma-linux:

| Application | Distro | Use case | Status target |
|-------------|--------|---------|--------------|
| `apt install vim` | Ubuntu | Text editor | Phase L1 |
| `pip install pandas` | Ubuntu | Data science | Phase L1 |
| `npm install` | Ubuntu | Node.js dev | Phase L1 |
| `gcc -o hello hello.c` | Ubuntu | C compilation | Phase L1 |
| `git clone <repo>` | Ubuntu | Version control | Phase L1 |
| `docker run hello-world` | Ubuntu | Docker-in-sigma | Phase L2 |
| LibreOffice Writer | Ubuntu | Office suite | Phase L3 |
| VSCode (code) | Ubuntu | IDE | Phase L3 |
| GIMP | Ubuntu | Image editing | Phase L3 |
| Firefox | Ubuntu | Browser | Phase L3 |
| `nmap -sV target` | Kali | Security audit | Phase L2 |
| `metasploit` | Kali | Pen testing | Phase L2 |
| `burpsuite` | Kali | Web security | Phase L3 |
| `pacman -S neovim` | Arch | Rolling release | Phase L2 |
| `nix-env -i git` | NixOS | Reproducible env | Phase L2 |
| Python ML (TensorFlow) | Ubuntu | AI/ML | Phase L2 |
| `systemd-nspawn` | Ubuntu | Nested containers | Phase L3 |

---

## Distro Image Build Pipeline

```bash
# How sigma-linux rootfs images are built and verified:

# 1. Build minimal rootfs (automated in CI):
./tools/sigma_rootfs_builder.sh ubuntu 24.04
# → debootstrap --variant=minbase jammy /tmp/ubuntu-rootfs
# → Removes: systemd (replaced by sigma-pod init), d-bus
# → Installs: sigma-linux-init (minimal PID 1 stub)
# → Compresses: squashfs → ubuntu-24.04.squashfs
# → Signs: ML-DSA-87 by sigma-team key
# → Packages: ubuntu-24.04.spkg (meta + squashfs + sig)

# 2. Verify before install:
sigma-pkg install ubuntu:24.04
# → Downloads ubuntu-24.04.spkg from packages.sigmaos.dev
# → pqc_verify(sha256(squashfs), sig, sigma-team.pub)
# → dm-verity hash tree computed
# → Stored at /sigma/linux-images/ubuntu-24.04/

# 3. Mount on exec:
sigma-linux exec ubuntu:24.04
# → dm-verity readonly mount of squashfs
# → Overlay filesystem: writes go to /sigma/linux-homes/ubuntu-24.04/
# → PID namespace: sigma-linux-init (PID 1) spawns bash
```

| Task | File | Branch | Detail |
|------|------|--------|--------|
| `sigma_rootfs_builder.sh` | `tools/sigma_rootfs_builder.sh` | `tools-dev` | Build minimal rootfs for any distro |
| sigma-linux-init (PID 1 stub) | `runtime/containers/sigma_linux_init.cpp` | `tools-dev` | Minimal init for Linux namespace (no systemd) |
| OverlayFS for writable layer | `kernel/vfs/sigma_overlayfs.cpp` | `fs-dev` | Read-only rootfs + writable overlay per instance |
| Rootfs image CI build | `.github/workflows/sigma_linux_images.yml` | `tools-dev` | Weekly: build + sign + publish all distro images |
| Image size targets | `sigma_pkg_registry/linux-images/` | `tools-dev` | Ubuntu < 250MB, Alpine < 10MB, Kali < 500MB |

---

## Per-Branch Tasks for sigma-linux

| Branch | Task | Priority |
|--------|------|---------|
| `kernel-exp` | Linux signal routing (SIGTERM/SIGINT/SIGSEGV) | 🔴 |
| `kernel-exp` | `/proc/` and `/sys/` minimal stubs | 🔴 |
| `kernel-exp` | `clone(2)` with namespace flags | 🔴 |
| `fs-dev` | OverlayFS for rootfs writable layer | 🔴 |
| `fs-dev` | Bind-mount VFS path into Linux namespace | 🟠 |
| `tools-dev` | Expand syscall table to 100 (Priority 1+2) | 🔴 |
| `tools-dev` | sigma-linux CLI (install/exec/list/stop) | 🔴 |
| `tools-dev` | sigma-linux-init (PID 1 stub) | 🔴 |
| `tools-dev` | sigma_rootfs_builder.sh | 🟠 |
| `tools-dev` | Ubuntu 24.04 .spkg image | 🟠 |
| `drivers-dev` | veth network pair for Linux namespace | 🟠 |
| `drivers-dev` | `/dev/` device nodes (null/zero/urandom/tty) | 🟠 |
| `release/standalone` | X11 bridge → Zenith compositor | 🟡 |
| `release/standalone` | Clipboard sync Linux ↔ SigmaOS | 🟡 |
| `release/standalone` | sigma-terminal (GPU-accelerated term) | 🟠 |
| `release/cloud` | sigma-linux in cloud profile (headless) | 🟡 |

---

## Summary

sigma-linux makes SigmaOS the platform of platforms:

```
                     SigmaOS
                        │
         ┌──────────────┼──────────────┐
         │              │              │
     sigma-pod      sigma-pod      sigma-pod
    ubuntu:24.04   fedora:41       kali:2026.1
         │              │              │
   Full Ubuntu    Full Fedora     Full Kali
   environment   environment    environment
   apt/dpkg       dnf/rpm        apt/dpkg
   All Ubuntu    All Fedora     All Kali tools
   packages      packages       (nmap, metasploit)
         │              │              │
         └──────────────┼──────────────┘
                        │
              All share: sigma-bus IPC
                         sigma-net PQC-TLS
                         sigma-audit trail
                         sigma-mac policy
                         DID identity
                         India Stack APIs
```

*See also: [Windows Compatibility Layer Roadmap](Windows-Compatibility-Layer-Roadmap) · [Branch Development Roadmap](Branch-Development-Roadmap) · [Production Readiness Roadmap](Production-Readiness-Roadmap) · [Modularisation Architecture Roadmap](Modularisation-Architecture-Roadmap)*
## Getting Started
- [Home](Home)
- [Building from Source](Building-from-Source)
- [FAQ](FAQ)
- [Branch Guide](Branch-Guide)
- [Contributor Roadmap](Contributor-Roadmap)
- [Improvements Overview](Improvements-Overview)
- [Development Roadmap](Development-Roadmap)
- [Version Timeline](Version-Timeline)
- [Phase A Execution Checklist](Phase-A-Execution-Checklist)
- [Branch Development Roadmap](Branch-Development-Roadmap)
- [Feature Branch Roadmap](Feature-Branch-Roadmap)
- [CLI Commands Roadmap](CLI-Commands-Roadmap)
- [Quality, Stability & Performance](Quality-Stability-Performance-Roadmap)
- [Stability & Performance Extended](Stability-Performance-Extended)
- [Compatibility, Automation & Personalisation](Compatibility-Automation-Personalisation-Roadmap)
- [Advanced Quality Roadmap](Advanced-Quality-Roadmap)
- [Systems Excellence Roadmap](Systems-Excellence-Roadmap)
- [Engineering Principles Roadmap](Engineering-Principles-Roadmap)
- [Modularisation Architecture Roadmap](Modularisation-Architecture-Roadmap)
- [Sovereignty & User-Defined Roadmap](Sovereignty-UserDefined-Roadmap)
- [Continuous Improvement Roadmap](Continuous-Improvement-Roadmap)
- [Final Excellence Roadmap](Final-Excellence-Roadmap)
- [Production Readiness Roadmap](Production-Readiness-Roadmap)
- [Linux as Application Roadmap](Linux-As-Application-Roadmap)
- [Feature Roadmap](Feature-Roadmap)
- [Utilities Roadmap](Utilities-Roadmap)
- [Gap Analysis](Gap-Analysis)
- [Future Development Ideas](Future-Development-Ideas)

## Architecture
- [Architecture Overview](Architecture-Overview)
- [Kernel Architecture](Kernel)
- [HAL](HAL)
- [Networking Stack](Networking)
- [System Daemons](System-Daemons)
- [Syscall Dispatcher](Syscall-Dispatcher)
- [OS Technical Superiority](OS-Technical-Superiority)
- [Competitive Gap Matrix](Competitive-Gap-Matrix)
- [Zenith System Improvement Plan](Zenith-System-Improvement-Plan)
- [System Improvement Plan v15](System-Improvement-Plan)
- [Differentiation Blueprint](Differentiation-Blueprint)

## Security
- [Security Model](Security-Model)
- [Post-Quantum Security](Post-Quantum-Security)

## Testing
- [Testing Infrastructure](Testing-Infrastructure)

## API Reference
- [navigator.sigmaos API](API-Reference)

## Application Development
- [Writing Your First App](Your-First-App)
- [App Manifest Format](App-Manifest)
- [Developer Guide](Developer-Guide)
- [Extension System](Utilities-Roadmap#plugin--extension-system)
- [Browser Demo](Browser-Demo)

## Profiles & Deployment
- [Release Profiles](Release-Profiles)
- [Zenith Desktop](Zenith-Desktop)
- [Performance Architecture](Performance-Architecture)
- [Kiosk & Thin Client](Utilities-Roadmap#kiosk--thin-client-mode)

## India & Business
- [India Business Strategy](India-Business-Strategy)
- [SigmaOS Vision for India](SigmaOS-Vision-India)
- [Indian Compliance Roadmap](Indian-Compliance-Roadmap)
- [India Profession Coverage](India-Profession-Coverage)
- [India Profession Tools Roadmap](India-Profession-Tools-Roadmap)
- [Extended Profession Tools](Extended-Profession-Tools)
- [Advanced India Features](Advanced-India-Features)
- [SigmaOS vs Ubuntu](SigmaOS-vs-Ubuntu)
- [SigmaOS vs Linux Distros](SigmaOS-vs-Linux)
- [SigmaOS Crushing Linux](SigmaOS-Crushing-Linux)
- [Windows Parity Roadmap](Windows-Parity-Roadmap)
- [Windows Compatibility Layer](Windows-Compatibility-Layer-Roadmap)

## Autonomous Systems
- [Sigma Self-Heal](Sigma-Self-Heal)
- [Sigma CommNet](Sigma-CommNet)

## Driver Development
- [Driver Development Guide](Driver-Development)

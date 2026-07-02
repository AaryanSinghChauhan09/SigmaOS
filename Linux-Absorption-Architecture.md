# Linux Absorption Architecture

> SigmaOS doesn't compete with Linux distros — it absorbs them.
> Every Linux app, package, and workflow runs on SigmaOS, plus you get AI-native automation.

---

## Strategy: Embrace + Surpass

SigmaOS is not a Linux distro. It is a sovereign OS that absorbs Linux workloads through compatibility layers while offering superior AI-native, automation-first capabilities.

```
Linux App/Binary
       │
       ▼
SigmaOS Linux Compat Layer
  ├── ELF Loader (sigma-compat-loader)
  ├── Syscall Translation (30+ syscalls mapped)
  ├── FHS Path Mapping (/usr/lib → /sigma/lib, etc.)
  ├── Dynamic Linker (ld-sigma-linux.so)
  └── Signal/Process compatibility

       │
       ▼
SigmaOS Kernel (sovereign, no Linux code)
  ├── sigma_pledge / sigma_unveil (security)
  ├── Native syscall interface
  └── SDF driver framework
```

---

## Package Absorption

Convert any Linux package to .sigpkg format:

```bash
# .deb (Debian/Ubuntu)
sigma-pkg absorb firefox.deb
sigma-pkg absorb code.deb

# .rpm (Red Hat/Fedora)
sigma-pkg absorb vlc.rpm

# AppImage (portable Linux apps)
sigma-pkg absorb Blender-4.0.AppImage

# Flatpak
sigma-pkg install --flatpak org.mozilla.firefox

# Then install the absorbed package
sigma-pkg install ~/.cache/sigma/absorbed/firefox-*.sigpkg
```

Or via sigma-agent (auto-detects format):
```bash
sigma-agent "install firefox.deb"   # automatically absorbs + installs
sigma-agent "run ubuntu container"  # runs OCI container
```

### Supported Input Formats

| Format | Tool Required | Status |
|---|---|---|
| `.deb` (Debian/Ubuntu) | `dpkg-deb` | ✅ Implemented |
| `.rpm` (Red Hat/Fedora) | `rpm2cpio` | ✅ Implemented |
| `.AppImage` | None (built-in) | ✅ Implemented |
| `.flatpak` / `.flatpakref` | `flatpak` | 🔄 Via flatpak compat |
| `.snap` | `snap` | 🔄 Via snap compat |
| `.pkg.tar.zst` (Arch) | `pacman` | 🔄 Planned |
| OCI/Docker image | `sigma-pod` | ✅ Via sigma-pod |

### Dependency Mapping

Linux package dependencies are automatically mapped to SigmaOS equivalents:

| Linux Dep | SigmaOS Equivalent |
|---|---|
| `libc6` | `sigma-libc` |
| `libssl3` | `sigma-tls` |
| `libgtk-3-0` | `sigma-gtk3-compat` |
| `python3` | `sigma-python3` |
| `nodejs` | `sigma-node` |
| `libvulkan1` | `sigma-vulkan` |

---

## Linux Binary Compatibility

Run unmodified Linux ELF binaries directly:

```bash
# Run any Linux binary
sigma-compat run /path/to/linux-binary --args

# Check compatibility status
sigma-compat status

# Translate a Linux path to SigmaOS path
sigma-compat path /usr/lib/x86_64-linux-gnu
# → /sigma/lib
```

### FHS Path Mapping (24 paths)

SigmaOS maps all standard Linux Filesystem Hierarchy Standard paths:

```
/usr/lib          → /sigma/lib
/usr/bin          → /sigma/bin
/usr/share        → /sigma/share
/etc/apt          → /sigma/compat/apt
/lib/x86_64-linux-gnu → /sigma/lib
/proc             → /sigma/proc
/sys              → /sigma/sys
... (24 total)
```

### Syscall Translation

30 core Linux x86_64 syscalls are translated to SigmaOS equivalents at runtime. The translation layer (`kernel/compat/`) handles:
- `read`, `write`, `open`, `close`, `stat`, `fstat`
- `mmap`, `mprotect`, `munmap`, `brk`
- `fork`, `vfork`, `execve`, `exit`, `wait4`, `kill`
- `getpid`, `getuid`, `getgid`, `uname`
- `gettimeofday`, `unlink`, `readlink`

---

## OCI/Docker Container Support

```bash
# Run any Docker/OCI image via sigma-pod
sigma-compat container ubuntu:22.04
sigma-compat container nginx:latest
sigma-compat container python:3.11 python3 -c "print('hello')"

# Or via sigma-agent
sigma-agent "run ubuntu container"
sigma-agent "start nginx in container"
```

sigma-pod is SigmaOS's sovereign OCI runtime — runc-compatible, using sigma_pledge/sigma_unveil for isolation instead of cgroups/namespaces fallbacks.

---

## What Linux Distros Have That SigmaOS Is Building

| Gap | Current Status | Roadmap |
|---|---|---|
| Hardware driver library | SDF framework + drivers for core hardware | Expand via Driver Porting Pipeline |
| Stable ABI | `kabi/` C-ABI layer defined | Formalize kabi stability guarantee |
| Package repository | sigma_pkg_registry with core packages | Community package contributions |
| Container ecosystem | sigma-pod (OCI-compatible) | Full Docker Compose support |
| Desktop environments | Zenith DE (in progress) | Phase G: native C++ compositor |
| Linux binary compat | Syscall translation + path mapping | Full gVisor-style compat layer |
| GPU drivers | sovereigngpu.rs + Mesa integration | Vendor driver absorption |

---

## Installation

```bash
# Install full Linux compatibility layer
sigma-pkg install sigma-compat-layer

# Install individual tools
sigma-pkg install dpkg-tools       # .deb absorption
sigma-pkg install rpm-tools        # .rpm absorption
sigma-pkg install sigma-pod        # OCI/Docker containers
sigma-pkg install flatpak          # Flatpak apps
sigma-pkg install bubblewrap       # Sandbox for Linux apps
```

---

*See also: [Migration Guide](Migration-Guide) · [sigma-pkg](SIGMA_PKG) · [Architecture Overview](Architecture-Overview)*

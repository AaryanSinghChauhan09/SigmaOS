# Linux Absorption Architecture

SigmaOS absorbs Linux workloads through three progressively deeper layers,
giving users immediate compatibility and a clear path to native performance.

---

## Overview

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Layer       │  Mechanism          │  Status    │ Overhead
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  1. MicroVM  │  QEMU microVM+OCI   │  ✅ Now     │  ~5-15%
  2. Compat   │  Kernel syscall     │  🔄 Phase B │  ~5-10%
              │  translation        │            │
  3. Native   │  Sigma-native       │  ⬜ Phase D │  ~0%
              │  recompile/port     │            │
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Layer 1: MicroVM + OCI (Available Now)

Run any Docker/OCI image inside a lightweight QEMU microVM:

```bash

# Run Ubuntu, Alpine, Nginx, etc.

sigma-compat run ubuntu:22.04 bash
sigma-compat run nginx:latest
sigma-compat run python:3.12 python3 -c "print('hello from linux')"
```

### How it works:

1. Pull OCI image via Docker

2. Extract rootfs

3. Boot minimal Linux kernel in QEMU microVM

4. Mount rootfs as VirtIO-blk device

5. Start container process inside VM

**Security:** Linux kernel is inside the VM — SigmaOS kernel is not exposed.

**Supported images:** Any OCI image that runs on Linux x86-64.

---

## Layer 2: Syscall Translation (Phase B)

Run Linux ELF binaries directly on the SigmaOS kernel via syscall translation.

**ELF Loader** (`kernel/linux_compat/elf_loader.rs`):

- Parses Linux ELF64 format

- Maps segments with ASLR randomization

- Sets up Linux-ABI initial stack (argc/argv/envp/auxv)

- W^X enforcement (stricter than Linux)

**vDSO Shim** (`kernel/linux_compat/vdso_shim.rs`):

- `__vdso_clock_gettime` → `sigma_clock_ns()`

- `__vdso_gettimeofday` → FILETIME conversion

- `__vdso_time` → Unix seconds

**/proc Shim** (`kernel/linux_compat/proc_shim.rs`):

- `/proc/cpuinfo` → synthesized CPU info

- `/proc/meminfo` → `sigma_mm_free_pages()` converted

- `/proc/self/maps` → VMA list

- `/dev/urandom` → sigma PRNG

- `/sys/class/net/` → sigma-bus NIC channels

**Syscall Dispatch** (`kernel/core/syscall_dispatch.rs`):

- 50+ Linux syscall numbers mapped to SigmaOS primitives

- Custom `SYS_SIGMA_*` extensions for pledge/unveil/attestation

---

## Layer 3: Native Port (Phase D)

For maximum performance, applications are rebuilt against SigmaOS APIs:

```bash

# sigma-migrate analyses and suggests porting changes

sigma-migrate --analyze /path/to/linux-app

# Output:

#   Detected: 23 Linux-specific syscalls

#   Required: open(), read(), write(), socket(), futex()

#   Suggested: Replace Linux errno.h with sigma_errno.h

#   Effort: 2-4 days

# AI-assisted migration

sigma-agent "port this C application to SigmaOS: $(cat myapp.c)"
```

---

## Package Absorption

Install packages from Linux distros using the compatibility layer:

```bash

# Install a Debian package in sigma-compat mode

sigma-pkg absorb-deb nginx_1.24.0_amd64.deb

# Install from Ubuntu PPA (runs in microVM)

sigma-compat apt-install nginx

# Convert RPM to sigpkg

sigma-pkg convert-rpm mypackage-1.0.x86_64.rpm

# Run Flatpak

sigma-compat flatpak run org.mozilla.firefox
```

---

## OCI / Docker Container Support

```bash

# Direct OCI support

sigma-pod run nginx:latest
sigma-pod run --port 8080:80 nginx:latest

# Docker Compose equivalent

sigma-pod compose up -f docker-compose.yml

# Kubernetes (Phase E)

sigma-pod kube apply -f deployment.yaml
```

---

## Compatibility Matrix

| Category | Coverage | Status |
|----------|---------|--------|
| x86-64 ELF binaries | Growing | 🔄 Phase B |
| OCI containers | Full (via microVM) | ✅ Now |
| Debian .deb packages | Growing | 🔄 Phase B |
| RPM packages | Partial | ⬜ Phase C |
| Flatpak | Via microVM | 🔄 |
| Snap | Via microVM | 🔄 |
| Python packages (pip) | Full (native Python) | ✅ |
| Node.js packages (npm) | Full (native Node) | ✅ |
| Rust crates | Full (native) | ✅ |

---

*See also: [OCI Container Runtime](OCI-Container-Runtime) · [Linux Driver Compat](Linux-Driver-Compat) · [Linux Compat RFC](../docs/LINUX_COMPAT_RFC.md)*

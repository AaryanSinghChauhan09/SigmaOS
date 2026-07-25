# Distro Absorption: Alpine Linux

> **Status**: 📋 Planned | **Source Paradigm**: Alpine Linux | **Target Shard**: `SigmaOS Container Base & Minimal Profile`

---

## 1. Executive Summary

Alpine Linux is the de facto standard base image for Docker containers. Its entire root filesystem is under 5MB because it uses **musl libc**, **BusyBox** for core utilities, and **APK** (Alpine Package Keeper) for package management. It prioritizes security through a hardened kernel and Position Independent Executables (PIE).

SigmaOS absorbs Alpine's **minimal container base** philosophy and **APK-style atomic package operations** to create an ultra-lean `PROFILE=container` target image.

---

## 2. Key Features to Absorb

### 2.1 Sub-5MB Base Image

SigmaOS `PROFILE=container` produces a minimal OCI-compatible container image containing only the sigma-kernel runtime stub, sigma-init, and sigma-sh — no glibc, no systemd, no coreutils bloat.

```bash
$ sigma build --profile container
Σ [BUILD] Container image built:
  Size:   3.8 MB (compressed)
  Base:   musl 1.2.5 + sigma-busybox
  Init:   sigma-init (runit-style)
  Shell:  sigma-sh
```

### 2.2 APK-Style Atomic Package Operations

Inspired by Alpine's APK, `sigma-pkg` operations in container mode are fully atomic — either all packages install successfully, or the entire transaction rolls back with zero side effects.

```bash
$ sigma-pkg add --no-cache helix ripgrep fd
Σ [PKG] Fetching 3 packages (1.2 MB total)...
Σ [PKG] Verifying Dilithium5 signatures... OK
Σ [PKG] Installing atomically...
Σ [PKG] Transaction committed. 3 packages installed.
```

### 2.3 Hardened Defaults (PIE + Stack Protector)

All SigmaOS binaries in container mode are compiled as Position Independent Executables with full RELRO, stack canaries, and fortify-source enabled by default.

---

## 3. References & Standards

- Alpine Linux — `alpinelinux.org` (MIT)
- APK Tools — `gitlab.alpinelinux.org/alpine/apk-tools`

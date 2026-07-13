# Distro Absorption: Void Linux

> **Status**: 📋 Planned | **Source Paradigm**: Void Linux | **Target Shard**: `SigmaOS Minimal Init & Package Layer`

---

## 1. Executive Summary

Void Linux is an independent, rolling-release distribution built from scratch (not forked from any other distro). It is renowned for using **runit** as its init system (instead of systemd), its custom **XBPS** package manager, and its first-class **musl libc** support as an alternative to glibc.

SigmaOS absorbs Void's philosophy of **simplicity-first init management** and **musl-native userland** to offer an ultra-minimal system profile that boots in under 2 seconds with a tiny memory footprint.

---

## 2. Key Features to Absorb

### 2.1 runit-Inspired Init (`sigma-init`)

Instead of the complexity of systemd unit files, SigmaOS offers a runit-style init where each service is a directory containing a `run` script. The init system supervises each service and restarts it on crash.

```
/etc/sigma/services/
├── sshd/
│   └── run          # #!/bin/sh\nexec /usr/bin/sshd -D
├── sigma-gateway/
│   └── run          # #!/bin/sh\nexec sigma-gateway serve
└── sigma-agent/
    └── run          # #!/bin/sh\nexec sigma-agent daemon
```

```bash
$ sigma service status
Σ [INIT] Service supervision:
  sshd            UP (pid 312)  uptime 4h
  sigma-gateway   UP (pid 418)  uptime 4h
  sigma-agent     UP (pid 501)  uptime 3h59m
```

### 2.2 XBPS-Inspired Template Build System

Void's `xbps-src` uses simple shell templates to build packages. SigmaOS adapts this as `sigma-recipe`, where each package is defined by a short TOML recipe file.

```toml
# recipes/helix-editor.toml
[package]
name = "helix"
version = "24.7"
source = "https://github.com/helix-editor/helix/archive/24.07.tar.gz"
checksum = "blake3:a1b2c3d4..."

[build]
system = "cargo"
features = ["default"]
```

### 2.3 musl libc Support for Minimal Profile

SigmaOS `PROFILE=microkernel` targets musl libc instead of glibc, producing statically linked binaries with zero runtime dependencies, ideal for embedded and container deployments.

---

## 3. References & Standards

- Void Linux — `voidlinux.org`
- runit — `smarden.org/runit` (BSD)
- XBPS — `github.com/void-linux/xbps` (BSD-2-Clause)

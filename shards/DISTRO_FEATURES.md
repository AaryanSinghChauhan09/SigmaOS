# DISTRO FEATURES

> **Status**: Implemented
> **Language**: Nim (build system and configuration)
> **Priority**: Medium
> **Estimated Effort**: 10 hours (documentation + prototypes)

Distro features define the different deployment profiles and configuration options available for SigmaOS. These features enable SigmaOS to be deployed in various environments from embedded systems to cloud platforms.

## Deployment Profiles

### Standalone (Full Desktop)

**Target**: Desktop/laptop hardware

**Features**:
- Full desktop environment
- All drivers enabled
- Zenith compositor
- AI tools enabled
- Package manager included

**Output**: `sigmaos-standalone.iso`

### Microkernel

**Target**: Embedded systems, containers

**Features**:
- Minimal kernel only
- Core shards only
- No desktop environment
- Minimal footprint

**Output**: `sigmaos-microkernel.bin`

### RTOS (Real-Time)

**Target**: Industrial control, medical devices

**Features**:
- Hard real-time scheduler
- Deterministic timing
- Safety-critical features
- Minimal latency

**Output**: `sigmaos-rtos.elf`

### Cloud

**Target**: AWS, GCP, Azure deployments

**Features**:
- Headless image
- Cloud-init support
- SSH access
- Container support

**Output**: `sigmaos-cloud.img.qcow2`

### Mobile

**Target**: ARM64 Android/iOS devices

**Features**:
- Touch-optimized UI
- Mobile drivers
- Battery optimization
- App store integration

**Output**: `sigmaos-mobile.apk` or `.ipa`

### Browser

**Target**: Web browsers via WebAssembly

**Features**:
- WASM-compiled kernel
- Web UI
- Browser sandbox
- Limited filesystem

**Output**: `sigmaos-browser.wasm`

## Feature Flags

Feature flags control which components are included in the build:

```toml
[features]
desktop = true
ai_tools = true
networking = true
graphics = true
audio = true
```

## Build Configuration

### Build Profiles

```toml
[profile]
name = "standalone"
include_shards = ["s-mm", "s-sched", "s-net", "s-fs", "s-ipc", "s-sec", "s-sys"]
include_drivers = ["gpu", "network", "storage", "input", "audio"]
include_optional = ["zenith", "desktop", "llm", "pkg"]
```

### Cross-Compilation

Cross-compilation targets:

```toml
[targets]
x86_64 = true
aarch64 = true
riscv64 = true
```

## Package Management

### Package Repositories

SigmaOS uses multiple package repositories:

- **Core Repository**: Essential system packages
- **Community Repository**: Community-contributed packages
- **Enterprise Repository**: Enterprise packages

### Package Recipes

Package recipes define how to build packages:

```toml
[package]
name = "example-app"
version = "1.0.0"
source = "https://github.com/example/app"
build_system = "cargo"
dependencies = []
```

## Boot Configuration

### Boot Parameters

Kernel boot parameters:

```
kernel.smm.enable=true
kernel.sched.eevdf=true
kernel.net.firewall=deny
```

### Init System

Systemd-compatible init system:

```toml
[init]
systemd_compatible = true
services = ["network", "ssh", "desktop"]
```

## Implementation Status

| Feature | Documentation | Implementation | Status |
|---------|--------------|---------------|--------|
| Standalone Profile | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| Microkernel Profile | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| RTOS Profile | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| Cloud Profile | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| Mobile Profile | ✅ Complete | ⏳ Pending | ⏳ Not Started |
| Browser Profile | ✅ Complete | ⏳ Pending | ⏳ Not Started |

## Next Steps

1. Implement build configuration prototype (Nim)
2. Implement package manager prototype (Nim)
3. Implement init system prototype (Nim)

---

*Last Updated: 2026-07-13*

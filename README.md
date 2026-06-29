# Σ SigmaOS

> **A sovereign, polyglot, bare-metal operating system built for privacy-first computing.**

SigmaOS is an experimental microkernel OS written in Rust (kernel/drivers), Zig (HAL leaf modules), Ada/SPARK (proof-critical security primitives), and Nim (userland tooling). It targets x86_64, AArch64, and RISC-V 64GC and ships with the Zenith desktop compositor.

**Current status:** Active development — boots to login in QEMU x86_64.

---

## Quick Start

> Requirements: Rust (nightly), QEMU ≥8, cmake ≥3.27, `just`

```bash
# 1. Clone the repo
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# 2. Validate your declarative config
sigma config validate

# 3. Build the kernel + tools
just build

# 4. Launch in QEMU
just run

# 5. Build a bootable ISO
just iso
```

---

## What SigmaOS Is

| Principle | Implementation |
|-----------|----------------|
| **Sovereign** | Zero telemetry, user-controlled root-of-trust, air-gap ready |
| **Secure** | Capability-based IPC, PQC-TLS, pledge/unveil, SPARK-proven security modules |
| **Polyglot** | Rust kernel + Zig HAL + Ada/SPARK proofs + Nim userland — unified via `kabi/` C-ABI |
| **Modern Desktop** | Zenith compositor with full GPU pipeline, AI scheduler, VR-ready renderer |

---

## Repository Layout

```
kernel/        Microkernel (Rust — scheduler, allocator, IPC, VFS)
hal/           Hardware Abstraction Layer (Zig — x86_64, ARM64, RISC-V)
drivers/       Device drivers (Rust/Zig)
security/      Zero-Trust enforcer, capability matrix, AVC (Ada/SPARK + Rust)
net/           QUIC, TCP/IP stack, WireGuard, DoH resolver
desktop/       Zenith compositor window manager
graphics/      GPU HAL, Vulkan-like pipeline
init/          Init system, service manager
userland/      Shell, coreutils, standard library
kabi/          Unified C-ABI FFI boundary types (shared across all languages)
tools/         sigma CLI, sigma-trace profiler
sigma-web/     Personalisation Hub web panel
docs/wiki/     Architecture, CLI, CI, config reference docs
config/profiles/ Declarative TOML config presets (desktop/server/edge/embedded/airgapped)
```

---

## Documentation

| Doc | Purpose |
|-----|---------|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | System block diagram, privilege rings, subsystem map |
| [ROADMAP.md](./ROADMAP.md) | Phased development plan |
| [LANGUAGE_POLICY.md](./LANGUAGE_POLICY.md) | Language domains and FFI ABI rules |
| [sigma.toml](./sigma.toml) | Declarative build/runtime config schema |
| [docs/wiki/CLI-Reference.md](./docs/wiki/CLI-Reference.md) | Full `sigma` CLI reference |
| [docs/wiki/CI-Workflows.md](./docs/wiki/CI-Workflows.md) | GHA pipeline reference |
| [docs/wiki/Config-Reference.md](./docs/wiki/Config-Reference.md) | sigma.toml schema reference |

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for build prerequisites and the good-first-issue path.  
Run `sigma doctor` to verify your local environment before your first build.

---

## License

GPL-2.0-or-later — see [LICENSE](./LICENSE)

# SigmaOS Roadmap
**Single source of truth — all phases and milestones**

Status key: ✅ Done · 🔄 In Progress · 🎯 Planned · ❌ Blocked

---

## v0.1 — Foundation ("Genesis")
> **DoD:** Boots in QEMU x86_64 → framebuffer login → 3 userspace apps run

| Task | Status | Location |
|---|---|---|
| Multiboot2 boot on x86_64 | ✅ Done | `S01_Genesis/` |
| Physical memory manager (buddy) | ✅ Done | `memory/pmm/` |
| Virtual memory + paging | ✅ Done | `memory/vmm/` |
| Basic EEVDF scheduler | ✅ Done | `scheduling/` |
| Serial (COM1) + VGA output | ✅ Done | `drivers/` |
| Keyboard PS/2 driver | ✅ Done | `drivers/` |
| VFS + ramfs | ✅ Done | `fs/` |
| ELF userland loader | ✅ Done | `userland/` |
| sigma-shell (minimal) | ✅ Done | `userland/shell/` |
| QEMU smoke test in CI | ❌ Blocked | CI stabilization |
| Green CI on `main` | ❌ Blocked | Top priority |

---

## v0.2 — Stability & Contributor Funnel
> **Goal:** Green CI, reproducible builds, top-level docs, one canonical build entrypoint

| Task | Status | Location |
|---|---|---|
| `just` task runner | ✅ Done | `justfile` |
| `LANGUAGE_POLICY.md` | ✅ Done | root |
| `kabi/` unified C-ABI crate | ✅ Done | `kabi/` |
| `README.md` top-level | ✅ Done | root |
| `Architecture.md` | ✅ Done | root |
| `sigma.toml` declarative config | ✅ Done | root |
| `sigma config validate` CLI | ✅ Done | `tools/` |
| **sigma-sh v0.2 (full Rust)** | 🔄 In Progress | `sigma-sh/` |
| **sigpkg v0.2 (resolver + crypto)** | 🔄 In Progress | `userland/sigpkg/` |
| **Absorption Matrix** | 🔄 In Progress | `docs/Absorption-Matrix.md` |
| **Security Model doc** | 🔄 In Progress | `docs/Security-Model.md` |
| **Coding Standards wiki** | 🔄 In Progress | `docs/wiki/` |
| QEMU boot smoke test CI | 🎯 Planned | `.github/workflows/` |
| Required status check on `main` | 🎯 Planned | Repo Settings |

---

## v0.3 — Networking & Security Hardening
> **Goal:** Functional TCP/IP + DoH + WireGuard; SPARK proofs gate on merge

| Task | Status | Location |
|---|---|---|
| TCP/IP stack | ✅ Done | `net/` |
| QUIC transport | ✅ Done | `net/quic/` |
| WireGuard VPN (`sigma-vpn`) | 🎯 Planned | `net/vpn/` |
| DoH resolver | ✅ Done | `net/dns/` |
| TLS 1.3 + Kyber-1024 | ✅ Done | `crypto/` |
| sigma-shield packet filter | 🎯 Planned | `net/firewall/` |
| Zero-Trust AVC matrix | ✅ Done | `security/` |
| gnatprove CI gate | 🎯 Planned | Ada/SPARK modules |
| PQC attestation (cosign) | 🎯 Planned | Release signing |
| **Syscall audit framework** | 🎯 Planned | `kernel/security/audit/` |
| **Capability sandbox v1** | 🎯 Planned | `kernel/security/caps/` |
| **Hardened allocator** | 🎯 Planned | `kernel/memory/hardened/` |

---

## v0.4 — Desktop & GPU
> **Goal:** Zenith compositor boots, GPU acceleration, basic app model

| Task | Status | Location |
|---|---|---|
| Zenith compositor | ✅ Done | `desktop/`, `zenith_desktop.js` |
| GPU HAL (Vulkan-like) | ✅ Done | `graphics/` |
| AI scheduler integration | ✅ Done | `zenith_desktop.js` |
| Browser (sigma-browse) | ✅ Done | `browser/` |
| Package manager (sigpkg v0.1) | ✅ Done | `sigma-pkg/` |
| **sigpkg v0.2** (real resolver) | 🔄 In Progress | `userland/sigpkg/` |
| Sigma Store registry | 🎯 Planned | `sigma_pkg_registry/` |
| **USB/HID stack** | 🎯 Planned | `drivers/usb/` |
| **Audio subsystem** | 🎯 Planned | `drivers/audio/` |
| **NVMe driver** | 🎯 Planned | `drivers/storage/nvme/` |

---

## v0.5 — Kernel Observability & Fleet
> **Goal:** Live kernel metrics, syscall tracing, fleet node orchestration

| Task | Status | Location |
|---|---|---|
| Kernel metrics exporter | ✅ Done | `kernel/core/metrics.rs` |
| `sigma-trace` syscall profiler | ✅ Done | `tools/sigma-trace.rs` |
| Thermal + power HAL daemon | 🎯 Planned | `hal/thermal/` |
| cgroup-aware namespace accounting | 🎯 Planned | `security/cgroups/` |
| Auto-enroll + TPM attestation | 🎯 Planned | `sigmad/` |
| Policy-as-code GitOps rollout | 🎯 Planned | `sigma update` + GHA |
| Self-healing A/B rollback | 🎯 Planned | `sigma node rollback` |
| **SigmaVCS** (sovereign Git) | 🎯 Planned | `sigma-vcs/` |

---

## v1.0 — Sovereign Production
> **Goal:** Reproducible SLSA L3 ISO, air-gapped deployments, SPARK-proven security

| Task | Status | Location |
|---|---|---|
| Reproducible ISO builds (SLSA L3) | 🎯 Planned | `release.yml` |
| SBOM (CycloneDX) per release | ✅ Done | `release.yml` |
| IPFS + CDN mirror distribution | ✅ Done | `mirror_sync.yml` |
| Air-gapped sovereign profile | ✅ Done | `config/profiles/` |
| Full sigma CLI (16 subcommands) | ✅ Done | `tools/sigma-cli.rs` |
| CODEOWNERS per subsystem | 🎯 Planned | `.github/CODEOWNERS` |
| **sigma-core-utils (full Rust)** | 🎯 Planned | `userland/coreutils/` |
| **sigma-vault** credential manager | 🎯 Planned | `userland/vault/` |
| **SovereignFS** (journaling FS) | 🎯 Planned | `fs/sovereign/` |
| **sigma-sandbox** (CAP-based) | 🎯 Planned | `security/sandbox/` |
| v1.0 tag + release announcement | 🎯 Planned | — |

---

## Absorption Roadmap

See [Absorption-Matrix](Absorption-Matrix.md) for the full map of 70+ external tools → SigmaOS sovereign replacements.

**Current absorption priorities:**
1. `sigma-core-utils` (GNU Coreutils → Rust)
2. `sigma-sh` v0.2 (bash/zsh → full Rust shell)
3. `sigpkg` v0.2 (apt/pacman → Rust pkg manager)
4. `SovereignFS` (ext4/btrfs ideas → sovereign FS)
5. `sigma-ssh` (OpenSSH → Rust)
6. `SigmaVCS` (Git → sovereign VCS)

---

## Parked / Experiments Branch
- Windows-parity syscall compatibility layer
- NPU/TPU HAL (neural hardware acceleration)
- VR/XR renderer
- Multi-tenant cloud orchestrator
- Android-compatible mobile profile

---

*[Back to Wiki Home](Home.md) | [Absorption Matrix](Absorption-Matrix.md) | [Contributing](Contributing.md)*

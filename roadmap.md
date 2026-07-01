# SigmaOS Roadmap

Consolidation of all prior phase documents into a single source of truth.  
Status key: 🔴 Blocked · 🟡 In Progress · 🟢 Done · ⚪ Planned

---

## v0.1 — Foundation ("Genesis")
> **Definition of done**: Boots in QEMU x86_64 → framebuffer login → 3 userspace apps run.

| Task | Status | Notes |
|------|--------|-------|
| Multiboot2 boot on x86_64 | 🟢 | `S01_Genesis/` |
| Physical memory manager (buddy) | 🟢 | `memory/pmm/` |
| Virtual memory + paging | 🟢 | `memory/vmm/` |
| Basic EEVDF scheduler | 🟢 | `scheduling/` |
| Serial (COM1) + VGA output | 🟢 | `drivers/` |
| Keyboard PS/2 driver | 🟢 | `drivers/` |
| VFS + ramfs | 🟡 | `fs/` |
| ELF userland loader | 🟡 | `userland/` |
| sigma-shell (minimal sh) | 🟡 | `userland/shell/` |
| QEMU smoke test in CI | 🔴 | See CI stabilization |
| Green CI on `main` | 🔴 | Top priority |

---

## v0.2 — Stability & Contributor Funnel
> **Goal**: Green CI, reproducible builds, top-level docs, one canonical build entrypoint.

| Task | Status | Notes |
|------|--------|-------|
| `just` task runner | 🟢 | `justfile` |
| `LANGUAGE_POLICY.md` | 🟢 | FFI rules, domain map |
| `kabi/` unified C-ABI crate | 🟢 | `kabi/src/lib.rs` |
| `README.md` top-level | 🟢 | Quick-start + layout |
| `ARCHITECTURE.md` | 🟢 | Block diagram + rings |
| Build artifact gitignore | 🟢 | `dist/`, `build/`, `target/` |
| Pinned toolchain devcontainer | 🟡 | `.devcontainer/` |
| `sigma.toml` declarative config | 🟢 | 5 profile presets |
| `sigma config validate` CLI | 🟢 | `tools/sigma-cli.rs` |
| QEMU boot smoke test CI | ⚪ | Target: v0.2.1 |
| Required status check on `main` | ⚪ | Repo Settings |

---

## v0.3 — Networking & Security Hardening
> **Goal**: Functional TCP/IP + DoH + WireGuard; SPARK proofs gate on merge.

| Task | Status | Notes |
|------|--------|-------|
| TCP/IP stack | 🟡 | `net/` |
| QUIC transport | 🟡 | `net/quic/` |
| WireGuard VPN | ⚪ | `net/vpn/` |
| DoH resolver | 🟡 | `net/dns/` |
| TLS 1.3 + Kyber-1024 | 🟡 | `crypto/` |
| sigma-shield packet filter | ⚪ | `net/firewall/` |
| Zero-Trust AVC matrix | 🟡 | `security/` |
| gnatprove CI gate | ⚪ | Ada/SPARK modules |
| PQC attestation (cosign) | ⚪ | Release signing |

---

## v0.4 — Desktop & GPU
> **Goal**: Zenith compositor boots, GPU acceleration, basic app model.

| Task | Status | Notes |
|------|--------|-------|
| Zenith compositor | 🟡 | `desktop/`, `zenith_desktop.js` |
| GPU HAL (Vulkan-like) | 🟡 | `graphics/` |
| AI scheduler integration | 🟡 | `zenith_desktop.js` |
| Browser (sigma-browse) | 🟡 | `browser/` |
| Package manager (sigma-pkg) | 🟡 | `sigma-pkg/` |
| Sigma Store registry | ⚪ | `sigma_pkg_registry/` |

---

## v0.5 — Kernel Observability & Fleet
> **Goal**: Live kernel metrics, syscall tracing, fleet node orchestration.

| Task | Status | Notes |
|------|--------|-------|
| Kernel metrics exporter (`/sigma/metrics`) | 🟢 | `kernel/core/metrics.rs` |
| `sigma-trace` syscall latency profiler | 🟢 | `tools/sigma-trace.rs` |
| Thermal + power HAL daemon | ⚪ | `hal/thermal/` |
| cgroup-aware namespace accounting | ⚪ | `security/cgroups/` |
| Auto-enroll + TPM attestation | ⚪ | `sigmad/` |
| Policy-as-code GitOps rollout | ⚪ | `sigma update` + GHA |
| Self-healing A/B rollback | ⚪ | `sigma node rollback` |

---

## v1.0 — Sovereign Production
> **Goal**: Reproducible SLSA L3 ISO, air-gapped deployments, SPARK-proven security layer.

| Task | Status | Notes |
|------|--------|-------|
| Reproducible ISO builds (SLSA L3) | ⚪ | `release.yml` |
| SBOM (CycloneDX) per release | 🟡 | `release.yml` |
| IPFS + CDN mirror distribution | 🟢 | `mirror_sync.yml` |
| Air-gapped sovereign profile | 🟢 | `config/profiles/airgapped.toml` |
| Personalisation Hub web panel | 🟢 | `sigma-web/personalisation/` |
| Full sigma CLI (16 subcommands) | 🟢 | `tools/sigma-cli.rs` |
| CODEOWNERS per subsystem | ⚪ | `.github/CODEOWNERS` |
| v1.0 tag + release announcement | ⚪ | — |

---

## Parked / Experiments Branch
These items are real goals but deliberately off the critical path until v0.2 is stable:
- Windows-parity syscall compatibility layer
- NPU/TPU HAL (neural hardware acceleration)
- VR/XR renderer
- Sigma Browser full engine
- Multi-tenant cloud orchestrator

See `experiments/` branch for active prototyping.

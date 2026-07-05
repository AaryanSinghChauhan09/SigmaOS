# SigmaOS Development Roadmap

## Executive Summary

Win by being measurably better where Linux distros struggle: smaller trusted TCB, cryptographically verifiable supply chain, faster boots & lower resource use for common profiles, easier deterministic packaging and updates, and a curated app experience (signed, sandboxed WASM apps + native apps).

## Strategic Pillars

- **Trust & supply chain**: deterministic builds, artifact signing, secure updates, measured/verified boot
- **Hardware & compatibility**: broad driver coverage (NICs, storage, GPU, Wi‑Fi), multi‑arch CI (x86_64, aarch64, riscv64)
- **Performance & reliability**: low boot time, low idle RAM, real-time class scheduling, energy efficiency
- **App ecosystem & UX**: curated signed packages (sigpkg), WASM-first app sandbox, polished desktop (Zenith), migration tools for Linux apps
- **Developer & ops experience**: reproducible SDK, easy cross‑compile, source-level debug, benchmarks and CI badges
- **Enterprise & adoption**: LTS releases, update/rollback, monitoring, cloud images and orchestration

## Phase 0 — Stabilize Core & Trust (0–3 months)

### Goals
Bootable ISO in CI, reproducible builds, signed releases, baseline metrics.

### Key Deliverables

- **Kernel boot + QEMU CI**: wire qemu-boot.sh into .github/workflows to run nightly and on PRs (map → kernel-exp, .github)
- **Reproducible builds job**: CI job that produces bit-for-bit identical images across runners; record build provenance (map → build/, Dockerfile, toolchain-x86_64-elf.cmake)
- **Release signing & attestation**: integrate signtool and metadata publishing (map → RELEASE_NOTES.md, sigma.toml, release/*)
- **Makefile + justfile gates**: add make check-abi and make ci-check targets (Makefile, justfile)

### Exit Criteria / KPIs

- CI produces bootable ISO that boots in QEMU (qemu -cdrom SigmaOS.iso) automatically
- Reproducible build artifacts with signed manifest (proof) in GitHub Actions
- Baseline metrics captured: cold boot time, idle RAM

## Phase 1 — Hardware Parity & Filesystem (3–7 months)

### Goals
Match top Linux desktop/server hardware and provide robust storage/FS.

### Key Deliverables

- **Drivers**: implement VirtIO (virtio-net, virtio-blk, virtio-gpu), NVMe (drivers/storage/sigma_nvme.cpp already flagged), USB xHCI, e1000 already present — expand to Intel/AMD GPUs and common Wi‑Fi (map → drivers-dev)
- **Filesystem**: tmpfs, robust VFS, SigmaFS mkfs + mount, ext4 read-only mount and dm-verity support (fs-dev)
- **Installer**: robust dual-boot installer, partition detection, and automatic layout (installer.html, userland/installer)
- **Multi-arch images**: build aarch64 and riscv64 images in CI (map → toolchain files, rust-toolchain.toml)

### Exit Criteria / KPIs

- Boot and run graphical Zenith demo in QEMU with virtio-gpu and VirtIO storage]
- NVMe + ext4 read/write validated; SigmaFS mkfs works
- Driver coverage target: top 20 NICs and top 10 GPUs supported for basic functionality

## Phase 2 — App Ecosystem, Packaging & Sandboxing (6–12 months)

### Goals
Make app installation, security, and developer experience a competitive advantage.

### Key Deliverables

- **sigpkg package manager + registry**: signed package format, store, web UI (sigma-pkg, sigma_pkg_registry, app_store.html)
- **WASM runtime & WASI support**: sandboxed apps with capability limits (sigmad-sandbox, runtime/wasm)
- **Legacy portability**: POSIX compatibility layer and container runtime (sigma-pod), lightweight Linux syscall shim where useful (userland + runtime/compat)
- **App porting guides**: "easy ports" for top 100 packages (docs/)

### Exit Criteria / KPIs

- 1,000 packages available via sigpkg (or clear migration strategy to populate)
- WASM runtime runs common apps (web server, SQLite) and tests in CI
- Users can install and run a set of desktop apps via sigpkg + sandbox within 2 commands

## Phase 3 — Performance, Security Hardening & Enterprise Readiness (9–15 months)

### Goals
Outperform Linux in predictable performance, security, update model, and enterprise features.

### Key Deliverables

- **AI‑assisted scheduler / performance tuning**: bring performance-optimized work (NUMA, lock-free runqueues, predictive pre-warming) into main (performance-optimized branch)
- **Secure update mechanism**: staged, signed updates with rollback & atomic swap (sigma-pkg + kernel support)
- **Verified boot & measured boot**: (sigma-boot, secure boot integration)
- **LTS release process + SLA docs + CVE handling SOP**: (SECURITY.md, SECURITY_POLICY.md)
- **Telemetry & observability**: lightweight observability agent (sigma_observatory)

### Exit Criteria / KPIs

- Context switch latency targets and benchmark improvements met (benchmarks in docs/bench)
- Time-to-fix security advisories (goal: initial triage <24h, patch or mitigation <72h)
- Enterprise images (cloud, bare-metal) with signed updates and rollback

## Phase 4 — Adoption Scaling & Ecosystem Partnerships (12–24 months)

### Goals
Reach parity on app ecosystem adoption and provide migration/installation customer workflows.

### Key Deliverables

- **Partner programs, hardware certification playbook, driver sponsorship pipeline**: (Open_Source_Drivers.md)
- **Migration tools**: easy importer for user profiles, dotfiles, package lists from Debian/Ubuntu/Arch
- **Cloud marketplace images + orchestration support**: (release/cloud, sigma-fleet agent)
- **Community growth**: onboarding docs, "good-first-bug", contributor roadmap events (CONTRIBUTOR_ROADMAP.md)

### Exit Criteria / KPIs

- Adoption metrics: X active installs / month (set realistic baseline), Y enterprise customers (pilot)
- Satisfy niche parity checklist (Niche-Parity-Certification.md) for target workloads

## Tactical Projects (Actionable Items)

1. Wire QEMU multi‑arch matrix in CI (x86_64, aarch64, riscv64) and include kernel boot smoke tests (qemu-boot.sh → .github/workflows/sigma_ci.yml)
2. Add deterministic-build verification step and publish build provenance JSON for every release (build → signed artifacts)
3. Implement minimal POSIX compatibility layer + container runtime to let many Linux userland programs run in a sandboxed environment quickly (userland/runtime/compat)
4. Prioritize drivers that unlock desktop/desktop apps: virtio-gpu, VESA/GOP, input (keyboard/mouse), NVMe
5. Build sigpkg MVP + registry + web UI and sign the first 200 curated packages (sigma-pkg, sigma_pkg_registry, app_store.html)
6. Create benchmark suite (boot time, boot-to-desktop, memory footprint, context-switch) and publish CI badges

## Advanced Performance Targets & Capabilities

### Boot Performance
- **Cold boot to desktop**: <2s on NVMe, <3s on SATA SSD, <5s on HDD
- **Resume from suspend**: <500ms to unlock screen
- **Service startup**: <100ms for critical services (init, network, display)
- **Boot optimization**: parallel init, lazy loading, predictive pre-fetch

### Memory Efficiency
- **Idle memory (desktop)**: <150 MB with Zenith running
- **Idle memory (server)**: <64 MB headless
- **Memory overhead per process**: <2 MB base overhead
- **Zero-copy IPC**: Shared memory buffers for inter-process communication
- **Memory compression**: zswap-style compression for swap

### CPU Performance
- **Context switch latency**: <500ns (vs Linux ~1-2µs)
- **Scheduler latency**: <10µs for high-priority tasks
- **Interrupt latency**: <5µs for real-time class interrupts
- **Lock-free data structures**: Minimal kernel lock contention
- **NUMA-aware scheduling**: Optimize for multi-socket systems

### I/O Performance
- **NVMe sequential**: >3 GB/s read, >2 GB/s write
- **NVMe random 4K**: >500K IOPS read, >300K IOPS write
- **Network throughput**: Line-rate 10GbE with <10µs latency
- **Filesystem operations**: <10µs for metadata operations
- **Async I/O**: Native async/await for all I/O operations

### Security Performance
- **Cryptographic operations**: AES-NI acceleration, post-quantum crypto optimization
- **Secure boot**: <500ms verification time
- **Capability checks**: <100ns per permission check
- **Sandbox overhead**: <5% performance penalty for WASM apps

### Scalability
- **Multi-core scaling**: Near-linear scaling up to 64 cores
- **Concurrent connections**: 100K+ network connections per server
- **Process limit**: 1M+ processes (vs Linux ~32K default)
- **File descriptor limit**: 10M+ open files

## Metrics to Beat Linux (Suggested KPIs)

- **Boot time (cold) to desktop**: <2s for a "desktop profile" in QEMU on NVMe
- **Idle memory (desktop profile)**: <150 MB with Zenith running
- **Deterministic build rate**: 100% of official artifacts reproducible and signed
- **Package coverage**: 1,000 curated packages in sigpkg in year 1; migration scripts for common packages
- **Security SLA**: triage <24h, fix/mitigation <72h for critical CVEs
- **Driver parity**: support for top 20 NICs / top 10 GPUs for modesetting & basic acceleration
- **Context switch latency**: <500ns (vs Linux ~1-2µs)
- **Boot-to-shell**: <1s on NVMe, <2s on SSD
- **Service startup time**: <100ms average for critical services
- **Memory overhead**: <2 MB per process base overhead

## Repository Mapping

- **kernel-exp**: finish Phase 0 tasks (scheduler, MM, syscall dispatch, APIC) → enables nearly everything else
- **drivers-dev**: finish VirtIO/virtio-gpu, NVMe, USB xHCI, common Wi‑Fi — accelerates desktop and cloud
- **fs-dev**: VFS, tmpfs, SigmaFS + dm-verity for verified images
- **tools-dev**: sigma-cli, sigma-sh improvements, sigpkg tooling
- **sigma-boot**: UEFI+measured boot & signed loader
- **sigma-pkg / sigma_pkg_registry**: package manager + registry + app_store UI
- **qemu-boot.sh + .github/workflows**: CI improvements and reproducible build steps
- **sigmad-sandbox**: WASM runtime and app sandboxing

## Immediate Next Steps (Pick 3)

1. Wire QEMU multi‑arch boot + reproducible-build CI job (high impact, quick win)
2. Implement sigpkg MVP + registry + sign-first-packages (visibility + UX)
3. Finish kernel-exp Phase 0 boot path for deterministic ISO (foundation)

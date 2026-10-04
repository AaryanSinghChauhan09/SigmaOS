> Imported repository document from [`src/distro/AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/distro/AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# AI Agent Guidelines & Distro Parity Roadmap (`src/distro/AGENTS.md`)

This document guides AI agents working on the SigmaOS distribution (`src/distro/`) subsystem, providing an operational roadmap for closing functional and architectural gaps between SigmaOS and major Linux & BSD distributions.

---

## 1. Executive Summary & Core Mission

SigmaOS aims to synthesize the best innovations across the Linux & BSD open-source ecosystem into a unified, zero-dependency, safe Rust operating system.

When developing features in `src/distro/`, agents must adhere to the **Tri-Agent Framework** (Bolt, Palette, Sentinel) and maintain 100% test pass rates across all test suites in `./run_sigma_tests.sh`.

---

## 2. Missing Components & Development Roadmap

### Tier 1: Linux Distribution Innovations

1. **Systemd 256 / 258 Advanced Parity (`systemd-homed`, `systemd-vmspawn`, Varlink IPC)**
   - *Target Shard*: `src/distro/sovereign_distro_outpacing_engine.rs` & `src/init/systemd_init.rs`
   - *Roadmap Goal*: Full userland integration for LUKS2 encrypted home directories (`systemd-homed`), lightweight container/microVM instantiation (`systemd-vmspawn`), and Varlink JSON-based IPC interface.

2. **CachyOS / BORE Scheduler & AI Workload Auto-Tuning**
   - *Target Shard*: `src/distro/sovereign_distro_dominance.rs` & `src/kernel/`
   - *Roadmap Goal*: Dynamic burst-aware CPU timeslice calculations, P-Core/E-Core microarchitecture topology auto-pinning, and eBPF sched_ext BORE v2 scheduler policy governor.

3. **Vanilla OS ABRoot & APX Containerized Subsystems**
   - *Target Shard*: `src/distro/linux_bsd_distro_strategic_innovations.rs`
   - *Roadmap Goal*: Immutable atomic A/B rootfs image switching, cryptographic checksum verification before boot pivot, and unprivileged APX subsystem container execution.

4. **NixOS & GNU Guix Hermetic Content-Addressed Store (CAS)**
   - *Target Shard*: `src/distro/sovereign_nextgen_distro_leap.rs` & `src/sigpkg/`
   - *Roadmap Goal*: Reproducible NAR archive validation, zero-copy store hardlinking, generation history management, and atomic transactional rollbacks.

5. **Clear Linux Stateless Architecture (`/usr` Defaults vs. `/etc` Overrides)**
   - *Target Shard*: `src/distro/clear_linux.rs` & `src/system/structure_synthesis.rs`
   - *Roadmap Goal*: Factory-reset capability via `/usr/share/defaults` state isolation and user override resolution in `/etc`.

6. **Gentoo Portage EAPI 8 Slot Operators & USE Flag Solver**
   - *Target Shard*: `src/distro/gentoo.rs` & `src/distro/missing_distro_innovations.rs`
   - *Roadmap Goal*: Subslot rebuild trigger tracking, conditional dependency expressions, and global vs. package-level USE flag resolution.

---

### Tier 2: BSD Distribution Innovations

1. **FreeBSD 14 / 15 VNET Virtualized Network Stack & bhyve MicroVM Bridge**
   - *Target Shard*: `src/distro/sovereign_distro_dominance.rs` & `src/network/`
   - *Roadmap Goal*: Per-jail independent network stack routing tables, epair virtual interfaces, and bhyve PCIe passthrough / VirtIO device emulation.

2. **OpenBSD Pinned Syscalls (`pinsyscall`), Fine-Grained IBT, & Pledge/Unveil**
   - *Target Shard*: `src/distro/sovereign_2028_distro_supremacy_engine.rs` & `src/security/`
   - *Roadmap Goal*: Enforcement of registered executable code regions for syscall execution, Indirect Branch Tracking (IBT) callsite validation, and process capability locking.

3. **DragonFly BSD HAMMER2 Multi-Master PFS Transaction Replication**
   - *Target Shard*: `src/distro/sovereign_linux_bsd_distro_next_gen_innovations.rs`
   - *Roadmap Goal*: Clustering quorum consensus across HAMMER2 Pseudo Filesystems (PFS), automatic read-only failover upon network partition, and Emergency CoW deduplication.

4. **NetBSD Rump Kernel Userland Isolation & Veriexec Fingerprinting**
   - *Target Shard*: `src/distro/ultimate_distro_innovations.rs`
   - *Roadmap Goal*: Running kernel drivers in userland sandboxes via rump hypercalls, SHA-256/SHA-512 in-kernel executable fingerprint auditing, and Veriexec enforcement modes.

5. **Illumos / Solaris DTrace Dynamic Tracing & Crossbow VNIC Subsystem**
   - *Target Shard*: `src/distro/open_source_distro_innovations.rs` & `src/distro/linux_bsd_parity_extended.rs`
   - *Roadmap Goal*: Dynamic kernel and userland probe providers (FBT, SDT, Profile), aggregation functions, and virtual network interface (VNIC) etherstub routing.

---

## 3. Operational Rules for AI Agents

1. **Zero External Dependencies**: All algorithms, parsers, and data structures must be written in **safe Rust** without third-party crate additions.
2. **Subsystem Verification Matrix**: When adding or updating a distro subsystem mode in `DistroSubsystemMode`, ensure that `verify_all_subsystems_compatibility_matrix()` passes cleanly across all 174 tracked subsystems.
3. **Standalone Test Executability**: Every new distro module must include a `#[cfg(test)]` module and be compilable standalone using `rustc --test`.
4. **CI Integration**: Add the standalone test binary compilation and execution steps to `./run_sigma_tests.sh`.

---

## 4. Verification Commands

```bash
# Run standalone test for distro next-gen innovations
mkdir -p build
rustc --test src/distro/sovereign_linux_bsd_distro_next_gen_innovations.rs --edition=2021 -o build/test_next_gen_distro
./build/test_next_gen_distro

# Run full SigmaOS test suite
./run_sigma_tests.sh
```

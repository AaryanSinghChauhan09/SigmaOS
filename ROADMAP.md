# SigmaOS Contributor Roadmap

> **Goal**: Transform SigmaOS into a production-grade, community-driven sovereign OS.
> 
> **Contact**: aaryansinghchauhan090305@gmail.com

---

## ✅ Phase 1 — Foundation Hardening (COMPLETE)

The core sovereign lattice foundation is now finalized.
- **Native Toolchain**: Migrated from Python/Node to a purely native C++20 `s-cli` orchestrator.
- **Zero-Dependency Core**: Eradicated all standard headers (`stdint.h`, etc.) from kernel space.
- **CI/CD Hardened**: Fully stabilized matrix builds for x86_64, AArch64, and RISC-V.
- **Lattice Consolidation**: Root directories modularized into `modules/` and `suites/`.

---

## 🔴 Phase 2 — IPC, Persistence & Scripting (Active)

Focusing on the native communication layer and user-space personalization.

| # | Task | Skill | Files |
|---|------|-------|-------|
| 1 | **Native Lua Bridge**: Integrate a minimal Lua VM as a sovereign shard for personalization. | C, Lua | `modules/ext/scripting/` |
| 2 | **IPC Persistence**: Implement `ipc_recv` recovery from persistence logs. | C, Systems | `modules/core/kernel/ipc.c` |
| 3 | **Capability Registry**: Finalize the zero-trust capability handshake protocol. | C, Security | `modules/security/capabilities/` |
| 4 | **Sovereign Dashboard**: Port Zenith UI telemetry to direct kernel FFI calls. | JS, WASM | `web_ui/` |

---

## 🟡 Phase 2 — IPC & Persistence Correctness (Next Sprint)

| # | Task | Skill | Files |
|---|------|-------|-------|
| 6 | Implement `ipc_recv` recovery: replay from persistence log in seq_id order | C, Systems | `suites/S03_Orchestrator/sigma_ipc.c` |
| 7 | Implement `persistence_write_ffi` / `persistence_rollback_ffi` C → Rust bridge | C, Rust FFI | `suites/S03_Orchestrator/` |
| 8 | Add exponential backoff retry when persistence backend is unavailable | C, Systems | `sigma_ipc.c` |
| 9 | Wire CRDT `merge()` into `replicate()` FFI for actual cross-shard sync | Rust | `crdt_store.rs`, `crdt_lww.rs` |

---

## 🟢 Phase 3 — Silicon Backend (Hardware Sprint)

| # | Task | Skill | Files |
|---|------|-------|-------|
| 10 | Complete V3D command submission loop (`bcm_v3d_matmul`) | C, GPU drivers | `suites/S04_HAL/drivers/bcm_v3d_npu.c` |
| 11 | Add RISC-V PLIC interrupt controller driver | C, RISC-V ASM | `suites/S04_HAL/arch/riscv64/` |
| 12 | Implement real NPU inference workload (tiny CNN on V3D) | C, GPU compute | `suites/S09_Intelligence/` |
| 13 | Add MMU page table setup for AArch64 EL1 | C, ARM ASM | `suites/S04_HAL/arch/aarch64/` |

---

## 🔵 Phase 4 — Formal Verification Completion

| # | Task | Skill | Files |
|---|------|-------|-------|
| 14 | Complete Coq proof: DMA allocator preserves IPC isolation after alloc/free | Coq | `verification/coq/ipc_isolation.v` |
| 15 | Complete Isabelle proof: LWW register convergence with finite map model | Isabelle/HOL | `verification/isabelle/crdt_merge.thy` |
| 16 | Add Kani harness for `sigma_dma_alloc` alignment guarantee | Rust, Kani | `suites/S08_Security/formal_proofs/` |

---

## How to Claim an Issue

1. Comment on the relevant GitHub Issue: "I'm working on #X"
2. Branch: `git checkout -b fix/issue-X-description`
3. Follow `CONTRIBUTING.md` for code style and PR format
4. Open a draft PR early — we give feedback before the code is complete

**Questions?** Email [aaryansinghchauhan090305@gmail.com](mailto:aaryansinghchauhan090305@gmail.com)

# SigmaOS Contributor Roadmap

> **Goal**: Transform SigmaOS into a production-grade, community-driven sovereign OS.
> 
> **Contact**: aaryansinghchauhan090305@gmail.com

---

## 🔴 Phase 1 — Foundation Hardening (Active)

Issues that are blocking further hardware work. Pick these up first.

| # | Task | Skill | Files |
|---|------|-------|-------|
| 1 | Fix CI/CD — ensure all 3 jobs pass on ubuntu + macOS + windows | DevOps, Rust | `.github/workflows/ci.yml` |
| 2 | Generate `package-lock.json` by running `npm install` | Node.js | `package.json` |
| 3 | Add `Cargo.lock` to version control for reproducible Rust builds | Rust | `suites/S03_Orchestrator/` |
| 4 | Add `compile_commands.json` so `clang-tidy` can lint C files properly | C, CMake | `suites/S04_HAL/`, `suites/S07_Scheduling/` |
| 5 | Document C/ASM compiler versions in `CONTRIBUTING.md` | Docs | `CONTRIBUTING.md` |

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

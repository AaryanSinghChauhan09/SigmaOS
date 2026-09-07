# AGENTS.md — AI Agent Operating Instructions for SigmaOS

Welcome, AI Agent! This file provides essential context, coding standards, instructions, and verification commands for working with the **SigmaOS** repository.

---

## ⚡ Tri-Agent Roles & Responsibilities

1. **Bolt ⚡ (Performance & Speed Optimization)**
   - Hunt for bottlenecks, heap allocation overhead, $O(N^2)$ algorithm loops, and cache misses.
   - Implement clean, measurable performance optimizations (<50 lines) without sacrificing readability.
   - Log critical performance learnings in `.jules/bolt.md`.

2. **Palette 🎨 (UX, Ergonomics & Accessibility)**
   - Enhance CLI output, Web UI components, and desktop tools.
   - Ensure WCAG 2.1 AA compliance, visible focus indicators (`:focus-visible`), and explicit ARIA annotations (`role="tablist"`, `aria-label`).
   - Log critical UX learnings in `.jules/palette.md`.

3. **Sentinel 🛡️ (Security, PQC Integrity & Compliance)**
   - Protect memory safety, driver execution boundaries, PII data masking (GDPR/HIPAA), and Dilithium-5 post-quantum signature verifications.
   - Ensure mock test credentials use `mock_` or `test_` variable prefixes.
   - Log critical security learnings in `.jules/sentinel.md`.

---

## 🏎️ Subsystem Management Protocols for AI Agents

- **Driver Management**: Refer to `docs/AI_AGENT_DRIVER_MANAGEMENT.md` for driver lifecycle directives.
- **Cache Operation Management**: Refer to `docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md` for explicit CPU cache line flushing (`clflush`, `clflushopt`, `clwb`), TLB invalidation/shootdown, Page Cache Radix-Tree operations, SLUB object cache recycling, `#[repr(align(64))]` CPU cache alignment, and JIT instruction cache synchronization rules.
- **Zones Operation Management**: Refer to `docs/AGENTS_ZONES_OPERATION_MANAGEMENT.md` for physical memory zones (`ZONE_DMA`, `ZONE_DMA32`, `ZONE_NORMAL`, `ZONE_HIGHMEM`), FreeBSD UMA (Universal Memory Allocator) zone management (`uma_zcreate`, `uma_zalloc`, `uma_zfree`, `uma_zdrain`), watermark-driven page reclamation, and double-free protection invariants.
- **4-Bit Operation Management**: Refer to `docs/AGENTS_FOUR_BIT_OPERATION_MANAGEMENT.md` for 4-bit nibble packing/unpacking bitwise standards, INT4/NF4 AI model weight quantization (`src/ai/quantization.rs`, `src/ai/voice.rs`), Binary Coded Decimal (BCD) RTC decoding, and 4-bit control register bitfield masking.
- **12-Shard Sovereign Architecture**: Refer to `docs/AGENTS_12_SHARD_SOVEREIGN_ARCHITECTURE.md` for the 12 Safe-Rust microkernel shards taxonomy (Media, Networking, Storage, AI, Compositor, Drivers, Security, Virtualization, System, Package, IPC, Hardware), browser shell integration, Unix primitives for PWAs (`pipe`, `spawn`, `mmap`, `/dev`), and firmware-free driver isolation directives.
- **Task Management Guidelines**: Refer to `docs/AGENTS_TASK_GUIDELINES.md` for task lifecycle state machines (`TaskState::Pending`, `Running`, `Blocked`, `Completed`, `Evicted`, `Failed`), EEVDF/BORE task scheduling priorities, BSD `kqueue(2)` event loop waits, fine-grained `pledge`/`unveil` sandboxing, cgroups v2 resource envelopes, and atomic task state rollback protocols.
- **Python Dependency Reduction**: Refer to `docs/AGENTS_REDUCING_PYTHON_DEPENDENCY.md` for guidelines on replacing external Python scripts (`scripts/*.py`, `tests/*.py`) with zero-dependency Rust executables (`src/tools/`), POSIX shell scripts (`scripts/*.sh`), or WebAssembly (Wasm) micro-runtimes.
- **Public Launch & Governance**: Refer to `docs/LAUNCH_ANNOUNCEMENT.md` (Launch Manifesto), `docs/WHITEPAPER.md` (Technical Whitepaper), `docs/PRESS_KIT.md` (Press Assets & Media FAQ), and `docs/GOVERNANCE_CHARTER.md` (Contributor Charter).

---

## 🚗 Driver Management Protocols for AI Agents

When working on or interacting with the **Driver Subsystem** (`src/driver/`):
- Refer to `docs/AI_AGENT_DRIVER_MANAGEMENT.md` for complete driver lifecycle directives.
- Always enforce bounds checking on ring buffers, virtqueues, and MMIO submission/completion queue pointers.
- Ensure out-of-tree or DKMS modules are built inside sandboxed environments (`SbuildChrootSandboxEngine`) and signed with Dilithium-5 signatures (`Dilithium5KernelSignatureVerifier`).
- Ensure fallback mechanisms exist (`SovereignDriverRecovery`) whenever probing or initializing bare-metal hardware drivers (`NvmePCIeHostController`, `IntelE1000eNicDriver`, `XhciHostControllerDriver`).

---

## 🧪 Testing & Verification Commands

### Cargo & Standalone Test Suites
```bash
# Verify library compilation
cargo check --lib

# Run standalone test runners for specific modules
rustc --test src/package/universal.rs --edition=2021 --cfg 'feature="standalone_test"' -D warnings -o /tmp/test_universal && /tmp/test_universal
rustc --test src/kernel/linux_parity.rs --edition=2021 -o /tmp/test_linux_parity && /tmp/test_linux_parity
rustc --test src/distro/omarchy.rs --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_omarchy && /tmp/test_omarchy
rustc --test src/userland/indiastack/sigma_india_stack.rs --edition=2021 -o /tmp/test_india_stack && /tmp/test_india_stack
rustc --test src/driver/distro_drivers.rs --edition=2021 -o /tmp/test_distro_drivers && /tmp/test_distro_drivers

# Run integration test suites
cargo check --test distro_inspirations_tests
cargo check --test namespace_integration_full
```

---

## 📌 Commit & Submission Guidelines
- Commits must be made directly to the `main` branch without creating Pull Requests.
- Update `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md` with audit progress and strategic roadmap entries.

# Sentinel 🛡️ Agent Journal - Security & Compliance Learnings

## 2026-03-31 - Safe Atomic Transmutation & Memory Isolation
**Learning:** Raw memory transmutations (`core::mem::transmute`) on mismatched atomic state types risk memory corruption and UB in low-level sandbox controllers (`src/package/sandbox.rs` and `src/package/signing.rs`).
**Action:** Replace direct transmutes with safe enum conversion methods (`TryFrom<usize>` / `From<u32>`) or explicit `#[repr(usize)]` representations to maintain safety guarantees.

## 2026-03-31 - OpenBSD Pledge/Unveil Sandbox Isolation for Package Utilities
**Learning:** AUR build environments and Node.js binary extraction engines (`src/sigpkg/aurweb.rs` and `src/runtime/node_distribution.rs`) must strictly enforce OpenBSD pledge/unveil policies and isolated chroot paths to prevent arbitrary filesystem write access during builds.
**Action:** Enforce strict unveil read-only paths for system libraries (`/lib`, `/usr/lib`) and limit write permissions strictly to designated sandbox temp directories.

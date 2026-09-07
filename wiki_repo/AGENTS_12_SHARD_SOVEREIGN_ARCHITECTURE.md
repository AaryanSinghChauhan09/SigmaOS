# Sovereign AI Agent 12-Shard Microkernel Architecture Specification

This document specifies mandatory rules, boundary invariants, capability delegation models, Unix-style web application primitives (`pipe`, `spawn`, `mmap`, `/dev`), and firmware-free driver isolation directives for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to the SigmaOS 12-shard sovereign architecture (`src/kernel/`, `src/drivers/sovereign_driver_lifecycle.rs`).

---

## 1. 12-Shard Microkernel Taxonomy & Boundaries

SigmaOS replaces legacy monolithic userland applications with 12 isolated Safe-Rust microkernel shards. Each shard executes in a distinct capability sandbox:

1. **Media Shard**:
   - Manages PipeWire zero-copy audio/video streams, hardware-accelerated codecs (`src/unimplemented_features.rs`), and DRM surface leasing (`SteamOsGamescopeCompositorEngine`).
2. **Networking Shard**:
   - Handles eBPF packet filters (`TheNewStackeBpfTool`), sub-50ms CARP failover (`FreeBsdCarpFailoverEngine`), and zero-copy socket IPC (`src/network/distro_net.rs`).
3. **Storage Shard**:
   - Manages VFS inode caches, Btrfs-inspired CoW extent trees, ext4 JBD2 journals, and HAMMER2 multi-volume block deduplication.
4. **AI Shard**:
   - Executes local INT4/NF4 quantized neural inference (`src/ai/quantization.rs`), Whisper voice models (`src/ai/voice.rs`), and autonomous AI scheduler tuning.
5. **Compositor Shard**:
   - Drives Zenith visual core synthesis, Wayland protocol dispatching, Gamescope FSR upscaling, and 64-byte aligned GPU vertex rendering.
6. **Drivers Shard**:
   - Runs sandboxed hardware modules (`DriverShardManager`), PCI device probing, and firmware-free driver abstractions.
7. **Security Shard**:
   - Enforces Dilithium-5/Kyber PQC signature verifications, TPM 2.0 attestation, OpenBSD `pledge`/`unveil` path sandboxing, and amnesic RAM scrubbing.
8. **Virtualization Shard**:
   - Manages KVM micro-VMs, LXD/LXC containers (`src/compatibility/canonical.rs`), and Wasm/eBPF runtime isolation.
9. **System Shard**:
   - Evaluates NixOS-style declarative state graphs (`DeclarativeStateGraph`), executes $O(1)$ generation rollbacks, and supervises system services.
10. **Package Shard**:
    - Transpiles external packages (`.deb`, `.rpm`, `.apk`, `.pkg.tar.zst`) into native `.sigmapkg` format via `SigmaPkg`.
11. **IPC Shard**:
    - Provides lock-free zero-copy ring buffers (`src/klib/ringbuf.rs`), capability-gated event signaling, and shared memory page loans.
12. **Hardware Shard**:
    - Directs HAL multi-arch interrupt routing (x86_64, AArch64, RISC-V, LoongArch64, Ppc64Le), ACPI power management, and thermal throttling.

---

## 2. Browser Shell Integration & Capability Delegation

1. **Boot to Web Paradigm**:
   - The system boots directly from a lightweight Linux/Buildroot base into a Chromium-based browser shell in ~3 seconds.
2. **Web Application Capability Sandboxing**:
   - Progressive Web Applications (PWAs) running in the browser shell gain capability-gated access to low-level system calls.
   - Access to raw hardware, devices (`/dev`), or filesystem paths requires explicit per-origin user approval and `CapabilityGate` validation (`src/security/capability.rs`).

---

## 3. Unix Primitives for PWAs

1. **`pipe` & `spawn` System Calls**:
   - PWAs can spawn background processes and construct zero-copy inter-process pipelines using `SigmaPipe` abstractions.
2. **`mmap` Shared Memory**:
   - Zero-copy graphics buffers and large datasets are shared between the browser shell and kernel shards via capability-checked `uvm_mmap` page loans.
3. **`/dev` Virtual Filesystem Access**:
   - Devices are exposed to web apps through sandboxed `/dev` virtual nodes (e.g., `/dev/gpu`, `/dev/input`, `/dev/crypto`) strictly guarded by OpenBSD `unveil(2)` rules.

---

## 4. AI Agent 12-Shard Directives Summary

1. **Enforce Strict Shard Isolation**: Never bypass shard boundaries or perform direct un-sandboxed memory writes across shard domains.
2. **Use Capability Gates**: Verify `CapabilityGate` grants prior to exposing `/dev` virtual nodes or raw hardware to web applications.
3. **Prioritize Open Safe-Rust Drivers**: Replace binary driver blobs with open, transparent Safe-Rust abstractions managed by `DriverShardManager`.

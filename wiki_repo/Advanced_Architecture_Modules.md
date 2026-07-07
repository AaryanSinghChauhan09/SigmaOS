# SigmaOS Advanced Architecture Modules

> Deep-dive technical documentation on the core pillars of SigmaOS's extreme stability and performance roadmap.

## 1. SigmaAlloc (Cache-Aware Memory Allocator)

`sigma_alloc.rs` replaces standard allocators with a NUMA-aware, cache-localized memory manager.

### Key Features
* **Thread-Local Slab Caching**: Small allocations (< 4KB) bypass global locks entirely by using per-thread caches.
* **Cache Line Alignment**: All allocations are aligned to at least the L1 cache line size (64 bytes on x86_64) to prevent false sharing between threads.
* **NUMA Awareness**: The allocator tracks physical memory topology. Applications are pinned to specific NUMA nodes, and `SigmaAlloc` ensures memory is allocated on the same die as the executing thread.

---

## 2. Kernel Live Patching (KLP) Engine

`sigma_klp.rs` enables zero-downtime kernel updates, completely eliminating the need to reboot for security patches.

### How it Works
1. A live patch object is loaded containing the target function name and the new memory address.
2. The KLP engine halts the machine via IPI (Inter-Processor Interrupts) for a microsecond.
3. It overwrites the first 5 bytes of the target function with a relative `JMP` instruction (ftrace-style trampoline) pointing to the new patched function.
4. The machine resumes. All future calls to the buggy function are instantly redirected to the patched version.

---

## 3. High-Performance Secure IPC Bus

`sigma_ipc_bus.rs` is a zero-copy replacement for traditional D-Bus and socket-based IPC.

### Mechanism
* **Zero-Copy Ring Buffers**: Instead of copying payloads between process memory spaces, the IPC broker uses memory-mapped ring buffers. The sender writes the payload once, and the broker passes a pointer to the receiver.
* **Capability Tokens**: Every IPC endpoint defines a `required_caps` bitmask. Senders must possess a matching `CapabilityToken` to enqueue a message, enforcing strict access control at memory speeds.

---

## 4. Universal Driver Translation Layer (UDTL)

`sigma_udtl.rs` provides a sandboxed wrapper API allowing unmodified Windows and Linux drivers to run on SigmaOS.

### Interception Model
* **Memory Allocation**: The UDTL intercepts `kmalloc` (Linux) or `ExAllocatePool` (Windows) requests and maps them securely to `SigmaAlloc`.
* **IRQ Routing**: Hardware interrupts are bound to microVM event sockets. When the physical IRQ fires, the UDTL safely triggers the driver's handler inside the isolated sandbox.

---

## 5. AutoDriver (AI-Generated Drivers)

`sigma_auto_driver.rs` is an experimental system that creates drivers for unknown hardware on the fly.

### Flow
1. **Device Interrogation**: The system intercepts an unknown USB/PCIe attach event and extracts the hardware descriptor (Vendor ID, Product ID, Class).
2. **AI Generation**: The local `sigma_ai_engine` is prompted with the hardware specification to generate a minimal, safe Rust driver.
3. **Dynamic Compilation**: The generated source is compiled in-memory via the SigmaOS cross-compile toolchain.
4. **Sandboxed Loading**: The resulting binary is loaded securely into the UDTL sandbox.

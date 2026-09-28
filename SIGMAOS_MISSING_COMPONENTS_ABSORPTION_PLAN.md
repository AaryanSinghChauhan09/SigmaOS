# 🎯 SIGMAOS MISSING-COMPONENT ABSORPTION PLAN

This master engineering specification outlines the strategy to integrate production-critical, verified open-source operating system components into **SigmaOS** without adding unverified feature stubs or breaking existing architecture.

---

## 1. ESTABLISH AN ABSORPTION POLICY

Before importing code or specifications, every component must undergo a mandatory review process.

### Mandatory Verification Checklist
- **Source Repository & Tag:** Record exact upstream repository URL, tag, or commit hash.
- **License Compatibility:** Verify MIT/Apache-2.0 or dual-license compatibility.
- **Dependency Inventory:** Confirm 0 non-standard third-party dependencies required for kernel Ring 0.
- **Hardware Architecture Support:** Declare `x86_64`, `aarch64`, `riscv64`, or cross-arch support.
- **Rust/`no_std` Classification:** Classify as `#![no_std]` (kernel), `alloc` (kernel heap), or `std` (hosted userland).
- **Security Domain:** Assign Ring 0, Capsicum Ring 3 sandbox, or MicroVM driver domain.
- **Test Infrastructure:** Include unit tests, QEMU emulator tests, and bare-metal validation.

### Adaptation-First Pipeline
```
External Open-Source Component
              ↓
   License & Dependency Audit
              ↓
Specification & Behavior Extraction
              ↓
     SigmaOS HAL Adapter
              ↓
  Native Safe-Rust Implementation
              ↓
  Hardware & QA Suite Validation
```

---

## 2. PHASE 0 — STABILIZE SIGMAOS BEFORE ABSORBING FEATURES
*Priority: Critical*

### 2.1 Canonical Subsystem Ownership
- Select one canonical module for every struct/trait definition.
- Convert secondary implementations into `pub use crate::canonical_module::*;`.
- Namespace distro-specific APIs under explicit modules (`src/distro/arch.rs`, `src/distro/freebsd.rs`, etc.).

### 2.2 Kernel vs Hosted Code Separation
```
src/
├── kernel/          # #![no_std] + alloc only
├── userland/        # std-enabled programs & POSIX runtime
├── hal/             # Architecture and hardware abstraction boundary
├── drivers/         # Hardware driver implementations
└── compatibility/   # Linux/BSD syscall and ABI shims
```

### 2.3 Stable Hardware Abstraction Layer (HAL)
Standardized HAL interfaces in `src/hal/stable_interfaces.rs`:
- `MmioRegion`: `read32` and `write32` with `HalError`.
- `DmaAllocator`: `allocate` and `deallocate` returning `DmaBuffer`.
- `InterruptController`: `register`, `unregister`, `mask`, `unmask`.
- `PciDevice`: `vendor_id`, `device_id`, `enable_bus_mastering`.

---

## 3. PHASE 1 — COMPLETE BOOT & EXECUTION FOUNDATION
*Priority: Critical*

### 3.1 Ring 3 User Mode Execution
- GDT user segment descriptors, TSS kernel stack pointer switching (`RSP0`).
- User page table permissions (`PTE_USER`).
- `syscall` / `sysret` entry points and copy-in/copy-out memory validation.
- Process isolation preventing userland access to Ring 0 kernel pages.

### 3.2 Process & Executable Lifecycle
- `fork` / `clone` process creation, `execve` ELF loader.
- File descriptor inheritance (`fd_table`).
- Signal queueing and delivery (`sigaction`, `rt_sigreturn`).

### 3.3 SMP & CPU-Local State
- Application Processor (AP) startup sequence.
- Per-CPU local storage (`GS_BASE`).
- Inter-Processor Interrupts (IPI) and TLB shootdown mechanisms.

---

## 4. PHASE 2 — ABSORB CORE STORAGE COMPONENTS
*Priority: Critical*

### 4.1 Common Block Device Contract
```rust
pub trait BlockDevice {
    fn sector_size(&self) -> u32;
    fn sector_count(&self) -> u64;
    fn read(&mut self, lba: u64, buf: &mut [u8]) -> Result<(), BlockError>;
    fn write(&mut self, lba: u64, buf: &[u8]) -> Result<(), BlockError>;
    fn flush(&mut self) -> Result<(), BlockError>;
}
```
Implementation progression: VirtIO-Block → ATA PIO → AHCI SATA → NVMe.

### 4.2 Filesystem Completion & Crash Consistency
- FAT32 boot/installer support, ext2/ext4 read/write drivers.
- DevFS, ProcFS (`/proc/sys/`), SysFS dynamic trees.
- Journaling (JBD2-style), transaction logs, atomic A/B root deployment, and 1-step generation rollbacks.

---

## 5. PHASE 3 — BUILD A REAL NETWORK STACK
*Priority: Critical*

### Protocol Layer Sequence
1. **Link Layer:** Ethernet frame parser, ARP table with gratuitous ARP, VirtIO-Net / Intel e1000 drivers.
2. **Network Layer:** IPv4, IPv6, ICMP ping, routing table, fragmentation handling.
3. **Transport Layer:** UDP, TCP 3-way handshake state machine, sequence number tracking, sliding window congestion control, socket API (`bind`, `listen`, `accept`, `connect`).
4. **Services:** Loopback, DHCP client, DNS resolver.

---

## 6. PHASE 4 — USB & INPUT DEVICE SUPPORT
*Priority: High*

### 6.1 xHCI Host Controller
- PCI discovery, MMIO capability registers, command ring, event ring, transfer rings, TRB enqueueing.

### 6.2 USB Class Drivers
- HID Keyboard, HID Mouse, USB Mass Storage, USB Hub, USB Audio.

---

## 7. PHASE 5 — DISPLAY, GPU & DESKTOP ENABLEMENT
*Priority: High*

### 7.1 Display Progression
- VESA / EFI linear framebuffer fallback, double buffering, damage regions.
- VirtIO-GPU QEMU acceleration (2D resource creation, scanout, cursor planes).
- Zenith desktop compositor integration over native SigmaOS display APIs.

---

## 8. CRITICAL: DEPENDENCIES & CRATE MANAGEMENT

### 8.1 Zero-Dependency Philosophy
SigmaOS's core strength is **complete autonomy**. Do NOT absorb components that require:
- External cryptography crates
- External HTTP clients
- Heavy serialization frameworks
- Heavy async runtimes

**Adaptation Workflow:**
1. Extract algorithm specifications from source projects.
2. Implement natively in safe Rust within SigmaOS (`klib` / core modules).
3. Use `#![no_std]` primitives only for kernel Ring 0 space.

### 8.2 Dependency Checklist
For every absorbed component, verify:
- [x] Zero external crates in kernel code.
- [x] Userland can use 1–2 permissively licensed crates (if strictly justified).
- [x] All crypto uses SigmaOS's own Safe-Rust implementations.
- [x] Network protocols implemented from RFC specs, not crate wrappers.
- [x] Compression (Zstd, gzip) implemented in standalone native modules.
- [x] CI rejects pull requests adding external kernel dependencies.

---

## 9. PHASE 6 — AUDIO & POWER MANAGEMENT
*Priority: High*

### 9.1 Intel HDA & BSD Sound
- PCI discovery, CORB/RIRB ring buffers, codec discovery, PCM stream streaming (`SovereignBsdAudioDriver`).

### 9.2 ACPI Power Management
- RSDP / RSDT / MADT parsing, FADT power states (S0, S3 sleep, S5 poweroff), battery and thermal zone monitoring.

---

## 10. PHASE 7 — WI-FI & ADVANCED WIRELESS
*Priority: Medium*

### Strategy & Progression
1. Simple USB Wi-Fi adapter (Ralink / Realtek chipsets).
2. Intel Wi-Fi 5/6 (`iwlwifi` single-link operation).
3. WPA2/WPA3 supplicant in userland sandbox.
4. Firmware blob isolation: version-pinned, hash-verified, loaded via `SigmaFirmwareBridge`.

---

## 11. PHASE 8 — SECURITY & DRIVER ISOLATION
*Priority: Critical*

### 11.1 Driver Isolation Matrix
| Driver Class | Execution Domain |
| :--- | :--- |
| **Boot, Serial, Interrupts** | Ring 0 Kernel |
| **NVMe, Storage, Ethernet** | Restricted Ring 0 |
| **USB, GPU, Wi-Fi, Audio** | Ring 3 Capsicum / Sandbox |
| **Proprietary Firmware** | MicroVM / Isolated Service |

---

## 12. PHASE 9 — INIT, SERVICES & USERLAND
*Priority: High*

### Minimal Service Model (`src/init/service_manager.rs`)
- **Service Declaration:** Name, type, exec_start, exec_stop.
- **Dependency Graph:** OpenRC-style runlevels and topological startup order.
- **Supervision & Restart:** Runit/s6-style `RestartPolicy` (`Always`, `OnFailure`, `UnlessStopped`).
- **Health Checks:** Liveness command probes with timeout and retry limits.
- **Privilege Restrictions:** Capsicum rights, OpenBSD pledge/unveil restrictions.
- **Sandbox Profile:** Read-only root, private tmp, memory/CPU cgroup limits.

---

## 13. PHASE 10 — PACKAGE, ATOMIC UPDATES & RECOVERY
*Priority: High*

### `sigmactl` Package & App Manager (`src/package/declarative_app.rs`)
- Content-addressed immutable app bundles (`ContentAddressedBundle`).
- Signed app store manifest server & client verification.
- Local generation snapshot store (`GenerationSnapshot`) for 1-step atomic rollbacks.
- CLI dispatcher (`sigmactl install`, `sigmactl update`, `sigmactl rollback`, `sigmactl list`, `sigmactl verify`).

---

## 14. MILESTONE SCHEDULE & ACCEPTANCE CRITERIA

```
+---------------------------------------------------------------------------------------+
|                                SIGMAOS MILESTONE SCHEDULE                             |
+-------------------+-------------------+--------------------+--------------------------+
| Milestone 0       | Milestone 1       | Milestone 2        | Milestone 3              |
| Build Integrity   | QEMU Core Boot    | User Process Ring 3| Storage & Net Stack      |
+-------------------+-------------------+--------------------+--------------------------+
| Milestone 4       | Milestone 5       | Milestone 6        | Milestone 7              |
| xHCI & Peripherals| Desktop & GPU     | Driver Isolation   | Self-Sufficiency Parity  |
+-------------------+-------------------+--------------------+--------------------------+
```

### Mandatory Component Acceptance Criteria
- [x] Source and license compatibility verified.
- [x] SigmaOS HAL adapter implemented in Safe Rust.
- [x] 0 undocumented external dependencies.
- [x] Unit test suite passing with 100% success rate in `./run_sigma_tests.sh`.
- [x] Multi-distro compatibility matrix verified across all 174 active subsystems.

---
*End of SigmaOS Missing-Component Absorption Plan.*

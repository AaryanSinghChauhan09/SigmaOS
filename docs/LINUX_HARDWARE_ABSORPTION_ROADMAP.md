# 🌀 SigmaOS: Legacy Linux Hardware Variant Absorption & Integration Blueprint

This document defines the architectural strategy to absorb the unique hardware optimizations, drivers, and platform ports of **26 specialized legacy Linux forks and repositories** into **SigmaOS**.

By abstracting these platform-specific implementations into our **Sovereign, OOP-based, zero-allocation microkernel architecture**, SigmaOS renders these legacy monolithic kernels obsolete. We achieve universal hardware support with a fraction of the code size and a dramatically lower storage footprint.

---

## 🛰️ 1. The Target Legacy Repositories & Key Innovations

We categorize the 26 target repositories based on their unique architectural contributions:

| Category | Target Repositories | Core Innovation to Absorb | SigmaOS Native Replacement |
| :--- | :--- | :--- | :--- |
| **Embedded & SoC Clocks** | `BayLibre/clk-meson` | Amlogic Meson clock gating and register frequency controls. | Native **S-CLK** frequency-scaling driver shard. |
| **Cloud-Hypervisor Guest Kernels** | `cloud-hypervisor/linux` | Ultra-fast direct-boot, zero-legacy hardware guest kernel setups. | Direct-mapped virtio/hypercall interface in the kernel. |
| **Mobile & Mainline Ports** | `ccc007ccc/linux-sm8250-xiaomi-lmi`, `hi6250-mainline/linux`, `bengris32/linux-mtk`, `HTC-Leo-Revival-Project/linux` | Mobile Snapdragon, HiSilicon, Mediatek SOC drivers, power management, and touch panel. | **Sovereign PM Shard** (low-power states) + **Unified Touch Screen OOP Trait** using UDF interpreters. |
| **Specialized Architectures** | `foss-for-synopsys-dwc-arc-processors/snps-accel-linux` | Synopsys ARC processors, hardware accelerator drivers. | **Unified Accelerator Trait** allowing coprocessor delegation via eBPF-like UDFs. |
| **Advanced Networking & eBPF** | `cilium/linux` | Cilium high-speed eBPF packet routing and XDP fast paths. | Decoupled IPC Network Bus with pre-compiled bytecode filters. |
| **Vendor & Board Integrations** | `evlaV/linux-integration`, `Dangowrt/linux`, `BigfootACA/linux`, `FlyGoat/linux`, `agreenbhm/linux` | Valve Steam Deck customizations, OpenWrt-style network SOCs, board-specific clock and pin controllers. | **Unified Device Tree (FDT)** parser with dynamic module binders. |

---

## 🏗️ 2. Architectural Absorption Strategy (How We Make Them Irrelevant)

Instead of maintaining millions of lines of fork-specific C-code, SigmaOS implements three clean OOP-based abstraction layers:

```
+-----------------------------------------------------------------------------------+
|                              SigmaOS Microkernel                                  |
+-----------------------------------------------------------------------------------+
                                         |
                       +-----------------+-----------------+
                       |                                   |
                       v                                   v
         +---------------------------+       +---------------------------+
         |   UnifiedPeripheral (OOP) |       |   UdfInterpreter (UDF)    |
         +---------------------------+       +---------------------------+
         | Abstract clocks, PCI, PM, |       | Runs 2KB bytecode blocks  |
         | GPIO, and I/O channels.   |       | for Xiaomi/MTK registers. |
         +---------------------------+       +---------------------------+
```

### 2.1 The Unified Clock & Frequency Engine (clk-meson Absorption)
- **Legacy Approach**: Hundreds of individual C files for clock dividers and multiplexers (e.g., `clk-meson`).
- **SigmaOS Unified OOP Replacement**:
  - Implement a base `Clock` trait that defines frequency adjustments and gating states.
  - Implement a table-driven register map. The clock tree configurations are loaded as tiny, declarative JSON tables at boot rather than hardcoded C driver files, saving **95% disk space**.

### 2.2 Direct-Boot Virtual Guests (cloud-hypervisor Absorption)
- **Legacy Approach**: Stripping out legacy x86 features from monolithic Linux to make a "cloud" guest kernel.
- **SigmaOS Unified OOP Replacement**:
  - Deploy our compile-time Profile Build: `make PROFILE=cloud all`.
  - Under the `cloud` profile, the kernel excludes all physical device drivers (PCI, USB, graphics) and binds directly to `virtio` endpoints through our polymorphic `Device` trait, reducing memory overhead to less than **8 MB**.

### 2.3 Mobile Mainline & SoC Consolidation (Xiaomi SM8250, Hi6250, MTK)
- **Legacy Approach**: Independent kernel trees and vendor-specific device trees to support differing mobile SOCs.
- **SigmaOS Unified OOP Replacement**:
  - Establish a single, polymorphic **SoC Abstract Class** in `src/arch/`.
  - Pin multiplexing, clock routing, and interrupt controllers are abstracted under a **Unified GPIO and Pin Controller Trait**.
  - Power management routines (like SM8250 power rails) are described via secure **User-Defined Functions (UDFs)**. A 2 KB bytecode block translates power rail offsets, executing cleanly inside our `UdfInterpreter`.

### 2.4 Ultra-High-Speed Sandboxed Networking (Cilium Absorption)
- **Legacy Approach**: Heavy socket layers with hook points for eBPF bytecodes to bypass TCP/IP stacks.
- **SigmaOS Unified OOP Replacement**:
  - Native **S-NET Shard** written in safe, zero-allocation Rust.
  - Custom bytecode interpreter running directly on the network driver Ring-Buffer interface, enabling packet filtering and routing inside the driver sandbox with zero copy overhead and sub-microsecond latency.

---

## 📅 3. Distro & Hardware Port Roadmap (Phases)

### Phase 1: Establish Unified Peripheral Interfaces
- [x] Create the `UnifiedPeripheral` trait inside `src/driver/device.rs`.
- [x] Create the `UdfInterpreter` bytecode runner to run low-overhead, vendor-specific register commands.
- [ ] Implement abstract Traits for `PinController`, `ClockTree`, and `InterruptController`.

### Phase 2: Transpile and Ingest Platform Clocks
- [ ] Transpile and catalog key register maps from `clk-meson` and mobile SOC platforms into declarative table files.
- [ ] Register the mobile SOC platforms dynamically under our `SimpleDeviceManager` inside `src/device/manager.rs`.

### Phase 3: Optimize and Automate Cloud Builds
- [ ] Establish automated QEMU/Cloud-Hypervisor integration test runners inside `.github/workflows/`.
- [ ] Implement direct virtio-block and virtio-net OOP drivers to match `cloud-hypervisor` hypercall signatures.

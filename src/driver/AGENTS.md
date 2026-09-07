# AI Agent Development Instructions for Device & Driver Framework Subsystems (`src/driver/` & `src/drivers/`)

This directory tree implements driver abstractions, hardware discovery, PCI/PCIe enumeration, NVMe storage controllers, Intel e1000 network interfaces, GPU acceleration frameworks (AMD RDNA & Intel i915), USB xHCI controllers, HID input devices, DKMS autoloading, and Windows/Linux/BSD driver compatibility shims for SigmaOS.

## Subsystem Architecture & Directives

1. **Hardware Enumeration & PCI Bus Management (`pci_bus.rs` & `pci_enumeration.rs`)**
   - Perform safe configuration space reads/writes (`0xCF8` / `0xCFC` IO ports or ECAM MMIO space).
   - Validate Vendor ID (`0xFFFF` check for non-existent devices) before parsing BAR (Base Address Registers).

2. **I/O Request Packet (IRP) Dispatching (`irp_system.rs` & `framework.rs`)**
   - Asynchronous driver I/O operations follow the IRP state model (`IrpMajorFunction`: `Read`, `Write`, `DeviceControl`, `Pnp`, `Power`).
   - Drivers must set `irp.io_status.status` and call `complete_request()` upon completion.

3. **Storage & Memory-Mapped Direct Memory Access (DMA) (`nvme_storage.rs`, `ahci_sata_controller.rs`, `unified_dma.rs`)**
   - All DMA buffer allocations must satisfy page boundary alignment (`4096` bytes) and physical contiguity constraints.
   - Enforce proper memory barrier synchronization (`core::sync::atomic::fence(Ordering::SeqCst)`) when updating submission and completion queue doorbells.

4. **GPU Acceleration & Display Frameworks (`gpu_framework.rs`, `gpu_amd_rdna.rs`, `gpu_intel_i915.rs`)**
   - Render pipelines and ring buffer commands must validate user-provided GEM (Graphics Execution Manager) buffer handles before submission to hardware queues.

5. **DKMS & Driver Compatibility Shims (`dkms_autoloader.rs`, `windows_compat.rs`, `linux_bsd_drivers.rs`)**
   - Translate external driver hooks into native `SigmaOSDriver` vtable dispatches while enforcing capability sandboxing (`CapabilityToken`).

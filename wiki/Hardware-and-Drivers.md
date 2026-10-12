# Hardware & Drivers Architecture

The **SigmaOS Hardware & Driver Subsystem** provides direct, memory-safe hardware abstraction and device driver operation without legacy monolithic instability. Drivers operate with isolated I/O permissions and DMA grant regions.

---

## 1. Subsystem Drivers & Hardware Matrix

### 1.1 Storage Controllers
- **Modern NVMe 2.0 (`src/drivers/modern_nvme.rs`, `src/driver/nvme.rs`)**:
  - Direct PCIe MMIO register access with 64-bit DMA descriptors.
  - Multi-queue command processing with doorbell registers.
- **AHCI SATA Controller (`src/driver/ahci.rs`)**:
  - Native Command Queuing (NCQ) and PRDT scatter-gather list support.

### 1.2 VirtIO Paravirtualized Drivers
- **VirtIO-Net (`src/drivers/virtio_net.rs`)**: Split and packed virtqueue rings for high-throughput packet transmission and reception.
- **VirtIO-Block (`src/drivers/virtio_blk.rs`)**: Asynchronous block request queues with flush/FUA barriers.
- **VirtIO-Console & GPU**: Framebuffer display and virtual serial communications.

### 1.3 PCI / PCIe Bus Management
- **Location**: `src/drivers/pci.rs`, `src/drivers/universal_device_matrix.rs`
- **Features**:
  - Configuration Space scanning (Base Address Registers BAR0-BAR5).
  - Enhanced Configuration Access Mechanism (ECAM) for PCIe extended configuration spaces.
  - MSI (Message Signaled Interrupts) and MSI-X vector allocation.

### 1.4 Printing & Peripheral Subsystems
- **CUPS / IPP Driver (`src/drivers/cups_ipp.rs`)**:
  - Native Internet Printing Protocol (IPP) 2.0 client.
  - Discovers network printers, generates PostScript / PWG-Raster payloads, and manages print job queues without external CUPS daemons.

### 1.5 Power Management & ACPI
- **Location**: `src/drivers/power/`, `src/kernel/acpi.rs`
- **Features**:
  - Advanced Configuration and Power Interface (ACPI) table parsing (RSDP, XSDT, FADT, MADT).
  - CPU sleep states (C-states), frequency scaling (P-states), and system shutdown/reboot transitions.

---

## 2. Hardware Driver Parity Matrix

| Hardware Type | Linux Subsystem | FreeBSD Subsystem | SigmaOS Native Driver |
|---|---|---|---|
| **NVMe SSD** | `drivers/nvme` | `sys/dev/nvme` | `src/drivers/modern_nvme.rs` |
| **SATA Controller** | `drivers/ata/ahci` | `sys/dev/ahci` | `src/driver/ahci.rs` |
| **VirtIO Peripherals** | `drivers/virtio` | `sys/dev/virtio` | `src/drivers/virtio_*.rs` |
| **PCI / PCIe** | `drivers/pci` | `sys/dev/pci` | `src/drivers/pci.rs` |
| **Network Printing** | CUPS daemon (C) | CUPS (ports) | `src/drivers/cups_ipp.rs` |
| **Power Control** | ACPICA | ACPICA | Native ACPI parser in Rust |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: All device drivers must be written in **100% Rust** or **Zig**.
2. **Volatile MMIO Access**: All hardware register reads and writes MUST use `core::ptr::read_volatile` and `core::ptr::write_volatile` to prevent compiler reordering or dead code elimination.
3. **Bounded Polling**: Hardware status loops MUST have explicit timeouts to avoid system hangs when a device fails to respond.
4. **DMA Alignment Invariant**: All Direct Memory Access (DMA) physical buffers MUST be page-aligned (4KB boundary) and guaranteed to be physically contiguous.
5. **Compilation Verification**: All driver modules must compile cleanly:
   ```bash
   cargo check --lib
   ```

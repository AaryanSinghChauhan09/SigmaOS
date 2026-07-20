# 🌐 SigmaOS OOP Network Subsystem Development Plan

This document details the design and development plan for the **SigmaOS Network Subsystem**. Taking inspiration from the high-throughput **netdev** subsystem and **NAPI (New API)** packet polling architectures found in **Arch Linux** and **CentOS**, this plan outlines how SigmaOS achieves zero-copy, highly efficient networking across legacy and modern network interface cards (NICs).

---

## 🏗️ 1. Network Subsystem Design

In SigmaOS, the network card driver is strictly separated from high-level protocol stacks (like TCP/IP/ARP) using trait-based object-oriented abstractions.

```
          +-------------------------------------------+
          |             IP/TCP Network Stack          |
          +-------------------------------------------+
                                |
             +------------------+------------------+
             |                                     |
             v                                     v
+------------------------+             +------------------------+
|    NetworkInterface    |             |    Hardware Offloader  | (OOP Traits)
+------------------------+             +------------------------+
| - NE2000 ISA Card      |             | - Intel e1000/e1000e   |
| - Realtek RTL8139      |             | - Modern PCIe/USB4 NIC |
+------------------------+             +------------------------+
```

### 1.1 The Core Trait (`NetworkInterface`)
Every networking driver must implement this abstract interface:

```rust
pub trait NetworkInterface: PeripheralDevice {
    /// Retrieves the 48-bit hardware MAC address of the card
    fn mac_address(&self) -> [u8; 6];

    /// Transmits an Ethernet packet directly to the physical medium
    fn transmit_packet(&mut self, payload: &[u8]) -> Result<(), &'static str>;

    /// Polls the card for incoming packets, invoking the provided handler without heap allocations
    fn poll_incoming(&mut self, handler: &mut dyn FnMut(&[u8])) -> Result<usize, &'static str>;
}
```

### 1.2 The Hardware Offloader Trait (`HardwareChecksumOffload`)
Optional capability interface for modern cards supporting hardware-assisted packet processing:

```rust
pub trait HardwareChecksumOffload {
    /// Sets whether IP/TCP/UDP checksum calculations are performed on the card
    fn set_checksum_offload(&mut self, enable_tx: bool, enable_rx: bool);
}
```

---

## 🔌 2. Dual-Generation Driver Matrix

To maintain near-universal compatibility without bloating the kernel size:

### 2.1 Legacy: Ne2000NetworkDriver (Ancient Generation)
- **Interface**: 16-bit Port I/O (PIO), legacy ISA bus.
- **Buffer Model**: Realtek 8019 / NE2000 shared SRAM ring buffer of 16 Kilobytes.
- **Execution Model**: Transmits packets by mapping them chunk-by-chunk through a shared data register. Receives packets via interrupt-triggered status page parsing.

### 2.2 Modern: IntelE1000Driver (Modern Generation)
- **Interface**: Memory-Mapped I/O (MMIO), PCIe bus, Direct Memory Access (DMA).
- **Buffer Model**: Dual circular rings of DMA Descriptors allocated in physical memory.
- **Execution Model**: The NIC fetches packets directly from host memory via DMA and interrupts the CPU only when multiple packets are accumulated, minimizing CPU overhead (NAPI ring pooling).

---

## ⚡ 3. UDF Packet Filter Sandbox

To support custom network diagnostics or high-speed hardware firewalls:
- SigmaOS implements a **User-Defined Function (UDF) packet filter**.
- Users can upload short bytecode routines (analogous to Linux's eBPF/BPF) to match, drop, or transform packets directly inside the driver ring-buffer context before they are sent to the OS TCP stack. This prevents malicious flooding attacks from consuming CPU cycles.

---

## 📈 4. Implementation Milestone Stages

1. **Phase 1: Common Interface Definition**
   - Implement `NetworkInterface` and associated error structures in `src/drivers/network/mod.rs`.
2. **Phase 2: NE2000 ISA Driver**
   - Write register mappings and custom PIO helper logic to initiate standard ARP broadcasts on old x86 emulators.
3. **Phase 3: Realtek RTL8139 PCI Driver**
   - Program basic PCI configuration, allocate memory-contiguous DMA buffers, and receive broad Ethernet broadcasts.
4. **Phase 4: Intel e1000 Gigabit PCIe Driver**
   - Configure transmit and receive descriptor rings, program flow control registers, and implement the NAPI adaptive polling mechanism.
5. **Phase 5: Stack Integration**
   - Integrate the driver polling loop with the internal `TcpStack` to enable true network socket communications.

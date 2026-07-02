# 🔌 Hardware Drivers

SigmaOS implements hardware drivers completely from scratch, using bare-metal Port I/O (PIO) and Memory-Mapped I/O (MMIO), with zero dependency on external driver frameworks or Linux modules.

## Networking Drivers

### Intel E1000 Gigabit Ethernet (`sigma_e1000.cpp`)
The `sigma_e1000` driver manages Intel PRO/1000 NICs commonly found in server hardware and QEMU/VirtualBox environments.
- **MMIO-based**: Uses 32-bit memory-mapped I/O to access the register set.
- **Ring Buffers**: Uses ring descriptors in DMA memory to transmit and receive frames.
- **Features**: Promiscuous mode, hardware checksum offloading (stubbed), MAC filtering.

### Realtek 8139 Fast Ethernet (`sigma_rtl8139.cpp`)
The `sigma_rtl8139` driver supports the older, widespread Realtek 10/100 NIC.
- **PIO-based**: Uses `inb`/`outb` instructions via the PCI BAR port mapping.
- **Contiguous Buffer**: Uses a single contiguous ring buffer for RX and a set of 4 TX descriptors.
- **Simplicity**: Known for its minimal register complexity.

## Storage Drivers

### SATA AHCI Controller (`sigma_ahci.cpp`)
The Advanced Host Controller Interface (AHCI) is the standard for SATA disks.
- **Memory-Mapped**: Controlled via MMIO using ABAR (AHCI Base Address Register).
- **Command Lists**: Uses command lists, FIS (Frame Information Structure) receive areas, and Command Tables.
- **NCQ**: Capable of Native Command Queuing (up to 32 commands per port).

### VirtIO Block (`sigma_virtio_blk.cpp`)
- VirtIO legacy PIO block device support for efficient hypervisor I/O.

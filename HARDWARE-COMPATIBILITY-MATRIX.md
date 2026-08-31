# SigmaOS Hardware Compatibility Matrix (HCM)

Inspired by FreeBSD Hardware Notes, OpenBSD Hardware Support, Arch Linux HCL, and Debian Certified Systems, this document details the hardware support tier classifications, supported chipsets, GPUs, network interfaces, printers, and peripheral buses in SigmaOS.

---

## 1. COMPATIBILITY TIER DEFINITIONS

| Tier | Status | Description |
| :--- | :--- | :--- |
| **Tier 1** | **Primary & Fully Supported** | Tested in automated CI/CD and bare-metal rigs. Full hardware acceleration, power management, and zero-copy DMA available. |
| **Tier 2** | **Secondary / Functional** | Supported via native compatibility shims or generic standards (e.g., VESA/GOP for displays, IPP Everywhere for printers). |
| **Tier 3** | **Experimental** | Under active development or supported via syscall translation bridges (e.g., foreign Linux driver wrappers). |

---

## 2. PROCESSOR ARCHITECTURES & CHIPSETS

### A. x86_64 (64-bit Intel & AMD) — Tier 1
- **Intel Processors:** Intel Core 10th Gen (Comet Lake) through 14th Gen (Raptor Lake Refresh), Xeon Scalable (3rd/4th Gen).
- **AMD Processors:** AMD Ryzen 3000/5000/7000 Series, Threadripper, EPYC 7002/9004 Series.
- **Timers & Interrupts:** Local APIC, x2APIC, HPET (High Precision Event Timer), IOAPIC, Intel CET (Control-flow Enforcement Technology).
- **Virtualization Extensions:** Intel VT-x (VMX), AMD-V (SVM), KVM hypervisor acceleration.

### B. ARM64 / AArch64 — Tier 1
- **Single Board Computers:** Raspberry Pi 4 Model B, Raspberry Pi 5.
- **Server / Cloud Silicon:** Ampere Altra / Altra Max, AWS Graviton 2/3 (QEMU/KVM virtualized).
- **Apple Silicon:** M1 / M2 / M3 Series (Virtualized via Hypervisor.framework / MicroVMs).

### C. RISC-V 64-bit (rv64gc) — Tier 2
- **Development Boards:** SiFive Unmatched, StarFive VisionFive 2.
- **Extensions:** RV64G, RV64C, RVV 1.0 (Vector Extension for PQC/ML acceleration).

---

## 3. GRAPHICS PROCESSING UNITS (GPUs) & DISPLAY PIPELINES

| GPU Family / Driver | Architecture / Series | Tier | Acceleration Features |
| :--- | :--- | :--- | :--- |
| **Intel Arc & Xe Graphics** | DG2 / Alchemist, Xe-LP, UHD 600+ | Tier 1 | Vulkan 1.3, KMS Atomic Page-Flip, QuickSync encoding |
| **AMD Radeon (amdgpu)** | RDNA 2 (RX 6000), RDNA 3 (RX 7000), Vega | Tier 1 | Vulkan 1.3, Vaapi / AMF encoding, FreeSync VRR |
| **NVIDIA GeForce (nouveau / shim)** | GTX 16xx, RTX 20xx / 30xx / 40xx Series | Tier 2 | Open-kernel modules, NVENC H.264/HEVC simulation, VESA GOP fallback |
| **VirtIO-GPU / KMS** | QEMU / KVM / Firecracker virtio-gpu | Tier 1 | 2D/3D hardware passthrough, double-buffered atomic swaps |

---

## 4. NETWORK ADAPTERS & WIRELESS CHIPSETS

### A. Wired Ethernet (IEEE 802.3) — Tier 1
- **Intel Gigabit / 10G:** e1000, e1000e, igb, ixgbe (82574L, I210, I225-V, I226-V).
- **Realtek Ethernet:** RTL8111 / RTL8168 / RTL8411 PCI-Express Gigabit controllers.
- **VirtIO Network:** virtio-net zero-copy packet processing, AF_XDP / DPDK ring buffers.

### B. Wireless Wi-Fi (IEEE 802.11a/b/g/n/ac/ax/be) — Tier 1 & Tier 2
- **Intel Wi-Fi (iwlwifi / iwlmvm):** Intel Wi-Fi 6 AX200/AX201, Wi-Fi 6E AX210/AX211, Wi-Fi 7 BE200. (Supports WPA2/WPA3 4-Way Handshakes).
- **Atheros / Qualcomm:** ath9k (802.11n), ath10k (802.11ac), ath11k (802.11ax).
- **Realtek Wi-Fi:** RTL8821CE, RTL8822CE, RTL8852AE (Tier 2 via NDIS / Linux bridge shim).

---

## 5. STORAGE CONTROLLERS & FILESYSTEM SUPPORT

| Controller / Device Type | Standard / Specification | Tier | Supported Filesystems |
| :--- | :--- | :--- | :--- |
| **NVMe SSDs** | NVMe 1.3 / 1.4 / 2.0 (PCIe Gen 3/4/5) | Tier 1 | SigmaFS 2.0 (CoW), Ext4, FAT32 |
| **AHCI SATA Drives** | SATA III (6 Gbps) HDDs / SSDs | Tier 1 | SigmaFS 2.0, Btrfs, ZFS shims |
| **USB Mass Storage** | USB 3.2 Gen 1/2 (xHCI), UAS (USB Attached SCSI) | Tier 1 | FAT32, exFAT, Ext4, ISO9660 |

---

## 6. PRINTERS & SCANNERS (PRINT & IMAGING SUBSYSTEM)

### A. Driverless Printing (CUPS / IPP Everywhere) — Tier 1
- **Standard:** IPP Everywhere / AirPrint / Mopria compliant network and USB printers.
- **Supported Vendors:** HP, Canon, Epson, Brother, Xerox, Lexmark, Ricoh (zero proprietary driver installation required).

### B. Vendor-Specific Printing & Scanning Shims — Tier 2
- **HP Lip (HPLIP Shim):** HP LaserJet, DeskJet, OfficeJet series.
- **SANE Scanner Engine:** USB SANE backend for Epson Perfection, Canon CanoScan, Brother MFC flatbed and ADF document scanners.

---

## 7. PERIPHERALS, HUMAN INTERFACE DEVICES (HID) & SOUND

- **USB Host Controllers:** USB 2.0 (EHCI), USB 3.2 / USB4 (xHCI) with dynamic speed negotiation.
- **Keyboards & Mice:** Standard USB HID keyboards, mice, trackballs, and precision touchpads (I2C-HID / Synaptics).
- **Audio Sound Cards:** Intel High Definition Audio (HDA / ALC887/ALC1220/ALC897), AC97 codecs, USB Audio Class 1.0/2.0 DACs.
- **Security Hardware:** TPM 2.0 (Platform Configuration Register attestation & LUKS2 key sealing), OpenTitan hardware root-of-trust.

---

## 8. SUMMARY & VERIFICATION

To verify your system's hardware compatibility with SigmaOS, run the builtin hardware diagnostic tool:

```bash
# Query PCI, USB, and CPU hardware details
sigma-hardware-diag --summary

# Validate secure boot and PQC Dilithium-5 certificates
sigma-boot-check --verify
```

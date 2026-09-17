# SigmaOS Desktop Edition: Hardware Support Matrix

## 1. Tier 1: Primary Target (Certified for M1–M5 Desktop Release)

| Category | Component | Driver / Support Status | Notes |
|---|---|---|---|
| **Platform** | x86_64 UEFI | ✅ Full Support | Standard 64-bit modern PC firmware |
| **Virtualization** | QEMU / KVM (`-machine q35`) | ✅ Full Support | Reference development and test target |
| **Video / Display** | VirtIO-GPU (`virtio-vga`) | ✅ Native Zenith Driver | Hardware acceleration via Wayland scanout |
| **Network** | VirtIO-Net / Realtek 8139/8169 | ✅ Native Driver & XDP | High-throughput zero-copy packet pipeline |
| **Storage** | VirtIO-Block & NVMe 1.4 | ✅ Native Block Driver | Fast boot and Merkle store CoW storage |
| **Input** | Standard USB HID Keyboard & Mouse | ✅ Zenith Evdev / WlSeat | Fluid input processing with low latency |

---

## 2. Tier 2: Physical Hardware Alpha (M4 Target)

| Category | Component | Target Milestone |
|---|---|---|
| **Processor** | Intel Core 8th Gen+ / AMD Ryzen Zen 2+ | M4 Hardware Alpha |
| **Integrated GPU** | Intel UHD/Iris Xe & AMD Radeon Vega/RDNA | M4 Hardware Alpha |
| **Wireless** | Intel Wi-Fi 6 (AX200/AX201) | M4/M5 Hardware Beta |
| **Audio** | Intel HDA / USB Audio Class 2.0 | M4 Hardware Alpha |

---

## 3. Tier 3: Research & Future Architectures (Deferred)

- **ARM64 (AArch64)**: Apple Silicon / Raspberry Pi 5 (Post-M5 release).
- **RISC-V 64**: QEMU virt & VisionFive 2 (Long-term research).
- **Legacy Architectures**: SPARC64, MIPS64, Alpha (Preserved in code, not actively tested for desktop images).

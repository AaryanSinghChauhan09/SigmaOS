# SigmaOS Hardware Support Matrix

SigmaOS aims for broad hardware compatibility through its specialized **Hardware Abstraction Layer (HAL)** shards.

## 💻 Supported Architectures

| Architecture | Status | Support Level |
| :--- | :--- | :--- |
| **x86_64** | **Production** | Full (QEMU & Physical) |
| **ARM64** | **Experimental** | Raspberry Pi 4 / Apple Silicon (Virtual) |
| **RISC-V** | **Roadmap** | T-Head C906 (Conceptual) |

## 🔌 Driver Matrix

| Component | Driver Shard | Status |
| :--- | :--- | :--- |
| **Display** | `VGA_Text` / `VESA_LFB` | Operational |
| **Input** | `PS2_Keyboard` / `PS2_Mouse` | Operational |
| **Storage** | `ATA_IDE` / `AHCI_SATA` | Operational |
| **Network** | `VirtIO_Net` / `E1000` | Testing |
| **Audio** | `Intel_HDA` | Roadmap |

## 🧪 Testing Environment

We primarily validate SigmaOS using **QEMU 7.0+** with the following configuration:
- `cpu`: `host` or `qemu64`
- `memory`: `2GB`
- `acceleration`: `kvm` (if available)
- `display`: `virtio-gpu-pci`

---
*To report hardware issues, please use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.yml).*

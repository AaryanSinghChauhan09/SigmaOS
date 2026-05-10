# SigmaOS Hardware Support Matrix

SigmaOS aims for broad hardware compatibility through its specialized **Hardware Abstraction Layer (HAL)** shards.

## 🐧 Universal Linux Driver Compatibility

To fill technical gaps and ensure drivers work for almost every device, SigmaOS features a [Universal Linux Driver Compatibility Layer](Linux-Driver-Compat.md). This allows SigmaOS to seamlessly load and execute drivers packaged for various Linux distros (Debian, Fedora, Arch, etc.) by wrapping the Linux kernel ABI natively.

## 💻 Supported Architectures | Architecture | Status | Support Level | | :--- | :--- | :--- | | **x86_64** | **Production** | Full (QEMU & Physical) | | **ARM64** | **Experimental** | Raspberry Pi 4 / Apple Silicon (Virtual) | | **RISC-V** | **Roadmap** | T-Head C906 (Conceptual) | ## 🔌 Native Driver Matrix | Component | Driver Shard | Status | | :--- | :--- | :--- | | **Display** | `VGA_Text` / `VESA_LFB` | Operational | | **Input** | `PS2_Keyboard` / `PS2_Mouse` | Operational | | **Storage** | `ATA_IDE` / `AHCI_SATA` | Operational | | **Network** | `VirtIO_Net` / `E1000` | Testing | | **Audio** | `Intel_HDA` | Roadmap | ## 🐧 Ported Linux Drivers (Compat Layer) | Component | Hardware | Driver Module | Status | | :--- | :--- | :--- | :--- | | **Network** | Realtek Ethernet | `RTL8111_Ethernet.cpp` | Operational | | **Network** | Intel Wi-Fi | `Intel_AX200_WiFi.cpp` | Operational | | **Graphics** | AMD Radeon | `AMDGPU_Graphics.cpp` | Beta | | **Graphics** | NVIDIA (Open) | `Nouveau_Graphics.cpp` | Beta | | **Graphics** | Intel Integrated | `Intel_I915_Graphics.cpp` | Operational | | **Storage** | NVMe SSDs | `NVMe_Core.cpp` | Operational | | **Storage** | USB Mass Storage | `USB_Mass_Storage.cpp` | Operational | | **Input/Bus**| USB 3.0 (xHCI) | `USB_XHCI.cpp` | Operational | | **Input** | Synaptics Touchpad | `Synaptics_Input.cpp` | Operational | | **Audio** | Realtek HDA | `Realtek_HDA_Audio.cpp` | Operational | | **Connectivity**| Intel Bluetooth | `Intel_Bluetooth.cpp` | Operational | | **Multimedia** | USB UVC Webcam | `USB_UVC_Camera.cpp` | Operational | | **Storage** | SD/MMC Reader | `SDHCI_CardReader.cpp` | Operational | | **Security** | Fingerprint Reader | `USB_Fingerprint.cpp` | Operational | | **Video** | Backlight Control | `ACPI_Backlight.cpp` | Operational | | **System** | Thermal Management | `Intel_Thermal.cpp` | Operational | ## 🧪 Testing Environment

We primarily validate SigmaOS using **QEMU 7.0+** with the following configuration:

* `cpu`: `host` or `qemu64`
* `memory`: `2GB`
* `acceleration`: `kvm` (if available)
* `display`: `virtio-gpu-pci`

---
*To report hardware issues, please use the [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.yml).*

# Universal Linux Driver Compatibility

SigmaOS implements a robust **Sovereign Linux Compatibility Layer** (`SovereignLinuxCompat.cpp`) to ensure that hardware compatibility is not a bottleneck for user adoption. By wrapping the Linux kernel ABI, SigmaOS can seamlessly load and execute drivers packaged for popular Linux distributions (Debian, Fedora, Arch, etc.).

## 🚀 How It Works

Instead of rewriting every driver from scratch, SigmaOS intercepts calls from Linux Kernel Modules (LKM) and maps them to the native SigmaOS Hardware Abstraction Layer (HAL). This provides:

*   **Universal Compatibility**: Support for nearly any device that has a Linux driver.
*   **Zero-Overhead Execution**: Directly maps memory buffers (like `sk_buff`) without heavy virtualization.
*   **Sandboxing**: Linux drivers are isolated via the `SovereignSandbox`, ensuring they cannot compromise the core SigmaOS kernel.

## 📦 Supported Ecosystems

SigmaOS's wrapper is tested against kernel drivers packaged for:
1.  **Debian/Ubuntu** (`.deb` / `apt` derived modules)
2.  **Fedora/RHEL** (`.rpm` / `dnf` derived modules)
3.  **Arch Linux** (`pacman` / AUR LKM sources)

## 🛠️ Usage

To load a Linux driver manually:
```cpp
linux_compat_load("/lib/modules/linux_driver.ko");
```

## 🔄 Roadmap
*   **Phase 1**: ABI wrapping for Network (WiFi/Ethernet) and Input devices. *(Completed)*
*   **Phase 2**: Full DRM/KMS translation for Linux GPU drivers (NVIDIA, AMD). *(In Progress)*
*   **Phase 3**: Automated driver fetching via `SovereignOrbManager`. *(Planned)*

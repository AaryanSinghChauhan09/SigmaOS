# Intellectual Property & Licensing Compliance

To guarantee that SigmaOS expands its hardware compatibility without breaching Intellectual Property (IP) laws, the OS employs the **Sovereign IP Auditor** (`SovereignIPAuditor.cpp`). [**STATUS: OPERATIONAL**]

## ⚖️ Strict License Enforcement

When the Universal Linux Driver Compatibility Layer attempts to load a Linux Kernel Module (LKM), the Sovereign IP Auditor inspects the module's license tag (e.g., `MODULE_LICENSE("GPL")`).
* **Permitted Licenses**: SigmaOS automatically accepts drivers licensed under open-source agreements such as **GPL, MIT, Apache**, and **BSD**.
* **Blocked Licenses**: Proprietary, closed-source drivers (e.g., official NVIDIA binary blobs) are **strictly blocked by default**. They cannot be loaded into the kernel memory space unless the user explicitly acknowledges a legal waiver.
* **Clean Room Implementations**: For hardware lacking open-source Linux drivers, SigmaOS encourages clean-room reverse engineering. Developers must prove they did not use proprietary vendor code when submitting patches.

## 🤝 Upstream Respect

By using the Compatibility Layer rather than blindly copying Linux source code into the SigmaOS tree, we maintain strict architectural boundaries. The LKM remains an isolated binary that interacts with our HAL, respecting the GPL boundaries of the Linux ecosystem while keeping the SigmaOS kernel pristine.

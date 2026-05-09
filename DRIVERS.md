# Σ SigmaOS Driver Ecosystem

To surpass mainstream distros, SigmaOS utilizes a high-fidelity **Linux Driver Compatibility Layer**.

## 🔌 Supported Devices (Operational)
*   **Wi-Fi**: Intel AX210 (Wi-Fi 6E), Realtek RTL8821CE.
*   **Graphics**: Intel i915, AMDGPU Southern Islands, Vulkan 1.3 Loader.
*   **Multimedia**: USB Video Class (UVC) 1.5.
*   **Storage**: NVMe Core, SATA.

## 🛠️ Porting Instructions
1.  Map the Linux LKM to the `SovereignLibC` shims.
2.  Implement the shard wrapper in `drivers/linux_distros/hardware/`.
3.  Verify the module license with `SovereignIPAuditor`.

## 📦 DKMS-Style Updates
Shard-based drivers allow for "Live-Update". Simply swap the `DRV` shard in the lattice without a reboot.

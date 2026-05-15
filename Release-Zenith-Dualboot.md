# Σ SIGMAOS: ZENITH DUAL-BOOT EDITION (v15.0)

Welcome to the **Sovereign Coexistence Shard**. The Dual-Boot edition is designed to dominate hardware while allowing legacy OSes to exist in partitioned isolation.

## 📥 Installation Guide (Lattice-Merge)

1. **Prepare**: Ensure you have a free partition (min 20GB) alongside your existing OS.
2. **Mount**: Boot from the `sigma-v15.0-dualboot.iso`.
3. **Merge**: Run the `SovereignBridge` installer. It will automatically detect Linux/Windows bootloaders.
4. **Attest**: Configure the **SovereignGRUB** entry.
5. **Sync**: On first boot, run `vfs-sync` to map legacy partitions into the SigmaOS Mesh Lattice.

## 🛠️ Core Functions

- **Legacy Partition Absorption**: Maps NTFS/Ext4/APFS directly into the Sovereign VFS with zero-latency translation.
- **Bootloader Interception**: Protects the boot sequence from legacy OS tampering via PQC signatures.
- **Hardware Direct Access**: Bypasses the need for virtualization, giving SigmaOS raw access to the GPU/NIC even when dual-booting.
- **Cross-OS IPC**: Secure messaging between SigmaOS and legacy guest processes (via SovereignBridge).

## 🌟 Premium Features

- **SovereignVFS Bridge**: Access your Windows/Linux files with bit-perfect integrity and no metadata leaks.
- **Dynamic Partition Resizing**: Safe, on-the-fly shard allocation for the SigmaOS partition.
- **Host-Guest Isolation**: Ensures legacy OS vulnerabilities cannot traverse the partition boundary.
- **Zenith Dual-Compositor**: Switch between SigmaOS and guest displays with a single keystroke.

## 📊 Technical Specs

- **Compatibility**: Linux (Ubuntu, Fedora, Arch), Windows 10/11, macOS (Intel/M1).
- **VFS Drivers**: NTFS, Ext4, Btrfs, APFS, ZFS.
- **Overhead**: < 1% CPU utilization for the Bridge shard.

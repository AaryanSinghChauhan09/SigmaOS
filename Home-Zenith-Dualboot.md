# Î£ SigmaOS v15.0 Zenith: Dual-Boot Edition

## ðŸ¤ Coexistence & Power

The **Dual-Boot Edition** is engineered for users who require the power of the Sovereign Lattice alongside legacy environments (Windows/Linux). It includes advanced partitioning logic and the `S-GRUB` recovery shard.

### ðŸ› ï¸ Key Features

- **Sovereign Partitioning (SPT)**: Safely resize legacy partitions to create shard space.

- **Cross-Lattice Bridge**: Read-only access to NTFS and Ext4 volumes via the S-FS shard.

- **Unified Bootloader**: PQC-signed entry points for all installed OSes.

- **Hybrid Security**: Shield legacy partitions from shard-level anomalies.

### ðŸ“¥ Installation Guide (Coexistence)

1. **Prepare Media**: Use `SigmaOS-v15.0-Zenith-Dualboot.iso`.

2. **Space Allocation**: Ensure at least 100GB of unallocated space or a secondary drive.

3. **Ignition**: Boot from media. Select "Hybrid Lattice Integration".

4. **Bootloader**: The installer will detect Windows/Linux and integrate them into the `S-GRUB` menu.

5. **Validation**: Run `sigma-pkg verify-boot` after the first login to ensure total integrity.

### ðŸ’Ž Exclusive Functions

- `legacy-mount`: Mount Windows/Linux drives with automatic PQC integrity scanning.

- `boot-manager-gui`: Visually manage boot priorities and themes.

---
[Return to Global Home](Home)

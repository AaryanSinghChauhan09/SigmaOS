# Σ SIGMAOS: Multi-OS Coexistence (release/dual-boot)

The `release/dual-boot` branch focuses on the seamless integration of SigmaOS with existing hardware environments alongside legacy operating systems.

## ⚙️ Performance Focus

- **Bootloader Optimization**: Validating the Sovereign Bootloader for sub-second OS selection.

- **Context Switching**: Minimizing overhead when transitioning between SigmaOS and legacy kernels.

- **Hardware Direct Access**: Ensuring zero-latency hardware passthrough for industrial shards.

## 🔒 Security & Isolation

- **Cross-OS Leakage Prevention**: Hardened memory protection (SMAP/SMEP) to ensure total isolation from legacy OSes.

- **Persistence Validation**: Verifying amnesic persistence routines to prevent data remanence across reboots.

## 🧪 Testing & Validation

- **Compatibility Matrix**: Automated testing against diverse UEFI/BIOS and partition configurations.

- **Integrity Audits**: Continuous verification of the sovereign boot partition during ASI ignition.

*"Coexistence is the first step toward sovereignty."*

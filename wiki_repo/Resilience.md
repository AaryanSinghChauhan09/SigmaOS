# SigmaOS Resilience & Sovereignty

SigmaOS is designed to survive hostile environments, hardware failures, and catastrophic software panics without losing data or network connectivity.

## 1. Resilient Mode Fallback (`sigma_micro_fallback`)
If the main scheduler, VMM, or zero-trust enforcer encounters an unrecoverable state:
- The system drops into **Resilient Mode**, a tightly scoped micro-kernel loop that requires zero external dependencies or memory allocations.
- Provides a minimal rescue shell via the serial console to allow operators to dump logs, trigger ACPI resets, or run filesystem repairs.

## 2. Self-Healing Subsystem & Live Patching (`SovereignSelfHealingKernel`)
- Monitors kernel health via custom registered watchpoints.
- Automatically applies live kernel patches without reboot when non-critical or recoverable faults occur (Clear Linux/SystemRescue-style).
- Schedules `kexec` reloads for critical subsystem failures.

## 3. Kernel Integrity Checker (`SovereignKernelIntegrityChecker`)
- Implements continuous runtime memory hashing based on FNV-1a.
- Compares active memory block signatures against baseline hashes captured during boot.
- Triggers instant security flags or panic mitigation when unauthorized mutations are detected.

## 4. Decentralized Peer-to-Peer Updates (`sigma_p2p_update`)
- Utilizes the `sigma_mesh_protocol` DHT to locate peers.
- Signed `.spkg` updates (verified via Dilithium PQC) are distributed in chunks across the mesh network.
- Ensures updates can propagate even in air-gapped or localized intranet scenarios.

## 5. Hardware-Bound Identity (`sigma_hw_identity`)
- Derives a unique, unforgeable identity (UDI) by hashing silicon fingerprints (e.g., TPM Endorsement Keys, CPU Serials, PCIe topology).
- Prevents drive cloning and ensures that the OS enclave only boots on the exact hardware it was provisioned for.

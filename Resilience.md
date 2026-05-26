# SigmaOS Resilience & Sovereignty

SigmaOS is designed to survive hostile environments, hardware failures, and catastrophic software panics without losing data or network connectivity.

## 1. Resilient Mode Fallback (`sigma_micro_fallback`)
If the main scheduler, VMM, or zero-trust enforcer encounters an unrecoverable state:
- The system drops into **Resilient Mode**, a tightly scoped micro-kernel loop that requires zero external dependencies or memory allocations.
- Provides a minimal rescue shell via the serial console to allow operators to dump logs, trigger ACPI resets, or run filesystem repairs.

## 2. Self-Healing Processes (`sigma_self_heal`)
Crucial system daemons are registered with the self-healing subsystem:
- Defined policies (`POLICY_ALWAYS`, `POLICY_ON_FAILURE`) dictate how the kernel should react to process segmentation faults.
- The kernel will instantly respawn crashed daemons (e.g., the network stack or UI renderer) without requiring a full system reboot.

## 3. Decentralized Peer-to-Peer Updates (`sigma_p2p_update`)
To maintain digital sovereignty, SigmaOS nodes do not rely solely on centralized update servers:
- Utilizes the `sigma_mesh_protocol` DHT to locate peers.
- Signed `.spkg` updates (verified via Dilithium PQC) are distributed in chunks across the mesh network.
- Ensures updates can propagate even in air-gapped or localized intranet scenarios.

## 4. Hardware-Bound Identity (`sigma_hw_identity`)
- Derives a unique, unforgeable identity (UDI) by hashing silicon fingerprints (e.g., TPM Endorsement Keys, CPU Serials, PCIe topology).
- Prevents drive cloning and ensures that the OS enclave only boots on the exact hardware it was provisioned for.

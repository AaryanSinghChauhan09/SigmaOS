# SigmaOS: Sovereign Threat Model

## Assets & Trust Boundaries

| Asset | Description | Sensitivity |
| :--- | :--- | :--- |
| **Kernel Shards** | Core logic for memory, scheduling, and security. | CRITICAL |
| **Lattice State** | Global system configuration and runtime shard mapping. | CRITICAL |
| **User Data** | Files and artifacts stored within SovereignVFS. | HIGH |
| **Silicon Bus** | Raw hardware access and instruction execution. | CRITICAL |

## Adversary Model

1. **Malicious Shard**: An application or driver that attempts to escape its sandbox or consume excessive silicon resources.
2. **Lattice Intruder**: An unauthorized node attempting to join the Sovereign Lattice and intercept IPC traffic.
3. **Hardware-Level Attacker**: Physical access to the silicon bus or memory attempting to bypass TEE/SGX protections.

## Mitigations & Controls

- **Capability-Based Security**: Shards have zero default permissions. Every syscall is tokenized and verified.
- **LBSV (Lattice-Based Shard Verification)**: All binaries are cryptographically signed and verified before execution.
- **Amnesic Execution**: Linear memory is scrubbed upon process termination to prevent data remnants.
- **Hardware Attestation**: Real-time measurement of the execution environment against a silicon-native root of trust.

## Out of Scope

- **User Phishing**: Protection against users executing malicious NLP commands voluntarily (handled via UI warnings).
- **Physical Destruction**: Protection against physical hardware destruction (handled via lattice redundancy).

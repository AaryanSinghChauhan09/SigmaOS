# 🛡️ SigmaOS Cyber Security & Capability-Based Threat Model

This document establishes the official cyber security architecture, threat model, and capability-ring boundaries governing the **SigmaOS Sovereign Microkernel**.

---

## 1. 🌐 The Sovereign Zero-Trust Security Paradigm

SigmaOS completely departs from the legacy Unix DAC (Discretionary Access Control) paradigm, where any process running with `UID 0` (root) possesses ambient, unrestricted authority over the entire operating system.

### Core Architecture Principles:
*   **Principle of Least Privilege**: A process is initialized with zero permissions and must explicitly present a verified capability token to invoke privileged system calls.
*   **Encapsulated Capability Rings**: The monolithic Linux kernel operates entirely in Ring 0. SigmaOS decomposes system responsibilities into independent user-space **Shards** running in Ring 3. Shards communicate exclusively via capability-checked message passing on the Sovereign IPC Bus.
*   **Post-Quantum Cryptography (PQC) Native Verification**: All dynamic drivers, loadable shards, and software packages must be cryptographically signed and verified using NIST FIPS 203/204 standard algorithms (`Kyber-1024` KEM and `Dilithium-5` signatures).

---

## 2. 📊 Threat Modeling and Attack Vector Mitigation

| Threat Vector | Legacy Linux Monolithic Vuln | SigmaOS Mitigation Paradigm | Blast Radius |
| :--- | :--- | :--- | :--- |
| **Driver Privilege Escalation** | A buggy driver in Ring 0 triggers a full kernel panic or allows local arbitrary code execution. | Drivers run as isolated Ring 3 Shards. System is isolated via `S-UDA` wrapper sandboxing. | **Negligible**: Faulty driver is auto-restarted without affecting the core kernel. |
| **Ambient Authority Abuse** | Compromised userland applications can read/write administrative structures if spawned under root or sudo. | Process privileges are locked at launch using `PledgePromise`. No ambient "root" exists. | **Minimal**: Application is restricted solely to declared capabilities (e.g., `stdio` only). |
| **Insecure IPC Injection** | Sniffing sockets, local pipes, or shared memory segments allows local data disclosure and spoofing. | All communication is routed through capability-checked channels managed by the `IpcManager` and `CapabilityGate`. | **Zero**: Unauthorized shards are blocked from subscribing to channels. |
| **Integer Overflow Manipulation** | Maliciously large filesystem writes trigger integer overflows, leading to heap corruption or out-of-bounds reads. | Filesystem size and offset increments are protected using compiler-checked `checked_add()` and `checked_sub()`. | **Negligible**: Errant write triggers a clean `FsError` and terminates immediately. |

---

## 🔒 3. Capability Delegation & Sandbox Gates

Every thread group or process executing in SigmaOS is assigned a `CapabilityToken` that acts as its secure access passport:

```rust
// Declaring a capability token
let token = CapabilityToken::new()
    .allow_network("tcp", 443)
    .allow_read("/var/www");
```

System calls are intercepted at the `CapabilityGate` microkernel boundary. The gate verifies that the calling process has the specific permission mapped to its active capability bitmask:

```rust
// Microkernel system call validation
if !gate.validate_syscall(Permission::NetworkTcp) {
    return Err(PledgeError::Violation);
}
```

By separating capabilities, sandboxing legacy hardware adapters, and enforcing post-quantum signatures, SigmaOS provides a mathematically resilient environment optimized for critical industrial, sovereign, and AI-native applications.

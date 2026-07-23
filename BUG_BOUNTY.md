# 🐛 SigmaOS Bug Bounty & Vulnerability Triage Guidelines

Welcome to the **SigmaOS Bug Bounty Program**. As a zero-trust, bare-metal sovereign operating system, we prioritize the cryptographic and structural security of our microkernel above all else.

This document outlines our vulnerability classification standards, capability-ring boundaries, and triage procedures.

---

## 1. 🛡️ Capability-Ring & Security Boundaries

In SigmaOS, vulnerabilities are assessed based on their ability to breach established security boundaries. Reports that demonstrate a true boundary cross are prioritized and triaged immediately.

### Prioritized Security Boundaries:
1.  **Microkernel Ring 0 Isolation**: The boundary between isolated user-space driver shards (running in Ring 3) and the microkernel core (running in Ring 0). Demonstrating arbitrary code execution in the microkernel core via a compromised driver Shard is classified as a **Critical** vulnerability.
2.  **Capability Delegation (`CapabilityGate`)**: The boundary preventing unauthorized system calls. Bypassing `CapabilityGate` checks to execute a privileged syscall (e.g., establishing a TCP connection without holding the `NetworkTcp` capability token) is classified as a **High/Critical** vulnerability.
3.  **Privilege Mitigation (`PledgeManager`)**: The boundary preventing a pledged process from escalating its permissions or violating its declared `PledgePromise` after activation.
4.  **Sandbox Isolation (`S-UDA`)**: The boundary containing legacy or wrapped hardware drivers. Escaping a sandboxed compatibility driver page to manipulate the memory of another active user-space Shard is classified as a **High** vulnerability.

---

## 📊 2. Vulnerability Severity Classification

We classify reported vulnerabilities into four clear severity categories based on threat model impact and exploit reproducibility.

| Severity | Description / Example | Priority |
| :--- | :--- | :--- |
| **Critical** | - Remote code execution (RCE) in the microkernel core.<br>- Bypass of the post-quantum signature verification engine (`Dilithium-5`).<br>- Arbitrary memory read/write across secure Ring 3 shard pages. | **P1 (Immediate)** |
| **High** | - Privilege escalation bypassing `PledgePromise` restrictions.<br>- Direct bypass of `CapabilityGate` validation checks.<br>- Secure credential disclosure within the local key store. | **P2 (High)** |
| **Medium** | - Denial of Service (DoS) attacks causing a crash in a core system shard (e.g., the local network stack Shard).<br>- Checked arithmetic overflow leading to minor local buffer errors inside sandboxed adapters. | **P3 (Normal)** |
| **Low** | - Minor memory leaks inside non-critical system widgets.<br>- Informational leaks or documentation inconsistencies. | **P4 (Low)** |

---

## 📥 3. Reporting a Vulnerability

If you discover a security vulnerability or boundary breach within SigmaOS, please report it securely:

1.  **Secure Communication**: Do not open a public GitHub issue. Send your report directly to the security team at **security@sigmaos.org**.
2.  **Required Deliverables**:
    - A detailed explanation of the vulnerability and the impacted security boundary.
    - A clean, minimal Proof of Concept (PoC) showing how to reproduce the issue.
    - Your suggested mitigation strategy or architectural patch.
3.  **Encrypted Submissions**: We highly encourage encrypting your submission utilizing our official public `Kyber-1024` secure communication key.

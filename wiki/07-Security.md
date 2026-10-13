# 07 Security Architecture, Hardening & Governance

The SigmaOS Security Subsystem provides defense-in-depth isolation, mandatory access control, cryptographic attestation, process sandboxing, and statutory compliance monitoring across the kernel and userspace stack.

## Core Security Pillars

1. **Kernel Privilege Boundaries & Syscall Dispatching:**
   - Strict Ring 3 user mode TSS and Ring 0 kernel isolation.
   - Fault-safe user memory validation, pointer sanitization, and length checks.
   - Retpoline, KPTI, SMEP/SMAP, and KASLR mitigations.

2. **Mandatory Access Control (MAC) & Multi-Level Security (MLS):**
   - SELinux Type Enforcement and AppArmor path-based profile matching.
   - FreeBSD MAC Framework policies (`mac_biba`, `mac_mls`).

3. **Capability Sandboxing & Isolation:**
   - OpenBSD `pledge()` syscall restriction and `unveil()` filesystem view hiding.
   - FreeBSD Capsicum `cap_rights_limit` descriptor capability sandboxing.
   - Linux Landlock v5 unprivileged file and network access rulesets.
   - Seccomp BPF JIT syscall filtering with `SECCOMP_RET_KILL_PROCESS`.

4. **Cryptographic Attestation & Hardware Root of Trust:**
   - Post-Quantum Cryptography (PQC) Dilithium5 signature verification and Kyber1024 key encapsulation.
   - TPM 2.0 PCR bank measurements, sealing, unsealing, and remote attestation.
   - UEFI Secure Boot key database (`PK`, `KEK`, `db`, `dbx`) validation.

5. **Filesystem Encryption & Vault Protection:**
   - `fscrypt` directory policy enforcement with PQC/AES-256/ChaCha20-Poly1305.
   - Encrypted File Vault and LUKS2 container management.

6. **Audit, Intrusion Detection & Compliance:**
   - `auditd` syscall event monitoring and watch rule compiler.
   - `fail2ban` log pattern parsing with automated packet filter shunting.
   - AIDE / Tripwire cryptographically verifiable file integrity database.
   - Lynis enterprise security auditing and compliance score engine.

7. **Security Governance & SSDF Alignment:**
   - Full alignment with NIST SP 800-218 (SSDF v1.1).
   - Evidence-based status tracking (**Proposed, Prototype, Integrated, Supported**).

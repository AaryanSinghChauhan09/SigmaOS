# AI Agent Security Management Protocol for SigmaOS

This document defines mandatory security management protocols, threat-modeling guidelines, vulnerability mitigation procedures, and cryptographic governance standards for AI agents (Jules, Sentinel, Bolt, Palette, and subagents) operating on the **SigmaOS** repository.

---

## 1. Capability-Based Access Control Auditing

All kernel syscalls, VFS file operations, network socket operations, and process interactions in SigmaOS are governed by fine-grained bitmask capability gates (`src/security/capability.rs`):

- **Capability Verification Gate**: AI agents MUST ensure every privileged operation checks corresponding capabilities (e.g., `Capability::PROCESS_EXEC`, `Capability::FILE_READ`, `Capability::NET_ADMIN`, `Capability::SYS_BOOT`).
- **Bitmask Overlap Prevention**: Ensure new capability definitions use non-overlapping bitwise left-shifts (`1 << N`) up to `u64` limits.
- **Revocation Testing**: Any modification to capability models MUST include unit tests verifying capability grant, check, and revocation behavior.

---

## 2. Least Privilege Sandboxing (OpenBSD `pledge` & `unveil` Rule)

AI agents introducing or modifying userland tools, shell implementations (`src/shell/sigma_sh.rs`), or daemon services MUST enforce least-privilege sandboxing:

- **Path Restrictions (`unveil(2)`)**:
  - Restrict file paths accessible to processes using `OpenBsdUnveilPathSandbox` or `OpenBsdPledgeUnveilSandboxGovernor` (`src/distro/missing_distro_innovations.rs`).
  - Read-only paths MUST be unveiled with `"r"`, data output directories with `"rw"`, and binaries with `"rx"`. Unveil MUST be locked via `unveil(NULL, NULL)` before entering untrusted execution loops.
- **Syscall Promises (`pledge(2)`)**:
  - Processes MUST pledge minimum required promise sets (e.g., `"stdio rpath wpath cpath inet"`).
  - Attempted execution of unpledged syscalls MUST trigger immediate process termination and audit logging (`SovereignSyscallAuditLogger`).

---

## 3. Post-Quantum Cryptography & Hardware Enclaves

- **PQC Key Exchange & Signatures**:
  - Cryptographic operations MUST support Dilithium-5 signatures and Kyber key encapsulation (`src/security/pqc_enclave.rs`).
- **TPM 2.0 PCR Attestation**:
  - Hardware attestation gates (`PqcTpmHardwareEnclaveGate`) MUST verify Platform Configuration Registers (PCRs 0–7) before unsealing disk encryption keys or PQC identity tokens.
- **Package Manifest Signing**:
  - Packages and release artifacts MUST be verified via Sigstore / Cosign / OpenBSD signify signatures (`OpenBsdSignifyPackageReproducer` in `src/sigpkg/declarative_build.rs`).

---

## 4. Hardcoded Secret & Credential Prevention

AI agents MUST prevent credentials or private keys from entering source code:

1. **Scanner Rules**: The automated secret detection workflow scans for assignments matching `password`, `secret`, `api_key`, or `token`.
2. **Bypass Rules**:
   - Variables in unit tests, test suites, or mock configs MUST contain `test`, `mock`, `example`, or `TODO` in their variable identifiers.
   - Example: `let mock_api_token = "example_secret_token_123";`

---

## 5. Amnesic Memory Scrubbing & Defense-in-Depth

- **Volatile Scrubbing**:
  - Memory buffers containing sensitive material (passwords, private keys, session tokens) MUST be zeroed out immediately after use or on process exit.
- **Guard Pages & ASLR**:
  - Stack and heap allocations MUST utilize hardened guard pages (`HardenedGuardPageAllocator` in `src/memory/resource_allocator.rs`) and Address Space Layout Randomization.

---

## 6. Security Vulnerability Patching Workflow

When an AI agent identifies or receives a security vulnerability alert:

```
+-------------------------------------------------------------------------------+
|                      AI AGENT SECURITY PATCH WORKFLOW                         |
+-------------------------------------------------------------------------------+
|  1. Isolate Root Cause  ->  2. Write Failing Test  ->  3. Apply Least-Priv Fix|
|     (Analyze Vulnerability)   (Reproduce Flaw)           (Minimal Diff Patch) |
+-------------------------------------------------------------------------------+
|  4. Run Audit Suite     ->  5. Update Sentinel Log ->  6. Submit PR           |
|     (run_sigma_tests.sh)      (.jules/sentinel.md)       (docs/AGENTS_PR_...)  |
+-------------------------------------------------------------------------------+
```

# SigmaOS Security Management Plan & Architecture

## Overview

SigmaOS is an experimental Rust operating system tracking component maturity as **Proposed, Prototype, Integrated, or Supported**.
The project security management plan follows NIST's Secure Software Development Framework (SSDF) [NIST SP 800-218] and mature distribution security practices (Debian, OpenBSD, FreeBSD).

---

## Baseline Security Rules

- **Evidence-Based Security Status:** "Supported" status requires that the feature passed documented runtime, security, and recovery tests for its declared target. Source code or unit tests alone do not constitute evidence of a secure system capability.
- **One PR, One Outcome:** Keep fixes narrowly scoped; separate refactoring from vulnerability remediation.
- **Private Disclosure:** Keep vulnerability details private until coordinated disclosure.
- **Risk-Based Triage:** Triage exploitability in SigmaOS's architecture rather than relying solely on third-party scanner scores.
- **Human Review Gate:** Do not merge security-sensitive changes on green CI alone. Require maintainer approval and two qualified reviewers for kernel privilege boundaries, cryptography, boot/update trust, sandboxing, or security policy changes.
- **Release Evidence:** No security claim without release evidence (build/test logs, advisories, signed metadata, SBOM).

---

## 16-PR Security Roadmap

| PR | Proposed Title | Objective |
|---|---|---|
| 1 | `docs(security): establish threat model and security ownership` | System threat model covering boot, kernel, drivers, userspace, package/update flow, desktop, network, and CI. |
| 2 | `docs(security): publish private vulnerability reporting and disclosure policy` | Private contact channel and coordinated disclosure SLA. |
| 3 | `ci(security): add Rust dependency and license policy checks` | Scheduled and PR-triggered Cargo lockfile vulnerability auditing (`cargo-audit`). |
| 4 | `ci(security): add secret, unsafe-code, and static-analysis checks` | Credential scanning and mandatory safety rationale for `unsafe` Rust. |
| 5 | `test(security): establish fuzzing and sanitization coverage` | Fuzz targets for parsers, syscalls, filesystems, and network packets. |
| 6 | `security(kernel): define syscall and privilege-boundary invariants` | Strict pointer/length validation and permission enforcement. |
| 7 | `security(memory): add kernel memory hardening and fault tests` | Memory mapping permissions, use-after-free, and address space isolation. |
| 8 | `security(drivers): add driver threat model and safe failure rules` | DMA buffer bounds, device reset rules, and malformed descriptor handling. |
| 9 | `security(crypto): define cryptographic API and key-handling policy` | Approved crypto primitives, key zeroization, and secret scrubbing. |
| 10 | `security(boot): specify verified boot and recovery trust model` | Measured boot, signature verification, and secure recovery. |
| 11 | `security(updates): implement signed, atomic, recoverable update policy` | Transactional updates with rollback protection. |
| 12 | `security(pkg): add package provenance and SBOM generation` | Release SBOMs and SLSA provenance metadata. |
| 13 | `security(release): establish advisory and backport workflow` | Security advisories and targeted patch backports for stable branches. |
| 14 | `security(services): define least-privilege and default exposure policy` | Default-off services and fine-grained capability sandboxing. |
| 15 | `ci(security): protect security workflows and release credentials` | Immutable action pinning and workflow token privilege minimization. |
| 16 | `docs(security): publish security status and supported-configuration matrix` | Transparent mapping of security capabilities to evidence and maturity status. |

---

## Component-Specific Rules

- **Kernel & Syscalls:** All userspace input is untrusted. Pointer, length, handle, and privilege checks are mandatory.
- **Memory Management:** Invariants required for page permissions, W^X, separation, and reclamation.
- **Filesystems:** Disk images, metadata, and filenames treated as untrusted attacker input.
- **Drivers:** Hardware responses treated as untrusted; DMA buffers and descriptors strictly bounded.
- **Networking:** Packets treated as untrusted; rate limits and protocol parser bounds enforced.
- **Crypto & Keys:** Reviewed primitives only; key custody rules; no secrets checked into git.
- **Init & Services:** Least privilege default; no default unauthenticated listeners.
- **Installer & Updates:** Verification before installation; atomic transactional updates.
- **Desktop & Applications:** Application sandboxing separate from kernel privileges.
- **Build & Release:** PR execution isolated from release credentials; SLSA provenance generated.

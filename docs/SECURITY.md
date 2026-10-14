# SigmaOS Security Management Plan

## Executive Summary

SigmaOS is an experimental, sovereign operating system written in Rust. Component maturity across the codebase is tracked explicitly using four states: **Proposed, Prototype, Integrated, or Supported**. A unit-tested subsystem model or source file alone does not constitute proof of a complete, secure boot, kernel, or userspace path.

This Security Management Plan establishes a comprehensive security baseline for SigmaOS aligned with NIST SP 800-218 (Secure Software Development Framework - SSDF) and best practices from mature Linux & BSD distributions (Debian, FreeBSD, OpenBSD, Fedora, Arch Linux).

---

## Baseline Security Rules

Apply these rules strictly to all security-related PRs, code changes, and release builds:

1. **Security status requires evidence:** "Supported" status strictly requires documented runtime, security, and recovery test results on declared hardware or hypervisor targets. Unit-tested source files do not constitute proof of supported security capability.
2. **One PR, one reviewable security outcome:** Keep security PRs narrowly scoped. Separate refactorings from security updates. Stable branches must receive minimal targeted patches to avoid regressions.
3. **Keep vulnerability details private until coordinated disclosure:** Maintain confidential intake channels (via `security@sigmaos.org` or GitHub Private Vulnerability Reporting). Never post zero-day details or unpatched exploits in public issues or PRs.
4. **Prioritize risk in product context:** Evaluate exploitability based on SigmaOS architecture, attack surface, privilege requirements, and system impact, rather than relying solely on automated CVSS scores.
5. **Human review required for security-sensitive changes:** Security-critical changes (kernel boundaries, crypto, boot trust, sandboxing, updates, authentication) require human review and approval from two qualified maintainers. CI green status alone is insufficient.
6. **No security claim without release evidence:** Releases must publish exact commit revisions, build/test artifacts, advisory references, package signatures, and SBOM metadata.

---

## Proposed Security PR Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(security): establish threat model and security ownership` | Threat model covering boot, kernel, drivers, userspace, package/update flow, desktop, network services, build infrastructure, and contributor access. Define security owners and escalation path. | Explicit threat actors, assets, trust boundaries, assumptions, non-goals, and review ownership; links reconciled; no duplicate policy. |
| 2 | `docs(security): publish private vulnerability reporting and disclosure policy` | Configure private vulnerability reporting channel; document report fields, confidentiality, acknowledgement, triage, coordination, and disclosure process. | Test report submitted privately; clear reporting instructions; public issues redirect security reports to private intake. |
| 3 | `ci(security): add Rust dependency and license policy checks` | Cargo.lock dependency auditing (`cargo-audit`); approved sources, license policy, exception process, duplicate/outdated dependency review. | CI detects known RustSec advisories and disallowed dependencies; time-bounded owner-approved exceptions. |
| 4 | `ci(security): add secret, unsafe-code, and static-analysis checks` | Secret scanning, unsafe Rust inventory, linting/static analysis (`clippy`, `semgrep`), review process for `unsafe` and FFI. | New secrets blocked/flagged; unsafe blocks require safety comments and tests; findings triaged. |
| 5 | `test(security): establish fuzzing and sanitization coverage` | Audit `fuzz/` coverage; prioritize parsers, syscalls, filesystem metadata, package inputs, IPC, network, image/config formats. | Fuzz targets reproducible; crashes minimized and turned into regression tests; bounded resource use. |
| 6 | `security(kernel): define syscall and privilege-boundary invariants` | Document syscall validation, pointer/length checks, handle ownership, namespace boundaries, and kernel/user copy rules. | Tests cover invalid pointers/lengths, unauthorized access, resource exhaustion, and boundary crossings. |
| 7 | `security(memory): add kernel memory hardening and fault tests` | Allocator, paging, process isolation, memory reclamation; invariant checks for UAF, double-free, overflow, mapping permissions. | Invariants tested or limitations documented; debug assertions not mistaken for mitigations; fault handling leaks no sensitive memory. |
| 8 | `security(drivers): add driver threat model and safe failure rules` | Driver trust assumptions, DMA/buffer ownership, device reset behavior, malformed descriptor handling, privilege separation. | Device input cannot cause unbounded allocation; reset/error paths tested; unsupported devices explicitly documented. |
| 9 | `security(crypto): define cryptographic API and key-handling policy` | Cryptographic algorithm inventory; approved primitives, randomness, key lifecycle, zeroization, deprecation process. | No unreviewed custom crypto; test vectors and negative tests exist; zeroization verified; secrets excluded from logs. |
| 10 | `security(boot): specify verified boot and recovery trust model` | Boot artifact trust, signing keys, key custody/rotation, verification failure handling, recovery image policy, rollback protection. | Artifact provenance and integrity verified; failure is closed/safe; recovery does not bypass verification. |
| 11 | `security(updates): implement signed, atomic, recoverable update policy` | Signed metadata and packages, freshness/rollback checks, transactional install, interruption recovery, downgrade prevention. | Tampered/unsigned artifacts rejected; interrupted updates recover to known state; rollback tested. |
| 12 | `security(pkg): add package provenance and SBOM generation` | SBOM generation (SPDX/CycloneDX), source revision recording, build inputs, dependency inventory, provenance verification. | SBOM matches shipped binaries; provenance identifies source and workflow; verification tested. |
| 13 | `security(release): establish advisory and backport workflow` | Security advisory template, version table, severity rationale, mitigations, backport decision framework. | Advisories describe impact and fixed versions; minimal targeted backports to stable branches. |
| 14 | `security(services): define least-privilege and default exposure policy` | Init/services, network listeners, desktop components, privilege helpers audit; explicit service enablement and sandboxing. | Default install exposes minimal necessary services; privilege rationale recorded; unauthorized access denied. |
| 15 | `ci(security): protect security workflows and release credentials` | GitHub Actions permissions, pinned action references, secret isolation, branch rules, trusted workflows for publishing. | Protected branches enforce reviews; release secrets hidden from untrusted PRs; workflows security-reviewed. |
| 16 | `docs(security): publish security status and supported-configuration matrix` | Map security capabilities to code, tests, threat assumptions, and maturity status (Proposed, Prototype, Integrated, Supported). | No unsupported feature claimed as Supported; all claims linked to test evidence; gaps tracked. |

---

## Periodic Security Cadence

| Cadence | Task | Required Output |
|---|---|---|
| **Every PR** | Required CI security checks | Build, unit/integration tests, secret scan, clippy lint, changed component validation. |
| **Weekly** | RustSec advisory review | Dependency audit, reachability analysis, upgrade plan or documented non-applicability. |
| **Weekly** | Incoming report triage | Private vulnerability case log, severity assessment, assignees, target fix date. |
| **Monthly** | CI workflow & action review | Audit workflow permissions, pinned action revisions, secret access controls. |
| **Monthly** | Fuzzing & crash disposition | Crash triage, regression tests, coverage gap analysis, resource limit checks. |
| **Monthly** | Maturity & feature status reconciliation | Synchronize `FEATURE_STATUS.toml`, documentation, and runtime evidence. |
| **Quarterly** | Threat model re-evaluation | Review attack surface changes across boot, kernel, drivers, network, package manager. |
| **Quarterly** | Supply chain & dependency assessment | Vendor audit, license compliance, SBOM validation, build isolation review. |
| **Release Candidate** | Release candidate security certification | Test execution evidence, zero unpatched high/critical advisories, signed artifacts, verified SBOM. |
| **Post-Incident** | Incident root-cause fix | Root cause analysis, regression test addition, backport deployment, policy improvement. |

---

## Component-Specific Security Rules

- **Kernel & Syscalls:** All user pointers, lengths, flags, handles, and buffer sizes must be strictly validated before dereference. Return `EFAULT` / `EINVAL` on invalid user input.
- **Memory Management:** Enforce W^X (Write XOR Execute), KASLR, guard pages, and zero-on-free memory allocation. Enforce page table permission limits.
- **Filesystems & Storage:** Mount images and metadata in fail-closed mode. Bound recursion and buffer allocation when parsing ext4, Btrfs, FAT, ISO9000, and tar/zip archives.
- **Drivers & DMA:** Require strict DMA buffer boundary checks and IOMMU mapping isolation. Handle hardware timeouts gracefully without kernel panics.
- **Networking:** Treat all incoming packets and frame headers as untrusted. Bounded buffer allocations, socket timeout limits, and rate-limiting on incoming connections.
- **Crypto & Key Management:** Enforce FIPS-compliant algorithms (AES-256-GCM, ChaCha20-Poly1305, Ed25519, Dilithium5/Kyber1024 PQC). Zeroize sensitive key material on drop (`Zeroize`).
- **Services & Init:** Principle of least privilege. Services run with minimal capabilities in isolated namespaces, pledge promises, or Landlock rulesets.
- **Updates & Packages:** Cryptographic signature verification (`sigpkg`, Signify, Cosign, GPG, PQC) before extraction or installation. Rollback snapshot creation before upgrades.
- **Build & Release Pipelines:** Hermetic builds, pinned dependencies, double-reviewer approvals on workflow changes, token permission minimization (`permissions: read-all`).

---

## Security PR Review and Merge Gates

Security PRs require the following PR template format:

```text
## Summary
What risk or security requirement does this PR address?

## Scope
Affected components, files, release branches, and explicitly excluded work.

## Security classification
Public hardening | Security fix | Dependency update | Policy/docs
(Do not include sensitive exploit details in public PRs)

## Threat model
Attacker capability, affected assets, trust boundary, and assumptions.

## Impact and versions
Affected configurations/releases, exploitability rationale, and known mitigations.

## Implementation
Why this fix is appropriate; compatibility/API impact; backport considerations.

## Tests and evidence
Regression test, fuzz/property tests, static analysis, integration/runtime results.

## Disclosure and release
Private report reference (if applicable), coordination status, advisory/release plan.

## Checklist
- [ ] Minimal, reviewable change
- [ ] Security tests or regression tests added
- [ ] No secrets or exploit details exposed publicly
- [ ] Docs and component status updated
- [ ] CI passes and review conversations resolved
- [ ] Backport/recovery plan recorded where relevant
```

For security boundary changes (kernel, crypto, boot trust, sandboxing, authentication), approvals from **two qualified security maintainers** are required. Direct pushes and force pushes are strictly disabled.

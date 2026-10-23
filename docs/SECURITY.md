# SigmaOS Security Management Plan

Aligned with NIST SP 800-218 (Secure Software Development Framework - SSDF) and Linux/BSD distribution security practices.

## Overview
SigmaOS is an experimental Rust operating system tracking component maturity as **Proposed, Prototype, Integrated, or Supported**. This plan outlines baseline security rules, a 16-PR roadmap, periodic security maintenance cadences, component-specific security rules, PR review/merge gates, and a rollout plan.

## Baseline Rules
1. **Security status requires evidence:** "Supported" requires documented runtime, security, and recovery test results on target hardware or hypervisors.
2. **One PR, one reviewable security outcome:** Keep fixes narrowly scoped; separate refactoring from vulnerability remediation.
3. **Keep vulnerability details private until coordinated disclosure:** Use private reporting channels (`security@sigmaos.org` / GitHub Private Vulnerability Reporting).
4. **Prioritize risk, not scanner scores alone:** Triage exploitability in SigmaOS's architecture, affected versions, attack surface, and required privileges.
5. **Do not merge security-sensitive changes on green CI alone:** Require human review from a subsystem maintainer; require two qualified reviewers for kernel privilege boundaries, crypto, boot/update trust, sandboxing, or security policy changes.
6. **No security claim without release evidence:** Releases must identify source revisions, build/test evidence, advisories, and artifact integrity information.

## Proposed PR Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(security): establish threat model and security ownership` | Audit existing security docs; add system threat model covering boot, kernel, drivers, userspace, package/update flow, desktop, network services, build infrastructure, contributor access. Define owners and escalation path. | Threat actors, assets, trust boundaries explicit; links reconciled; no duplicate policy. |
| 2 | `docs(security): publish private vulnerability reporting and disclosure policy` | Configure GitHub private reporting or equivalent channel; document report fields, confidentiality, acknowledgement, triage, coordination, public disclosure. | Test report can be submitted privately; reporting instructions explicit; public issues direct to private intake. |
| 3 | `ci(security): add Rust dependency and license policy checks` | Add `Cargo.lock` dependency auditing (`cargo-audit`); approved sources, license policy, exception process. | CI detects known RustSec advisories and disallowed dependencies; exceptions time-bounded and approved. |
| 4 | `ci(security): add secret, unsafe-code, and static-analysis checks` | Scan for secrets; inventory `unsafe` Rust; add lint/static-analysis checks; review process for unsafe/FFI. | New secrets blocked; unsafe blocks require safety comment and tests; findings triaged. |
| 5 | `test(security): establish fuzzing and sanitization coverage` | Audit fuzz targets; prioritize parsers, syscall boundaries, filesystem metadata, archive/package input, IPC, network, image/config formats. | Fuzz targets reproducible; crashes minimized and turned into regression tests; resource use bounded. |
| 6 | `security(kernel): define syscall and privilege-boundary invariants` | Document syscall validation, pointer/length checks, permission checks, handle ownership, namespace boundaries, kernel/user copy rules. | Tests cover invalid pointers/lengths, unauthorized access, resource exhaustion, boundary crossing. |
| 7 | `security(memory): add kernel memory hardening and fault tests` | Review allocator, paging, process isolation, memory reclamation; test UAF, double-free, overflow, mapping permissions, isolation failures. | Invariants tested or documented; fault handling leaks no sensitive memory. |
| 8 | `security(drivers): add driver threat model and safe failure rules` | Driver trust assumptions, DMA and buffer ownership, device reset behavior, malformed descriptor handling, privilege separation. | Device input cannot cause unbounded allocation; reset/error paths tested; unsupported devices documented. |
| 9 | `security(crypto): define cryptographic API and key-handling policy` | Inventory algorithms and call sites; approved primitives, randomness, key lifecycle, zeroization, algorithm deprecation. | No unreviewed custom crypto; test vectors exist; secrets excluded from logs/diagnostics. |
| 10 | `security(boot): specify verified boot and recovery trust model` | Boot artifact trust, signing keys, key custody/rotation, verification failure behavior, recovery image policy, rollback protection. | Artifact provenance and integrity verified; failure closed/safe; recovery does not bypass verification. |
| 11 | `security(updates): implement signed, atomic, recoverable update policy` | Signed metadata/packages, freshness/rollback handling, transactional install, interruption recovery, downgrade prevention. | Tampered/unsigned artifacts rejected; interrupted updates recover to known state; rollback tested. |
| 12 | `security(pkg): add package provenance and SBOM generation` | Generate SBOM for release artifacts; record source revision, build inputs, dependency inventory, provenance verification. | SBOM matches shipped binaries; provenance identifies source and workflow; verification tested. |
| 13 | `security(release): establish advisory and backport workflow` | Advisory template, affected/fixed version table, severity rationale, mitigations, backport decision process. | Advisories describe impact and fixed versions; release branches receive minimal tested fixes. |
| 14 | `security(services): define least-privilege and default exposure policy` | Audit init/services, network listeners, desktop components, privilege helpers; require explicit service enablement. | Default install exposes minimal necessary services; privilege rationale recorded; unauthorized access denied. |
| 15 | `ci(security): protect security workflows and release credentials` | Review GitHub Actions permissions, third-party actions, secrets, release automation, branch rules, artifact publishing. | Protected branches enforce reviews; release secrets hidden from untrusted PR code; workflows security-reviewed. |
| 16 | `docs(security): publish security status and supported-configuration matrix` | Map security capabilities to code, tests, threat assumptions, maturity status (Proposed, Prototype, Integrated, Supported). | No unsupported feature claimed as Supported; all claims linked to test evidence; gaps tracked. |

## Periodic Security Cadence
- **Every PR:** `ci(security): run required checks for changed components`
- **Weekly:** `security(deps): review new RustSec advisories`
- **Weekly:** `security(issues): triage incoming reports and newly disclosed vulnerabilities`
- **Monthly:** `security(ci): review workflow permissions and external actions`
- **Monthly:** `security(fuzz): review crashes and coverage gaps`
- **Monthly:** `security(status): reconcile feature claims with evidence`
- **Quarterly:** `security(threat-model): re-evaluate attack surfaces and assumptions`
- **Quarterly:** `security(supply-chain): assess dependencies and release provenance`
- **Each release candidate:** `release(security): certify security gates for candidate`
- **After each incident:** `fix(security): prevent recurrence of <issue>`

## Component-Specific Security Rules
- **Kernel and syscalls:** Treat all userspace input as untrusted. Validate lengths, indices, flags, handles, and permissions before dereference.
- **Memory management:** Enforce W^X page permissions, KASLR, guard pages, address-space separation, and zero-on-free memory allocation.
- **Filesystems and storage:** Treat disk images and metadata as attacker-controlled. Bound recursion and buffer allocations when parsing ext4, Btrfs, FAT, ISO.
- **Drivers and DMA:** Require DMA buffer boundary checks, IOMMU mapping isolation, and non-panicking timeout recovery.
- **Networking:** Treat packet headers as untrusted. Bound buffer allocations, socket timeouts, and rate limits.
- **Crypto and keys:** Use reviewed algorithms (AES-256-GCM, Ed25519, Dilithium-5). Zeroize sensitive keys on drop.
- **Init and services:** Principle of least privilege. Services run with minimal capabilities in isolated namespaces, pledge promises, or Landlock rulesets.
- **Installer and updates:** Verify cryptographic signatures before installation. Create rollback snapshots before system upgrades.
- **Desktop and applications:** Separate app permissions from kernel privileges. Permission-mediated portal access for privileged actions.
- **Build and release:** Isolate untrusted PR builds from release secrets. Pin action dependencies and generate signed SBOMs.

## PR Review and Merge Gates
Required description template:
- `## Summary`
- `## Scope`
- `## Security classification`
- `## Threat model`
- `## Impact and versions`
- `## Implementation`
- `## Tests and evidence`
- `## Disclosure and release`
- `## Checklist`

Approvals required:
- **Standard security change:** 1 maintainer approval.
- **Privileged boundary / crypto / boot / update / sandbox / policy change:** 2 qualified maintainers.

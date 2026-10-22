# SigmaOS Security Management Plan

## Executive Summary

SigmaOS is an experimental Rust-based operating system. Subsystem models and unit tests establish component design and isolated logic, but maturity claims (**Proposed, Prototype, Integrated, Supported**) require target runtime evidence. This document defines SigmaOS's security program, aligned with NIST SP 800-218 (Secure Software Development Framework - SSDF) and borrowing mature Linux/BSD distribution practices such as coordinated triage, tested fixes, signed advisories, and stable-release backports.

## Baseline Rules

Apply these rules to all security-related PRs and releases:

- **Security status requires evidence:** "Supported" maturity strictly means the feature passed documented runtime, security, and recovery tests for its declared target hardware or hypervisor. A source module or unit test alone is not evidence of a secure system capability.
- **One PR, one reviewable security outcome:** Keep fixes narrowly scoped; separate refactors from vulnerability remediation. For release branches, prefer minimal targeted fixes and avoid unrelated behavior or API/ABI changes.
- **Keep vulnerability details private until coordinated disclosure:** Do not use public issues or PRs for unpatched zero-day vulnerabilities. Provide a private intake channel (`security@sigmaos.org` / GitHub Private Vulnerability Reporting), define report handlers, and coordinate disclosure.
- **Prioritize risk, not scanner scores alone:** Triage exploitability in SigmaOS's architecture, affected versions, attack surface, privileges required, and impact. Record CVEs where applicable, but evaluate issues in product context rather than treating raw third-party scores as proof.
- **Human review required for security-sensitive changes:** Kernel privilege boundaries, cryptography, boot/update trust, sandboxing, or security policy changes require approval from two qualified maintainers.
- **No security claim without release evidence:** Releases must identify the source revision, build/test evidence, advisories, and artifact integrity checksums. Package authenticity must be cryptographically verifiable.

## Proposed PR Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(security): establish threat model and security ownership` | Audit existing security docs and agent guidance; add a system threat model covering boot, kernel, drivers, userspace, package/update flow, desktop, network services, build infrastructure, and contributor access. Define security owners and escalation path. | Threat actors, assets, trust boundaries, assumptions, non-goals, and review ownership are explicit; links to existing policies are reconciled; no duplicate or conflicting policy remains. |
| 2 | `docs(security): publish private vulnerability reporting and disclosure policy` | Verify GitHub private vulnerability reporting or configure an equivalent private channel; document report fields, confidentiality, acknowledgement, triage, coordination, and public disclosure process. | A test report can be submitted privately; reporters are told where to report and what to include; public issues/PRs direct people away from posting exploit details. |
| 3 | `ci(security): add Rust dependency and license policy checks` | Add scheduled and PR-triggered dependency auditing for `Cargo.lock`; define approved sources, license policy, exception process, and duplicate/outdated dependency review. | CI detects known RustSec advisories and disallowed dependencies; exceptions are time-bounded, owner-approved, and documented; scanner failures cannot be silently ignored (`cargo-audit`). |
| 4 | `ci(security): add secret, unsafe-code, and static-analysis checks` | Scan for secrets and accidental credentials; inventory `unsafe` Rust; add lint/static-analysis checks appropriate to the codebase; define a review process for unsafe blocks and FFI. | New secrets are blocked or flagged; unsafe blocks have a safety rationale and focused tests; intentional exceptions are scoped and reviewed; findings have a triage route. |
| 5 | `test(security): establish fuzzing and sanitization coverage` | Audit current `fuzz/` and test coverage; prioritize parsers, syscall boundaries, filesystem metadata, archive/package input, IPC, network packet handling, and image/config formats. Add regression corpus conventions. | Fuzz targets are reproducible; crashes are minimized and turned into regression tests; resource use is bounded; no claim of fuzz coverage without documented targets and execution evidence. |
| 6 | `security(kernel): define syscall and privilege-boundary invariants` | Document syscall validation, pointer/length checks, permission checks, handle ownership, namespace boundaries, and kernel/user copying rules. Prioritize implemented APIs and mark modeled APIs clearly. | Tests cover invalid pointers/lengths, unauthorized access, resource exhaustion, and boundary crossing; public status reflects integration maturity. |
| 7 | `security(memory): add kernel memory hardening and fault tests` | Review allocator, paging, process isolation, and memory reclamation; add invariant checks and tests for use-after-free, double-free, overflow, mapping permissions, and isolation failures where feasible. | Each claimed invariant has a test or documented limitation; debug assertions are not mistaken for production mitigations; fault handling does not expose sensitive memory. |
| 8 | `security(drivers): add driver threat model and safe failure rules` | Define driver trust assumptions, DMA and buffer ownership, device reset behavior, malformed descriptor handling, and privilege separation. Start with the best-tested virtual device. | Device input cannot cause unbounded allocation or unchecked indexing; reset/error paths are tested; unsupported devices are documented instead of implied by driver names. |
| 9 | `security(crypto): define cryptographic API and key-handling policy` | Inventory cryptographic algorithms and call sites; define approved primitives, randomness requirements, key lifecycle, zeroization expectations, and algorithm deprecation process. | No custom cryptography is introduced without a design/security review; test vectors and negative tests exist; secrets are excluded from logs and diagnostics. |
| 10 | `security(boot): specify verified boot and recovery trust model` | Define boot artifact trust, signing keys, key custody/rotation, verification failure behavior, recovery image policy, and rollback protection requirements. Implement only after the boot path is measured. | A boot artifact’s provenance and integrity can be verified; failure is closed and understandable; recovery does not silently bypass verification; claims are limited to tested boot targets. |
| 11 | `security(updates): implement signed, atomic, recoverable update policy` | Specify signed metadata and packages, freshness/rollback handling, transactional install, interruption recovery, and downgrade policy. | Tampered, unsigned, stale, or unauthorized artifacts are rejected; interrupted updates recover to a known state; rollback behavior is tested and documented. |
| 12 | `security(pkg): add package provenance and SBOM generation` | Generate an SBOM for release artifacts; record source revision, build inputs, dependency inventory, and provenance; define artifact verification instructions. | Release SBOM matches shipped contents; provenance identifies the source and build workflow; verification instructions are tested. OpenSSF Scorecard and SLSA provenance provide reference models. |
| 13 | `security(release): establish advisory and backport workflow` | Add an advisory template, affected/fixed version table, severity rationale, mitigation, credits, and backport/release decision process. Keep security-only fixes separate from feature changes. | Advisory describes impact and fixed versions; release branches receive minimal tested fixes; build/test results are recorded; signed release metadata is verifiable. |
| 14 | `security(services): define least-privilege and default exposure policy` | Audit init/services, network listeners, desktop components, diagnostics, and privileged helpers. Require explicit service enablement and least privilege; document sandbox/capability boundaries. | Default installation exposes only documented necessary services; each service has a privilege rationale and shutdown behavior; tests verify unauthorized access is denied. |
| 15 | `ci(security): protect security workflows and release credentials` | Review GitHub Actions permissions, third-party actions, secrets, release automation, branch rules, and artifact publishing. Pin actions to immutable references where practical; minimize token permissions. | Protected branches require review and checks; release secrets are unavailable to untrusted PR code; publishing requires trusted workflow/ref; workflow changes receive security review. |
| 16 | `docs(security): publish security status and supported-configuration matrix` | Add a security component page and map each security capability to code, tests, threat assumptions, and maturity status. Include known gaps for kernel, boot, installer, network, package manager, desktop, and architectures. | No unsupported security feature is described as present; all “Integrated” and “Supported” claims have linked evidence; gaps have owners or explicit deferred status. |

## Periodic Security Cadence

| Cadence | Proposed recurring task | Required output |
|---|---|---|
| Every PR | `ci(security): run required checks for changed components` | Formatting, build, unit/integration tests, secret scan, clippy lints. |
| Weekly | `security(deps): review new RustSec advisories` | Affected dependency tree, reachability analysis, upgrade plan or non-applicability rationale (`cargo-audit`). |
| Weekly | `security(issues): triage incoming reports and newly disclosed vulnerabilities` | Private case record, affected component/versions, severity rationale, owner, target fix date. |
| Monthly | `security(ci): review workflow permissions and external actions` | Audit action revisions, permission caps, secret exposure, branch protections. |
| Monthly | `security(fuzz): review crashes and coverage gaps` | Crash disposition, regression test creation, target health, unresolved findings. |
| Monthly | `security(status): reconcile feature claims with evidence` | Synchronize `FEATURE_STATUS.toml`, documentation, and runtime test evidence. |
| Quarterly | `security(threat-model): re-evaluate attack surfaces and assumptions` | Review attack boundaries across boot, kernel, drivers, network, package manager. |
| Quarterly | `security(supply-chain): assess dependencies and release provenance` | Dependency audit, license compliance, build isolation review, SBOM validation. |
| Each release candidate | `release(security): certify security gates for candidate` | Target test results, zero unpatched high/critical advisories, signed artifacts, verified SBOM. |
| After each incident | `fix(security): prevent recurrence of <issue>` | Root cause analysis, regression test addition, backport decision, policy improvement. |

## Component-Specific Security Rules

- **Kernel and syscalls:** Treat all userspace input as untrusted. Validate lengths, indices, flags, handles, and permissions before use. Define whether each syscall is implemented, modeled, or unsupported; test error paths and isolation boundaries.
- **Memory management:** State invariants for mapping permissions, ownership, reclamation, and address-space separation (W^X, KASLR, guard pages). Add tests for boundary conditions and failure cases before making hardening claims.
- **Filesystems and storage:** Treat disk images, metadata, archives, filenames, and recovery media as attacker-controlled. Fuzz parsers; bound recursion, allocation, and I/O; test crash consistency and mount failure behavior.
- **Drivers and DMA:** Treat device responses as untrusted input. Specify buffer/descriptor validation, lifecycle, reset, and isolation behavior; hardware claims require tested devices and configurations.
- **Networking:** Treat packets and protocol state as untrusted. Add fuzzing, limits, service exposure policy, and IPv4/IPv6 parity checks where both are supported.
- **Crypto and keys:** Use reviewed primitives (AES-256-GCM, Ed25519, Dilithium-5) and explicit key custody/rotation rules. Never check private release keys or secrets into the repository; exclude sensitive material from debug logs.
- **Init and services:** Least privilege by default; avoid starting unnecessary listeners; define startup dependencies and safe failure behavior.
- **Installer and updates:** Verify authenticity before installation; keep updates transactional and recoverable; document downgrade and interrupted-update behavior.
- **Desktop and applications:** Separate app permissions from kernel privileges; define user consent and revocation semantics; avoid treating UI confirmation as a substitute for authorization enforcement.
- **Build and release:** Isolate untrusted PR execution from release secrets; preserve provenance; publish integrity checks and SBOMs with release artifacts.

## PR Review and Merge Gates

All security-related PRs must use the standard PR template with required checklist sections:

```text
## Summary
What risk or security requirement does this PR address?

## Scope
Affected components, files, release branches, and explicitly excluded work.

## Security classification
Public hardening | Security fix | Dependency update | Policy/docs
If this is a vulnerability, do not include sensitive details in a public PR.

## Threat model
Attacker capability, affected assets, trust boundary, and assumptions.

## Impact and versions
Affected configurations/releases, exploitability rationale, and known mitigations.

## Implementation
Why this fix is appropriate; compatibility/API impact; backport considerations.

## Tests and evidence
Regression test, fuzz/property tests, static analysis, integration/runtime results,
commands run, and anything not tested.

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

For normal code changes, require one maintainer approval plus all applicable CI checks. For changes to privileged boundaries, cryptography, boot/update trust, package signing, or release workflows, require two qualified reviewers and a security owner.

## Rollout Plan

Begin with PRs 1–5: establish ownership and private reporting, audit existing components, and make automated checks trustworthy before expanding the security surface. Then address the highest-risk real runtime paths—syscalls, memory, drivers, boot, and update integrity—based on the threat model and verified project status.

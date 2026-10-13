# SigmaOS Security Management Plan

## Executive Summary & Scope

SigmaOS is an experimental, sovereign Rust operating system. Unit-tested subsystem models do not prove a complete boot or installation path. Component maturity across kernel, memory, filesystems, drivers, networking, security, and userspace is tracked as **Proposed, Prototype, Integrated, or Supported**.

The SigmaOS Security Management Plan establishes a comprehensive security governance framework aligned with NIST SP 800-218 (Secure Software Development Framework / SSDF v1.1) and mature Linux and BSD distribution security practices (Debian, FreeBSD, OpenBSD, Fedora).

---

## Baseline Rules

Apply these baseline evidence-based security rules to all security-related PRs and release candidates:

1. **Security status requires evidence.** "Supported" status requires that the feature passed documented runtime, security, and recovery tests for its declared target architecture. A source module or unit test alone is not evidence of a secure production capability.
2. **One PR, one reviewable security outcome.** Keep security changes narrowly scoped; separate refactorings from vulnerability remediation. For release/stable branches, prefer minimal targeted fixes to reduce regression risks.
3. **Keep vulnerability details private until coordinated disclosure.** Do not post unpatched vulnerability details or working exploit payloads in public issues or PRs. Provide a private reporting channel (`security@sigmaos.org` / GitHub Private Vulnerability Reporting).
4. **Prioritize risk, not scanner scores alone.** Triage exploitability in SigmaOS's architecture, affected versions, attack surface, privileges required, and impact. Record CVEs where applicable, but evaluate issues in project context.
5. **Do not merge security-sensitive changes on green CI alone.** Require human review from maintainers familiar with the affected subsystem. Require two qualified maintainers for changes to privilege boundaries, cryptography, boot/update trust, sandboxing, or security policies.
6. **No security claim without release evidence.** Releases identify source revisions, build/test evidence, advisories, and artifact integrity info (PQC/Signify/GPG signatures and SBOMs).

---

## Proposed PR Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(security): establish threat model and security ownership` | Audit existing security docs and agent guidance; add a system threat model covering boot, kernel, drivers, userspace, package/update flow, desktop, network services, build infrastructure, and contributor access. Define security owners and escalation path. | Threat actors, assets, trust boundaries, assumptions, non-goals, and review ownership are explicit; links to existing policies are reconciled; no duplicate or conflicting policy remains. |
| 2 | `docs(security): publish private vulnerability reporting and disclosure policy` | Verify GitHub private vulnerability reporting or configure an equivalent private channel; document report fields, confidentiality, acknowledgement, triage, coordination, and public disclosure process. | A test report can be submitted privately; reporters are told where to report and what to include; public issues/PRs direct people away from posting exploit details. |
| 3 | `ci(security): add Rust dependency and license policy checks` | Add scheduled and PR-triggered dependency auditing for `Cargo.lock`; define approved sources, license policy, exception process, and duplicate/outdated dependency review. | CI detects known RustSec advisories and disallowed dependencies; exceptions are time-bounded, owner-approved, and documented; scanner failures cannot be silently ignored. |
| 4 | `ci(security): add secret, unsafe-code, and static-analysis checks` | Scan for secrets and accidental credentials; inventory `unsafe` Rust; add lint/static-analysis checks appropriate to the codebase; define a review process for unsafe blocks and FFI. | New secrets are blocked or flagged; unsafe blocks have a safety rationale and focused tests; intentional exceptions are scoped and reviewed; findings have a triage route. |
| 5 | `test(security): establish fuzzing and sanitization coverage` | Audit current `fuzz/` and test coverage; prioritize parsers, syscall boundaries, filesystem metadata, archive/package input, IPC, network packet handling, and image/config formats. Add regression corpus conventions. | Fuzz targets are reproducible; crashes are minimized and turned into regression tests; resource use is bounded; no claim of fuzz coverage without documented targets and execution evidence. |
| 6 | `security(kernel): define syscall and privilege-boundary invariants` | Document syscall validation, pointer/length checks, permission checks, handle ownership, namespace boundaries, and kernel/user copying rules. Prioritize implemented APIs and mark modeled APIs clearly. | Tests cover invalid pointers/lengths, unauthorized access, resource exhaustion, and boundary crossing; public status reflects integration maturity. |
| 7 | `security(memory): add kernel memory hardening and fault tests` | Review allocator, paging, process isolation, and memory reclamation; add invariant checks and tests for use-after-free, double-free, overflow, mapping permissions, and isolation failures where feasible. | Each claimed invariant has a test or documented limitation; debug assertions are not mistaken for production mitigations; fault handling does not expose sensitive memory. |
| 8 | `security(drivers): add driver threat model and safe failure rules` | Define driver trust assumptions, DMA and buffer ownership, device reset behavior, malformed descriptor handling, and privilege separation. Start with the best-tested virtual device. | Device input cannot cause unbounded allocation or unchecked indexing; reset/error paths are tested; unsupported devices are documented instead of implied by driver names. |
| 9 | `security(crypto): define cryptographic API and key-handling policy` | Inventory cryptographic algorithms and call sites; define approved primitives, randomness requirements, key lifecycle, zeroization expectations, and algorithm deprecation process. | No custom cryptography is introduced without a design/security review; test vectors and negative tests exist; secrets are excluded from logs and diagnostics. |
| 10 | `security(boot): specify verified boot and recovery trust model` | Define boot artifact trust, signing keys, key custody/rotation, verification failure behavior, recovery image policy, and rollback protection requirements. Implement only after the boot path is measured. | A boot artifact's provenance and integrity can be verified; failure is closed and understandable; recovery does not silently bypass verification; claims are limited to tested boot targets. |
| 11 | `security(updates): implement signed, atomic, recoverable update policy` | Specify signed metadata and packages, freshness/rollback handling, transactional install, interruption recovery, and downgrade policy. | Tampered, unsigned, stale, or unauthorized artifacts are rejected; interrupted updates recover to a known state; rollback behavior is tested and documented. |
| 12 | `security(pkg): add package provenance and SBOM generation` | Generate an SBOM for release artifacts; record source revision, build inputs, dependency inventory, and provenance; define artifact verification instructions. | Release SBOM matches shipped contents; provenance identifies the source and build workflow; verification instructions are tested. |
| 13 | `security(release): establish advisory and backport workflow` | Add an advisory template, affected/fixed version table, severity rationale, mitigation, credits, and backport/release decision process. Keep security-only fixes separate from feature changes. | Advisory describes impact and fixed versions; release branches receive minimal tested fixes; build/test results are recorded; signed release metadata is verifiable. |
| 14 | `security(services): define least-privilege and default exposure policy` | Audit init/services, network listeners, desktop components, diagnostics, and privileged helpers. Require explicit service enablement and least privilege; document sandbox/capability boundaries. | Default installation exposes only documented necessary services; each service has a privilege rationale and shutdown behavior; tests verify unauthorized access is denied. |
| 15 | `ci(security): protect security workflows and release credentials` | Review GitHub Actions permissions, third-party actions, secrets, release automation, branch rules, and artifact publishing. Pin actions to immutable references where practical; minimize token permissions. | Protected branches require review and checks; release secrets are unavailable to untrusted PR code; publishing requires trusted workflow/ref; workflow changes receive security review. |
| 16 | `docs(security): publish security status and supported-configuration matrix` | Add a security component page and map each security capability to code, tests, threat assumptions, and maturity status. Include known gaps for kernel, boot, installer, network, package manager, desktop, and architectures. | No unsupported security feature is described as present; all "Integrated" and "Supported" claims have linked evidence; gaps have owners or explicit deferred status. |

---

## Periodic Security Maintenance Cadence

| Cadence | Task | Required Output |
|---|---|---|
| Every PR | `ci(security): run required checks for changed components` | Formatting, build, unit/integration tests, RustSec dependency check, secret scan, and unsafe Rust review. |
| Weekly | `security(deps): review new RustSec advisories` | Audit `Cargo.lock` against RustSec advisory database; publish reachability analysis and upgrade plan. |
| Weekly | `security(issues): triage incoming reports and newly disclosed vulnerabilities` | Triage private security reports; assign severity rating, subsystem owner, and target fix date. |
| Monthly | `security(ci): review workflow permissions and external actions` | Audit GitHub Actions permissions, third-party action pinning, release secrets, and branch protection rules. |
| Monthly | `security(fuzz): review crashes and coverage gaps` | Analyze fuzzing crash corpora, convert crashes to regression tests, and expand fuzz target boundaries. |
| Monthly | `security(status): reconcile feature claims with evidence` | Reconcile maturity matrix in `FEATURE_STATUS.toml`, `CAPABILITY_MATRIX.toml`, and docs with runtime test evidence. |
| Quarterly | `security(threat-model): re-evaluate attack surfaces and assumptions` | Re-assess kernel, boot, network, driver, package, desktop, and contributor threat boundaries. |
| Quarterly | `security(supply-chain): assess dependencies and release provenance` | Review crate dependencies, SBOM accuracy, SLSA provenance generation, and build isolation. |
| Release Candidate | `release(security): certify security gates for candidate` | Verify target test suite results, advisory status, artifact signatures, rollback safety, and release signoff. |
| Post-Incident | `fix(security): prevent recurrence of issue` | Root-cause analysis, regression test addition, backport patch delivery, and detection workflow improvements. |

---

## Response Service Targets

- **Acknowledgement:** Within 3 business days of private vulnerability receipt.
- **Triage & Initial Severity Rating:** Within 10 business days.
- **Remediation & Coordinated Disclosure:** Fixed in next security update or within 90 days depending on severity and vendor coordination.
- **Performance Evaluation:** Evaluated quarterly against maintainer capacity.

---

## Component-Specific Security Rules

- **Kernel and Syscalls:** Treat all userspace input as untrusted. Validate lengths, pointers, flags, handles, and permissions before use. Clearly distinguish implemented APIs from modeled/stubbed APIs.
- **Memory Management:** Enforce address-space separation, mapping permissions (W^X / NX bit), page allocation bounds, and zero-on-free memory reclamation invariants.
- **Filesystems and Storage:** Treat disk images, metadata, archives, and recovery media as untrusted inputs. Fuzz metadata parsers; enforce crash consistency and fail-closed mount error handling.
- **Drivers and DMA:** Validate buffer descriptors, ring indexes, and DMA addresses before dispatch. Ensure safe device reset and error recovery.
- **Networking:** Treat network packets, frames, and protocol state as untrusted input. Enforce rate limits, packet filtering, and socket sandbox isolation.
- **Crypto and Keys:** Use audited, peer-reviewed primitives (Kyber/Dilithium PQC, AES-GCM, Ed25519). Keep private release keys out of repository source.
- **Init and Services:** Enforce least privilege by default; disable unneeded network listeners; sandbox service processes with Landlock/Seccomp/Capsicum.
- **Installer and Updates:** Require cryptographic attestation before package/system installation; enforce atomic, transactional updates with automatic rollback.
- **Desktop and Applications:** Isolate application capabilities using typed portal access mediators and Wayland client buffer isolation.
- **Build and Release:** Enforce isolated PR builds; pin action dependencies; sign release artifacts with PQC and Signify keys; publish SBOMs.

---

## PR Review and Merge Gates

All security PRs must use the following PR template:

```text
## Summary
What risk or security requirement does this PR address?

## Scope
Affected components, files, release branches, and explicitly excluded work.

## Security classification
Public hardening | Security fix | Dependency update | Policy/docs

## Threat model
Attacker capability, affected assets, trust boundary, and assumptions.

## Impact and versions
Affected configurations/releases, exploitability rationale, and known mitigations.

## Implementation
Why this fix is appropriate; compatibility/API impact; backport considerations.

## Tests and evidence
Regression test, fuzz/property tests, static analysis, integration/runtime results, commands run.

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

For privilege boundary, crypto, boot/update trust, or security policy changes, require **two qualified maintainer reviews** and security owner approval prior to merge.

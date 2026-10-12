# SigmaOS Security Management Plan

## Overview & Alignment

SigmaOS is an experimental Rust operating system tracking component maturity as **Proposed, Prototype, Integrated, or Supported**. To ensure system safety, software integrity, and vulnerability governance, SigmaOS implements a comprehensive Security Management Plan aligned with NIST's Secure Software Development Framework (SSDF) [NIST SP 800-218] and mature distribution security practices (e.g., Debian, OpenBSD, FreeBSD).

This policy integrates secure development practices across the OS lifecycle, establishing coordinated triage, tested fixes, signed advisories, and stable release backports.

---

## Baseline Rules

Apply these rules to all security-related PRs and releases:

- **Security status requires evidence:** "Supported" status requires that the feature passed documented runtime, security, and recovery tests for its declared target. Source code or unit tests alone do not constitute evidence of a secure system capability.
- **One PR, one reviewable security outcome:** Keep fixes narrowly scoped; separate refactoring from vulnerability remediation. For release branches, prefer minimal targeted fixes to prevent regressions.
- **Keep vulnerability details private until coordinated disclosure:** Do not use public issues or PRs for unpatched vulnerabilities. Maintain a private contact channel and coordinated disclosure timeline.
- **Prioritize risk, not scanner scores alone:** Triage exploitability in SigmaOS's architecture, affected versions, attack surface, required privileges, and impact. Use CVE records when applicable without treating severity scores as a substitute for architecture-specific analysis.
- **Do not merge security-sensitive changes on "green CI" alone:** Require human review from a domain maintainer; require two qualified reviewers for kernel privilege boundaries, cryptography, boot/update trust, sandboxing, or policy changes.
- **No security claim without release evidence:** Releases must state source revisions, build/test evidence, advisories, and artifact integrity signatures.

---

## Proposed PR Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(security): establish threat model and security ownership` | Audit existing security docs and agent guidance; add a system threat model covering boot, kernel, drivers, userspace, package/update flow, desktop, network services, build infrastructure, and contributor access. Define security owners and escalation path. | Threat actors, assets, trust boundaries, assumptions, non-goals, and review ownership are explicit; links to existing policies are reconciled; no duplicate or conflicting policy remains. |
| 2 | `docs(security): publish private vulnerability reporting and disclosure policy` | Verify GitHub private vulnerability reporting or configure an equivalent private channel; document report fields, confidentiality, acknowledgement, triage, coordination, and public disclosure process. | A test report can be submitted privately; reporters are told where to report and what to include; public issues/PRs direct people away from posting exploit details. |
| 3 | `ci(security): add Rust dependency and license policy checks` | Add scheduled and PR-triggered dependency auditing for `Cargo.lock`; define approved sources, license policy, exception process, and duplicate/outdated dependency review. | CI detects known RustSec advisories and disallowed dependencies; exceptions are time-bounded, owner-approved, and documented; scanner failures cannot be silently ignored. `cargo-audit` is designed to audit lockfiles against RustSec advisories. |
| 4 | `ci(security): add secret, unsafe-code, and static-analysis checks` | Scan for secrets and accidental credentials; inventory `unsafe` Rust; add lint/static-analysis checks appropriate to the codebase; define a review process for unsafe blocks and FFI. | New secrets are blocked or flagged; unsafe blocks have a safety rationale and focused tests; intentional exceptions are scoped and reviewed; findings have a triage route. |
| 5 | `test(security): establish fuzzing and sanitization coverage` | Audit current `fuzz/` and test coverage; prioritize parsers, syscall boundaries, filesystem metadata, archive/package input, IPC, network packet handling, and image/config formats. Add regression corpus conventions. | Fuzz targets are reproducible; crashes are minimized and turned into regression tests; resource use is bounded; no claim of fuzz coverage without documented targets and execution evidence. |
| 6 | `security(kernel): define syscall and privilege-boundary invariants` | Document syscall validation, pointer/length checks, permission checks, handle ownership, namespace boundaries, and kernel/user copying rules. Prioritize implemented APIs and mark modeled APIs clearly. | Tests cover invalid pointers/lengths, unauthorized access, resource exhaustion, and boundary crossing; public status reflects integration maturity. |
| 7 | `security(memory): add kernel memory hardening and fault tests` | Review allocator, paging, process isolation, and memory reclamation; add invariant checks and tests for use-after-free, double-free, overflow, mapping permissions, and isolation failures where feasible. | Each claimed invariant has a test or documented limitation; debug assertions are not mistaken for production mitigations; fault handling does not expose sensitive memory. |
| 8 | `security(drivers): add driver threat model and safe failure rules` | Define driver trust assumptions, DMA and buffer ownership, device reset behavior, malformed descriptor handling, and privilege separation. Start with the best-tested virtual device. | Device input cannot cause unbounded allocation or unchecked indexing; reset/error paths are tested; unsupported devices are documented instead of implied by driver names. |
| 9 | `security(crypto): define cryptographic API and key-handling policy` | Inventory cryptographic algorithms and call sites; define approved primitives, randomness requirements, key lifecycle, zeroization expectations, and algorithm deprecation process. | No custom cryptography is introduced without a design/security review; test vectors and negative tests exist; secrets are excluded from logs and diagnostics. |
| 10 | `security(boot): specify verified boot and recovery trust model` | Define boot artifact trust, signing keys, key custody/rotation, verification failure behavior, recovery image policy, and rollback protection requirements. Implement only after the boot path is measured. | A boot artifact’s provenance and integrity can be verified; failure is closed and understandable; recovery does not silently bypass verification; claims are limited to tested boot targets. |
| 11 | `security(updates): implement signed, atomic, recoverable update policy` | Specify signed metadata and packages, freshness/rollback handling, transactional install, interruption recovery, and downgrade policy. | Tampered, unsigned, stale, or unauthorized artifacts are rejected; interrupted updates recover to a known state; rollback behavior is tested and documented. |
| 12 | `security(pkg): add package provenance and SBOM generation` | Generate an SBOM for release artifacts; record source revision, build inputs, dependency inventory, and provenance; define artifact verification instructions. | Release SBOM matches shipped contents; provenance identifies the source and build workflow; verification instructions are tested. OpenSSF Scorecard and SLSA provenance provide reference models for assessing supply-chain practices and artifact provenance. |
| 13 | `security(release): establish advisory and backport workflow` | Add an advisory template, affected/fixed version table, severity rationale, mitigation, credits, and backport/release decision process. Keep security-only fixes separate from feature changes. | Advisory describes impact and fixed versions; release branches receive minimal tested fixes; build/test results are recorded; signed release metadata is verifiable. |
| 14 | `security(services): define least-privilege and default exposure policy` | Audit init/services, network listeners, desktop components, diagnostics, and privileged helpers. Require explicit service enablement and least privilege; document sandbox/capability boundaries. | Default installation exposes only documented necessary services; each service has a privilege rationale and shutdown behavior; tests verify unauthorized access is denied. |
| 15 | `ci(security): protect security workflows and release credentials` | Review GitHub Actions permissions, third-party actions, secrets, release automation, branch rules, and artifact publishing. Pin actions to immutable references where practical; minimize token permissions. | Protected branches require review and checks; release secrets are unavailable to untrusted PR code; publishing requires trusted workflow/ref; workflow changes receive security review. |
| 16 | `docs(security): publish security status and supported-configuration matrix` | Add a security component page and map each security capability to code, tests, threat assumptions, and maturity status. Include known gaps for kernel, boot, installer, network, package manager, desktop, and architectures. | No unsupported security feature is described as present; all “Integrated” and “Supported” claims have linked evidence; gaps have owners or explicit deferred status. |

---

## Periodic Security PR Cadence

| Cadence | Proposed Recurring PR/Task | Required Output |
|---|---|---|
| Every PR | `ci(security): run required checks for changed components` | Formatting/build/tests plus relevant dependency, secret, lint, and security checks; skipped checks explained in the PR. |
| Weekly | `security(deps): review new RustSec advisories` | Affected dependency tree, SigmaOS reachability analysis, fix/upgrade plan, or documented reason an advisory does not apply. |
| Weekly | `security(issues): triage incoming reports and newly disclosed vulnerabilities` | Private case record, affected component/versions, severity rationale, owner, target fix date, and disclosure coordination status. |
| Monthly | `security(ci): review workflow permissions and external actions` | Changes in actions, permissions, secrets exposure, release access, and branch protections; risks and remediation recorded. |
| Monthly | `security(fuzz): review crashes and coverage gaps` | Crash disposition, regression tests, target health, new attack surfaces, and unresolved high-risk findings. |
| Monthly | `security(status): reconcile feature claims with evidence` | Updates to `FEATURE_STATUS.toml`, project status, wiki pages, and security docs; no component is upgraded in maturity without runtime evidence. |
| Quarterly | `security(threat-model): re-evaluate attack surfaces and assumptions` | Review of boot, update, package, driver, syscall, network, desktop, and contributor threat boundaries; changed assumptions become tracked work. |
| Quarterly | `security(supply-chain): assess dependencies and release provenance` | Dependency/source/license review, build isolation review, SBOM/provenance check, and outstanding exceptions. |
| Each Release Candidate | `release(security): certify security gates for candidate` | Supported-target test results, known vulnerabilities, advisory status, artifact integrity/provenance, rollback instructions, and release approval. |
| After Each Incident | `fix(security): prevent recurrence of <issue>` | Root-cause analysis, regression test, affected-version assessment, patch/backport decision, and improvement to detection or process. |

---

## Component-Specific Security Rules

- **Kernel and Syscalls:** Treat all userspace input as untrusted. Validate lengths, indices, flags, handles, and permissions before use. Define whether each syscall is implemented, modeled, or unsupported; test error paths and isolation boundaries.
- **Memory Management:** State invariants for mapping permissions, ownership, reclamation, and address-space separation. Add tests for boundary conditions and failure cases before making hardening claims.
- **Filesystems and Storage:** Treat disk images, metadata, archives, filenames, and recovery media as attacker-controlled. Fuzz parsers; bound recursion, allocation, and I/O; test crash consistency and mount failure behavior.
- **Drivers and DMA:** Treat device responses as untrusted input. Specify buffer/descriptor validation, lifecycle, reset, and isolation behavior; hardware claims require tested devices and configurations.
- **Networking:** Treat packets and protocol state as untrusted. Add fuzzing, limits, service exposure policy, and IPv4/IPv6 parity checks where both are supported.
- **Crypto and Keys:** Use reviewed primitives and explicit key custody/rotation rules. Never check private release keys or secrets into the repository; exclude sensitive material from debug logs.
- **Init and Services:** Least privilege by default; avoid starting unnecessary listeners; define startup dependencies and safe failure behavior.
- **Installer and Updates:** Verify authenticity before installation; keep updates transactional and recoverable; document downgrade and interrupted-update behavior.
- **Desktop and Applications:** Separate app permissions from kernel privileges; define user consent and revocation semantics; avoid treating UI confirmation as a substitute for authorization enforcement.
- **Build and Release:** Isolate untrusted PR execution from release secrets; preserve provenance; publish integrity checks and SBOMs with release artifacts.

---

## PR Review and Merge Gates

Use this template for security PRs:

```text
## Summary
What risk or security requirement does this PR address?

## Scope
Affected components, files, release branches, and explicitly excluded work.

## Security Classification
Public hardening | Security fix | Dependency update | Policy/docs
If this is a vulnerability, do not include sensitive details in a public PR.

## Threat Model
Attacker capability, affected assets, trust boundary, and assumptions.

## Impact and Versions
Affected configurations/releases, exploitability rationale, and known mitigations.

## Implementation
Why this fix is appropriate; compatibility/API impact; backport considerations.

## Tests and Evidence
Regression test, fuzz/property tests, static analysis, integration/runtime results,
commands run, and anything not tested.

## Disclosure and Release
Private report reference (if applicable), coordination status, advisory/release plan.

## Checklist
- [ ] Minimal, reviewable change
- [ ] Security tests or regression tests added
- [ ] No secrets or exploit details exposed publicly
- [ ] Docs and component status updated
- [ ] CI passes and review conversations resolved
- [ ] Backport/recovery plan recorded where relevant
```

For normal code changes, require one maintainer approval plus all applicable CI checks. For changes to privileged boundaries, cryptography, boot/update trust, package signing, or release workflows, require two qualified reviewers and a security owner. Protected default branches must require approvals and checks before merging.

---

## Suggested Rollout

Begin with PRs 1–5: establish ownership and private reporting, audit what already exists, and make automated checks trustworthy before expanding the security surface. Then address the highest-risk real runtime paths—syscalls, memory, drivers, boot, and update integrity—based on the threat model and verified project status, not on a checklist’s order alone.

# 07 Security - SigmaOS Wiki

## Security Management Plan Overview
SigmaOS implements a NIST SP 800-218 (SSDF) aligned Security Management Plan backed by the `SovereignSecurityManagementPlanEngine` in `src/security/sovereign_security_management_plan.rs`.

### Baseline Evidence Rules
- Evidence required for maturity claims (`Supported` requires hardware/hypervisor test logs).
- Single PR per reviewable security outcome.
- Private vulnerability reporting and coordinated disclosure.
- Contextual risk triage based on architecture and impact.
- Dual maintainer review for privileged boundary changes.
- Release artifacts backed by signed metadata and SBOM provenance.

### 16-PR Security Gateway Roadmap
1. `docs(security): establish threat model and security ownership`
2. `docs(security): publish private vulnerability reporting and disclosure policy`
3. `ci(security): add Rust dependency and license policy checks`
4. `ci(security): add secret, unsafe-code, and static-analysis checks`
5. `test(security): establish fuzzing and sanitization coverage`
6. `security(kernel): define syscall and privilege-boundary invariants`
7. `security(memory): add kernel memory hardening and fault tests`
8. `security(drivers): add driver threat model and safe failure rules`
9. `security(crypto): define cryptographic API and key-handling policy`
10. `security(boot): specify verified boot and recovery trust model`
11. `security(updates): implement signed, atomic, recoverable update policy`
12. `security(pkg): add package provenance and SBOM generation`
13. `security(release): establish advisory and backport workflow`
14. `security(services): define least-privilege and default exposure policy`
15. `ci(security): protect security workflows and release credentials`
16. `docs(security): publish security status and supported-configuration matrix`

### PR Review Gate Inspector
PRs in security-sensitive subsystems must provide all 9 required template sections and obtain 2 maintainer approvals if modifying privileged boundaries, cryptography, boot trust, or update channels.

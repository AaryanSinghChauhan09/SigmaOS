# Governance: SigmaOS-security

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-security owns the security policy and enforcement layer for the operating system: defining secure defaults, the user privilege model and root/admin boundaries, default firewall rules, the package trust model and signing keys, secure boot policy integration, the default audit and security event logging model, the hardened profile, process isolation policy, and memory safety requirements for system components.
- **Runtime layer:** core-system (policy authority, runs as a privileged service)
- **Scope boundary with adjacent repos:** SigmaOS-package-manager and SigmaOS-repo-manager enforce the package trust this repo defines; SigmaOS-boot enforces secure boot policy this repo specifies; SigmaOS-kernel applies the hardening profile this repo curates; SigmaOS-core applies the privilege model this repo defines.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-security` CLI | CLI | stable |
| `org.sigmaos.Security1` D-Bus service | D-Bus | stable |
| `/etc/sigmaos/security.toml` | Config file | stable |
| Trust store keyring format (`/etc/sigmaos/trust/`) | File format | stable |
| Hardened profile definition (`profiles/hardened.toml`) | Config file | unstable |
| Audit event schema | Schema | stable |
| Internal policy engine | internal | internal-only |

- **Breaking-change policy:** Trust store and audit schema stable within a MAJOR version; policy changes that alter default behavior require a deprecation cycle and migration guide.
- **API documentation location:** `docs/api/`, `docs/security-policy.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-kernel | 6.6.0 | hardening profile config | No |
| SigmaOS-boot | 0.2 | secure boot policy | No |
| SigmaOS-package-manager | 0.3 | package trust enforcement | No |
| nftables | 1.0 | default firewall rules | No |
| auditd / libaudit | 3.1 | audit event logging | No |
| polkit | 122 | privilege escalation policy | No |
| gpgme | 1.20 | trust keyring operations | No |

- **Optional/soft dependencies:** `apparmor` for MAC profiles; `tpm2-tools` for measured boot attestation.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** security-critical deps updated within 48 hours of upstream release.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-security-team
- **Co-maintainers:** @kernel-lead, @core-lead
- **Review quorum:** 2 maintainer approvals required; any change to defaults or trust model requires @sigmaos-security-team lead approval.
- **Escalation path:** Security vulnerabilities are `priority:critical` and handled under coordinated disclosure; contact security@sigmaos.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 70% — policy engine, trust verification, and firewall rule generation unit-tested.
- **Integration tests:** Present — secure boot attestation, package signature enforcement, and hardened profile applied-and-verified end-to-end.
- **CI pipeline file:** `.github/workflows/security-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `policy-engine-test`, `firewall-rules-test`, `trust-verify-test`, `lint`
- **Known flaky tests:** none
- **CI build badge:** `![security CI](https://github.com/SigmaOS/SigmaOS-security/actions/workflows/security-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** desktop, laptop, server, VM — all supported.
- **Backend variants:** Linux tested in CI.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** External security audit completed, hardened profile validated on 3 reference devices, trust keyring rotation procedure documented and tested, secure boot attestation passing.

## 8. What This Repo Does NOT Do

- Does not apply kernel hardening directly (defines the profile) → applied by SigmaOS-kernel
- Does not enforce secure boot at firmware level → enforced by SigmaOS-boot + firmware
- Does not install or verify packages (defines trust) → enforced by SigmaOS-package-manager
- Does not implement the display/login auth flow → handled by SigmaOS-auth
- Does not provide the polkit decision daemon (defines policy) → enforced by polkit

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/security.toml`, `/etc/sigmaos/trust/`, `/etc/sigmaos/firewall/`
- **Schema file location:** `schemas/security.toml.schema.json`
- **Schema format:** TOML (policy), GPG keyring (trust), nftables (firewall)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_SECURITY_PROFILE` | `default` | `default` / `hardened` |
| `SIGMAOS_SECURE_BOOT` | auto | Override secure boot enforcement |
| `SIGMAOS_AUDIT_ENABLED` | `1` | Enable audit event logging |

- **Breaking config change policy:** Policy key changes require a migration script and a release-cycle deprecation; trust keyring additions are additive; key removals require a revocation ceremony documented in `docs/key-rotation.md`.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Trust key expired | Rotated key not refreshed | Package verification fails | Refresh keyring via `sigmaos-security --refresh-trust` |
| Firewall blocks legitimate traffic | Overly strict default rule | Service unreachable | Adjust rule in `/etc/sigmaos/firewall/`; reload nftables |
| Hardened profile breaks app | Strict isolation too aggressive | App fails to start | Switch to default profile; file issue |
| Audit log full | Log rotation not configured | Audit events dropped | Configure logrotate; clear old audit logs |
| Polkit policy misconfigured | Bad rule file | Privileged actions fail | Validate with `sigmaos-security --validate-polkit`; restore default rules |

- **Fallback behavior:** If the policy engine fails to load, the system falls back to the most restrictive known-good profile and logs a critical event; if trust verification is unavailable, package installs are refused (fail-closed).
- **Operator escalation path:** Collect `/var/log/sigmaos/security.log` and audit logs; file security issue under coordinated disclosure for vulnerabilities.
- **Crash dump artifacts:** Security log at `/var/log/sigmaos/security.log`; audit logs at `/var/log/audit/`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager updates this service; post-install hook restarts the D-Bus service and reloads firewall rules.
- **Rollback mechanism:** Previous policy package retained; `sigmaos-security --rollback` restores previous policy snapshot; trust keyring rollback preserves old keys for one release.
- **Expected downtime on update:** Brief D-Bus restart; firewall rules reloaded with no traffic interruption.
- **Pre-update state checks:** No active privileged operations in progress.
- **Post-update validation:** `sigmaos-security --self-check` verifies policy integrity and trust keyring.
- **Atomic update guarantee:** Partial — policy files written atomically; firewall reload is transactional via nftables.

## 12. Security Boundary

- **Privilege level required:** root (manages trust keyring, firewall rules, audit config); D-Bus queries unprivileged, mutations gated by polkit.
- **Trust boundary:** This repo IS the trust root; it does not trust untrusted input; keyring operations require hardware-backed key store when available.
- **Attack surface:** Trust keyring files, firewall rule parsing, polkit rule evaluation, D-Bus IPC
- **Hardening measures:** Keyring stored root-owned mode 700; policy engine runs with `NoNewPrivileges`, `ProtectSystem=strict`; private key operations require TPM2 when available.
- **Threat model reference:** `docs/threat-model.md` (comprehensive)
- **Known CVEs / mitigations:** None unmitigated.
- **Security audit status:** Internal review 2024-12; external audit planned for v1.0.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-kernel | 6.6.x | tested |
| SigmaOS-boot | 0.2+ | tested |
| SigmaOS-package-manager | 0.3+ | tested |
| SigmaOS-core | 0.4+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| nftables | 1.0+ | tested |
| auditd | 3.1+ | tested |
| polkit | 122+ | tested |

- **Known incompatibilities:** polkit < 122 lacks features for the privilege model.
- **Forward-compatibility pledge:** Trust store and audit schema stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-security-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/security-policy.md`, `docs/key-rotation.md`
- **Man page or in-app help status:** present — `sigmaos-security(8)`
- **Documentation parity policy:** Any default policy change or trust model change requires doc update in same PR and a security advisory review.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:firewall` | Firewall rule issues |
| `component:trust` | Package trust / keyring issues |
| `component:audit` | Audit and event logging issues |
| `component:hardened-profile` | Hardened profile issues |
| `component:privilege` | Privilege model / polkit issues |
| `component:secure-boot` | Secure boot policy issues |
| `disclosure:coordinated` | Vulnerability reports under embargo |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-security/milestones`
- **Issue triage cadence:** Every Monday and Thursday; security reports triaged within 24 hours

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated; security advisories published separately
- **Breaking change signal:** MAJOR bump for trust model or audit schema changes; security-relevant changes also flagged in release notes.
- **Version pinning in dependents:** All security-touching repos pin `sigmaos-security >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depended-on-by-core — SigmaOS-core applies the privilege model this repo defines and queries its policy engine for authorization decisions.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Security1`); polkit policy files; shared trust store files.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — privilege model enforcement tested with core's service lifecycle.
- **Change coordination policy:** Privilege model changes require a tracking issue in SigmaOS-core; default policy changes announced 30 days before they ship.
- **Core team review required for cross-boundary changes:** yes

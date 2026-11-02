# Governance: SigmaOS-package-manager

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-package-manager owns the package lifecycle for the operating system: install, remove, update, and rollback of software packages. It performs dependency resolution, signature verification against the SigmaOS trust model, transaction state management, and exposes a D-Bus API for other subsystems to query and trigger package operations. This is the canonical merger of the former SigmaOS-packages, SigmaOS-packages-kit, and SigmaOS-packages-tools repos.
- **Runtime layer:** core-system
- **Scope boundary with adjacent repos:** SigmaOS-repo-manager owns the repository metadata and mirror sync that feeds this manager's metadata; SigmaOS-update owns the scheduling and policy layer that calls this manager; SigmaOS-security owns the package trust/signature policy this manager enforces; SigmaOS-installer calls this manager to install the base system.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-pkg` CLI | CLI | stable |
| `org.sigmaos.PackageManager1` D-Bus service | D-Bus | stable |
| `libsigmaos-pkg.so` | Shared library | stable |
| Transaction state directory `/var/lib/sigmaos/pkg/transactions/` | File format | stable |
| Package metadata schema (`*.sigmaos-meta.json`) | File format | stable |
| Hooks directory `/etc/sigmaos/pkg/hooks.d/` | Hook API | unstable |
| Internal resolver data structures | internal | internal-only |

- **Breaking-change policy:** Stable CLI subcommands and D-Bus methods are frozen within a MAJOR version; new methods are additive; removed flags require a 2-release deprecation.
- **API documentation location:** `docs/api/`, D-Bus introspection at `dbus/org.sigmaos.PackageManager1.xml`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-repo-manager | 0.2 | repository metadata source | No |
| SigmaOS-security | 0.3 | signature verification trust policy | No |
| gpgme | 1.20 | OpenPGP signature verification | No |
| libsolv | 0.7.25 | dependency resolution engine | No |
| zstd | 1.5.5 | package compression | No |
| sqlite | 3.44 | local package database | No |
| dbus | 1.14 | IPC bus | No |

- **Optional/soft dependencies:** `appstream` for GUI software center metadata; `libcurl` for network mode downloads.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** gpgme and libsolv updated within 1 week of security releases; routine review quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-pkg-team
- **Co-maintainers:** @repo-lead, @security-lead
- **Review quorum:** 2 maintainer approvals required; transaction-model changes require @repo-lead approval.
- **Escalation path:** Package-management regressions that break install/update are `priority:critical`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 65% — resolver, signature verification, and transaction state machine unit-tested.
- **Integration tests:** Present — install/remove/update/rollback cycle tested against a fixture repository.
- **CI pipeline file:** `.github/workflows/pkg-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `resolver-tests`, `signature-tests`, `transaction-rollback-test`, `lint`
- **Known flaky tests:** `test-network-install` flaky on CI runner network (issue #31).
- **CI build badge:** `![pkg CI](https://github.com/SigmaOS/SigmaOS-package-manager/actions/workflows/pkg-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |
| BSD | 14.0 | planned |

- **Hardware classes:** desktop, laptop, server, VM — all supported.
- **Backend variants:** Linux tested in CI; BSD planned for v0.6.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** Transaction rollback validated on 5 real scenarios, signature verification end-to-end with SigmaOS-security, resolver handles 10k-package repository without regression, D-Bus API frozen.

## 8. What This Repo Does NOT Do

- Does not manage repository mirrors or sync → handled by SigmaOS-repo-manager
- Does not schedule updates → handled by SigmaOS-update
- Does not define signature trust policy → handled by SigmaOS-security
- Does not build packages → handled by the SigmaOS build system (separate tooling)
- Does not provide a GUI → handled by SigmaOS-control-center's software panel

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/pkg.toml`, `/var/lib/sigmaos/pkg/db.sqlite` (package database)
- **Schema file location:** `schemas/pkg.toml.schema.json`
- **Schema format:** TOML (config), SQLite (database), JSON (package metadata)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_PKG_REPO` | `main` | Active repository name |
| `SIGMAOS_PKG_ASSUME_YES` | `0` | Auto-confirm transactions |
| `SIGMAOS_PKG_DOWNLOAD_ONLY` | `0` | Download without installing |
| `SIGMAOS_PKG_NO_SIGNATURE` | `0` | Disable signature check (dangerous) |

- **Breaking config change policy:** Config keys are additive within a MAJOR version; database schema migrations ship with every upgrade and are reversible.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Dependency resolution failure | Conflicting package requirements | Transaction aborted with conflict report | Review conflict; pin or exclude a package; rerun |
| Signature verification failure | Tampered or unsigned package | Transaction aborted; package quarantined | Re-sync repo metadata; if intentional, file security issue |
| Transaction interrupted | Power loss mid-transaction | System in inconsistent state | On reboot, `sigmaos-pkg --finish-transaction` resumes or rolls back |
| Disk full mid-transaction | Insufficient space | Transaction fails before write | Free space; rerun transaction |
| Database lock stale | Previous process crashed holding lock | New transactions block | Run `sigmaos-pkg --clear-lock` after confirming no process holds it |

- **Fallback behavior:** If the active repo is unreachable, the manager falls back to cached metadata and previously downloaded packages; if signatures cannot be verified, transactions are refused (never silently installed).
- **Operator escalation path:** Collect `/var/log/sigmaos/pkg.log` and `sigmaos-pkg --history`; file issue.
- **Crash dump artifacts:** Transaction state at `/var/lib/sigmaos/pkg/transactions/`; log at `/var/log/sigmaos/pkg.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager updates itself like any other package; post-install hook restarts the D-Bus service.
- **Rollback mechanism:** Every transaction writes a snapshot of the affected file set before applying changes; `sigmaos-pkg --rollback <transaction-id>` restores the previous state. The last 5 transactions are retained.
- **Expected downtime on update:** Brief D-Bus service restart (~2 seconds); in-flight transactions complete before restart.
- **Pre-update state checks:** No active transactions; database not locked; sufficient disk space.
- **Post-update validation:** `sigmaos-pkg --self-check` verifies the local database integrity and signature verification chain.
- **Atomic update guarantee:** Partial — file writes are atomic per-file; transaction as a whole is journaled so it can be completed or rolled back after interruption.

## 12. Security Boundary

- **Privilege level required:** root for system package operations; D-Bus policy allows unprivileged queries but restricts mutations to authorized callers via polkit.
- **Trust boundary:** Trusts only packages signed by keys in the SigmaOS trust store managed by SigmaOS-security; does not trust network-provided packages without signature.
- **Attack surface:** Network (repo downloads), filesystem (package extraction), D-Bus IPC
- **Hardening measures:** All packages signature-verified by default; extraction runs in a restricted namespace; D-Bus policy restricts callers; package scripts run with limited capabilities.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Internal review 2024-12; external audit planned for v1.0.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-repo-manager | 0.2+ | tested |
| SigmaOS-security | 0.3+ | tested |
| SigmaOS-update | 0.3+ | tested |
| SigmaOS-installer | 0.2+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| libsolv | 0.7.25+ | tested |
| gpgme | 1.20+ | tested |
| sqlite | 3.44+ | tested |

- **Known incompatibilities:** None currently.
- **Forward-compatibility pledge:** D-Bus API and CLI stable for v1.x series; package metadata format stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-pkg-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/transaction-model.md`
- **Man page or in-app help status:** present — `sigmaos-pkg(8)`
- **Documentation parity policy:** D-Bus method or CLI flag changes require doc updates in the same PR.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:resolver` | Dependency resolution issues |
| `component:signatures` | Signature verification issues |
| `component:transactions` | Transaction state and rollback issues |
| `component:dbus-api` | D-Bus interface issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-package-manager/milestones`
- **Issue triage cadence:** Every Monday and Thursday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for D-Bus API or CLI incompatibility.
- **Version pinning in dependents:** SigmaOS-update and SigmaOS-installer pin `sigmaos-package-manager >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depended-on-by-core — SigmaOS-core packages are installed and updated by this manager; core's service lifecycle depends on package transactions completing cleanly.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.PackageManager1`), shared library (`libsigmaos-pkg.so`).
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — core package install/upgrade tested end-to-end.
- **Change coordination policy:** Transaction model or D-Bus API changes require a tracking issue in SigmaOS-core; core's post-install hooks depend on this manager's hook execution order.
- **Core team review required for cross-boundary changes:** yes

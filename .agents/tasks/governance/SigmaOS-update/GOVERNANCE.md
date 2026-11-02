# Governance: SigmaOS-update

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-update owns the system update orchestration: scheduling update checks, staging updates before applying them, coordinating health checks before and after update, performing rollback when an update fails validation, supporting delta updates for efficiency, and notifying the user and administrator of update status. It is the policy layer that drives SigmaOS-package-manager.
- **Runtime layer:** core-system
- **Scope boundary with adjacent repos:** SigmaOS-package-manager performs the actual package transactions; SigmaOS-repo-manager provides the metadata; SigmaOS-core provides the health-check API this repo queries to validate updates; SigmaOS-security defines the secure update flow policy.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-update` CLI | CLI | stable |
| `org.sigmaos.Update1` D-Bus service | D-Bus | stable |
| `/etc/sigmaos/update.toml` | Config file | stable |
| Update status notification (D-Bus signal) | D-Bus signal | stable |
| Delta update manifest format | File format | unstable |
| Internal scheduler | internal | internal-only |

- **Breaking-change policy:** CLI and D-Bus stable within a MAJOR version; new methods additive; flag removals require 2-release deprecation.
- **API documentation location:** `docs/api/`, `man/sigmaos-update.8`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-package-manager | 0.3 | performs package transactions | No |
| SigmaOS-repo-manager | 0.2 | metadata for update availability | No |
| SigmaOS-core | 0.4 | health-check API for validation | No |
| SigmaOS-security | 0.3 | secure update flow policy | No |
| systemd | 254 | timer-based scheduling | No |
| dbus | 1.14 | IPC bus | No |

- **Optional/soft dependencies:** `libnotify` for desktop notifications; `zstd` for delta patch compression.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** routine review quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-update-team
- **Co-maintainers:** @pkg-lead, @security-lead
- **Review quorum:** 2 maintainer approvals required; rollback logic changes require @security-lead approval.
- **Escalation path:** Update regressions that leave systems unbootable are `priority:critical`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 60% — scheduler, staging, and rollback logic unit-tested.
- **Integration tests:** Present — staged update + health check + rollback cycle tested on QEMU.
- **CI pipeline file:** `.github/workflows/update-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `staged-update-test`, `rollback-test`, `health-check-test`, `lint`
- **Known flaky tests:** `test-network-check` flaky on CI network (issue #18).
- **CI build badge:** `![update CI](https://github.com/SigmaOS/SigmaOS-update/actions/workflows/update-ci.yml/badge.svg)`

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
- **Criteria for promotion:** Staged update validated on 5 reference systems, rollback verified after failed health check, delta update bandwidth savings confirmed, notification system reliable.

## 8. What This Repo Does NOT Do

- Does not perform package install/remove → handled by SigmaOS-package-manager
- Does not sync repositories → handled by SigmaOS-repo-manager
- Does not define signature trust → handled by SigmaOS-security
- Does not manage kernel rollback boot entries → handled by SigmaOS-boot
- Does not provide a GUI → handled by SigmaOS-control-center

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/update.toml`
- **Schema file location:** `schemas/update.toml.schema.json`
- **Schema format:** TOML
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_UPDATE_CHANNEL` | `stable` | `stable` / `testing` |
| `SIGMAOS_UPDATE_AUTO` | `0` | Apply updates automatically |
| `SIGMAOS_UPDATE_STAGING` | `1` | Stage before applying |
| `SIGMAOS_UPDATE_DELTA` | `1` | Prefer delta updates |

- **Breaking config change policy:** Config keys additive within a MAJOR version; removed keys warn for one release before deletion.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Health check fails post-update | Update broke a service | Update auto-rolled back; user notified | Check health-check log; file issue if rollback also fails |
| Staging disk full | Insufficient space to stage | Update refused; user notified | Free space; retry |
| Delta patch corrupt | Damaged delta manifest | Falls back to full package download | Automatic; logged as warning |
| Scheduler missed window | System asleep during window | Update deferred to next window | Resumes on wake; logged |
| Rollback failure | Previous state also broken | System enters degraded mode | Boot previous kernel via SigmaOS-boot recovery; manual recovery |

- **Fallback behavior:** If staging fails, falls back to direct (non-staged) update with explicit warning; if health check is unavailable, update proceeds but marks system as "unvalidated".
- **Operator escalation path:** Collect `/var/log/sigmaos/update.log` and `sigmaos-update --history`; file issue.
- **Crash dump artifacts:** Update log at `/var/log/sigmaos/update.log`; staging state at `/var/lib/sigmaos/update/staging/`.

## 11. Update / Rollback Behavior

- **Update mechanism:** This service coordinates its own updates via SigmaOS-package-manager; post-install hook restarts the D-Bus service.
- **Rollback mechanism:** Maintains a pre-update snapshot of affected package versions; `sigmaos-update --rollback` restores via SigmaOS-package-manager; kernel updates additionally rely on SigmaOS-boot's boot-entry rollback.
- **Expected downtime on update:** Brief D-Bus restart; system updates may require reboot (coordinated with user notification).
- **Pre-update state checks:** No critical services failing; sufficient staging disk space; battery > 20% on laptops.
- **Post-update validation:** Runs SigmaOS-core health-check API; if any critical check fails, automatic rollback is triggered.
- **Atomic update guarantee:** Partial — staging is atomic; application is transactional via the package manager; rollback restores previous state.

## 12. Security Boundary

- **Privilege level required:** Runs as a dedicated `sigmaos-update` system user; D-Bus queries unprivileged, trigger/rollback gated by polkit.
- **Trust boundary:** Trusts only packages verified by SigmaOS-package-manager's signature check; does not trust network-provided update metadata without signature.
- **Attack surface:** Network (update checks), D-Bus IPC, filesystem (staging)
- **Hardening measures:** Staging directory is root-owned and mode 700; runs with `NoNewPrivileges`, `ProtectSystem=strict`; D-Bus policy restricts callers.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Internal review 2024-12.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-package-manager | 0.3+ | tested |
| SigmaOS-repo-manager | 0.2+ | tested |
| SigmaOS-core | 0.4+ | tested |
| SigmaOS-security | 0.3+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| systemd | 254+ | tested |
| dbus | 1.14+ | tested |

- **Known incompatibilities:** None.
- **Forward-compatibility pledge:** D-Bus API and CLI stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-update-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/rollback-design.md`
- **Man page or in-app help status:** present — `sigmaos-update(8)`
- **Documentation parity policy:** Config or D-Bus changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:scheduler` | Update scheduling issues |
| `component:staging` | Update staging issues |
| `component:rollback` | Rollback logic issues |
| `component:delta` | Delta update issues |
| `component:notifications` | User notification issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-update/milestones`
- **Issue triage cadence:** Every Monday and Thursday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for D-Bus API or CLI incompatibility.
- **Version pinning in dependents:** No downstream dependents; this is a top-level orchestrator.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depends-on-core — uses core's health-check API to validate updates and coordinates with core's service lifecycle during update.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Update1` queries `org.sigmaos.Core1` health-check); also drives SigmaOS-package-manager via D-Bus.
- **Core API version it targets:** `SigmaOS-core >= 0.4` (health-check API)
- **Integration tested in CI:** yes — update cycle test boots SigmaOS-core and validates health checks.
- **Change coordination policy:** Health-check API changes require a tracking issue in this repo; rollback triggers must be coordinated with core service stop/start.
- **Core team review required for cross-boundary changes:** yes

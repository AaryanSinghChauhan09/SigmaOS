# Governance: SigmaOS-repo-manager

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-repo-manager owns the repository metadata lifecycle: synchronizing remote repositories to local mirrors, managing mirror selection and priority, verifying repository metadata integrity, and exposing the synchronized metadata that SigmaOS-package-manager consumes. It defines repository priority ordering and the policy for falling back between mirrors.
- **Runtime layer:** core-system (runs as a background service)
- **Scope boundary with adjacent repos:** SigmaOS-package-manager consumes the metadata this repo produces; SigmaOS-update schedules when syncs occur but does not perform them; SigmaOS-security defines the metadata verification trust anchors this repo enforces.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-repo` CLI | CLI | stable |
| `org.sigmaos.RepoManager1` D-Bus service | D-Bus | stable |
| `/etc/sigmaos/repos.d/*.repo` repo definition format | Config file | stable |
| Mirror metadata cache `/var/cache/sigmaos/repos/` | File format | unstable |
| Sync event notification on D-Bus | D-Bus signal | unstable |
| Internal sync scheduler | internal | internal-only |

- **Breaking-change policy:** Repo definition format is stable within a MAJOR version; CLI flag removals require a 2-release deprecation.
- **API documentation location:** `docs/api/`, `man/sigmaos-repo.8`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-package-manager | 0.3 | consumes synced metadata | No |
| SigmaOS-security | 0.3 | metadata verification trust anchors | No |
| libcurl | 8.4 | HTTP(S) repository downloads | No |
| gpgme | 1.20 | metadata signature verification | No |
| rsync | 3.2 | mirror sync (rsync protocol) | No |
| sqlite | 3.44 | local metadata index | No |

- **Optional/soft dependencies:** `metalink`/`mirrorlist` generators for geo-based mirror selection.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** libcurl and gpgme updated within 1 week of security releases.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-repo-team
- **Co-maintainers:** @pkg-lead, @infra-lead
- **Review quorum:** 2 maintainer approvals required.
- **Escalation path:** Repo sync failures that block updates are `priority:high`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 55% — mirror selection, priority logic, and metadata parsing unit-tested.
- **Integration tests:** Present — sync against a fixture repository with signed metadata.
- **CI pipeline file:** `.github/workflows/repo-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `sync-fixture-test`, `metadata-verify-test`, `lint`
- **Known flaky tests:** `test-network-sync` flaky on CI network (issue #12).
- **CI build badge:** `![repo CI](https://github.com/SigmaOS/SigmaOS-repo-manager/actions/workflows/repo-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** runs on any; tested in CI on QEMU VMs.
- **Backend variants:** Linux tested in CI.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** Multi-mirror failover validated, metadata signature verification end-to-end, priority ordering deterministic, D-Bus API frozen.

## 8. What This Repo Does NOT Do

- Does not install or remove packages → handled by SigmaOS-package-manager
- Does not schedule updates → handled by SigmaOS-update
- Does not define signature trust policy → handled by SigmaOS-security
- Does not host the canonical repositories → handled by release infrastructure
- Does not provide a GUI → handled by SigmaOS-control-center

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/repos.d/*.repo`, `/etc/sigmaos/repo-manager.toml`
- **Schema file location:** `schemas/repo-definition.schema.json`
- **Schema format:** INI-style `.repo` files, TOML for manager config
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_REPO_SYNC_INTERVAL` | `3600` | Seconds between automatic syncs |
| `SIGMAOS_REPO_MIRROR_PRIORITY` | `auto` | Mirror selection strategy |
| `SIGMAOS_REPO_OFFLINE` | `0` | Use only cached metadata |

- **Breaking config change policy:** `.repo` file format is additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| All mirrors unreachable | Network outage | Sync fails; package-manager uses stale cache | Wait for network; `sigmaos-repo --sync --force` |
| Metadata signature mismatch | Tampered or stale metadata | Sync refused; error logged | Re-fetch trusted key from SigmaOS-security; resync |
| Disk full during sync | Insufficient cache space | Sync aborts | Clean cache with `sigmaos-repo --clean-cache`; resync |
| Mirror priority deadlock | Misconfigured priority config | No mirror selected | Validate config with `sigmaos-repo --validate` |
| Stale metadata lock | Previous sync crashed | New sync blocks | Run `sigmaos-repo --clear-lock` |

- **Fallback behavior:** If primary mirror fails, automatically falls through to lower-priority mirrors; if all fail, uses the last good cached metadata.
- **Operator escalation path:** Collect `/var/log/sigmaos/repo.log`; file issue.
- **Crash dump artifacts:** Sync log at `/var/log/sigmaos/repo.log`; cache at `/var/cache/sigmaos/repos/`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager updates this service; post-install hook restarts the D-Bus service.
- **Rollback mechanism:** Standard package rollback via SigmaOS-package-manager; repository definitions are user data and never auto-rolled back.
- **Expected downtime on update:** Brief D-Bus restart (~2 seconds).
- **Pre-update state checks:** No active sync in progress.
- **Post-update validation:** `sigmaos-repo --self-check` verifies metadata index integrity.
- **Atomic update guarantee:** Partial — metadata index written to temp file and renamed atomically.

## 12. Security Boundary

- **Privilege level required:** Runs as a dedicated `sigmaos-repo` system user; D-Bus queries unprivileged, sync operations gated by polkit.
- **Trust boundary:** Trusts only metadata signed by keys in the SigmaOS trust store; does not trust mirror-provided content without signature verification.
- **Attack surface:** Network (mirror downloads), filesystem (cache writes), D-Bus IPC
- **Hardening measures:** TLS certificate verification on all HTTPS downloads; signature verification on all metadata; runs with `NoNewPrivileges`, `ProtectSystem=strict`.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Internal review 2024-12.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-package-manager | 0.3+ | tested |
| SigmaOS-security | 0.3+ | tested |
| SigmaOS-update | 0.3+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| libcurl | 8.4+ | tested |
| gpgme | 1.20+ | tested |
| rsync | 3.2+ | tested |

- **Known incompatibilities:** None.
- **Forward-compatibility pledge:** `.repo` format and D-Bus API stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-repo-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`
- **Man page or in-app help status:** present — `sigmaos-repo(8)`
- **Documentation parity policy:** Repo definition format or D-Bus changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:sync` | Repository sync issues |
| `component:mirrors` | Mirror selection and priority issues |
| `component:metadata` | Metadata verification issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-repo-manager/milestones`
- **Issue triage cadence:** Every Wednesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for `.repo` format or D-Bus incompatibility.
- **Version pinning in dependents:** SigmaOS-package-manager pins `sigmaos-repo-manager >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depended-on-by-core — core's package operations depend on the metadata this service provides.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.RepoManager1`), shared cache files consumed by SigmaOS-package-manager.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — package install test uses this repo's synced metadata.
- **Change coordination policy:** Metadata format changes require a tracking issue in SigmaOS-package-manager.
- **Core team review required for cross-boundary changes:** yes

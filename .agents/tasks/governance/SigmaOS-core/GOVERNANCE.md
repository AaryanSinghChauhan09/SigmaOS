# Governance: SigmaOS-core

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-core is the canonical runtime foundation of the operating system. It owns the process lifecycle model, base system APIs, service startup dependency graph, privilege boundary definitions, first-boot initialization, and the central configuration store that all other SigmaOS subsystems read from.
- **Runtime layer:** core-system
- **Scope boundary with adjacent repos:** SigmaOS-kernel provides the kernel ABI that core builds on top of; SigmaOS-security owns privilege enforcement policy; SigmaOS-update owns delivery of new core packages; SigmaOS-desktop consumes the session lifecycle API core exposes.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `org.sigmaos.Core1` D-Bus service | D-Bus | stable |
| `libsigmaos-core.so` | Shared library | stable |
| `/etc/sigmaos/core.toml` | Config file | stable |
| `sigmaos-core-status` CLI | CLI | stable |
| `SIGMAOS_CORE_VERSION` env var | Environment variable | stable |
| `core_event` Unix socket at `/run/sigmaos/core.sock` | IPC socket | unstable |
| Internal service registry D-Bus interface | D-Bus | internal-only |

- **Breaking-change policy:** Stable D-Bus interfaces follow semver; breaking changes require a MAJOR version bump with 60-day notice and a migration guide published before the change lands.
- **API documentation location:** `docs/api/` and generated from D-Bus introspection XML at `dbus/org.sigmaos.Core1.xml`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-kernel | 6.6.0 | kernel ABI, `/proc`/`/sys` | No |
| systemd | 254 | service orchestration on Linux target | No |
| dbus | 1.14 | IPC bus | No |
| polkit | 122 | privilege escalation gating | No |
| glib | 2.78 | core utilities, GObject type system | No |

- **Optional/soft dependencies:** `libaudit` for audit log integration (enabled when SigmaOS-security is present); `NetworkManager` for network-aware startup ordering.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** Dependencies reviewed quarterly; security-relevant updates applied within one week of upstream release.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-core-team
- **Co-maintainers:** @runtime-lead, @arch-lead
- **Review quorum:** 2 maintainer approvals required; at least one must be @sigmaos-core-team
- **Escalation path:** Critical issues escalate to @sigmaos-core-team lead via the `priority:critical` label; response SLA is 4 hours.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 72% line coverage via `cargo test` / `pytest` (language TBD per implementation)
- **Integration tests:** Present — service lifecycle integration tests run on QEMU with a minimal SigmaOS rootfs
- **CI pipeline file:** `.github/workflows/core-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `integration-smoke`, `dbus-api-compat-check`, `lint`, `security-scan`
- **Known flaky tests:** `test_first_boot_timeout` intermittently fails under high CI load (issue #88)
- **CI build badge:** `![core CI](https://github.com/SigmaOS/SigmaOS-core/actions/workflows/core-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |
| FreeBSD | 14.0 | planned |

- **Hardware classes:** desktop, laptop, server, VM — all supported.
- **Backend variants:** Linux backend tested in CI; BSD backend planned for v0.6.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** All public D-Bus APIs frozen, health-check subsystem passing on 5 reference hardware platforms, first-boot flow validated end-to-end, zero P0 bugs open.

## 8. What This Repo Does NOT Do

- Does not manage bootloader or kernel selection → handled by SigmaOS-boot
- Does not enforce security policy → handled by SigmaOS-security
- Does not deliver package updates → handled by SigmaOS-update
- Does not manage the desktop session → handled by SigmaOS-desktop
- Does not provide a display manager or greeter → handled by SigmaOS-display-manager
- Does not own network configuration → handled by SigmaOS-network

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/core.toml`, `/var/lib/sigmaos/core-state.json` (runtime state)
- **Schema file location:** `schemas/core.toml.schema.json`
- **Schema format:** TOML (user-facing), JSON (runtime state)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_CORE_VERSION` | set by package | Advertises running core version to subsystems |
| `SIGMAOS_CORE_LOG_LEVEL` | `info` | Controls structured log verbosity |
| `SIGMAOS_FIRST_BOOT` | `0` | Set to `1` by installer on first boot |

- **Breaking config change policy:** Removed keys are deprecated for one release cycle before removal; migration scripts live in `migrations/` and run automatically on upgrade.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| D-Bus service crash | Unhandled exception in `org.sigmaos.Core1` | Dependent services fail to communicate | systemd auto-restarts the service; check `journalctl -u sigmaos-core` |
| Config parse error | Malformed `core.toml` | Boot halts at core initialization | Boot recovery mode edits `/etc/sigmaos/core.toml`; `sigmaos-core --validate-config` |
| First-boot loop | Failed post-install hook | System reboots repeatedly | Emergency shell via SigmaOS-boot recovery entry; run `sigmaos-firstboot --reset` |
| Service dependency deadlock | Circular startup dependency | System hangs during boot | Boot with `sigmaos.debug.startup=1` kernel param to emit dependency graph to serial |

- **Fallback behavior:** If D-Bus fails to start, core falls back to a minimal shell-based service registry for emergency recovery.
- **Operator escalation path:** Collect `journalctl -b -u sigmaos-core` and `/var/log/sigmaos/core.log`; file issue with `priority:critical`.
- **Crash dump artifacts:** Core dumps under `/var/crash/sigmaos-core/`; structured logs at `/var/log/sigmaos/`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager installs new `sigmaos-core` package; service is restarted post-install.
- **Rollback mechanism:** Previous `sigmaos-core` package retained for one version; `sigmaos-rollback core` CLI reverts to it.
- **Expected downtime on update:** Brief restart (~2 seconds D-Bus service restart); dependent services reconnect automatically.
- **Pre-update state checks:** All dependent services must be in a stable state; no active transactions in service registry.
- **Post-update validation:** `sigmaos-core --self-check` runs D-Bus API smoke tests; result reported to SigmaOS-update.
- **Atomic update guarantee:** Partial — config files written atomically; binary swap is package-manager-atomic.

## 12. Security Boundary

- **Privilege level required:** Runs as a dedicated `sigmaos-core` system user with specific capabilities; does not run as root.
- **Trust boundary:** Trusts SigmaOS-security's polkit policy; does not trust user-space processes calling D-Bus without proper authentication.
- **Attack surface:** D-Bus IPC, Unix socket at `/run/sigmaos/core.sock`, config file parsing
- **Hardening measures:** Runs in a systemd service with `NoNewPrivileges=yes`, `ProtectSystem=strict`, `PrivateTmp=yes`, `RestrictAddressFamilies=AF_UNIX AF_NETLINK`; D-Bus policy restricts callers to authorized UIDs.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Internal review 2024-12; external audit planned for v1.0.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-kernel | 6.6.x | tested |
| SigmaOS-security | 0.3+ | tested |
| SigmaOS-desktop | 0.4+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| systemd | 254–256 | tested |
| dbus | 1.14+ | tested |
| polkit | 122+ | tested |

- **Known incompatibilities:** polkit < 120 is incompatible with the privilege model; systemd < 250 lacks required unit features.
- **Forward-compatibility pledge:** D-Bus API stable interface remains compatible for the entire v1.x series.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/architecture.md`
- **Man page or in-app help status:** present — `sigmaos-core(8)`, `sigmaos-core-status(1)`
- **Documentation parity policy:** Every D-Bus API change must include updated introspection XML and updated `docs/api/`; PR is blocked until docs are included.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:service-lifecycle` | Issues with service start/stop/restart |
| `component:config-store` | Issues with central config schema |
| `component:health-checks` | Issues with health monitoring subsystem |
| `component:first-boot` | Issues with first-boot initialization flow |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-core/milestones`
- **Issue triage cadence:** Every Monday and Thursday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated via `git-cliff`
- **Breaking change signal:** MAJOR bump required; D-Bus stable interfaces must not break within a MAJOR version.
- **Version pinning in dependents:** All SigmaOS subsystem repos pin `sigmaos-core >= MAJOR.MINOR` (minor-compatible range).

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** This IS the core repo. All other SigmaOS repos either depend on it or provide services it orchestrates.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Core1`), shared library (`libsigmaos-core.so`), Unix socket.
- **Core API version it targets:** N/A — this repo defines the core API version.
- **Integration tested in CI:** yes — integration tests pull in key dependent repos.
- **Change coordination policy:** Any change to stable D-Bus interfaces requires a cross-repo tracking issue; affected repos are listed and must have PRs ready before core merges.
- **Core team review required for cross-boundary changes:** yes — by definition.

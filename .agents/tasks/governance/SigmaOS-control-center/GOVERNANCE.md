# Governance: SigmaOS-control-center

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-control-center owns the system settings user interface and the settings panel trait that other SigmaOS components implement to expose their configuration. It provides the daemon config interface that reads and writes to SigmaOS-core's central config store, integrates polkit for privileged settings changes, and presents a unified settings shell for display, network, sound, power, accounts, and updates. This is the canonical merger of the former SigmaOS-control-center, SigmaOS-settings, and SigmaOS-daemon-settings repos.
- **Runtime layer:** desktop-shell (user-facing settings app) + core-system (daemon config interface)
- **Scope boundary with adjacent repos:** SigmaOS-core owns the central config store this reads/writes; SigmaOS-security owns polkit policy this integrates; SigmaOS-desktop owns the session; SigmaOS-network, SigmaOS-hardware, SigmaOS-update each implement the settings panel trait to surface their config here.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-control-center` CLI | CLI | stable |
| `org.sigmaos.ControlCenter1` D-Bus service | D-Bus | stable |
| Settings panel trait (panel plugin API) | Interface | stable |
| `/etc/sigmaos/control-center.toml` | Config file | stable |
| Daemon config interface D-Bus | D-Bus | stable |
| Internal panel registry | internal | internal-only |

- **Breaking-change policy:** Panel trait and D-Bus API stable within a MAJOR version; new panels additive; trait method removals require a 2-release deprecation.
- **API documentation location:** `docs/api/`, `docs/panel-trait.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-core | 0.4 | central config store read/write | No |
| SigmaOS-security | 0.3 | polkit integration | No |
| SigmaOS-desktop | 0.2 | session integration | No |
| gtk4 / libadwaita | 4.12 / 1.4 | settings UI toolkit | No |
| polkit | 122 | privilege gating for settings | No |
| dbus | 1.14 | IPC with daemons | No |

- **Optional/soft dependencies:** `accountsservice` for user management panel; `NetworkManager` for network panel; `upower` for power panel.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** routine review quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-control-center-team
- **Co-maintainers:** @ux-lead, @core-lead
- **Review quorum:** 2 maintainer approvals required; panel trait changes require @core-lead approval.
- **Escalation path:** Settings regressions are `priority:high` if they prevent changing critical config.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 50% — panel trait loader and config interface unit-tested.
- **Integration tests:** Present — control center launches with fixture panels; config write round-trip tested.
- **CI pipeline file:** `.github/workflows/control-center-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `panel-trait-test`, `config-roundtrip-test`, `polkit-gate-test`, `lint`
- **Known flaky tests:** `polkit-gate-test` flaky in CI (policykit daemon timing, issue #14).
- **CI build badge:** `![control-center CI](https://github.com/SigmaOS/SigmaOS-control-center/actions/workflows/control-center-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** desktop, laptop — supported.
- **Backend variants:** Linux only; requires GUI.

## 7. Release Maturity

- **Current maturity label:** `alpha`
- **Last label change date:** 2025-01-01
- **Planned next label:** `alpha → beta` at v0.6
- **Criteria for promotion:** Panel trait frozen, 5 core panels (display, network, sound, power, accounts) production-quality, polkit integration validated, daemon config interface stable.

## 8. What This Repo Does NOT Do

- Does not own the central config store → handled by SigmaOS-core (this reads/writes it)
- Does not define privilege policy → handled by SigmaOS-security (this enforces via polkit)
- Does not manage network config → handled by SigmaOS-network (provides the panel)
- Does not manage power policy → handled by SigmaOS-hardware (provides the panel)
- Does not perform updates → handled by SigmaOS-update (provides the panel)

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/control-center.toml`, `/etc/sigmaos/control-center/panels/` (panel definitions)
- **Schema file location:** `schemas/control-center.toml.schema.json`, `schemas/panel-trait.schema.json`
- **Schema format:** TOML (config), JSON (panel trait), D-Bus XML (daemon interface)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_CONTROL_CENTER_PROFILE` | `full` | `full` / `minimal` panel set |
| `XDG_CURRENT_DESKTOP` | `SigmaOS` | Freedesktop identifier |

- **Breaking config change policy:** Config keys and panel trait additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Panel fails to load | Broken panel plugin | Panel missing from UI | Panel isolated; other panels load; check log |
| Config write fails | Central config store locked | Setting change does not persist | Retry; ensure no other process holds the store lock |
| Polkit denial | User lacks privilege for setting | Setting change refused | Authenticate via polkit prompt; check policy |
| Daemon config interface down | Target daemon not running | Panel shows stale data | Restart target daemon; panel reconnects |
| Control center crash | UI bug | App exits | Relaunch; logs in `~/.local/share/sigmaos/control-center.log` |

- **Fallback behavior:** A failed panel is isolated and does not prevent other panels from loading; config writes that fail are rolled back; if polkit is unavailable, privileged settings are hidden rather than silently failing.
- **Operator escalation path:** Collect `~/.local/share/sigmaos/control-center.log`; file issue.
- **Crash dump artifacts:** Log at `~/.local/share/sigmaos/control-center.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager; control center process restart picks up new version.
- **Rollback mechanism:** Previous package retained via SigmaOS-package-manager; user config untouched.
- **Expected downtime on update:** None — app is on-demand.
- **Pre-update state checks:** None required.
- **Post-update validation:** Next launch verifies panels load.
- **Atomic update guarantee:** Partial — binary swap atomic; config writes transactional via SigmaOS-core.

## 12. Security Boundary

- **Privilege level required:** Runs as the logged-in user; privileged settings changes gated by polkit (which may require admin auth).
- **Trust boundary:** Trusts SigmaOS panels; third-party panels run with reduced capabilities; does not trust panel-provided config without schema validation.
- **Attack surface:** D-Bus IPC, config file parsing, panel plugin loading
- **Hardening measures:** Panel plugins validated against trait schema; privileged writes gated by polkit; runs with `NoNewPrivileges`; config writes validated against schema.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-core | 0.4+ | tested |
| SigmaOS-security | 0.3+ | tested |
| SigmaOS-desktop | 0.2+ | tested |
| SigmaOS-network | 0.2+ | tested (network panel) |
| SigmaOS-hardware | 0.2+ | tested (power panel) |

| Package | Version Range | Status |
|---------|---------------|--------|
| gtk4 | 4.12+ | tested |
| libadwaita | 1.4+ | tested |
| polkit | 122+ | tested |

- **Known incompatibilities:** polkit < 122 lacks required policy features.
- **Forward-compatibility pledge:** Panel trait and D-Bus API stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-settings-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/panel-trait.md`
- **Man page or in-app help status:** present — `sigmaos-control-center(8)`
- **Documentation parity policy:** Panel trait or D-Bus changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:panels` | Panel plugin and registry issues |
| `component:daemon-config` | Daemon config interface issues |
| `component:polkit` | Polkit integration issues |
| `component:ui` | Settings UI issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-control-center/milestones`
- **Issue triage cadence:** Every Tuesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for panel trait or D-Bus incompatibility.
- **Version pinning in dependents:** Panels pin `sigmaos-control-center >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depends-on-core — control center reads and writes SigmaOS-core's central config store and queries core service status for display.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.ControlCenter1` queries `org.sigmaos.Core1`); config store files.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — config write round-trip tested against core's config store.
- **Change coordination policy:** Central config store schema changes require a tracking issue in this repo; panel additions for new core config keys must land together.
- **Core team review required for cross-boundary changes:** yes

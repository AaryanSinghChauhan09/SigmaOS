# Governance: SigmaOS-panel

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-panel owns the desktop status bar / panel component of the SigmaOS shell. It defines the panel contract that applets implement, exposes the applet API for third-party panel widgets, integrates with Waybar as the rendering backend, and manages the IPC channel between applets and the rest of the desktop shell (clock, notifications, system tray, workspace indicator, power menu).
- **Runtime layer:** desktop-shell
- **Scope boundary with adjacent repos:** SigmaOS-desktop owns the session lifecycle that starts this panel; SigmaOS-launcher is a separate component invoked from the panel; SigmaOS-control-center provides the settings the panel exposes; SigmaOS-network provides network status the panel displays.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-panel` CLI | CLI | stable |
| Applet manifest schema (`applets/*.toml`) | Config file | stable |
| Applet IPC protocol (D-Bus) | D-Bus | stable |
| `org.sigmaos.Panel1` D-Bus service | D-Bus | stable |
| Waybar module config contract | Config file | unstable |
| Internal renderer | internal | internal-only |

- **Breaking-change policy:** Applet manifest schema and IPC protocol stable within a MAJOR version; new signals/methods additive; applet API removals require a 2-release deprecation.
- **API documentation location:** `docs/api/`, `docs/applet-api.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-desktop | 0.2 | session lifecycle, theme contract | No |
| Waybar | 0.10 | rendering backend | No |
| wlroots / Wayland | 0.17 | layer-shell protocol for panel | No |
| gtk3 / gtk4 | 4.12 | applet widget toolkit | No |
| dbus | 1.14 | applet IPC | No |
| jsoncpp / nlohmann json | 3.11 | Waybar config generation | No |

- **Optional/soft dependencies:** `libappindicator` for legacy tray; `pipewire` for volume applet.
- **Upstream patch inventory:** Local Waybar config generator; no patches against Waybar source.
- **Dependency update policy:** Waybar updated within 1 week of security releases.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-panel-team
- **Co-maintainers:** @ux-lead, @desktop-lead
- **Review quorum:** 2 maintainer approvals required; applet API changes require @desktop-lead approval.
- **Escalation path:** Panel crash regressions are `priority:high`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 50% — applet manifest parsing and IPC handler unit-tested.
- **Integration tests:** Present — headless panel render test with fixture applets.
- **CI pipeline file:** `.github/workflows/panel-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `applet-ipc-test`, `config-validation`, `lint`
- **Known flaky tests:** `headless-render-test` flaky on GPU-less runners (issue #6).
- **CI build badge:** `![panel CI](https://github.com/SigmaOS/SigmaOS-panel/actions/workflows/panel-ci.yml/badge.svg)`

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
- **Backend variants:** Linux only; requires Wayland layer-shell.

## 7. Release Maturity

- **Current maturity label:** `alpha`
- **Last label change date:** 2025-01-01
- **Planned next label:** `alpha → beta` at v0.6
- **Criteria for promotion:** Applet API frozen, IPC protocol stable, 3 built-in applets (clock, workspaces, power) production-quality, theme integration validated.

## 8. What This Repo Does NOT Do

- Does not manage the desktop session → handled by SigmaOS-desktop
- Does not provide the app launcher → handled by SigmaOS-launcher
- Does not provide settings UI → handled by SigmaOS-control-center
- Does not provide themes → handled by SigmaOS-theme (panel only consumes the contract)
- Does not manage notifications daemon → handled by external notification daemon (mako/dunst)

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/panel.toml`, `~/.config/sigmaos/panel.toml`, `~/.config/sigmaos/panel/applets/`
- **Schema file location:** `schemas/applet-manifest.schema.json`, `schemas/panel.toml.schema.json`
- **Schema format:** TOML (config + manifests), D-Bus XML (IPC)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_PANEL_POSITION` | `top` | `top` / `bottom` |
| `SIGMAOS_PANEL_HEIGHT` | `30` | Panel height in pixels |
| `SIGMAOS_PANEL_BACKEND` | `waybar` | Rendering backend |
| `WAYLAND_DISPLAY` | `wayland-0` | Wayland socket (inherited) |

- **Breaking config change policy:** Config keys and applet manifest schema additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Panel fails to start | Wayland layer-shell unavailable | No status bar | Verify compositor supports layer-shell; restart session |
| Applet crash | Bug in third-party applet | Applet disappears; panel continues | Panel isolates applets; reload applet via `sigmaos-panel --reload-applet <name>` |
| IPC mismatch | Panel and applet version skew | Applet shows no data | Restart applet; ensure matching panel version |
| Config parse error | Malformed panel.toml | Panel uses default config | Validate config; fix syntax |
| Waybar backend crash | Waybar process dies | Panel disappears | Panel supervisor restarts Waybar automatically; logs error |

- **Fallback behavior:** A crashed applet is isolated and does not take down the panel; the panel restarts its Waybar backend automatically; if config is invalid, defaults are used.
- **Operator escalation path:** Collect `~/.local/share/sigmaos/panel.log`; file issue.
- **Crash dump artifacts:** Panel log at `~/.local/share/sigmaos/panel.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager; panel restart picks up new version; applets reloaded without panel restart where possible.
- **Rollback mechanism:** Previous package retained via SigmaOS-package-manager; user config untouched.
- **Expected downtime on update:** Brief panel flicker on restart (~1 second).
- **Pre-update state checks:** Panel running.
- **Post-update validation:** Panel restarts and all applets reconnect.
- **Atomic update guarantee:** Partial — binary swap atomic; panel state not transactional but applets are isolated.

## 12. Security Boundary

- **Privilege level required:** Runs as the logged-in user; no special privileges; system-status applets query via D-Bus with polkit gating for privileged actions (e.g., reboot).
- **Trust boundary:** Trusts SigmaOS applets; third-party applets run in a restricted IPC sandbox; does not trust applet-provided input without validation.
- **Attack surface:** D-Bus IPC (applet protocol), Wayland socket, config parsing
- **Hardening measures:** Applets isolated; D-Bus policy restricts applet capabilities; config validated against schema before load.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-desktop | 0.2+ | tested |
| SigmaOS-launcher | 0.2+ | tested |
| SigmaOS-control-center | 0.2+ | tested |
| SigmaOS-theme | 0.2+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| Waybar | 0.10+ | tested |
| gtk4 | 4.12+ | tested |
| wlroots | 0.17+ | tested |

- **Known incompatibilities:** Waybar < 0.9 lacks required module hooks.
- **Forward-compatibility pledge:** Applet API and IPC stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-panel-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/applet-api.md`
- **Man page or in-app help status:** present — `sigmaos-panel(8)`
- **Documentation parity policy:** Applet API or IPC changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:applets` | Applet API and built-in applets |
| `component:ipc` | Applet IPC protocol |
| `component:waybar` | Waybar backend integration |
| `component:theme` | Panel theme consumption |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-panel/milestones`
- **Issue triage cadence:** Every Tuesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for applet API or IPC incompatibility.
- **Version pinning in dependents:** Applets pin `sigmaos-panel >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depends-on-core (transitively via SigmaOS-desktop) — panel runs within the desktop session that core's session manager governs.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Panel1`); queries core's status service for system info.
- **Core API version it targets:** `SigmaOS-core >= 0.4` (status query API)
- **Integration tested in CI:** yes — panel startup tested within desktop session integration.
- **Change coordination policy:** Status query API changes require a tracking issue in this repo.
- **Core team review required for cross-boundary changes:** yes

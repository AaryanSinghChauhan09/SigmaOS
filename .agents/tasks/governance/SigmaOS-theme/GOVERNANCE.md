# Governance: SigmaOS-theme

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-theme owns the visual theming system for SigmaOS: the theme schema, dark/light mode switching, GTK and Wayland compositor theme application, and the install/switch/preview lifecycle for themes. It is the canonical merger of the former SigmaOS-theme-registry and SigmaOS-theme-marketplace repos, owning both the schema/registry and the install/switch/preview tooling.
- **Runtime layer:** desktop-shell (user-facing theme management) + tooling (theme packages)
- **Scope boundary with adjacent repos:** SigmaOS-desktop defines the theme contract this repo implements; SigmaOS-panel, SigmaOS-launcher, and SigmaOS-control-center consume themes via the contract; SigmaOS-package-manager delivers theme packages.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-theme` CLI | CLI | stable |
| `org.sigmaos.Theme1` D-Bus service | D-Bus | stable |
| Theme package schema (`theme.toml`) | Config file | stable |
| Theme contract interface (consumed by desktop/panel/launcher) | Interface | stable |
| Theme registry index format | File format | unstable |
| Marketplace metadata format | File format | unstable |
| Internal preview renderer | internal | internal-only |

- **Breaking-change policy:** Theme schema and D-Bus API stable within a MAJOR version; new theme keys additive; schema field removals require a 2-release deprecation.
- **API documentation location:** `docs/api/`, `docs/theme-schema.md`, `docs/theme-contract.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-desktop | 0.2 | theme contract | No |
| gtk4 | 4.12 | GTK theme application | No |
| glib | 2.78 | config utilities | No |
| dbus | 1.14 | IPC with desktop components | No |
| librsvg | 2.57 | SVG rendering for theme previews | No |

- **Optional/soft dependencies:** `kvantum` for Qt theming; `gnome-themes-extra` for compatibility.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** routine review quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-theme-team
- **Co-maintainers:** @ux-lead, @desktop-lead
- **Review quorum:** 2 maintainer approvals required; theme schema changes require @desktop-lead approval.
- **Escalation path:** Theme regressions are `priority:normal`; contract-breaking changes are `priority:high`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 60% — schema validation, mode switching, and index management unit-tested.
- **Integration tests:** Present — theme apply/preview/switch cycle tested headless; GTK theme application verified.
- **CI pipeline file:** `.github/workflows/theme-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `schema-validation-test`, `apply-switch-test`, `lint`
- **Known flaky tests:** `gtk-theme-apply-test` flaky on GPU-less runners (issue #5).
- **CI build badge:** `![theme CI](https://github.com/SigmaOS/SigmaOS-theme/actions/workflows/theme-ci.yml/badge.svg)`

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
- **Backend variants:** Linux only; requires GTK and a Wayland compositor.

## 7. Release Maturity

- **Current maturity label:** `alpha`
- **Last label change date:** 2025-01-01
- **Planned next label:** `alpha → beta` at v0.6
- **Criteria for promotion:** Theme schema frozen, 3 default themes (light, dark, high-contrast) production-quality, install/switch/preview validated, marketplace metadata format stable.

## 8. What This Repo Does NOT Do

- Does not define the theme contract (implements it) → defined by SigmaOS-desktop
- Does not render the panel/launcher UI → handled by those components (they consume themes)
- Does not manage the desktop session → handled by SigmaOS-desktop
- Does not deliver theme packages over the network → handled by SigmaOS-package-manager
- Does not provide icon themes as a separate concern → included in theme packages

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/theme.toml`, `~/.config/sigmaos/theme.toml`, theme packages at `/usr/share/sigmaos/themes/` and `~/.local/share/sigmaos/themes/`
- **Schema file location:** `schemas/theme.toml.schema.json`, `schemas/theme-package.schema.json`
- **Schema format:** TOML (theme packages and config)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_THEME` | `sigmaos-light` | Active theme name |
| `SIGMAOS_THEME_MODE` | `light` | `light` / `dark` / `auto` |
| `GTK_THEME` | derived | GTK theme env var (exported by apply) |
| `QT_QPA_PLATFORMTHEME` | `sigmaos` | Qt theme integration |

- **Breaking config change policy:** Theme schema fields additive within a MAJOR version; removed fields warn for one release before deletion.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Theme fails to apply | Malformed theme package | Fallback theme used | Automatic fallback; reinstall theme via SigmaOS-theme |
| Theme not found | Uninstalled or renamed | Default theme used | Reinstall theme; check name in registry |
| GTK theme apply fails | GTK config conflict | Theme partially applied | Reset GTK config: `sigmaos-theme --reset-gtk` |
| Dark/light auto-switch fails | Missing portal/time config | Stuck in one mode | Configure auto-switch source in theme.toml |
| Preview render fails | Missing SVG assets | Preview blank | Validate theme package assets; reinstall |
| Marketplace sync fails | Network issue | Theme list stale | Retry sync; use cached list |

- **Fallback behavior:** If the active theme fails to load, falls back to the default SigmaOS theme and logs a warning; if mode auto-switch is unavailable, stays in the last mode.
- **Operator escalation path:** Collect `~/.local/share/sigmaos/theme.log`; file issue.
- **Crash dump artifacts:** Theme log at `~/.local/share/sigmaos/theme.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager updates the theme tooling and default themes; running components pick up new theme on next apply.
- **Rollback mechanism:** Previous theme packages retained; `sigmaos-theme --switch <previous>` reverts active theme; user-installed themes are user data and never auto-removed.
- **Expected downtime on update:** None — theme apply is live; components re-read theme config.
- **Pre-update state checks:** None required.
- **Post-update validation:** `sigmaos-theme --validate-active` checks the active theme applies cleanly.
- **Atomic update guarantee:** Partial — theme config writes atomic; GTK config apply is transactional via gsettings.

## 12. Security Boundary

- **Privilege level required:** Runs as the logged-in user for user themes; system theme install requires polkit auth.
- **Trust boundary:** Trusts SigmaOS-provided themes; third-party themes validated against schema before apply; SVG assets rendered in a sandboxed renderer.
- **Attack surface:** Theme package parsing, SVG rendering, filesystem (theme install), D-Bus IPC
- **Hardening measures:** Theme packages validated against schema; SVG renderer sandboxed; system theme install gated by polkit; runs with `NoNewPrivileges`.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-desktop | 0.2+ | tested |
| SigmaOS-panel | 0.2+ | tested |
| SigmaOS-launcher | 0.2+ | tested |
| SigmaOS-control-center | 0.2+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| gtk4 | 4.12+ | tested |
| librsvg | 2.57+ | tested |
| glib | 2.78+ | tested |

- **Known incompatibilities:** None.
- **Forward-compatibility pledge:** Theme schema and D-Bus API stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-theme-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/theme-schema.md`, `docs/theme-contract.md`
- **Man page or in-app help status:** present — `sigmaos-theme(8)`
- **Documentation parity policy:** Theme schema or D-Bus changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:schema` | Theme schema issues |
| `component:dark-light` | Dark/light mode switching issues |
| `component:gtk` | GTK theme application issues |
| `component:wayland` | Wayland compositor theming issues |
| `component:marketplace` | Marketplace/metadata issues |
| `component:preview` | Theme preview issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-theme/milestones`
- **Issue triage cadence:** Every Tuesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for theme schema or D-Bus incompatibility.
- **Version pinning in dependents:** SigmaOS-desktop pins `sigmaos-theme >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depends-on-core (transitively via SigmaOS-desktop) — theme runs within the desktop session; does not directly depend on core APIs except via the desktop.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Theme1`); theme config files read by desktop components.
- **Core API version it targets:** `SigmaOS-core >= 0.4` (transitive)
- **Integration tested in CI:** yes — theme apply tested within desktop session integration.
- **Change coordination policy:** Theme contract changes defined by SigmaOS-desktop require a tracking issue here.
- **Core team review required for cross-boundary changes:** no (mediated by SigmaOS-desktop)

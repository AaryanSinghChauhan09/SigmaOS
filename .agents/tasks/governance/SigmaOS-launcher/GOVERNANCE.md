# Governance: SigmaOS-launcher

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-launcher owns the application launcher component of the SigmaOS shell. It defines the launcher contract (how it is invoked, what it returns), implements fuzzy search over the `.desktop` application catalog, indexes installed applications, and delivers a keyboard-first user experience for discovering and launching apps. It is invoked from the panel and via a global keybinding.
- **Runtime layer:** desktop-shell
- **Scope boundary with adjacent repos:** SigmaOS-desktop owns the session and the global keybinding that invokes this launcher; SigmaOS-panel provides the trigger button; SigmaOS-package-manager notifies this launcher when apps are installed/removed so its index updates; SigmaOS-theme provides the visual styling.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-launcher` CLI | CLI | stable |
| `org.sigmaos.Launcher1` D-Bus service | D-Bus | stable |
| Launcher invocation contract (Wayland protocol) | Interface | stable |
| `.desktop` catalog index format | File format | unstable |
| Search ranking config | Config file | unstable |
| Internal indexer | internal | internal-only |

- **Breaking-change policy:** D-Bus API and invocation contract stable within a MAJOR version; index format is internal and may change between versions (rebuilt on demand).
- **API documentation location:** `docs/api/`, `docs/launcher-contract.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-desktop | 0.2 | session and keybinding | No |
| gtk4 | 4.12 | launcher UI toolkit | No |
| dbus | 1.14 | IPC with panel and package manager | No |
| Wayland | 1.22 | layer-shell for launcher popup | No |
| fuzzy matching library (e.g., fzf-style) | vendored | fuzzy search ranking | No |

- **Optional/soft dependencies:** `appstream` for richer app metadata; `desktop-file-utils` for `.desktop` validation.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** routine review quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-launcher-team
- **Co-maintainers:** @ux-lead, @desktop-lead
- **Review quorum:** 2 maintainer approvals required.
- **Escalation path:** Launcher regressions are `priority:normal`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 55% — fuzzy search ranking and `.desktop` parsing unit-tested.
- **Integration tests:** Present — launcher popup render test headless; index build from fixture `.desktop` files.
- **CI pipeline file:** `.github/workflows/launcher-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `fuzzy-search-test`, `index-build-test`, `lint`
- **Known flaky tests:** none
- **CI build badge:** `![launcher CI](https://github.com/SigmaOS/SigmaOS-launcher/actions/workflows/launcher-ci.yml/badge.svg)`

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
- **Backend variants:** Linux only; requires Wayland.

## 7. Release Maturity

- **Current maturity label:** `alpha`
- **Last label change date:** 2025-01-01
- **Planned next label:** `alpha → beta` at v0.6
- **Criteria for promotion:** Fuzzy search ranking validated by user testing, `.desktop` catalog covers freedesktop spec, keyboard navigation complete, theme integration validated.

## 8. What This Repo Does NOT Do

- Does not manage the desktop session → handled by SigmaOS-desktop
- Does not provide the panel → handled by SigmaOS-panel
- Does not install or remove apps → handled by SigmaOS-package-manager (launcher only indexes)
- Does not provide app settings → handled by SigmaOS-control-center
- Does not manage themes → handled by SigmaOS-theme

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/launcher.toml`, `~/.config/sigmaos/launcher.toml`
- **Schema file location:** `schemas/launcher.toml.schema.json`
- **Schema format:** TOML
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_LAUNCHER_KEYBIND` | `Super+Space` | Global keybinding to invoke launcher |
| `SIGMAOS_LAUNCHER_MAX_RESULTS` | `8` | Max search results shown |
| `SIGMAOS_LAUNCHER_HISTORY` | `1` | Enable launch history ranking |
| `XDG_DATA_DIRS` | system default | Where `.desktop` files are searched |

- **Breaking config change policy:** Config keys additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Launcher fails to open | Wayland layer-shell unavailable | Keybind does nothing | Verify compositor; restart session |
| Search returns no results | Stale or empty index | Launcher shows empty list | Rebuild index: `sigmaos-launcher --reindex` |
| App fails to launch | Broken `.desktop` Exec path | App does not start | Validate `.desktop` with `desktop-file-validate`; fix path |
| Index build failure | Malformed `.desktop` file | Reindex aborts | Bad file skipped and logged; reindex continues |
| Launcher popup invisible | Theme/render issue | Keybind opens but nothing visible | Switch to default theme via SigmaOS-theme |

- **Fallback behavior:** If the index is missing, the launcher rebuilds it on first invocation; malformed `.desktop` files are skipped without blocking the index build.
- **Operator escalation path:** Collect `~/.local/share/sigmaos/launcher.log`; file issue.
- **Crash dump artifacts:** Launcher log at `~/.local/share/sigmaos/launcher.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager; launcher process restart picks up new version; index rebuilt automatically.
- **Rollback mechanism:** Previous package retained via SigmaOS-package-manager.
- **Expected downtime on update:** None — launcher is on-demand; next invocation uses new version.
- **Pre-update state checks:** None required.
- **Post-update validation:** Next launch verifies index loads.
- **Atomic update guarantee:** Partial — binary swap atomic; index rebuilt on demand.

## 12. Security Boundary

- **Privilege level required:** Runs as the logged-in user; no special privileges.
- **Trust boundary:** Trusts `.desktop` files from system and user data dirs; validates Exec paths; does not auto-run anything without user confirmation.
- **Attack surface:** `.desktop` file parsing, Wayland socket, D-Bus IPC
- **Hardening measures:** `.desktop` files validated against spec; Exec paths resolved safely (no shell injection); runs with `NoNewPrivileges`.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-desktop | 0.2+ | tested |
| SigmaOS-panel | 0.2+ | tested |
| SigmaOS-package-manager | 0.3+ | tested (index update on install) |
| SigmaOS-theme | 0.2+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| gtk4 | 4.12+ | tested |
| Wayland | 1.22+ | tested |

- **Known incompatibilities:** None.
- **Forward-compatibility pledge:** D-Bus API and invocation contract stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-launcher-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/launcher-contract.md`
- **Man page or in-app help status:** present — `sigmaos-launcher(8)`
- **Documentation parity policy:** D-Bus or invocation contract changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:search` | Fuzzy search ranking issues |
| `component:index` | `.desktop` catalog index issues |
| `component:keyboard-nav` | Keyboard navigation issues |
| `component:theme` | Launcher theme consumption |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-launcher/milestones`
- **Issue triage cadence:** Every Tuesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for D-Bus API or invocation contract incompatibility.
- **Version pinning in dependents:** SigmaOS-desktop pins `sigmaos-launcher >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depends-on-core (transitively via SigmaOS-desktop) — launcher runs within the desktop session.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Launcher1`); Wayland layer-shell for the popup.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — launcher startup tested within desktop session integration.
- **Change coordination policy:** Index update triggers from SigmaOS-package-manager require coordinated D-Bus signal agreement.
- **Core team review required for cross-boundary changes:** yes

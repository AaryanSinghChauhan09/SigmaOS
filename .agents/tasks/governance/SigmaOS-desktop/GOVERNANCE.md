# Governance: SigmaOS-desktop

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-desktop owns the desktop session lifecycle and the integration of the Wayland compositor into SigmaOS. It manages the curated default desktop configuration, defines the theme contract that SigmaOS-theme implements, orchestrates session startup ordering for the compositor, panel, launcher, and control center, and defines the desktop profile that distinguishes a workstation from a server install.
- **Runtime layer:** desktop-shell
- **Scope boundary with adjacent repos:** SigmaOS-panel provides the status bar; SigmaOS-launcher provides the app launcher; SigmaOS-control-center provides settings UI; SigmaOS-theme provides visual theming; SigmaOS-core provides the session lifecycle primitives this repo builds on.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-desktop-session` CLI | CLI | stable |
| `org.sigmaos.Desktop1` D-Bus service | D-Bus | stable |
| Desktop session config `/etc/sigmaos/desktop.toml` | Config file | stable |
| Session startup hook directory | Hook API | unstable |
| Wayland compositor config contract | Config file | unstable |
| Theme contract interface | Interface | stable |

- **Breaking-change policy:** D-Bus API and theme contract stable within a MAJOR version; session config keys additive.
- **API documentation location:** `docs/api/`, `docs/theme-contract.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-core | 0.4 | session lifecycle primitives | No |
| SigmaOS-panel | 0.2 | status bar component | No |
| SigmaOS-launcher | 0.2 | app launcher component | No |
| SigmaOS-control-center | 0.2 | settings UI component | No |
| SigmaOS-theme | 0.2 | theme contract | No |
| Wayland compositor (sway/Hyprland) | sway 1.9 | Wayland compositor runtime | No |
| wlroots | 0.17 | compositor library | No |
| polkit | 122 | privilege for desktop actions | No |

- **Optional/soft dependencies:** `xdg-desktop-portal` for sandboxed app integration; `pipewire` for audio in session; `xwayland` for X11 app compatibility.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** compositor and wlroots updated within 1 week of security releases.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-desktop-team
- **Co-maintainers:** @ux-lead, @compositor-lead
- **Review quorum:** 2 maintainer approvals required; theme contract changes require @ux-lead approval.
- **Escalation path:** Session-startup regressions are `priority:high`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 45% — session state machine and config parsing unit-tested.
- **Integration tests:** Present — session startup test runs the compositor headless in CI and verifies component startup ordering.
- **CI pipeline file:** `.github/workflows/desktop-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `headless-session-test`, `config-validation`, `lint`
- **Known flaky tests:** `headless-session-test` flaky on GPU-less CI runners (issue #9).
- **CI build badge:** `![desktop CI](https://github.com/SigmaOS/SigmaOS-desktop/actions/workflows/desktop-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** desktop, laptop — supported; server/VM use headless profile (no desktop).
- **Backend variants:** Linux only; requires DRM/KMS for compositor.

## 7. Release Maturity

- **Current maturity label:** `alpha`
- **Last label change date:** 2025-01-01
- **Planned next label:** `alpha → beta` at v0.6
- **Criteria for promotion:** Session startup deterministic, theme contract frozen, compositor integration stable on 3 reference GPUs, panel/launcher/control-center all integrate cleanly.

## 8. What This Repo Does NOT Do

- Does not provide the status bar / panel → handled by SigmaOS-panel
- Does not provide the app launcher → handled by SigmaOS-launcher
- Does not provide settings UI → handled by SigmaOS-control-center
- Does not provide visual themes → handled by SigmaOS-theme
- Does not manage login / greeter → handled by SigmaOS-display-manager
- Does not provide window manager policy beyond compositor config → compositor-native

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/desktop.toml`, `~/.config/sigmaos/desktop.toml` (user override)
- **Schema file location:** `schemas/desktop.toml.schema.json`
- **Schema format:** TOML
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_DESKTOP_PROFILE` | `desktop` | `desktop` / `laptop` / `headless` |
| `SIGMAOS_DESKTOP_COMPOSITOR` | `sway` | Compositor backend |
| `SIGMAOS_WAYLAND_DISPLAY` | `wayland-0` | Wayland display socket |
| `XDG_CURRENT_DESKTOP` | `SigmaOS` | Freedesktop desktop identifier |

- **Breaking config change policy:** Config keys additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Compositor fails to start | GPU driver missing or misconfig | Black screen, session aborts | Switch to TTY; check `journalctl`; verify SigmaOS-drivers |
| Panel fails to start | IPC mismatch with compositor | No status bar | Restart panel: `sigmaos-desktop-session --restart-panel` |
| Theme fails to load | Broken theme package | Default/fallback theme used | Automatic fallback; reinstall theme via SigmaOS-theme |
| Session lockup | Compositor hang | Desktop unresponsive | Switch to TTY; kill compositor; restart session |
| User config parse error | Malformed `~/.config/sigmaos/desktop.toml` | Falls back to system config | Validate user config; fix syntax |

- **Fallback behavior:** If the primary compositor fails, falls back to a minimal compositor or TTY login; if a component fails, the session continues with that component disabled and logged.
- **Operator escalation path:** Collect `journalctl --user` and `~/.local/share/sigmaos/desktop.log`; file issue.
- **Crash dump artifacts:** Session log at `~/.local/share/sigmaos/desktop.log`; compositor log at `~/.local/share/sigmaos/compositor.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager updates components; session restart picks up new versions; user can restart session without reboot.
- **Rollback mechanism:** Previous component packages retained; rollback via SigmaOS-package-manager; user config is not touched.
- **Expected downtime on update:** None if user restarts session; components hot-swappable where possible.
- **Pre-update state checks:** Session components in stable state.
- **Post-update validation:** Session restart verifies all components start.
- **Atomic update guarantee:** Partial — component binaries swapped atomically; session state not transactional.

## 12. Security Boundary

- **Privilege level required:** Runs as the logged-in user (not root); privileged desktop actions gated by polkit.
- **Trust boundary:** Trusts the compositor and SigmaOS components; does not trust arbitrary Wayland clients (sandboxed via xdg-desktop-portal).
- **Attack surface:** Wayland socket, D-Bus IPC, config file parsing, compositor IPC
- **Hardening measures:** Wayland clients run with reduced privileges; portal gates filesystem/clipboard access; compositor runs with `NoNewPrivileges`; screen locking enforces re-auth.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-core | 0.4+ | tested |
| SigmaOS-panel | 0.2+ | tested |
| SigmaOS-launcher | 0.2+ | tested |
| SigmaOS-control-center | 0.2+ | tested |
| SigmaOS-theme | 0.2+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| sway | 1.9+ | tested |
| wlroots | 0.17+ | tested |
| xdg-desktop-portal | 1.18+ | tested |

- **Known incompatibilities:** sway < 1.8 lacks features for the theme contract.
- **Forward-compatibility pledge:** Theme contract and D-Bus API stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-desktop-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/theme-contract.md`
- **Man page or in-app help status:** present — `sigmaos-desktop-session(8)`
- **Documentation parity policy:** Theme contract or D-Bus changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:session` | Session lifecycle issues |
| `component:compositor` | Compositor integration issues |
| `component:theme-contract` | Theme contract issues |
| `component:defaults` | Curated defaults issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-desktop/milestones`
- **Issue triage cadence:** Every Tuesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for theme contract or D-Bus incompatibility.
- **Version pinning in dependents:** Panel, launcher, control-center, theme pin `sigmaos-desktop >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depends-on-core — desktop session builds on core's session lifecycle primitives and privilege model.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Desktop1` queries `org.sigmaos.Core1`); Wayland socket for compositor.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — session startup test runs against core's session service.
- **Change coordination policy:** Session lifecycle API changes require a tracking issue in this repo.
- **Core team review required for cross-boundary changes:** yes

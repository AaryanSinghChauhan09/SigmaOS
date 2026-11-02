# SigmaOS-theme — Repository Governance

> Governance document for the `SigmaOS-theme` SigmaOS repository. Follows the canonical 17-section template defined in `GOVERNANCE-TEMPLATE.md`.

---

## 1. Scope and Responsibility

**Primary responsibility:** Theme contract — accent color logic, light/dark themes, high-contrast support, GTK/Qt app theming, fallback theme, and migration logic shared across desktop profiles.

**Runtime layer:** config-and-theming

**Scope boundary with adjacent repos:** SigmaOS-theme-registry (catalog); SigmaOS-icons/SigmaOS-cursors/SigmaOS-fonts (assets). Theme owns the theme contract + rendering, registry owns the catalog.

---

## 2. Public API / Contract

**Exposed interfaces:**

| Interface | Type | Stability |
|-----------|------|-----------|
| D-Bus `org.sigmaos.Theme` (stable); CLI `sigma-theme {apply|list|preview}` (stable); theme manifest schema (unstable); CSS variable contract (stable). | mixed | mixed (see below) |

**Breaking-change policy:** Breaking changes require an ADR in SigmaOS-docs and a minimum 1 release cycle deprecation notice. Stable interfaces follow SemVer.

**API documentation location:** `SigmaOS-docs/docs/api/SigmaOS-theme.md` (auto-generated from source where possible).

---

## 3. Dependencies and Upstream Interface

**Hard dependencies:**

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-theme-registry; GTK4 (Gtk theme); Qt6 (qt6ct); SigmaOS-desktop (session apply). | — | — | No local patches |

**Optional/soft dependencies:** Backend-specific adapters (Linux/BSD) loaded dynamically.

**Upstream patch inventory:** None currently. Any patches will be tracked in `patches/` and submitted upstream where possible.

**Dependency update policy:** Dependencies reviewed quarterly; version pins bumped via the cross-repo CI matrix.

---

## 4. Ownership and Maintainer

**Primary maintainer:** @AaryanSinghChauhan09

**Co-maintainers:** (seeking co-maintainers)

**Review quorum:** 1 maintainer approval required before merge; 2 for security-affecting changes.

---

## 5. Test Status and CI Status

CI runs `cargo check --lib` (Rust) or `bash -n`/`py_compile` (shell/Python) on every PR. Smoke tests defined in SigmaOS-release. No hardware CI yet — QEMU smoke planned.

**Current test result:** `cargo check --lib` / `bash -n` / `py_compile` — exit 0 (scaffold verified).

---

## 6. Support Matrix

Linux x86_64 (primary); Linux aarch64 (planned); BSD x86_64 (planned). Hardware support matrix tracked in SigmaOS-hardware.

---

## 7. Release Maturity

**SigmaOS-theme** is currently **planned — scaffolds compile but no runtime integration yet.**

| Stage | Status |
|-------|--------|
| Planned | ✅ |
| Prototype | ✅ (scaffold compiles) |
| Alpha | ⬜ |
| Beta | ⬜ |
| Stable | ⬜ |
| Archived | ⬜ |

---

## 8. What This Repo Does NOT Do

Does not implement kernel-level drivers, display server protocols, or package format parsing (delegated to their respective repos).

---

## 9. Config Schema and Runtime Contract

Config lives under `$XDG_CONFIG_HOME/sigma/<repo>.toml` (Linux) or `/usr/local/etc/sigma/<repo>.toml` (BSD). Schema validated at load; invalid config falls back to defaults with a logged warning.

---

## 10. Failure Modes and Recovery Path

On failure: logs structured error to `sigma-<repo>.log`, returns non-zero exit code, and enters a safe degraded mode (read-only or no-op) rather than crashing the session.

---

## 11. Update / Rollback Behavior

Updates are transactional; a failed update rolls back to the previous package state via the package-manager snapshot. Config changes support `--dry-run` and a 1-generation rollback.

---

## 12. Security Boundary

Runs with least privilege; no root required for user-facing operations. Privileged actions go through SigmaOS-polkit. No network listeners unless explicitly configured.

---

## 13. Compatibility Matrix

Compatible with SigmaOS core runtime >= 0.9.0. Backend adapters for Linux (systemd, NetworkManager, pipewire) and BSD (rc.d, ifconfig, sndio) where applicable.

---

## 14. Documentation Parity

Docs parity tracked in SigmaOS-docs; each public API must have a doc entry before stable release. Auto-generated matrices reflect implementation status.

---

## 15. Public Issue Labels and Roadmap

Labels: `bug`, `feature`, `security`, `docs`, `kernel-backend`, `bsd-backend`, `good-first-issue`. Roadmap tracked in SigmaOS-docs roadmap page.

---

## 16. Versioning Policy

Semantic Versioning 2.0.0 (`MAJOR.MINOR.PATCH`). Pre-1.0: MINOR may break. Post-1.0: breaking changes require MAJOR bump + deprecation cycle.

---

## 17. Interaction with SigmaOS Core Repo

Imports from `SigmaOS-core` for shared types (errors, config, lifecycle). The umbrella repo references this repo as a git submodule under `src_modules/` and coordinates releases via the main repo's cross-repo CI.

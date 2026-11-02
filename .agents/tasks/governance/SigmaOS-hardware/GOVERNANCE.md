# Governance: SigmaOS-hardware

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-hardware owns the hardware support matrix, hardware detection at install and runtime, firmware packaging and delivery, power management policy and integration with upower, and hardware compatibility reporting. It is the source of truth for which hardware SigmaOS officially supports and the firmware that hardware requires.
- **Runtime layer:** core-system (hardware support services) + tooling (support matrix curation)
- **Scope boundary with adjacent repos:** SigmaOS-drivers consumes the support matrix for driver selection; SigmaOS-kernel provides the in-tree drivers this matrix references; SigmaOS-installer queries the matrix during install; SigmaOS-update delivers firmware via this repo's packages.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-hardware` CLI | CLI | stable |
| `org.sigmaos.Hardware1` D-Bus service | D-Bus | stable |
| `/etc/sigmaos/hardware.toml` | Config file | stable |
| Hardware support matrix schema (`matrix.yaml`) | Config file | stable |
| Firmware package manifest format | File format | unstable |
| Power management policy schema | Config file | unstable |
| Internal detection engine | internal | internal-only |

- **Breaking-change policy:** Support matrix schema stable within a MAJOR version; D-Bus API stable; firmware manifest format may change with auto-rebuild.
- **API documentation location:** `docs/api/`, `docs/support-matrix.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-kernel | 6.6.0 | in-tree drivers referenced by matrix | No |
| SigmaOS-drivers | 0.2 | driver selection from matrix | No |
| SigmaOS-update | 0.3 | firmware delivery | No |
| fwupd | 1.9 | firmware update daemon | No |
| upower | 1.90 | power management | No |
| linux-firmware | latest | firmware blobs | No |
| dbus | 1.14 | IPC bus | No |
| systemd | 254 | service management | No |

- **Optional/soft dependencies:** `tlp` for advanced laptop power tuning; `thermald` for Intel thermal management.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** fwupd and upower updated within 1 week of security releases; linux-firmware tracked upstream continuously.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-hardware-team
- **Co-maintainers:** @kernel-lead, @drivers-lead
- **Review quorum:** 2 maintainer approvals required; support matrix additions require @drivers-lead confirmation of driver coverage.
- **Escalation path:** Hardware regressions affecting boot are `priority:critical`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 45% — matrix parsing, detection logic, and config parsing unit-tested.
- **Integration tests:** Present — hardware detection tested on QEMU with fixture hardware IDs; power management profile applied-and-verified.
- **CI pipeline file:** `.github/workflows/hardware-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `matrix-validation-test`, `detection-test`, `lint`
- **Known flaky tests:** none
- **CI build badge:** `![hardware CI](https://github.com/SigmaOS/SigmaOS-hardware/actions/workflows/hardware-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** desktop, laptop, server, VM — all supported; embedded is best-effort.
- **Backend variants:** Linux only; tested in CI with QEMU virtual hardware.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** Support matrix validated against 10 reference devices, firmware delivery end-to-end with SigmaOS-update, power management profiles tested on laptop/desktop/server, detection reliable across hotplug.

## 8. What This Repo Does NOT Do

- Does not build kernel or drivers → handled by SigmaOS-kernel and SigmaOS-drivers
- Does not perform driver selection (provides the matrix) → handled by SigmaOS-drivers
- Does not deliver package updates → handled by SigmaOS-update (this provides firmware packages)
- Does not manage thermal/kernel-level power → handled by SigmaOS-kernel (this provides policy)
- Does not provide hardware diagnostics UI → handled by SigmaOS-control-center (hardware panel)

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/hardware.toml`, `/etc/sigmaos/hardware/matrix.yaml`, `/etc/sigmaos/power/`
- **Schema file location:** `schemas/hardware.toml.schema.json`, `schemas/matrix.schema.yaml`
- **Schema format:** TOML (config), YAML (matrix)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_HW_PROFILE` | `desktop` | `desktop` / `laptop` / `server` / `vm` |
| `SIGMAOS_POWER_PROFILE` | `balanced` | `balanced` / `performance` / `powersave` |
| `SIGMAOS_FWUPDATE_AUTO` | `0` | Auto-apply firmware updates |

- **Breaking config change policy:** Config keys additive within a MAJOR version; matrix schema additive.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Detection misidentifies hardware | Stale or wrong matrix entry | Wrong profile applied | Update matrix; `sigmaos-hardware --redetect` |
| Firmware update fails | fwupd issue or bad firmware | Device may be in inconsistent state | fwupd has its own rollback; check `fwupdmgr history` |
| Power profile apply fails | upower unavailable | Profile not active | Restart upower; reapply profile |
| Unknown hardware | No matrix entry | Defaults used; may lack features | File issue with hardware ID; matrix updated |
| Battery not detected | ACPI/driver issue | Power applet shows no battery | Verify kernel ACPI driver; check SigmaOS-drivers |
| Overheating | Thermal driver missing | System throttles or shuts down | Install thermal driver; verify thermald |

- **Fallback behavior:** Unknown hardware falls back to a generic profile and logs a reportable event; failed firmware updates leave the previous firmware intact (fwupd-managed); failed power profile falls back to balanced.
- **Operator escalation path:** Collect `sigmaos-hardware --diagnostics` and `/var/log/sigmaos/hardware.log`; file issue with hardware ID.
- **Crash dump artifacts:** Hardware log at `/var/log/sigmaos/hardware.log`; fwupd history via `fwupdmgr`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager updates the matrix and tooling; firmware packages delivered via SigmaOS-update with fwupd.
- **Rollback mechanism:** Previous matrix/tooling package retained; firmware rollback managed by fwupd; power profile is user config and preserved.
- **Expected downtime on update:** None for matrix; firmware updates may require reboot.
- **Pre-update state checks:** No active firmware update in progress.
- **Post-update validation:** `sigmaos-hardware --self-check` verifies matrix integrity and detection engine.
- **Atomic update guarantee:** Partial — matrix files written atomically; firmware updates are fwupd-transactional.

## 12. Security Boundary

- **Privilege level required:** Runs as a dedicated `sigmaos-hardware` system user; firmware updates and power changes gated by polkit.
- **Trust boundary:** Trusts only firmware signed by the device vendor (verified by fwupd); trusts SigmaOS-curated matrix; does not trust unverified firmware.
- **Attack surface:** fwupd integration, power management, matrix parsing, D-Bus IPC
- **Hardening measures:** Firmware signature verification via fwupd; matrix validated against schema; runs with `NoNewPrivileges`; D-Bus policy restricts callers.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Internal review 2024-12.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-kernel | 6.6.x | tested |
| SigmaOS-drivers | 0.2+ | tested |
| SigmaOS-update | 0.3+ | tested (firmware delivery) |
| SigmaOS-control-center | 0.2+ | tested (hardware panel) |

| Package | Version Range | Status |
|---------|---------------|--------|
| fwupd | 1.9+ | tested |
| upower | 1.90+ | tested |
| linux-firmware | latest | tested |

- **Known incompatibilities:** None.
- **Forward-compatibility pledge:** Matrix schema and D-Bus API stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-hardware-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/support-matrix.md`
- **Man page or in-app help status:** present — `sigmaos-hardware(8)`
- **Documentation parity policy:** Matrix schema or D-Bus changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:matrix` | Support matrix issues |
| `component:firmware` | Firmware packaging and delivery issues |
| `component:power` | Power management issues |
| `component:detection` | Hardware detection issues |
| `hardware:unknown` | Reports for unsupported/unknown hardware |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-hardware/milestones`
- **Issue triage cadence:** Every Monday and Thursday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for matrix schema or D-Bus incompatibility.
- **Version pinning in dependents:** SigmaOS-drivers pins `sigmaos-hardware >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depended-on-by-core (transitively) — core's power-aware behavior and startup ordering depend on hardware profile; firmware delivery feeds SigmaOS-update.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Hardware1`); matrix files consumed by SigmaOS-drivers; firmware packages via SigmaOS-update.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — hardware detection tested within boot integration.
- **Change coordination policy:** Matrix changes that affect driver selection require a tracking issue in SigmaOS-drivers; power profile changes require coordination with SigmaOS-core startup ordering.
- **Core team review required for cross-boundary changes:** yes

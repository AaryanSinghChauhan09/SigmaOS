# Governance: SigmaOS-drivers

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-drivers owns the out-of-tree and packaged driver model for SigmaOS: driver detection at boot and hotplug, the hardware abstraction layer that presents a consistent interface to higher components, hotplug event handling via udev, and the packaging of GPU, audio, and input drivers that are not built into the SigmaOS-kernel. It coordinates with SigmaOS-kernel for module ABI compatibility.
- **Runtime layer:** core-system (driver modules loaded at boot and runtime)
- **Scope boundary with adjacent repos:** SigmaOS-kernel provides the in-tree drivers and the module ABI this repo builds against; SigmaOS-hardware owns the hardware support matrix and firmware; SigmaOS-desktop consumes the GPU driver for the compositor; SigmaOS-network consumes network drivers.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-drivers` CLI | CLI | stable |
| `org.sigmaos.Drivers1` D-Bus service | D-Bus | stable |
| `/etc/sigmaos/drivers.toml` | Config file | stable |
| Hardware abstraction layer (HAL) interface | Interface | stable |
| Driver package manifest format | File format | unstable |
| udev hotplug hook API | Interface | unstable |
| Internal detection engine | internal | internal-only |

- **Breaking-change policy:** D-Bus API and HAL interface stable within a MAJOR version; driver manifest format may change between versions with auto-rebuild on upgrade.
- **API documentation location:** `docs/api/`, `docs/hal.md`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-kernel | 6.6.0 | module ABI for out-of-tree drivers | No |
| SigmaOS-hardware | 0.2 | hardware support matrix and firmware | No |
| udev (systemd-udev) | 254 | hotplug event handling | No |
| kmod | 31 | module loading | No |
| dkms | 3.0 | out-of-tree module rebuild on kernel update | No |
| mesa | 24.0 | userspace GPU drivers (open) | No |
| dbus | 1.14 | IPC bus | No |

- **Optional/soft dependencies:** proprietary GPU drivers (NVIDIA) packaged separately; `pipewire`/`alsa` for audio HAL; `libinput` for input HAL.
- **Upstream patch inventory:** DKMS config files for out-of-tree modules; no source patches.
- **Dependency update policy:** kernel ABI-sensitive deps rebuilt on every kernel update; mesa updated within 1 week of security releases.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-drivers-team
- **Co-maintainers:** @kernel-lead, @hardware-lead
- **Review quorum:** 2 maintainer approvals required; HAL interface changes require @kernel-lead approval.
- **Escalation path:** Driver regressions that prevent boot or break GPU are `priority:critical`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 40% — detection logic, HAL interface, and config parsing unit-tested.
- **Integration tests:** Present — driver detection and load tested on QEMU with fixture devices; DKMS rebuild tested against kernel update.
- **CI pipeline file:** `.github/workflows/drivers-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `detection-test`, `dkms-rebuild-test`, `hal-interface-test`, `lint`
- **Known flaky tests:** `dkms-rebuild-test` occasionally times out (issue #8).
- **CI build badge:** `![drivers CI](https://github.com/SigmaOS/SigmaOS-drivers/actions/workflows/drivers-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |

- **Hardware classes:** desktop, laptop, server, VM — all supported; GPU drivers vary by class.
- **Backend variants:** Linux only; tested in CI with QEMU and fixture devices.

## 7. Release Maturity

- **Current maturity label:** `alpha`
- **Last label change date:** 2025-01-01
- **Planned next label:** `alpha → beta` at v0.6
- **Criteria for promotion:** HAL interface frozen, GPU/audio/input driver packages validated on 3 reference devices each, DKMS rebuild reliable across kernel updates, hotplug detection robust.

## 8. What This Repo Does NOT Do

- Does not build the kernel or in-tree drivers → handled by SigmaOS-kernel
- Does not own the hardware support matrix → handled by SigmaOS-hardware
- Does not own firmware blobs → handled by SigmaOS-hardware
- Does not manage power policy → handled by SigmaOS-hardware
- Does not provide the compositor → handled by SigmaOS-desktop (consumes GPU driver)

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/drivers.toml`, `/etc/sigmaos/drivers.d/` (per-driver config)
- **Schema file location:** `schemas/drivers.toml.schema.json`, `schemas/driver-manifest.schema.json`
- **Schema format:** TOML (config + manifests)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_DRIVERS_AUTODETECT` | `1` | Auto-detect and load drivers at boot |
| `SIGMAOS_DRIVERS_HOTPLUG` | `1` | Handle hotplug events |
| `SIGMAOS_DRIVERS_FREE_ONLY` | `0` | Restrict to free/open drivers only |

- **Breaking config change policy:** Config keys additive within a MAJOR version; driver manifest format rebuilt on upgrade.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Module load failure | ABI mismatch with kernel | Device unavailable; `modprobe` error | Rebuild via DKMS; install matching kernel-headers |
| GPU driver crash | Driver bug | Display freezes/compositor fails | Restart compositor; switch to fallback driver (llvmpipe) |
| Hotplug not detected | udev rule missing or wrong | New device not recognized | Reload udev rules: `sigmaos-drivers --reload-udev` |
| DKMS rebuild fails | Source incompatible with new kernel | Out-of-tree module unavailable after kernel update | Pin to previous kernel; rebuild module manually |
| Detection misidentifies hardware | Wrong DB match | Wrong driver loaded | Correct hardware DB; force driver in drivers.toml |
| Audio driver fails | ALSA/PipeWire mismatch | No sound | Restart PipeWire; verify audio HAL driver |

- **Fallback behavior:** If the primary GPU driver fails, falls back to llvmpipe (software rendering); if a driver module fails to load, the device is reported as unavailable and a fallback (if any) is tried.
- **Operator escalation path:** Collect `journalctl -k` (kernel), `sigmaos-drivers --diagnostics`, and `/var/log/sigmaos/drivers.log`; file issue.
- **Crash dump artifacts:** Driver log at `/var/log/sigmaos/drivers.log`; kernel module log in dmesg.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager updates driver packages; DKMS rebuilds out-of-tree modules against the active kernel; hot reload where supported.
- **Rollback mechanism:** Previous driver packages retained; `sigmaos-drivers --rollback <driver>` reverts; kernel rollback via SigmaOS-boot also reverts module ABI.
- **Expected downtime on update:** GPU driver update may require compositor restart; most drivers hot-reload.
- **Pre-update state checks:** Matching kernel-headers installed for DKMS; no active critical GPU operation.
- **Post-update validation:** `sigmaos-drivers --verify-all` checks all loaded modules match kernel ABI.
- **Atomic update guarantee:** Partial — module swap atomic per-module; DKMS rebuild is journaled.

## 12. Security Boundary

- **Privilege level required:** Module loading requires root; D-Bus queries unprivileged, load/unload gated by polkit.
- **Trust boundary:** Trusts only signed driver packages from SigmaOS-repo-manager; rejects unsigned modules when Secure Boot is active; does not trust hotplug-provided device descriptors without validation.
- **Attack surface:** Module loading interface, udev event parsing, D-Bus IPC, device descriptor parsing
- **Hardening measures:** Module signature verification; udev rules validated; runs with restricted capabilities; Secure Boot enforces signed modules.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-kernel | 6.6.x | tested (module ABI) |
| SigmaOS-hardware | 0.2+ | tested |
| SigmaOS-desktop | 0.2+ | tested (GPU driver) |
| SigmaOS-network | 0.2+ | tested (network drivers) |

| Package | Version Range | Status |
|---------|---------------|--------|
| mesa | 24.0+ | tested |
| dkms | 3.0+ | tested |
| libinput | 1.25+ | tested |

- **Known incompatibilities:** NVIDIA proprietary driver requires specific kernel version matching.
- **Forward-compatibility pledge:** HAL interface and D-Bus API stable for v1.x.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-drivers-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/hal.md`
- **Man page or in-app help status:** present — `sigmaos-drivers(8)`
- **Documentation parity policy:** HAL interface or D-Bus changes require doc updates in same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:gpu` | GPU driver issues |
| `component:audio` | Audio driver issues |
| `component:input` | Input driver issues |
| `component:dkms` | DKMS rebuild issues |
| `component:hotplug` | Hotplug detection issues |
| `component:hal` | Hardware abstraction layer issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-drivers/milestones`
- **Issue triage cadence:** Every Monday and Thursday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for HAL interface or D-Bus incompatibility.
- **Version pinning in dependents:** SigmaOS-desktop pins GPU driver package; module ABI pins to kernel MAJOR.MINOR.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depended-on-by-core (transitively) — core services depend on devices being available; the desktop and network layers depend on drivers this repo provides.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Drivers1`); HAL interface consumed by desktop and network; module loading via kmod.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — driver detection tested within boot integration.
- **Change coordination policy:** Module ABI changes require coordination with SigmaOS-kernel; HAL interface changes require tracking issues in SigmaOS-desktop and SigmaOS-network.
- **Core team review required for cross-boundary changes:** yes

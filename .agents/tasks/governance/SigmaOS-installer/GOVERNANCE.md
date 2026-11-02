# Governance: SigmaOS-installer

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-installer owns the ISO-based installation flow that takes a target machine from bare metal to a bootable SigmaOS system. It manages disk partitioning, filesystem creation, the user and admin account setup, locale and timezone selection, and the post-install hooks that configure the installed system before first boot.
- **Runtime layer:** tooling (runs from live ISO environment)
- **Scope boundary with adjacent repos:** SigmaOS-boot provides the bootloader configuration this installer writes to the target disk; SigmaOS-kernel and SigmaOS-initramfs provide the images copied to the target `/boot`; SigmaOS-package-manager is invoked to install the base system packages; SigmaOS-security provides the initial secure defaults the installer applies.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-installer` CLI (interactive) | CLI | stable |
| `sigmaos-installer --unattended <profile.toml>` | CLI | stable |
| Installer profile schema (`profiles/*.toml`) | Config file | stable |
| Post-install hook directory `/etc/sigmaos/installer/hooks/` | Hook API | unstable |
| D-Bus `org.sigmaos.Installer1` (during live session) | D-Bus | unstable |
| Partition layout JSON output (for automation) | CLI output | stable |

- **Breaking-change policy:** CLI flag removals require a 2-release deprecation with a warning; profile schema changes are additive within a MAJOR version.
- **API documentation location:** `docs/installer-api.md`, `man/sigmaos-installer.8`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-package-manager | 0.3 | installs base system on target | No |
| SigmaOS-boot | 0.2 | writes bootloader config to target | No |
| SigmaOS-kernel | 6.6.0 | image copied to target `/boot` | No |
| SigmaOS-initramfs | 0.3 | image copied to target `/boot` | No |
| partclone / parted | 3.6 / 3.6 | disk partitioning operations | No |
| cryptsetup | 2.6 | optional LUKS during install | No |
| systemd | 254 | runs in the live ISO environment | No |

- **Optional/soft dependencies:** `NetworkManager` for network install mode; `archinstall`-style python bindings (planned).
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** parted and cryptsetup updated within 1 week of security releases; routine review quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-installer-team
- **Co-maintainers:** @release-lead, @security-lead
- **Review quorum:** 2 maintainer approvals required; partitioning changes require @release-lead approval.
- **Escalation path:** Install-blocking bugs are `priority:critical`; escalate to @sigmaos-core-team.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 40% — partitioning logic and profile parsing unit-tested.
- **Integration tests:** Present — automated QEMU install runs the full installer against a virtual disk and boots the result.
- **CI pipeline file:** `.github/workflows/installer-ci.yml`
- **Required passing checks before merge:** `unit-tests`, `qemu-install-smoke`, `profile-validation`, `lint`
- **Known flaky tests:** `qemu-install-smoke` occasionally fails on disk timing (issue #7).
- **CI build badge:** `![installer CI](https://github.com/SigmaOS/SigmaOS-installer/actions/workflows/installer-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 (UEFI) | supported |
| x86_64 (BIOS) | supported |
| aarch64 (UEFI) | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Live ISO (Linux) | 6.6 | supported |
| Target filesystem | ext4 / btrfs / xfs | supported |

- **Hardware classes:** desktop, laptop, server, VM — all supported.
- **Backend variants:** Linux only; tested in CI with QEMU virtual disks.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** Unattended install reproducible on 5 reference disk layouts, LUKS install path validated, BIOS+UEFI both green, post-install hook framework stable.

## 8. What This Repo Does NOT Do

- Does not build the ISO image itself → handled by SigmaOS-release
- Does not manage the package repository contents → handled by SigmaOS-repo-manager
- Does not provide ongoing system updates post-install → handled by SigmaOS-update
- Does not manage firmware → handled by SigmaOS-hardware
- Does not define security policy (only applies initial defaults) → handled by SigmaOS-security

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/installer.toml` (live env), `profiles/*.toml` (install profiles)
- **Schema file location:** `schemas/installer-profile.schema.json`
- **Schema format:** TOML
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_INSTALL_MODE` | `interactive` | `interactive` / `unattended` / `network` |
| `SIGMAOS_INSTALL_TARGET` | unset | Target disk device path |
| `SIGMAOS_INSTALL_PROFILE` | `desktop` | Profile name to apply |

- **Breaking config change policy:** Profile schema keys are additive within a MAJOR version; removed keys produce a warning for one release before deletion.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Disk partitioning failure | Unpartitionable disk or wrong target | Install aborts before writes | Re-select target disk; run `sigmaos-installer --diagnose-disk` |
| User creation failure | Invalid username or locale | Post-install hook fails | Edit `/etc/passwd` from a chroot in live env |
| Package install failure | Network or repo issue in network mode | Base system incomplete | Verify network; rerun install with `--resume` |
| Bootloader write failure | EFI variable store full or /boot too small | System not bootable after install | Boot live USB; run `sigmaos-boot-entry --regenerate` in chroot |
| Unattended profile parse error | Malformed profile TOML | Installer refuses to start | Validate with `sigmaos-installer --validate-profile` |

- **Fallback behavior:** Installer writes a detailed log to the target disk and to the live environment; if install fails midway, the target disk's pre-existing state is restored from a partition backup the installer takes before modifying the disk.
- **Operator escalation path:** Collect `/var/log/sigmaos/installer.log` from the live env; file issue with `priority:critical`.
- **Crash dump artifacts:** Install log at `/var/log/sigmaos/installer.log`; partition backup at `/tmp/sigmaos-partition-backup/`.

## 11. Update / Rollback Behavior

- **Update mechanism:** The installer itself ships on each new ISO; there is no in-place update of the installer on an installed system.
- **Rollback mechanism:** Not applicable — installer is ephemeral; target disk rollback is via the partition backup taken before install.
- **Expected downtime on update:** N/A (runs from live ISO).
- **Pre-update state checks:** N/A.
- **Post-update validation:** `sigmaos-installer --self-check` validates the installer environment on ISO boot.
- **Atomic update guarantee:** Partial — partitioning is not atomic, but a full-disk backup is taken before destructive operations so the original state can be restored.

## 12. Security Boundary

- **Privilege level required:** root (runs as root in the live ISO environment)
- **Trust boundary:** Trusts the live ISO environment (signed ISO); trusts user-provided install profiles only after validation; does not trust network-installed packages without signature verification by SigmaOS-package-manager.
- **Attack surface:** Target disk I/O, network (in network install mode), user input parsing
- **Hardening measures:** Install profiles validated against schema before use; package signatures verified before install; default user account is unprivileged; SSH disabled by default on installed system.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-boot | 0.2+ | tested |
| SigmaOS-kernel | 6.6.x | tested |
| SigmaOS-package-manager | 0.3+ | tested |
| SigmaOS-security | 0.3+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| parted | 3.6+ | tested |
| cryptsetup | 2.6+ | tested |
| systemd | 254+ | tested |

- **Known incompatibilities:** parted < 3.5 lacks some GPT operations the installer relies on.
- **Forward-compatibility pledge:** Profile schema remains compatible for v1.x series.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-install-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/profile-schema.md`
- **Man page or in-app help status:** present — `sigmaos-installer(8)`
- **Documentation parity policy:** New profile keys or CLI flags require doc updates in the same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:partitioning` | Disk layout and partitioning issues |
| `component:unattended` | Unattended / profile-driven install issues |
| `component:post-install-hooks` | Post-install hook framework issues |
| `component:luks-install` | LUKS-encrypted install issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-installer/milestones`
- **Issue triage cadence:** Every Tuesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump when profile schema or CLI flags change incompatibly.
- **Version pinning in dependents:** SigmaOS-release pins the installer version per ISO release.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depends-on-core — the installer installs SigmaOS-core as part of the base system and applies core's first-boot configuration.
- **IPC / integration mechanism:** Package install via SigmaOS-package-manager; config file generation consumed by SigmaOS-core at first boot.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — QEMU install test boots into SigmaOS-core and runs a smoke check.
- **Change coordination policy:** Changes to first-boot config keys core reads require a tracking issue in both repos.
- **Core team review required for cross-boundary changes:** yes

# Governance: SigmaOS-boot

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-boot owns the bootloader configuration, boot entry management, boot profile selection, Secure Boot key enrollment, and the collection and structured export of boot-time diagnostic logs. It produces the GRUB/systemd-boot configuration consumed during system startup and manages the fallback/recovery boot entries.
- **Runtime layer:** kernel (pre-OS boot stage)
- **Scope boundary with adjacent repos:** SigmaOS-kernel produces the vmlinuz images that this repo references; SigmaOS-initramfs produces the initramfs images attached to boot entries; SigmaOS-installer calls this repo's scripts during OS installation to register initial boot entries.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-boot-entry` CLI | CLI | stable |
| `sigmaos-boot-log` CLI | CLI | stable |
| Boot entry config format (`/boot/sigmaos/entries/*.conf`) | Config file | stable |
| `boot_profile` key in `/etc/sigmaos/boot.toml` | Config file | stable |
| GRUB fragment output (`/boot/grub/sigmaos.cfg`) | Generated file | unstable |
| D-Bus `org.sigmaos.Boot1` service (post-boot metadata) | D-Bus | unstable |

- **Breaking-change policy:** Boot entry config format changes require a migration script; CLI flag removals require a 2-release deprecation window with a warning on invocation.
- **API documentation location:** `docs/boot-api.md`, man pages at `man/`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| GRUB2 | 2.12 | Primary bootloader on x86_64 | No |
| systemd-boot | 254 | EFI-only boot on modern hardware | No |
| efibootmgr | 18 | EFI variable management for boot order | No |
| SigmaOS-kernel | 6.6.0 | vmlinuz images referenced by boot entries | No |
| SigmaOS-initramfs | 0.3 | initramfs images referenced by boot entries | No |
| shim | 15.8 | Secure Boot chain of trust for GRUB | No |
| mokutil | 0.6 | MOK key enrollment for Secure Boot | No |

- **Optional/soft dependencies:** `os-prober` for multi-boot detection (disabled by default for security).
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** GRUB and shim updates are applied immediately when security fixes are available; routine updates quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-boot-team
- **Co-maintainers:** @kernel-lead, @installer-lead
- **Review quorum:** 2 maintainer approvals required before merge
- **Escalation path:** Boot regressions are `priority:critical` by default; escalate to @sigmaos-core-team if unresolved within 24 hours.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 35% — boot entry generation scripts have unit tests; GRUB config generation is harder to unit-test.
- **Integration tests:** Present — QEMU boot tests on x86_64 and aarch64 verify correct entry generation and successful boot.
- **CI pipeline file:** `.github/workflows/boot-ci.yml`
- **Required passing checks before merge:** `entry-generation-tests`, `qemu-boot-x86_64`, `qemu-boot-aarch64`, `secure-boot-shim-check`
- **Known flaky tests:** `qemu-boot-aarch64` occasionally times out on slow CI runners (issue #15).
- **CI build badge:** `![boot CI](https://github.com/SigmaOS/SigmaOS-boot/actions/workflows/boot-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 (UEFI) | supported |
| x86_64 (BIOS/Legacy) | supported |
| aarch64 (UEFI) | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |
| UEFI firmware | 2.5+ | supported |
| BIOS/CSM | any | supported |

- **Hardware classes:** desktop, laptop, server, VM — all supported; embedded is best-effort.
- **Backend variants:** Linux only; tested in CI with QEMU for both UEFI and BIOS modes.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** Secure Boot end-to-end validated on 3 reference devices, GRUB and systemd-boot configs pass on all supported arch/firmware combos, rollback boot entry always preserved after upgrade.

## 8. What This Repo Does NOT Do

- Does not build kernel images → handled by SigmaOS-kernel
- Does not build initramfs images → handled by SigmaOS-initramfs
- Does not manage partition layout or disk formatting → handled by SigmaOS-installer
- Does not manage firmware updates (UEFI capsule) → handled by SigmaOS-hardware
- Does not provide a graphical boot splash → handled by plymouth (external dependency)

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/boot.toml`, `/boot/sigmaos/entries/` (generated per-kernel boot entries)
- **Schema file location:** `schemas/boot.toml.schema.json`
- **Schema format:** TOML
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_BOOT_PROFILE` | `desktop` | Selects boot profile (desktop/server/recovery) |
| `SIGMAOS_BOOT_TIMEOUT` | `5` | Bootloader menu timeout in seconds |
| `SIGMAOS_SECURE_BOOT` | auto-detected | Override Secure Boot enforcement |

- **Breaking config change policy:** `/etc/sigmaos/boot.toml` key removals require a migration script shipped with the update package; old keys produce a deprecation warning for one release before removal.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| No boot entry found | Corrupt `/boot/sigmaos/entries/` | Bootloader drops to shell | Run `sigmaos-boot-entry --regenerate` from live USB |
| Secure Boot MOK not enrolled | New SigmaOS key not accepted | "Security Policy Violation" at boot | Run `mokutil --import sigmaos.der` and reboot to enroll |
| GRUB config syntax error | Bad merge or manual edit | Boot menu missing or corrupt | Boot from recovery entry; run `sigmaos-boot-entry --repair` |
| Boot loop after kernel update | Bad kernel + boot entry pair | System reboots 3× then enters recovery | Recovery entry boots previous kernel automatically after 3 failures |
| UEFI variable store full | Too many boot entries | `efibootmgr` fails to write | Run `sigmaos-boot-entry --prune-old` to remove stale entries |

- **Fallback behavior:** A permanent recovery boot entry is written at install time and is never removed by updates; it boots the last-known-good kernel with a minimal initramfs.
- **Operator escalation path:** Collect boot log via `sigmaos-boot-log --export` and file issue with `priority:critical`.
- **Crash dump artifacts:** Boot logs at `/var/log/sigmaos/boot.log`; EFI variable dump via `efibootmgr -v > /tmp/efi.txt`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager post-install hook calls `sigmaos-boot-entry --update`; new entry added, old entry retained.
- **Rollback mechanism:** Previous boot entry retained; `sigmaos-boot-entry --set-default previous` reverts the default; GRUB counter-based fallback automatically boots previous entry after 3 failed attempts.
- **Expected downtime on update:** Reboot required to switch active kernel; boot entry update itself takes < 5 seconds.
- **Pre-update state checks:** `/boot` partition must have ≥ 100 MB free; EFI variable store writable.
- **Post-update validation:** `sigmaos-boot-entry --verify` checks new entry integrity after write.
- **Atomic update guarantee:** Partial — entry files written atomically; GRUB config regeneration is not transactional but old config is backed up before overwrite.

## 12. Security Boundary

- **Privilege level required:** root (bootloader config files, EFI variables require root write access)
- **Trust boundary:** Trusts Secure Boot MOK keys enrolled by the user/admin; does not trust boot entries signed with unknown keys when Secure Boot is active.
- **Attack surface:** EFI variable store, `/boot` partition, GRUB config file
- **Hardening measures:** Secure Boot enforced when UEFI supports it; GRUB password protection available for interactive menu; MOK key management via `mokutil`; shim-signed GRUB binary.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** Follows upstream GRUB CVE advisories; patched packages released within 48 hours of upstream disclosure.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-kernel | 6.6.x | tested |
| SigmaOS-initramfs | 0.3+ | tested |
| SigmaOS-installer | 0.2+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| GRUB2 | 2.12+ | tested |
| systemd-boot | 254+ | tested |
| shim | 15.8 | tested |

- **Known incompatibilities:** GRUB < 2.06 lacks Secure Boot support for modern shim versions.
- **Forward-compatibility pledge:** Boot entry config format remains compatible for the entire v1.x series.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-boot-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`
- **Man page or in-app help status:** present — `sigmaos-boot-entry(8)`, `sigmaos-boot-log(8)`
- **Documentation parity policy:** Any change to boot entry format or new `boot.toml` key must include doc update in the same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:secure-boot` | Secure Boot and MOK-related issues |
| `component:grub` | GRUB config and entry issues |
| `component:systemd-boot` | systemd-boot specific issues |
| `component:recovery` | Boot recovery and fallback issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-boot/milestones`
- **Issue triage cadence:** Every Tuesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump when boot entry config format changes incompatibly.
- **Version pinning in dependents:** SigmaOS-installer pins `sigmaos-boot >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** peer — SigmaOS-boot runs before SigmaOS-core is active; after boot, it exposes a D-Bus metadata service that core may query.
- **IPC / integration mechanism:** D-Bus (`org.sigmaos.Boot1`) post-boot; package dependency for installer integration.
- **Core API version it targets:** `SigmaOS-core >= 0.4` (for post-boot D-Bus query)
- **Integration tested in CI:** yes — end-to-end QEMU boot test includes core startup after boot.
- **Change coordination policy:** Changes to the post-boot D-Bus interface that core queries require joint review.
- **Core team review required for cross-boundary changes:** yes

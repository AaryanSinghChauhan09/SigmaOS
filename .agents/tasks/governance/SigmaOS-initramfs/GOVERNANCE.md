# Governance: SigmaOS-initramfs

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-initramfs owns the build toolchain and configuration for the initial RAM filesystem image that the kernel loads before the root filesystem is mounted. It handles early userspace initialization, root device discovery, cryptographic disk unlocking, and handoff to the real root via `switch_root`. It also provides the emergency shell and diagnostic environment when boot fails.
- **Runtime layer:** kernel (early userspace, pre-pivot_root)
- **Scope boundary with adjacent repos:** SigmaOS-kernel builds the vmlinuz that loads this initramfs; SigmaOS-boot embeds the initramfs path in boot entries; SigmaOS-core takes over after `switch_root` completes; SigmaOS-security provides the disk-encryption key management policy this repo enforces.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `sigmaos-mkinitramfs` CLI | CLI | stable |
| `initramfs.toml` build config format | Config file | stable |
| Early userspace hook directory `/etc/initramfs/hooks/` | Hook API | stable |
| Kernel command-line parameters (`sigmaos.root=`, `sigmaos.crypt=`, etc.) | Kernel cmdline | stable |
| Emergency shell environment variables | Environment variable | internal-only |
| dracut compatibility module API | Module API | unstable |

- **Breaking-change policy:** Kernel command-line parameter names are stable within a MAJOR version; hook directory API changes require a deprecation notice and parallel support for one release cycle.
- **API documentation location:** `docs/initramfs-api.md`, `man/sigmaos-mkinitramfs.8`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| SigmaOS-kernel | 6.6.0 | kernel that loads the generated initramfs | No |
| busybox | 1.36 | core userspace utilities in early environment | No |
| cryptsetup | 2.6 | LUKS disk decryption | No |
| kmod | 31 | kernel module loading in early userspace | No |
| dracut | 059 | optional compatibility layer for hook writers | No |
| udev (systemd-udev) | 254 | device node population in early boot | No |

- **Optional/soft dependencies:** `tpm2-tools` for TPM-backed disk unlock; `plymouth` for graphical passphrase prompt; `mkinitcpio` for Arch-style compatibility mode.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** `cryptsetup` and `kmod` are updated within 1 week of upstream security releases; routine dependency review quarterly.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-initramfs-team
- **Co-maintainers:** @kernel-lead, @security-lead
- **Review quorum:** 2 maintainer approvals required; security-touching changes require @security-lead approval.
- **Escalation path:** Boot-blocking regressions are `priority:critical`; ping @sigmaos-boot-team for rapid triage.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** 30% — hook scripts tested with shellcheck and bats; image build pipeline unit-tested.
- **Integration tests:** Present — QEMU boots generated initramfs image for plain, LUKS, and LVM root scenarios.
- **CI pipeline file:** `.github/workflows/initramfs-ci.yml`
- **Required passing checks before merge:** `shellcheck`, `build-image`, `qemu-plain-boot`, `qemu-luks-boot`, `hook-api-compat`
- **Known flaky tests:** `qemu-luks-boot` occasionally times out on slow runners (issue #23).
- **CI build badge:** `![initramfs CI](https://github.com/SigmaOS/SigmaOS-initramfs/actions/workflows/initramfs-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |
| dracut hooks | 059+ | best-effort compatibility |
| mkinitcpio hooks | 35+ | best-effort compatibility |

- **Hardware classes:** desktop, laptop, server, VM — all supported; embedded is best-effort.
- **Backend variants:** Linux only; tested in CI with QEMU for x86_64 and aarch64.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** LUKS2 unlock tested on 5 reference devices, TPM2 auto-unlock validated, dracut and mkinitcpio hook compatibility confirmed, emergency shell regression-free on all supported arches.

## 8. What This Repo Does NOT Do

- Does not build the kernel → handled by SigmaOS-kernel
- Does not manage boot entries or bootloader config → handled by SigmaOS-boot
- Does not manage disk partitioning → handled by SigmaOS-installer
- Does not manage encryption key policy (only enforces it) → handled by SigmaOS-security
- Does not provide a full rescue/live environment → handled by a separate SigmaOS-rescue image

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/initramfs.toml`, `/etc/initramfs/hooks/` (hook scripts)
- **Schema file location:** `schemas/initramfs.toml.schema.json`
- **Schema format:** TOML
- **Runtime environment variables (kernel cmdline):**

| Parameter | Default | Purpose |
|-----------|---------|---------|
| `sigmaos.root=` | auto-detect | Root device path or UUID |
| `sigmaos.crypt=` | none | LUKS device UUID to unlock before mount |
| `sigmaos.rootflags=` | `ro` | Mount flags for root filesystem |
| `sigmaos.emergency=1` | unset | Force drop to emergency shell |
| `sigmaos.debug.udev=1` | unset | Enable verbose udev output in early boot |

- **Breaking config change policy:** Kernel cmdline parameter names will not be removed within a MAJOR version; new parameters are additive and documented in CHANGELOG.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Root device not found | Wrong UUID or missing driver | Drops to emergency shell with "Cannot find root device" | Boot with `sigmaos.emergency=1`, inspect `/dev` and `/proc/partitions`, check cmdline |
| LUKS decryption failed | Wrong passphrase or missing keyfile | Three retry prompts then emergency shell | Boot with correct passphrase or insert keyfile device |
| Module load failure | Missing driver in initramfs | Device not present in `/dev` | Rebuild initramfs with `sigmaos-mkinitramfs --include-module <name>` |
| `switch_root` failure | Target rootfs corrupt or wrong path | Kernel panic with "Failed to execute init" | Boot recovery image; run `fsck` on root partition |
| Hook script error | Broken custom hook in `/etc/initramfs/hooks/` | Boot halts at hook stage | Remove broken hook; rebuild initramfs |

- **Fallback behavior:** On any unrecoverable failure, drops to a busybox emergency shell with essential tools available; the recovery boot entry (from SigmaOS-boot) provides an alternative path.
- **Operator escalation path:** Boot with `sigmaos.debug.udev=1`; collect emergency shell output; attach serial log and file issue.
- **Crash dump artifacts:** Early boot logs written to `/run/sigmaos/initramfs.log` after `switch_root`, readable by SigmaOS-core.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager post-install hook triggers `sigmaos-mkinitramfs --all-kernels` to rebuild all installed kernel initramfs images; SigmaOS-boot adds new boot entries.
- **Rollback mechanism:** Previous initramfs images retained alongside previous kernel boot entries; rollback boot entry uses old initramfs automatically.
- **Expected downtime on update:** Rebuild takes 5–30 seconds per kernel; reboot required to use new initramfs.
- **Pre-update state checks:** `/boot` partition must have sufficient space; no active boot in progress.
- **Post-update validation:** `sigmaos-mkinitramfs --verify` checks image integrity after build.
- **Atomic update guarantee:** Partial — new initramfs written to a temp path and renamed atomically; old image preserved until boot confirms success.

## 12. Security Boundary

- **Privilege level required:** root (initramfs runs as PID 1 before privilege separation)
- **Trust boundary:** Trusts kernel (verified by Secure Boot); trusts disk encryption keys from TPM or user passphrase; does not trust kernel cmdline parameters when Secure Boot lockdown is active.
- **Attack surface:** Kernel cmdline (pre-boot attacker), LUKS keyslots, hook scripts in `/etc/initramfs/hooks/`
- **Hardening measures:** Secure Boot ensures initramfs image is signed; hook scripts are root-owned and mode 700; LUKS2 with Argon2 KDF; TPM2 PCR binding available.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** cryptsetup CVEs tracked and applied within 48 hours.
- **Security audit status:** Not yet audited externally.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-kernel | 6.6.x | tested |
| SigmaOS-boot | 0.2+ | tested |
| SigmaOS-security | 0.3+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| cryptsetup | 2.6+ | tested |
| busybox | 1.36+ | tested |
| dracut | 059+ | best-effort |

- **Known incompatibilities:** dracut hooks using deprecated `inst_hook` API need updating.
- **Forward-compatibility pledge:** Kernel cmdline parameters remain stable within v1.x series.

## 14. Documentation Parity

- **User documentation status:** up-to-date | `docs/user-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`, `docs/hook-api.md`
- **Man page or in-app help status:** present — `sigmaos-mkinitramfs(8)`
- **Documentation parity policy:** New kernel cmdline parameters or hook API changes require doc updates in the same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:luks` | LUKS/disk-encryption issues |
| `component:hooks` | Initramfs hook API issues |
| `component:emergency-shell` | Emergency shell / recovery issues |
| `component:tpm` | TPM2 auto-unlock issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-initramfs/milestones`
- **Issue triage cadence:** Every Wednesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`)
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump when kernel cmdline parameter names or hook API changes incompatibly.
- **Version pinning in dependents:** SigmaOS-boot pins `sigmaos-initramfs >= MAJOR.MINOR`.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** peer — SigmaOS-initramfs completes its work before SigmaOS-core starts; it writes early-boot diagnostic data that core reads after `switch_root`.
- **IPC / integration mechanism:** File-based handoff via `/run/sigmaos/initramfs.log`; no D-Bus (D-Bus is not available during early userspace).
- **Core API version it targets:** `SigmaOS-core >= 0.4` (for log handoff format compatibility)
- **Integration tested in CI:** yes — end-to-end QEMU boot test confirms core reads initramfs log correctly.
- **Change coordination policy:** Changes to the log handoff format require a joint issue in both repos.
- **Core team review required for cross-boundary changes:** yes

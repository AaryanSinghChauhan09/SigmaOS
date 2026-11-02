# Governance: SigmaOS-kernel

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-kernel owns the kernel configuration profiles, build system, patch set, and boot-time integration for the Linux kernel that powers SigmaOS. It defines which kernel features are enabled, carries SigmaOS-specific patches, and produces the vmlinuz/kernel packages consumed by the installer and update subsystem.
- **Runtime layer:** kernel
- **Scope boundary with adjacent repos:** SigmaOS-initramfs owns the early userspace that runs immediately after the kernel decompresses; SigmaOS-boot owns the bootloader configuration; SigmaOS-drivers owns out-of-tree driver modules built against this kernel.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `/proc` and `/sys` virtual filesystems | Kernel ABI | stable |
| `sigmaos-kernel-headers` package | Shared headers for module build | stable |
| `sigmaos_kernel_version` environment variable (set by initramfs) | Environment variable | stable |
| `kernel.toml` config profile schema | Config file | unstable |
| Kconfig fragment files (`configs/sigmaos-*.config`) | Build input | internal-only |

- **Breaking-change policy:** Any change that removes a previously exported `/proc` or `/sys` node, or drops a `CONFIG_` symbol that out-of-tree modules depend on, requires a MAJOR version bump and a 30-day deprecation notice posted to the SigmaOS developer list.
- **API documentation location:** `docs/kernel-api.md` and generated man pages under `man/`

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| Linux kernel | 6.6 LTS | base kernel source | Yes — SigmaOS low-latency scheduler patch, ZSTD initramfs patch |
| GCC / Clang | GCC 13 / Clang 17 | kernel compilation | No |
| pahole / BTF tools | 1.24 | eBPF BTF generation | No |
| OpenSSL | 3.0 | module signing | No |

- **Optional/soft dependencies:** LLVM for LTO builds (best-effort), DKMS for third-party module rebuild.
- **Upstream patch inventory:** `patches/0001-sigmaos-sched-latency.patch` (local-only, upstreaming planned); `patches/0002-zstd-initramfs.patch` (upstreamed in 6.7, kept as compat shim).
- **Dependency update policy:** LTS kernel base is reviewed every 6 months; security backports are applied within 48 hours of upstream release.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-kernel-team
- **Co-maintainers:** @arch-lead, @security-lead
- **Review quorum:** 2 maintainer approvals required before merge to `main`; 1 approval sufficient for patch branches
- **Escalation path:** File an issue tagged `priority:critical`; ping @sigmaos-core-team if no response within 24 hours
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** Kernel self-tests via `make kselftest` — coverage tracked per subsystem; overall ~40% of touched subsystems
- **Integration tests:** Present — boot smoke tests on QEMU (x86_64, aarch64) on every PR
- **CI pipeline file:** `.github/workflows/kernel-ci.yml`
- **Required passing checks before merge:** `kernel-build-x86_64`, `kernel-build-aarch64`, `kselftest-smoke`, `module-sign-check`
- **Known flaky tests:** `kselftest/net` on aarch64 emulation (timeout-related, tracked in issue #42)
- **CI build badge:** `![kernel CI](https://github.com/SigmaOS/SigmaOS-kernel/actions/workflows/kernel-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| x86_64 | supported |
| aarch64 | supported |
| riscv64 | best-effort |
| i686 | unsupported |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Linux LTS | 6.6 | supported |
| Linux stable | 6.8+ | best-effort |
| BSD | — | unsupported (separate SigmaOS-kernel-bsd planned) |

- **Hardware classes:** desktop, laptop, server, VM — all supported; embedded is best-effort.
- **Backend variants:** Linux only; tested in CI on QEMU for both x86_64 and aarch64.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` on v1.0 release
- **Criteria for promotion:** Zero P0 boot regression bugs, full CI green on all supported arches, module-signing pipeline validated end-to-end, hardware test matrix passing on reference devices.

## 8. What This Repo Does NOT Do

- Does not build bootloaders → handled by SigmaOS-boot
- Does not build initramfs images → handled by SigmaOS-initramfs
- Does not manage out-of-tree driver packaging → handled by SigmaOS-drivers
- Does not define update scheduling or delivery → handled by SigmaOS-update
- Does not manage hardware detection at runtime → handled by SigmaOS-hardware

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `/etc/sigmaos/kernel.toml`, Kconfig fragments at `configs/sigmaos-desktop.config`, `configs/sigmaos-server.config`
- **Schema file location:** `schemas/kernel.toml.schema.json`
- **Schema format:** TOML (user-facing), Kconfig (build-time)
- **Runtime environment variables:**

| Variable | Default | Purpose |
|----------|---------|---------|
| `SIGMAOS_KERNEL_PROFILE` | `desktop` | Selects Kconfig profile at build time |
| `SIGMAOS_KERNEL_LOGLEVEL` | `4` | Sets default printk log level |

- **Breaking config change policy:** Removed Kconfig symbols are announced in CHANGELOG 2 releases before removal; `/etc/sigmaos/kernel.toml` keys are migrated automatically by the upgrade hook.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Kernel panic on boot | Bad kernel config or hardware regression | System halts, panic message on console | Boot to previous kernel via GRUB entry; file bug with serial log |
| Module load failure | ABI mismatch between kernel and module | `modprobe` error; device unavailable | Rebuild DKMS modules; install matching kernel-headers package |
| Kernel OOM kill | Memory pressure with bad cgroup limits | Processes killed unexpectedly | Review cgroup memory limits; check `/proc/meminfo`; tune vm.overcommit |
| Build failure in CI | Upstream patch conflict after LTS rebase | CI red; no kernel package produced | Rebase patch set; resolve conflicts; re-tag |

- **Fallback behavior:** If new kernel fails to boot, GRUB retains the previous two kernel entries automatically.
- **Operator escalation path:** Collect `/var/log/boot.log` and serial console output; file issue with `priority:critical` label.
- **Crash dump artifacts:** kdump configured to write to `/var/crash/`; kernel logs at `/var/log/kern.log`.

## 11. Update / Rollback Behavior

- **Update mechanism:** Package manager (sigmaos-package-manager) installs new `sigmaos-kernel` package; new kernel coexists with old until reboot.
- **Rollback mechanism:** Previous kernel package is retained for 2 versions; GRUB menu exposes rollback entry; `sigmaos-kernel-rollback` CLI triggers automatic previous-entry boot.
- **Expected downtime on update:** Reboot required; typical reboot cycle is 30–60 seconds.
- **Pre-update state checks:** Disk space > 500 MB, `/boot` partition has space for new vmlinuz + initramfs, no active kdump capture in progress.
- **Post-update validation:** First-boot systemd unit `sigmaos-kernel-health.service` runs `kself-test-smoke` and reports pass/fail to SigmaOS-update.
- **Atomic update guarantee:** Partial — new kernel files are written atomically per file, but GRUB config update is not transactional; rollback entry is always preserved.

## 12. Security Boundary

- **Privilege level required:** root (kernel build and install); kernel itself runs at ring 0
- **Trust boundary:** Trusts signed module packages from SigmaOS-repo-manager; rejects unsigned or untrusted modules when Secure Boot is active.
- **Attack surface:** Module loading interface, `/proc`/`/sys`, eBPF subsystem
- **Hardening measures:** `CONFIG_SECURITY_LOCKDOWN=y`, `CONFIG_FORTIFY_SOURCE=y`, `CONFIG_STACKPROTECTOR_STRONG=y`, `CONFIG_RANDOMIZE_BASE=y` (KASLR), `CONFIG_MODULE_SIG_FORCE=y` when Secure Boot is active, seccomp enabled.
- **Threat model reference:** `docs/threat-model.md`
- **Known CVEs / mitigations:** None unmitigated; CVE patch backport policy is 48-hour SLA for CVSS ≥ 7.
- **Security audit status:** Not yet audited externally; internal review 2024-12.

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| SigmaOS-initramfs | 0.3+ | tested |
| SigmaOS-boot | 0.2+ | tested |
| SigmaOS-drivers | 0.1+ | tested |

| Package | Version Range | Status |
|---------|---------------|--------|
| systemd | 254+ | tested |
| glibc | 2.38+ | tested |
| GRUB2 | 2.12+ | tested |

- **Known incompatibilities:** None currently tracked.
- **Forward-compatibility pledge:** Kernel ABI (module interface) is compatible within the same MAJOR.MINOR version series.

## 14. Documentation Parity

- **User documentation status:** stale — kernel profile docs need update for v0.5 Kconfig changes | `docs/user-kernel-guide.md`
- **Developer/contributor docs status:** up-to-date | `docs/contributing.md`
- **Man page or in-app help status:** planned — `sigmaos-kernel-rollback(8)` man page pending
- **Documentation parity policy:** Any Kconfig profile change or new `kernel.toml` key must include a doc update in the same PR.
- **Last documentation review date:** 2025-01-10

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `subsystem:scheduler` | Issues relating to the SigmaOS scheduler patch |
| `subsystem:security-hardening` | Kernel hardening configuration issues |
| `arch:aarch64` | Architecture-specific aarch64 issues |
| `arch:riscv64` | Architecture-specific RISC-V issues |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-kernel/milestones`
- **Issue triage cadence:** Every Wednesday

## 16. Versioning Policy

- **Versioning scheme:** `MAJOR.MINOR.PATCH` — MAJOR tracks the upstream LTS series (e.g., 6 for Linux 6.x), MINOR tracks SigmaOS patch set revisions, PATCH tracks hotfixes.
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags on `main`
- **Changelog format:** Conventional Commits; auto-generated CHANGELOG via `git-cliff`
- **Breaking change signal:** MAJOR bump; announced in CHANGELOG and sent to the developer mailing list.
- **Version pinning in dependents:** SigmaOS-initramfs and SigmaOS-drivers pin to `~MAJOR.MINOR` (patch-compatible range).

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** depended-on-by-core — SigmaOS-core's system services require the kernel ABI and `/proc`/`/sys` interfaces this repo defines.
- **IPC / integration mechanism:** Package dependency; kernel headers shared library; runtime `/proc`/`/sys` interface.
- **Core API version it targets:** `SigmaOS-core >= 0.4`
- **Integration tested in CI:** yes — SigmaOS-core smoke tests run against the kernel QEMU image produced by this repo's CI.
- **Change coordination policy:** Kernel ABI changes that affect core system services require a tracking issue in both repos and a joint review before merge.
- **Core team review required for cross-boundary changes:** yes

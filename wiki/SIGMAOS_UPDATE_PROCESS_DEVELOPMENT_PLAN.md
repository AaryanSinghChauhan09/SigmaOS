# 🔄 SigmaOS System Update Pipeline & Transactional Migration Subsystem (`update_process`) Strategic Development Plan

## Executive Summary & Design Vision

Modern operating system updates must balance atomic package transactions, pre-flight space verification, snapshot rollbacks, sleep inhibitors, post-update migration idempotence, and user notification. Directly invoking package managers (`pacman -Syu` or `apt upgrade`) mid-session without coordination risks killing active user sessions during systemd/system library re-execs, leaving dirty unapplied migration state, or filling disk root partitions.

Drawing direct architectural inspiration from **Omarchy Update Pipeline**, **Fedora rpm-ostree / Silverblue**, **openSUSE MicroOS transactional-update**, **Arch ALPM hooks**, and **snapper Btrfs CoW snapshots**, the **SigmaOS System Update Subsystem** (`SigmaUpdateEngine`, `SigmaPacmanGuard`, `SigmaMigrateNotifier`, `SigmaUpdateStatus`) provides a locked, transactional, rollback-safe, and migration-aware update manager built natively in Safe Rust.

---

## 1. State & Coordination File Architecture

| Path Location | Owner | Purpose & Lifecycle |
| :--- | :--- | :--- |
| `${XDG_RUNTIME_DIR:-/tmp}/sigma-update.lock` | User | Flisk lock preventing overlapping concurrent update pipeline runs. |
| `/tmp/sigma-update.log` | System | Comprehensive transcript log of the active or last update transaction. |
| `~/.local/state/sigma/current/` | User | Active desktop theme, wallpaper symlinks, and runtime profile state. |
| `~/.local/state/sigma/migrations/` | User | Per-user migration marker tracking files (`<migration_id>.sh`). |
| `~/.local/state/sigma/reboot-required` | User / System | Kernel or fundamental glibc update marker triggering reboot prompt. |
| `~/.local/state/sigma/restart-*-required` | User | Component/app restart markers evaluated post-update. |

---

## 2. Multi-Distro & Multi-OS Update Inspirations

### 2.1 Package Manager Guards (`sigma-update-pacman-guard`) & ALPM Hooks
- **Inspirations**:
  - **Omarchy ALPM Pre-Transaction Guard**: Installing `/usr/share/libalpm/hooks/00-sigma-update-guard.hook` to intercept raw `pacman -Syu` or `apt-get upgrade` commands. Unless executed via `sigma update` (`SIGMA_UPDATE_PACMAN=1`) or explicitly bypassed (`SIGMA_ALLOW_DIRECT_PACMAN=1`), the transaction aborts with `AbortOnFail` to protect session stability.
- **SigmaOS Integration**: `SigmaPacmanGuard` in `src/sigpkg/universal_engine.rs` and `src/bin/sigpkg.rs`.

### 2.2 Pre-Flight Disk Checks, Btrfs/Snapper Snapshots & Sleep Inhibitors
- **Inspirations**:
  - **10 GiB Free-Space Requirement**: Verifying `/` root partition capacity before package downloading starts (`SIGMA_UPDATE_FORCE=1` override).
  - **Btrfs / Snapper CoW Snapshots**: Automatically taking a read-only root volume snapshot before applying package transactions, allowing instant point-in-time rollbacks.
  - **Sleep & Idle Inhibitor (`systemd-inhibit`)**: Holding a system sleep inhibitor while updating packages to prevent laptop suspend mid-transaction.
- **SigmaOS Integration**: `TemporalFilesystemEngine` pre-flight snapshotting and `SigmaUpdateStayAwake` inhibitor in `src/distro/future_roadmap_innovations.rs`.

### 2.3 Per-User Idempotent Migrations (`sigma-migrate`)
- **Inspirations**:
  - **User-Session Migration Execution**: Running `migrations/*.sh` scripts as the non-root current user after package transactions complete. Every user gets a chance to apply migrations against `$HOME` and user D-Bus session services.
  - **Pending Migration Notifier (`sigma-migrate-notify`)**: Prompting user login sessions when missing per-user migration markers are detected after direct package upgrades.
- **SigmaOS Integration**: `SigmaMigrateEngine` and `sigma-migrate --pending` query in `src/sigpkg/`.

---

## 3. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                     Userland CLI & Zenith Update Applet                   │
│                        (`sigma update` / `sigpkg update`)                 │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│              `sigma-update-lock` & Pre-Flight Verification                │
│     - Acquire `/tmp/sigma-update.log` Transcript & Lock Descriptor        │
│     - Verify Root Partition Free Space (>= 10 GiB Threshold)              │
│     - Create Pre-Update Btrfs / ZFS CoW Snapshot (`snapper create`)       │
│     - Start Sleep & Idle Inhibitor (`sigma-update-stay-awake start`)      │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│            Package Manager Transaction & ALPM Guard Bypass                │
│     - Execute `SIGMA_UPDATE_PACMAN=1 systemd-run --scope pacman -Syu`    │
│     - ALPM Pre-Transaction Guard (`00-sigma-update-guard.hook`)           │
│     - Automatic Pacman Cache Pruning (`paccache -rk2`)                    │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│            User-Session Migrations & Status Notification                  │
│     - Execute Per-User Idempotent Migrations (`sigma-migrate`)            │
│     - Refresh Shell Update Bar Indicator (`sigma-update-status`)         │
│     - Release Sleep Inhibitor & Check Component Restart / Reboot Markers  │
└───────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Phased Development Roadmap

### Phase 1: Pre-Transaction Guard & Lock Manager (Q4 2026)
- Deploy ALPM pre-transaction hook `00-sigma-update-guard.hook` and `sigma-update-pacman-guard` binary.
- Implement `/tmp/sigma-update.lock` lock descriptor manager and transcript logging to `/tmp/sigma-update.log`.
- Add 10 GiB root partition free-space pre-flight checker.

### Phase 2: Systemd Scope Isolation, Sleep Inhibitor & Snapper Rollback (Q1 2027)
- Wrap package transactions in `systemd-run --scope` to isolate pacman from user-session re-exec kills.
- Integrate Btrfs/snapper pre-update CoW snapshot creation.
- Implement `sigma-update-stay-awake` sleep inhibitor manager.

### Phase 3: Per-User Migration Engine & Login Notifier (Q2 2027)
- Build `sigma-migrate` runner applying `migrations/*.sh` idempotently against `~/.local/state/sigma/migrations/`.
- Deploy `sigma-migrate-notify.service` desktop login notification unit.
- Implement `sigma-update-available` 6-hour interval shell update indicator.

### Phase 4: Automated Log Analysis & CI Integration (Q3 2027+)
- Implement `sigma-update-analyze-logs` post-update log analyzer detecting initramfs or DKMS module build failures.
- Include update transaction and rollback benchmarks in `scripts/tech_media_benchmark_suite.sh`.
- Conduct failure mode fuzz testing on update lock and migration engines (`cargo fuzz`).

---

## 5. Verification & Testing Standards

All update process components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/sigpkg/universal_engine.rs` (`SigmaPacmanGuard` & package update pipeline)
- `src/sigpkg/universal_oop_system.rs` (`AtomicUpdateManager` profile rollbacks)
- `src/distro/future_roadmap_innovations.rs` (`TemporalFilesystemEngine` snapshot rollbacks)
- `src/security/input_validation.rs` (update path & lock file sanitization)

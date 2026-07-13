# Distro Absorption: openSUSE / Tumbleweed / MicroOS

> **Status**: 🔄 Active | **Source Paradigm**: openSUSE Tumbleweed + MicroOS | **Target Shard**: `SigmaOS Package & Update Layer`

---

## 1. Executive Summary

openSUSE offers two paradigms worth absorbing into SigmaOS:

- **Tumbleweed** (rolling-release with massive QA): Contributes the `openQA` automated testing framework philosophy and the `zypper` transactional update concept.
- **MicroOS** (immutable OS container host): Contributes the `transactional-update` atomic rootfs update model and `snapper` Btrfs snapshot management.

Both projects are published under the GNU GPL and are cleanly re-implementable in Rust for the Sovereign Lattice.

---

## 2. Key Features to Absorb

### 2.1 Transactional Updates (`transactional-update` → `sigma-txn-update`)

openSUSE MicroOS makes all system updates transactional: updates are applied to a new Btrfs subvolume snapshot, verified, and only made active on next reboot. The running system is never modified.

```
┌───────────────────────────────────────────────────────────────┐
│               SIGMA TRANSACTIONAL UPDATE FLOW                 │
│                                                               │
│  sigma pkg upgrade                                            │
│       │                                                       │
│       ▼                                                       │
│  1. Create Btrfs snapshot of current rootfs (@base)          │
│  2. Apply package deltas into new snapshot (@next)           │
│  3. Run post-install hooks inside new snapshot (chroot)      │
│  4. Verify new snapshot passes health checks                 │
│  5. Atomically redirect bootloader default → @next           │
│  6. User reboots → boots @next, old snapshot kept            │
│  7. `sigma rollback` → set bootloader to @base               │
└───────────────────────────────────────────────────────────────┘
```

**Rust Implementation:**

```rust
// userland/package_manager/transactional.rs
// SPDX-License-Identifier: MIT

pub struct TransactionalUpdate {
    fs:        BtrfsManager,
    boot:      BootloaderConfig,
    snapshots: SnapshotStore,
}

impl TransactionalUpdate {
    pub fn begin_transaction(&mut self) -> Result<Snapshot> {
        let base = self.snapshots.current_root()?;
        let next = self.fs.create_snapshot(&base, "@next")?;
        println!("Σ [TXN] Snapshot created: {}", next.id);
        Ok(next)
    }

    pub fn apply_packages(&self, snap: &Snapshot, pkgs: &[PackageDelta]) -> Result<()> {
        // Chroot into snapshot and apply package deltas
        let chroot = ChrootEnv::new(&snap.mount_path)?;
        for pkg in pkgs {
            chroot.install_package(pkg)?;
        }
        chroot.run_triggers()?;
        Ok(())
    }

    pub fn commit(&mut self, snap: Snapshot) -> Result<()> {
        self.verify_snapshot(&snap)?;
        self.boot.set_default_snapshot(snap.id)?;
        self.snapshots.mark_active(snap)?;
        println!("Σ [TXN] Transaction committed. Reboot to activate.");
        Ok(())
    }

    pub fn rollback(&mut self) -> Result<()> {
        let prev = self.snapshots.previous_root()?;
        self.boot.set_default_snapshot(prev.id)?;
        println!("Σ [ROLLBACK] Set boot target to snapshot {}", prev.id);
        Ok(())
    }
}
```

### 2.2 Snapper Snapshot Management (`snapper` → `sigma snap`)

```bash
# List all rootfs snapshots
$ sigma snap list
Σ [INFO] System snapshots (Btrfs):

  #   Type      Date                 Description
  1   pre       2025-11-01 08:00     Before: sigma pkg upgrade firefox
  2   post      2025-11-01 08:04     After: sigma pkg upgrade firefox
  3   manual    2025-11-10 14:20     Before: configuration change
  4   auto      2025-11-10 23:00     Daily auto-snapshot
  5 ◄ active    2025-11-11 09:00     Current (booted)

# Diff between snapshots
$ sigma snap diff 1 2
--- /etc/sigma/network.toml  (snapshot 1)
+++ /etc/sigma/network.toml  (snapshot 2)
+dns_over_https = true

# Roll back to snapshot 3
$ sigma snap rollback 3
Σ [WARN] Will roll back /etc/sigma/network.toml and 4 other changed files.
Proceed? [y/N] y
Σ [SUCCESS] Rolled back. Reboot required.
```

### 2.3 openQA-Inspired CI Testing (`sigma-qa`)

openQA automates full OS installation and regression tests in VMs. SigmaOS adapts this as `sigma-qa`, a QEMU-backed integration test runner for every PR:

```toml
# .sigma-qa/tests/boot.toml
[test]
name = "Basic Boot Test"
timeout_s = 120

[[steps]]
type = "boot"
target = "sigma-latest.iso"

[[steps]]
type = "wait_screen"
match = "SigmaOS login:"
timeout_s = 60

[[steps]]
type = "type"
text = "root\n"

[[steps]]
type = "wait_screen"
match = "sigma@sigma:~$"

[[steps]]
type = "assert_command"
cmd = "sigma status"
expect_exit = 0
```

---

## 3. CLI Reference

```bash
sigma pkg upgrade           # Transactional system upgrade (requires reboot)
sigma snap list             # List all Btrfs snapshots
sigma snap create           # Create manual snapshot
sigma snap diff <id1> <id2> # Show file diff between snapshots
sigma snap rollback <id>    # Roll back to a previous snapshot
sigma qa run                # Run openQA-style integration test suite
```

---

## 4. References & Standards

- openSUSE MicroOS documentation — `microos.opensuse.org`
- `transactional-update` source — `github.com/openSUSE/transactional-update` (GPL-2.0)
- `snapper` source — `github.com/openSUSE/snapper` (GPL-2.0)
- openQA — `open.qa` (GPL-2.0)

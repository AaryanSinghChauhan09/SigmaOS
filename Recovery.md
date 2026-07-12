# SigmaOS Recovery

SigmaOS includes multiple layers of recovery — from kernel-level self-healing to a full live recovery environment.

---

## Recovery Layers

| Layer | Trigger | Action |
|-------|---------|--------|
| Kernel self-heal | Kernel panic / NULL deref | Attempt in-place recovery, fallback to rollback |
| Watchdog | Daemon unresponsive for > 30s | Restart daemon; escalate to kernel WDT |
| Rollback boot | 3 consecutive failed boots | Boot to previous OSTree A/B snapshot |
| Recovery shell | Emergency serial console | `sigma-sh` with read-only root |
| Recovery ISO | Physical/USB boot | Full forensic + repair environment |

---

## Self-Healing Kernel (`kernel/self_healing/`)

```cpp
// Kernel registers a recovery handler per subsystem
sigma_register_recovery_handler(SUBSYS_NET, net_recovery_fn);

// On panic — attempt recovery before full reboot
void sigma_panic_handler(const char* msg) {
    if (sigma_try_recover(current_subsystem())) return; // recovered
    sigma_rollback_to_snapshot();  // fallback
}
```

Self-healing actions:

- Restart failed kernel threads

- Flush and re-initialise corrupted driver state

- Trigger OSTree A/B boot switch on unrecoverable faults

- Write forensic log to immutable audit trail

---

## OSTree A/B Rollback

On every successful boot, SigmaOS marks the current partition as "good". If 3 consecutive boots fail (watchdog timeout), the bootloader switches to the alternate A/B partition.

```
/dev/sda1  (EFI)
/dev/sda2  (SigmaOS A — current)   ← active
/dev/sda3  (SigmaOS B — fallback)
/dev/sda4  (data — shared, never rolled back)
```

Manual rollback:
```bash
sigma rollback list           # show available snapshots

sigma rollback to v15.0.0    # restore specific version

sigma rollback cancel         # stay on current

```

---

## Snapshot Management

```bash

# Create snapshot before risky operation

sigma snapshot create pre-update

# List snapshots

sigma snapshot list

# Diff two snapshots

sigma snapshot diff pre-update HEAD

# Restore

sigma snapshot restore pre-update
```

Snapshots are stored as delta-compressed OSTree commits — restoring a snapshot never touches the `/data` partition.

---

## Recovery Shell (Emergency)

If the GUI won't start, SigmaOS falls through to an emergency serial shell:

```
SigmaOS Recovery Shell v15.0
Type 'help' for available commands

sigma-sh# fsck /dev/sda2        # check filesystem

sigma-sh# sigma-mount /dev/sda2 /mnt  # mount root

sigma-sh# sigma-pkg repair      # reinstall broken packages

sigma-sh# sigma-log tail 100    # read last 100 kernel log lines

sigma-sh# reboot                # restart

```

---

## Recovery ISO (Planned — Phase G)

A bootable recovery image (`sigma-rescue.iso`) will provide:

- Full filesystem repair tools (`sigma-fsck`, `sigma-badblocks`)

- Snapshot restore GUI

- Forensic imaging (`sigma-dd`, `sigma-forensics`)

- Network-accessible SSH recovery session

- Factory reset option (wipes `/` but preserves `/data`)

---

## Recovery Source Files

| File | Purpose |
|------|---------|
| `kernel/self_healing/` | In-kernel recovery hooks |
| `kernel/recovery/` | Rollback orchestration |
| `recovery/SovereignRecoverySuite.cpp` | High-level recovery API |
| `recovery/forensic/` | Forensic tools |
| `recovery/sync/` | Snapshot sync |
| `tools/sigma_recover.cpp` | CLI recovery tool |
| `scripts/regression_check.sh` | Boot health check |

---

### See also: [System-Daemons](System-Daemons) · [Kernel](Kernel) · [Storage](Storage)

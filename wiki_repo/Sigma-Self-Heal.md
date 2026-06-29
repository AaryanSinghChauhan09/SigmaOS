# Sigma Self-Heal — Autonomous OS Repair

sigma-heal is the SigmaOS daemon that fixes the OS itself without IT support. It runs continuously in the background, monitors every subsystem, and repairs faults before the user even notices them.

No IT department. No StackOverflow. No re-installation. The OS just fixes itself.

---

## What It Detects and Fixes

### Filesystem Corruption

```
Filesystem corruption:
├── Detects: bad sectors, corrupted inodes, orphaned files
├── Fixes: btrfs scrub + repair, fsck equivalent per filesystem type
└── If unfixable: restore from sigma-mirror automatically
```

sigma-heal polls filesystem health every 60 seconds using `btrfs device stats` and checks for:
- Bad sectors and read errors
- Corrupted inodes and orphaned files
- Filesystem tree inconsistencies

**Repair flow:**
1. `btrfs scrub start /` → waits for result
2. If errors found → `btrfs scrub -r` (repair mode)
3. If still unfixable → pulls clean snapshot from `sigma-mirror` (encrypted remote backup)
4. Logs event to immutable DID-signed audit trail

### Kernel Panic Recovery

```
Kernel panic recovery:
├── On crash: capture full memory dump (kdump)
├── Boot to recovery kernel (slot-B always available)
├── sigma-ai analyzes crash → identifies likely cause
└── Apply hotfix (sigma-livepatch) or rollback to last known good state
```

SigmaOS always keeps two kernel slots (A/B). When a panic occurs:
1. kdump captures full memory dump to `/var/sigma-crash/`
2. System boots to slot-B (recovery kernel)
3. `sigma-ai` analyzes the dump — identifies faulting module, stack trace pattern
4. If a known hotfix exists → apply via `sigma-livepatch` (no reboot needed)
5. If no hotfix → roll back to last known-good generation

Every crash and its resolution is logged with a DID-signed event for audit.

### Package Conflicts

```
Package conflicts:
├── Detects: broken dependencies after failed upgrade
├── Fixes: dependency solver + rollback broken packages
└── Log: what was broken, what was done
```

After any `sigma-pkg upgrade`, sigma-heal verifies the dependency graph. If a broken state is found:
1. Dependency resolver identifies the offending package
2. Either re-fetches the correct version or rolls back to the previous generation
3. System is never left in a partial upgrade state

### Network Self-Heal

```
Network self-heal:
├── DNS not resolving → try alternate DNS (1.1.1.1, 8.8.8.8, Cloudflare DoT)
├── Default route gone → try DHCP renew (sigma-netd --renew)
└── Wi-Fi driver crashed → reload module (rmmod + modprobe)
```

sigma-heal subscribes to `sigma-bus` network events from `sigma-netd`. On failure:
- DNS probe fails → switch to fallback DNS automatically, restore original when it recovers
- Route table empty → trigger DHCP renew on all interfaces
- Wi-Fi kernel module crash → unload + reload (`iwlwifi`, `mt7921`, etc.)

### Security Self-Heal

```
Security self-heal:
├── sigma-ids detects intrusion → auto-isolate compromised process
├── Rootkit detected → integrity restore from PQ-signed verified backup
└── PQC key compromise → auto-generate new DID keypair
```

sigma-heal integrates with `sigma-ids` (intrusion detection) and `sigma-trustd` (key management):
- **Process isolation**: compromised process gets sandboxed with `sigma-jail --isolate <pid>`
- **Rootkit**: file integrity baseline (Dilithium3-signed) compared — deviations restored from baseline
- **Key compromise**: DID keypair automatically revoked and regenerated; all services notified via `sigma-bus`

### Hardware Self-Heal

```
Hardware self-heal:
├── GPU driver crash → switch to software rendering (llvmpipe — no black screen)
├── Sound card failure → mute gracefully (no kernel panic)
└── USB disconnect during operation → safe state, no data loss
```

Because SigmaOS drivers run in userspace (SDF), a driver crash is isolated and handled:
- GPU driver dies → display server switches to `llvmpipe`/`softpipe` software renderer — user sees degraded performance, not a black screen
- Audio driver dies → sigma-audio mutes gracefully, no kernel involvement
- USB device pulled during a write → write barrier was already issued; file system state is consistent

---

## CLI Commands

```bash
# What was healed in the last 30 days
sigma-heal status

# Full repair history (paginated)
sigma-heal log

# Full history filtered by category
sigma-heal log --category filesystem
sigma-heal log --category security

# Run simulation: what would happen if this component fails?
sigma-heal simulate --component "nvidia.ko"
sigma-heal simulate --component "wlan0"
sigma-heal simulate --component "dns"
sigma-heal simulate --component "/dev/sda1"

# Export stats as JSON
sigma-heal stats --output /tmp/heal-stats.json
```

### Sample `sigma-heal status` output

```
sigma-heal status (last 30 days)
─────────────────────────────────────────────────────
Category         Events   Fixed   Mitigated   Failed
─────────────────────────────────────────────────────
Filesystem          2       2         0          0
Network             7       7         0          0
Package             1       1         0          0
Hardware            3       2         1          0
Security            0       0         0          0
─────────────────────────────────────────────────────
Total              13      12         1          0
Last event: 3 days ago  (network: DNS fallback activated)
```

### Sample `sigma-heal simulate` output

```
sigma-heal simulate --component "nvidia.ko"

Scenario: NVIDIA GPU driver crash
─────────────────────────────────
Predicted impact: Display server loses GPU context
Sigma-heal action: Switch to llvmpipe software rendering
Estimated recovery: 800ms (display flicker, then back)
User impact: Yes — brief flicker, then degraded 3D performance
Fallback mode: Software rendering (llvmpipe)
User notification: "GPU driver restarted — performance reduced"
Permanent fix needed: sigma-pkg update nvidia-sdf-driver
```

---

## Architecture

sigma-heald is a Go daemon (`sigmad/heal/main.go`) that:
1. Subscribes to `sigma-bus` for hardware, security, and package events
2. Runs a 60-second poll loop for filesystem and service health
3. Calls into `sigma_heal.h` C library for actual repair operations
4. Writes all events to the DID-signed immutable audit journal

```
sigma-bus events
    │
    ├── HARDWARE_CRASH → sigma_heal_hw_*()
    ├── SECURITY_ALERT → sigma_heal_sec_*()
    ├── PKG_BROKEN     → sigma_heal_pkg_fix()
    └── NET_DOWN       → sigma_heal_net_repair()

Poll loop (60s)
    ├── sigma_heal_fs_check() for all mounted filesystems
    └── sigma_heal_daemon_run() service health checks
```

---

## What sigma-heal Does NOT Do

- Does not modify data files (only system files and configurations)
- Does not make network connections on your behalf (no phoning home)
- Does not hide events — everything is logged to the DID-signed audit trail
- Does not replace a human decision for FAILED repairs — admin is notified

---

## Comparison with Other Systems

| OS | Self-Repair Capability |
|---|---|
| Ubuntu | None — manual intervention required |
| Windows | `sfc /scannow` — filesystem only, manual |
| macOS | Disk First Aid — manual, filesystem only |
| Android | Factory reset — destructive |
| **SigmaOS** | **6 categories, fully autonomous, DID-audited** |

---

*See also: [System Daemons](System-Daemons) · [Security Model](Security-Model) · [Testing Infrastructure](Testing-Infrastructure) · [SigmaOS Vision for India](SigmaOS-Vision-India)*

# 🛡️ Recovery, Forensics & Reliability

> **"When Linux distros crash, you boot a rescue ISO. When SigmaOS crashes, the kernel heals itself."**

SigmaOS integrates recovery, forensic auditing, and self-healing directly into the kernel — no external rescue media needed. This rivals tools like Rescuezilla, SystemRescue, and CAINE while being natively integrated.

---

## 🆚 Comparison with Linux Recovery Tools

| Feature | Rescuezilla | SystemRescue | CAINE (Forensic) | SigmaOS Recovery |
|:--|:--|:--|:--|:--|
| Deployment | External ISO boot | External ISO boot | External ISO boot | **Built into kernel** |
| Snapshot engine | Clonezilla backend | fsarchiver | dd / dcfldd | **Atomic VFS snapshot** |
| Rollback | Manual clone restore | Manual | N/A | **One-click `atomicLatticeSync()`** |
| Boot menu integration | No (separate tool) | GRUB chainload | No | **Native boot menu entry** |
| Crypto verification | None | None | Hash verification | **FNV-1a + Dilithium-5 signed** |
| Forensic audit trail | None | None | Autopsy / Sleuth Kit | **`sigma_compliance_cli` signed logs** |

---

## 1. SovereignRecoverySuite

The core recovery engine (`recovery/SovereignRecoverySuite.cpp`):

### Snapshot Types
| Type | What it captures |
|:--|:--|
| `FULL_DISK` | Entire block device — bootable recovery point |
| `PARTITION` | Single partition (e.g., `/sigma/pkgs/`) |
| `FILESYSTEM` | VFS tree state only |
| `CONFIG` | Configuration-only snapshot (<1MB) |

### Key Operations
```cpp
recovery_init();                           // Initialize suite + register devices
recovery_snapshot("pre-upgrade", 0, "/dev/sda");  // Create FULL_DISK snapshot
recovery_restore(1);                       // Restore snapshot ID 1
recovery_atomic_sync("EMERGENCY");         // Emergency Lattice Sync (atomic)
recovery_boot_menu();                      // Print recovery boot menu
```

---

## 2. Emergency Lattice Sync (ELS)

The ELS is SigmaOS's flagship recovery feature — a one-button atomic state capture:

```
User clicks 🔄 in Zenith Dock
        │
        ▼
atomicLatticeSync("EMERGENCY")
        │
        ├── Lock VFS state (atomic operation)
        ├── Block-level copy to recovery partition
        ├── Compute FNV-1a checksum
        ├── Dilithium-5 sign the snapshot
        └── Mark snapshot as verified + bootable
```

- **Trigger points:** Zenith Dock button, pre-install hook in OmniPkg, boot menu
- **Recovery:** Select snapshot from boot menu → instant rollback

---

## 3. Forensic Audit System

Inspired by CAINE and Autopsy, SigmaOS provides native forensic capabilities:

### `sigma_compliance_cli` Audit Features
- **Syscall logging** — Every Ring-0 operation is logged with timestamps
- **Capability audit** — Which shards accessed which hardware capabilities
- **Integrity proof** — Generates Dilithium-5 signed attestation reports
- **Export formats** — JSON, CSV, compliance-ready PDF

### Compliance Frameworks
| Framework | Auto-Generated Proof |
|:--|:--|
| ISO 27001 | ✅ Information security controls attestation |
| NIST 800-53 | ✅ Security control baseline verification |
| GDPR Art. 32 | ✅ Data protection technical measures proof |
| HIPAA | ✅ PHI access audit trail |
| SOC 2 Type II | ✅ Continuous monitoring evidence |

---

## 4. Self-Healing Resilience Engine

Beyond manual recovery, SigmaOS includes proactive self-healing:

- **`EmergencyLatticeSync`** — Auto-triggers on kernel panic (captures state before reboot)
- **Watchdog Daemon** — Monitors shard health; restarts failed services within 50ms
- **Declarative Rollback** — If a config change breaks boot, the `DeclarativeEngine` automatically rolls back to the last known-good generation

---

## 5. Recovery Boot Menu

```
╔══════════════════════════════════════════╗
║     SigmaOS Recovery Boot Menu           ║
╠══════════════════════════════════════════╣
║  1. Restore: pre-upgrade-2025-05-26     ║
║  2. Restore: EMERGENCY_SYNC             ║
║  R. Recovery Shell                       ║
║  D. Disk Diagnostics                     ║
╚══════════════════════════════════════════╝
```

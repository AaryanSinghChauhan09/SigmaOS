# Sovereign Recovery Suite (`sigma-recovery`)

The **Sovereign Recovery Suite** (inspired by Rescuezilla, SystemRescue, and CAINE) provides SigmaOS with system-level resilience, secure partition backups, and forensic audit protections. 

## 🛠️ Key Recovery Subsystems

### 1. Zero-Footprint Memory Forensics
Enables deep memory scanning of active process shards to detect anomalies or heap tampering dynamically. 
- **Path:** `recovery/forensic/ForensicEngine.cpp`

### 2. Sleuthkit-Style File Carving
Scans unallocated sectors and reconstructs deleted administrative files or packages directly into safe, air-gapped forensic vault directories.
- **Path:** `recovery/forensic/ForensicEngine.cpp`

### 3. Emergency Lattice Sync (ELS) Snapshots
Automates high-fidelity block backups, committing all active system shards directly to read-only, hardware-isolated backup partitions.
- **Path:** `recovery/sync/EmergencyLatticeSync.cpp`

---

## 💻 Command Line Tool Reference (`sigma-recovery`)

```bash
# Snapshot the entire active lattice into air-gapped recovery sectors
sigma-recovery sync

# Scan running shards for memory forensics anomalies
sigma-recovery scan

# Carve and restore deleted system files from a partition
sigma-recovery carve <device_volume_path>

# View the Dilithium-5 signed SHA-256 system audit reports
sigma-recovery report
```

---

## 🧪 Vitest Validation

Verification includes comprehensive checking inside **[sovereign_recovery.test.js](file:///c:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/tests/sovereign_recovery.test.js)**:
- Air-gapped snapshot sector checksum consistency.
- Successful memory carving and artifact recovery.
- Verification of signed forensic logs.

All test gates are passing with **100% success**.

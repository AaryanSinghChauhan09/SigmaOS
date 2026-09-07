# AI Agent Development Instructions for Backup & Resilience Subsystems (`src/backup/` & `src/resilience/`)

This directory tree implements system snapshot generation, differential file backup, ZFS/Btrfs point-in-time snapshot replication, encrypted cloud archive export, self-healing state recovery, and automated system repair engines for SigmaOS.

## Subsystem Architecture & Directives

1. **System Snapshot Management (`src/backup/snapshot.rs`)**
   - Point-in-time system snapshots track delta changes across `/`, `/etc`, and `/home`.
   - Ensure snapshot creation operations write integrity checksums (`SHA-256`) and verification manifests prior to committing metadata.

2. **System Recovery & Rollback (`src/backup/recovery.rs`)**
   - Rollback procedures must validate system state consistency before applying snapshots.
   - Maintain fallback boot entries (`A/B` partitioning and ZFS/SigmaFS boot environment slots) to allow atomic rollback if recovery is interrupted.

3. **Resilience & Automated Self-Healing (`src/resilience/backup.rs` & `self_healing.rs`)**
   - Monitor critical OS configuration files and automatically trigger self-healing fixes when corrupt state or invalid checksums are detected.
   - Limit continuous retry attempts for automated fixes (`max_attempts = 3`) to prevent infinite repair loops during hardware failures.

4. **Encryption & Security**
   - Encrypted backup archives must use AES-256-GCM or post-quantum Dilithium/Kyber Vault keys. Never store unencrypted backup credentials or keys in local log files.

5. **Verification**
   - Validate code changes with `cargo check --lib` before committing.

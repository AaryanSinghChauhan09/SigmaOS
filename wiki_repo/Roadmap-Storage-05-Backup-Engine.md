# SigmaOS Roadmap: Incremental Backup Engine (sigma-backup)
Automated, encrypted, incremental backups with deduplication.
## Goals
- Content-addressed deduplication (BLAKE3 hash)
- Encrypted backup archives with sigma-vault keys
## Key Milestones
- [ ] Chunk-based deduplication pipeline
- [ ] Incremental snapshot diff
- [ ] Remote backup to sigma-store or rclone targets
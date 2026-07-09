# SigmaOS Roadmap: Full-Disk Encryption
Transparent full-disk encryption using AES-256-XTS.
## Goals
- dm-crypt equivalent in the block layer
- TPM2 sealed key for auto-unlock
## Key Milestones
- [ ] AES-256-XTS cipher implementation
- [ ] Key derivation via Argon2id
- [ ] TPM2 sealed LUKS-equivalent header
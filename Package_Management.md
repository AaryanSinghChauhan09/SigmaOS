# Package Management Roadmap (sigpkg)

## sigpkg v1 Specification
`sigpkg` is the atomic, declarative package manager for SigmaOS. 

### Key Features
1. **Cryptographic Provenance:** Every package is signed via Ed25519. Unsigned packages are rejected by the daemon.
2. **Delta Updates:** Updates utilize binary deltas to minimize bandwidth.
3. **Atomic Commits:** Updates are extracted to a passive BTRFS/SigmaFS subvolume. The active bootloader pointer is swapped atomically upon success.
4. **Instant Rollback:** If an update fails verification or triggers an anomaly during the next boot, the system instantly rolls back to the previous snapshot.

## Implementation Timeline
- **Month 1:** `sigpkg` CLI stub and daemon IPC.
- **Month 2:** Ed25519 signature verification integration.
- **Month 3:** BTRFS snapshot integration and delta patching engine.

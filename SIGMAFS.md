
# SigmaFS — Sovereign Filesystem


SigmaFS is SigmaOS's flagship cryptographically verifiable filesystem, implemented in `modules/core/fs/sigmafs.c`.


## Design Goals

- **Zero Silent Corruption**: Every block stores a hash of its own data. Any bit-flip is immediately detected.
- **Tamper-Proof Directories**: Directory entries carry Merkle hashes of their children.
- **Versioned Rollback**: Snapshot any filesystem state, roll back without data loss.
- **Crash Recovery**: Full journaling ensures atomic writes — no half-written state survives a power loss.


## Superblock


```c
typedef struct {
    uint64_t magic;         // 0x5369676D61465300 = "SigmaFS"
    uint32_t version;
    uint32_t block_count;
    uint32_t free_blocks;
    uint8_t  root_hash[32]; // Merkle root of all block hashes
    uint64_t last_snapshot_id;
} sigmafs_superblock_t;
```


## Operations


| Operation | Function | Description |
| :--- | :--- | :--- |
| Write block | `sigmafs_write_block()` | Writes data and stamps hash |
| Verify block | `sigmafs_verify_block()` | Checks stored vs computed hash |
| Snapshot | `sigmafs_snapshot()` | Records Merkle root checkpoint |
| Journal begin | `journal_begin()` | Saves pre-write hash |
| Journal commit | `journal_commit()` | Records post-write hash |


## Roadmap

- [ ] Real SHA-256 via `libsovereign_crypto`
- [ ] Merkle tree root computation across all blocks
- [ ] IPFS-style distributed block references
- [ ] Encryption-at-rest for all blocks (AES-256-GCM)

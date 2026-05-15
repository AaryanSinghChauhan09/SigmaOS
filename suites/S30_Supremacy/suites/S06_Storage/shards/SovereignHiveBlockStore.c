// =============================================================================
// SigmaOS — S06_Storage — SovereignHiveBlockStore.c
// Decentralized Mesh-Mirrored File System
// =============================================================================
// Exceeding Competitors:
//   • iCloud / OneDrive / Dropbox — Centralized, server-dependent, privacy-risk.
//   • Sigma BlockStore — P2P distributed VFS. Files are split into encrypted 
//     shards and mirrored across all Hive nodes (S12).
// Architecture:
//   • No central server. Data is immutability-anchored via PQC hashes (S08).
//   • Automatic block-repair: If one node fails, S13 triggers a mesh-rebuild.
//   • Near-Zero latency local cache with lazy-mesh synchronization.
// =============================================================================

#include "../../../../../include/core/sigma_types.h"


#define BLOCK_SIZE          4096
#define REPLICATION_FACTOR  3

typedef struct {
    uint8_t  block_hash[64]; // PQC Dilithium Hash
    uint32_t owner_node_ids[REPLICATION_FACTOR];
    uint32_t version;
    bool     is_cached_locally;
} HiveBlock;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Mesh-mirrored BlockStore
void hive_store_init(void);

// Write a block to the Hive (triggers replication across S12 Mesh)
bool hive_store_write_block(uint64_t lba, void* data);

// Read a block (tries local S06 cache first, then mesh fetch)
bool hive_store_read_block(uint64_t lba, void* out_data);

// Sweep the mesh for missing block replicas (Self-Healing Step 3)
void hive_store_repair_mesh(void);

// Encrypt and anchor a file to the Hive State (unmodifiable global ref)
uint64_t hive_store_anchor_file(const char* path);

// Synchronize block metadata handles with S10_Registry
void hive_store_sync_registry(void);




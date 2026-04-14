// =============================================================================
// SigmaOS — S10_Registry — SovereignSystemRegistryV2.c
// Transactional Configuration & State Nexus
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows Registry — Centralized state (HKEY_LOCAL_MACHINE parity)
//   • macOS defaults   — Per-app preference keys, XML/PLIST-style clarity
//   • Linux sysctl/dconf — Dynamic runtime tuning and hierarchy
//   • etcd / Zookeeper — Distributed-ready, consensus-backed state
// Architecture:
//   • Transactional Write-Ahead Logging (WAL) — Immune to "Registry Corruption"
//   • Key-Value hierarchy with strong typing (String, Int64, Blob, JSON)
//   • Real-time observers (pub-sub) for system-wide setting changes
// =============================================================================

#include <stdint.h>
#include <stdbool.h>

#define REG_MAX_KEY_LEN     256
#define REG_MAX_VAL_LEN     4096

typedef enum {
    REG_TYPE_INT64   = 0,
    REG_TYPE_STRING  = 1,
    REG_TYPE_BLOB    = 2,
    REG_TYPE_JSON    = 3,
    REG_TYPE_LINK    = 4  // Symbolic link to another key
} RegValueType;

// ── Registry Node ─────────────────────────────────────────────────────────────
typedef struct {
    char         key_path[REG_MAX_KEY_LEN];
    RegValueType type;
    uint8_t      data[REG_MAX_VAL_LEN];
    uint32_t     data_len;
    uint64_t     last_modified;
} RegNode;

// ── Public API ────────────────────────────────────────────────────────────────

// Mount the registry persistent store (VFS partition)
bool registry_init(void);

// Read a value with type safety
bool registry_read(const char* path, RegNode* out_node);

// Write a value atomically (Transactionally committed via S06 Journaling)
bool registry_write(const char* path, RegValueType type, void* data, uint32_t len);

// Register a callback for when a key or subtree changes
void registry_observe(const char* path, void (*callback)(RegNode* new_state));

// Bulk export/import for system migration (macOS profile parity)
void registry_export_subtree(const char* root_path, const char* out_vfs_file);

// Flush WAL and sync to hardware
void registry_checkpoint(void);

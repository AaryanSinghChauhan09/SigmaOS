// =============================================================================
// SigmaOS — S10_Registry — S10_01_HkeyLocalMachine.c
// Persistent System Configuration Hive Shard
// =============================================================================

#include "../../../../../include/core/sigma_types.h"


#define MAX_SYSTEM_KEYS 4096

typedef struct {
    char     key[128];
    uint8_t  data[1024];
    uint32_t type;
} RegEntry;

// ── Public API ────────────────────────────────────────────────────────────────

// Mount the HKLM hive from S06 Storage
void registry_hklm_init(void);

// Read a system-level configuration key
bool registry_hklm_read(const char* key, RegEntry* out);

// Atomic write to HKLM (gated by S08 capability)
bool registry_hklm_write(const char* key, RegEntry* in);

// Synchronize global settings with Hive peers (Ecosystem hook)
void registry_hklm_broadcast_change(const char* key);




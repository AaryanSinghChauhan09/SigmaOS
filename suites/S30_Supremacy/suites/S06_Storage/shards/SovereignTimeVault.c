// =============================================================================
// SigmaOS — S06_Storage — SovereignTimeVault.c
// atomic State Snapshots & Universal Revert
// =============================================================================
// Competitor USPs Absorbed:
//   • macOS Time Machine — user-friendly historical file recovery
//   • Windows Shadow Copy (VSS) — block-level snapshotting for backups
//   • APFS/ZFS Snapshots — near-instant, zero-cost CoW snapshots
// Exceeding Competitors:
//   • "Total System Revert": Roll back the kernel, registry, and userland 
//     to any timestamp in under 5 seconds (Atomic boot flip).
// =============================================================================

#include "sigma_types.h"


#define MAX_SNAPSHOTS       128

typedef struct {
    uint32_t snapshot_id;
    uint64_t timestamp;
    char     label[64];
    uint64_t root_lba;     // S06 Root Journal LBA at time of snapshot
    bool     is_bootable;  // True = Full OS restore point
} OSStateSnapshot;

static OSStateSnapshot vault[MAX_SNAPSHOTS];
static uint32_t         snapshot_count = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the TimeVault nexus using S06 Journaling primitives
void timevault_init(void);

// Create an atomic system-wide snapshot (Zero-copy CoW)
uint32_t timevault_capture_state(const char* label);

// List all historical state points
uint32_t timevault_list(OSStateSnapshot* out, uint32_t max);

// The "NUCLEAR" Revert: Instantly roll back the OS to a previous snapshot
// Requires reboot or hot-kernel-reload (S13 parity)
void timevault_revert_to(uint32_t snapshot_id);

// Purge old snapshots based on retention policy
void timevault_prune(uint32_t days_to_keep);

// Mount a snapshot as Read-Only for granular file recovery
bool timevault_mount_ro(uint32_t snapshot_id, const char* mount_point);




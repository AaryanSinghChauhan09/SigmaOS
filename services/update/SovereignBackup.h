#ifndef SOVEREIGN_BACKUP_H
#define SOVEREIGN_BACKUP_H

#include <stdint.h>
#include <stdbool.h>

// Timeline-based Snapshot (Time Machine / File History style)
typedef struct {
    uint64_t snapshot_id;
    uint64_t timestamp;
    char description[128];
    
    // Abstract pointer to the VFS differential storage tree
    void* vfs_diff_tree;
    bool is_encrypted;
} BackupSnapshot_t;

// --- API ---

/**
 * Executes an atomic differential backup of the VFS state.
 * Only blocks that have changed since the last snapshot are stored.
 */
SovereignStatus_t backup_create_snapshot(const char* description);

/**
 * Restores the file system state to the exact timestamp of the target snapshot.
 * Requires a system reboot to apply kernel-locked file reverts.
 */
SovereignStatus_t backup_restore_snapshot(uint64_t snapshot_id);

/**
 * Prunes old snapshots to strictly abide by maximum storage allocations.
 */
void backup_enforce_retention_policy(uint32_t max_snapshots);

#endif // SOVEREIGN_BACKUP_H

#ifndef SIGMA_BACKUP_RESTORE_H
#define SIGMA_BACKUP_RESTORE_H

#include <sigma_types.h>

// SigmaOS Sovereign Backup & Restore Utility
// Absorbs Apple Time Machine and ZFS snapshot paradigms.

// Trigger an immediate deduplicated snapshot of the VFS layer
uint32_t util_backup_trigger_snapshot(const char* volume_mount);

// Restore a given volume to a mathematically verified snapshot ID
void util_restore_from_snapshot(const char* volume_mount, uint32_t snapshot_id);

// Seamlessly defragment mechanical or optimize solid-state blocks post-restore 
void util_defragment_and_trim(const char* volume_mount);

#endif // SIGMA_BACKUP_RESTORE_H


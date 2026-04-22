#ifndef SOVEREIGN_UPDATER_H
#define SOVEREIGN_UPDATER_H

#include <stdint.h>
#include <stdbool.h>

// Defines the Dual-Partition (A/B) strategy for unbreakable OTA updates
typedef enum {
    PARTITION_A = 0,
    PARTITION_B = 1
} BootPartition_t;

typedef enum {
    UPDATE_STATUS_IDLE = 0,
    UPDATE_STATUS_DOWNLOADING = 1,
    UPDATE_STATUS_VERIFYING = 2,
    UPDATE_STATUS_READY_FOR_REBOOT = 3,
    UPDATE_STATUS_FAILED_ROLLBACK_INITIATED = 4
} UpdateStatus_t;

typedef struct {
    BootPartition_t active_partition;
    BootPartition_t inactive_partition;
    uint32_t current_os_version;
    uint32_t staged_os_version;
    bool boot_successful_flag;  // Must be asserted true by Kernel within 30s of boot
} PartitionTable_t;

// --- API ---

/**
 * Downloads a delta-patch into the inactive partition in the background.
 */
UpdateStatus_t supt_stage_update(const char* remote_url);

/**
 * If boot_successful_flag is never set (Kernel Panic on boot), the BIOS/Bootloader
 * automatically invokes this to revert to the previous working partition.
 */
void supt_trigger_rollback(PartitionTable_t* p_table);

/**
 * Commits the update by swapping the active partition flags and rebooting.
 */
void supt_apply_and_reboot();

#endif // SOVEREIGN_UPDATER_H

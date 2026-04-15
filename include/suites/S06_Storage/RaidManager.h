#ifndef SIGMA_RAID_MANAGER_H
#define SIGMA_RAID_MANAGER_H

#include "sigma_types.h"

// SigmaOS Sovereign RAID & Quota Manager
// Integrated directly into S06_Storage for modular hardware disk arrays

// Initialize a new software RAID array natively (Mirror, Striping, Parity)
void storage_init_raid_array(uint32_t raid_level, uint32_t* disk_ids, uint8_t disk_count);

// Reconstruct a degraded array asynchronously in the background via S03_Orchestrator
void storage_rebuild_raid(uint32_t array_id, uint32_t replacement_disk_id);

// Enforce modular disk quotas per user-capability token
void storage_set_quota(uint32_t user_id, uint64_t max_bytes);

#endif // SIGMA_RAID_MANAGER_H


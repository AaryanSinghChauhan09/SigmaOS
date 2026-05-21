#ifndef SIGMA_VIRT_H
#define SIGMA_VIRT_H

#include "sigma_kernel_types.h"

// Sovereign Type-0 Virtualization Header
// Provides definitions for hardware-backed microVM shards.

#ifdef __cplusplus
extern "C" {
#endif

// Represents a hardware-isolated secure container
typedef struct {
    sigma_u32 shard_id;
    sigma_u64 root_page_table;
    sigma_u64 ept_pointer; // Extended Page Table for Intel VT-x
    sigma_bool is_active;
} sigma_virt_shard_t;

void virt_bridge_init(void);
sigma_status virt_create_secure_shard(sigma_u32 shard_id);
void virt_enter_shard(sigma_u32 shard_id);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_VIRT_H

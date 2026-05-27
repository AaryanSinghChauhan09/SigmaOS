/**
 * @file sigma_domain_manager.h
 * @brief Roadmap Feature #1 — Qubes-style Domain Isolation Manager
 *
 * Isolates apps and driver workloads into strict memory domains.
 * Each domain gets its own page table root, IPC channel, and
 * capability bitmask.  Zero shared memory between domains unless
 * explicitly granted via the inter-domain firewall (Feature #2).
 */

#ifndef SIGMA_DOMAIN_MANAGER_H
#define SIGMA_DOMAIN_MANAGER_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ---- Domain limits ---- */
#define SIGMA_MAX_DOMAINS        64u
#define SIGMA_DOMAIN_NAME_LEN    32u

/* ---- Domain isolation levels ---- */
typedef enum {
    SIGMA_DOMAIN_STRICT   = 0,   /* No shared memory, no IPC unless whitelisted */
    SIGMA_DOMAIN_STANDARD = 1,   /* IPC permitted to approved peers             */
    SIGMA_DOMAIN_RELAXED  = 2    /* Full IPC, shared buffers allowed            */
} sigma_domain_level_t;

/* ---- Domain descriptor ---- */
typedef struct {
    sigma_u32           domain_id;
    char                name[SIGMA_DOMAIN_NAME_LEN];
    sigma_domain_level_t isolation;
    sigma_paddr_t       page_table_root;   /* CR3 value for this domain   */
    sigma_u64           mem_limit_bytes;
    sigma_u64           mem_used_bytes;
    sigma_u32           process_count;
    sigma_bool          active;
} sigma_domain_t;

/* ---- Inter-domain firewall rule (Feature #2) ---- */
typedef struct {
    sigma_u32  src_domain;
    sigma_u32  dst_domain;
    sigma_bool allow_ipc;
    sigma_bool allow_shared_mem;
    sigma_bool allow_network;
} sigma_domain_fw_rule_t;

/* ---- API ---- */
sigma_status domain_init(void);
sigma_u32    domain_create(const char* name, sigma_domain_level_t level,
                           sigma_u64 mem_limit);
sigma_status domain_destroy(sigma_u32 domain_id);
sigma_status domain_assign_process(sigma_u32 domain_id, sigma_u32 pid);
sigma_status domain_set_firewall(const sigma_domain_fw_rule_t* rule);
const sigma_domain_t* domain_get_info(sigma_u32 domain_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DOMAIN_MANAGER_H */

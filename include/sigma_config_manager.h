/**
 * @file sigma_config_manager.h
 * @brief Roadmap Features #41 & #42 — Declarative Config & Atomic Rollbacks
 *
 * Implements a NixOS-style declarative configuration manager for SigmaOS.
 * System state is determined by a cryptographically hashed configuration tree.
 * Applying a new config creates a ZFS-inspired atomic snapshot.
 */

#ifndef SIGMA_CONFIG_MANAGER_H
#define SIGMA_CONFIG_MANAGER_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ---- Config Node Types ---- */
typedef enum {
    SIGMA_CFG_ROOT     = 0,
    SIGMA_CFG_NETWORK  = 1,
    SIGMA_CFG_DAEMONS  = 2,
    SIGMA_CFG_HARDWARE = 3,
    SIGMA_CFG_SECURITY = 4
} sigma_cfg_type_t;

/* ---- Config Snapshot Descriptor ---- */
#define SIGMA_SNAPSHOT_HASH_LEN 64

typedef struct {
    sigma_u64 generation_id;
    sigma_u8  state_hash[SIGMA_SNAPSHOT_HASH_LEN];
    sigma_u64 timestamp;
    sigma_bool is_active;
} sigma_cfg_snapshot_t;

/* ---- API ---- */

/**
 * @brief Parses a declarative YAML/JSON configuration tree into kernel state.
 * (Feature #41)
 */
sigma_status sigma_config_apply(const void* config_data, sigma_u64 size);

/**
 * @brief Creates an atomic rollback snapshot of the current configuration.
 * (Feature #42)
 */
sigma_status sigma_config_snapshot_create(sigma_cfg_snapshot_t* out_snap);

/**
 * @brief Reverts the system to a previous generation ID without rebooting.
 * (Feature #53, #65)
 */
sigma_status sigma_config_rollback(sigma_u64 target_generation_id);

/**
 * @brief Lists all available rollback snapshots.
 */
sigma_status sigma_config_list_snapshots(sigma_cfg_snapshot_t* buffer, 
                                         sigma_u32 max_count, 
                                         sigma_u32* actual_count);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONFIG_MANAGER_H */

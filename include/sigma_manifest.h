/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SHARD MANIFEST (S-MANIFEST)
 * =========================================================================
 * Mission: Declarative, atomic lattice configuration and state persistence.
 * Inspired by NixOS / Declarative Infrastructure.
 * =========================================================================
 */

#ifndef SIGMA_MANIFEST_H
#define SIGMA_MANIFEST_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char shard_name[32];
    uint32_t shard_id;
    uint32_t state_flags; // 0x01: Enabled, 0x02: Critical, 0x04: Auto-Heal
    uint32_t memory_quota;
} sigma_shard_config_t;

/* --- Manifest Primitives --- */
void manifest_init(void);
void manifest_apply_state(const char* declarative_blob);
void manifest_rollback_atomic(void);
sigma_shard_config_t* manifest_get_config(uint32_t shard_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MANIFEST_H */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CROSS-DEVICE CONTINUITY (S-CONTINUITY)
 * =========================================================================
 * Mission: Seamless, cryptographically secure machine-state synchronization
 * across desktop, server, and embedded devices.
 * =========================================================================
 */

#ifndef SIGMA_CONTINUITY_H
#define SIGMA_CONTINUITY_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t sync_id;
    char device_signature[64];
    uint32_t state_hash;
    bool is_synced;
} sigma_continuity_state_t;

/* --- Continuity Primitives --- */
void continuity_init(void);
void continuity_push_state(uint32_t state_hash);
void continuity_pull_state(const char* device_signature);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONTINUITY_H */

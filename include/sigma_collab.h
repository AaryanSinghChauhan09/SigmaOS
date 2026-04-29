/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN COLLABORATIVE WORKSPACE (S-COLLAB)
 * =========================================================================
 * Mission: Real-time, multiplayer document and canvas collaboration
 * built natively into the OS, with zero-trust encrypted shared state.
 * =========================================================================
 */

#ifndef SIGMA_COLLAB_H
#define SIGMA_COLLAB_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- Collaborative Workspace Primitives --- */
void collab_init(void);
void collab_start_session(uint32_t resource_id);
void collab_broadcast_change(const void* delta, uint32_t delta_size);
void collab_apply_remote_change(const void* delta, uint32_t delta_size);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_COLLAB_H */

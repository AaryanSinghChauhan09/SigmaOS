/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIVE KERNEL PATCH (S-LIVEKERNEL)
 * =========================================================================
 * Mission: Apply critical security patches and shard updates to a running
 * kernel without requiring a system reboot — inspired by Livepatch.
 * =========================================================================
 */

#ifndef SIGMA_LIVEKERNEL_H
#define SIGMA_LIVEKERNEL_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Live Kernel Patch Primitives --- */
void livekernel_init(void);
bool livekernel_apply_patch(const void* patch_data, uint32_t patch_size);
void livekernel_verify_integrity(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LIVEKERNEL_H */

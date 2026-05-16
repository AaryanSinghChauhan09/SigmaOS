/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SENTINEL (S-SENTINEL)
 * =========================================================================
 * Mission: Zero-trust, fine-grained access control and capability mediation.
 * =========================================================================
 */

#ifndef SIGMA_SENTINEL_H
#define SIGMA_SENTINEL_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    CAP_NETWORK,
    CAP_STORAGE,
    CAP_SILICON_DIRECT,
    CAP_LATTICE_FORGE
} sigma_capability_t;

/* --- Sentinel Primitives --- */
void sentinel_init(void);
bool sentinel_check_capability(uint32_t shard_id, sigma_capability_t cap);
void sentinel_enforce_policy(const char* policy_blob);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SENTINEL_H */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MICRO-ORCHESTRATOR (S-MICRO)
 * =========================================================================
 * Mission: Isolated, microkernel-style service mediation and shard isolation.
 * =========================================================================
 */

#ifndef SIGMA_MICRO_H
#define SIGMA_MICRO_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    MICRO_USERLAND,
    MICRO_KERNEL_SHARD,
    MICRO_SILICON_DIRECT
} sigma_micro_context_t;

/* --- Micro-Orchestrator Primitives --- */
void micro_init(void);
bool micro_spawn_isolated_shard(uint32_t shard_id, sigma_micro_context_t context);
void micro_mediate_ipc(uint32_t source_id, uint32_t target_id, void* msg);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MICRO_H */

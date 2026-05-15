/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LATTICE ORCHESTRATOR
 * =========================================================================
 * Mission: Automated shard deployment and lattice configuration patterns.
 * Competitor parity: Terraform, Ansible, SaltStack.
 * =========================================================================
 */

#ifndef SIGMA_ORCHESTRATOR_H
#define SIGMA_ORCHESTRATOR_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Orchestration Primitives --- */
void      orchestrator_init(void);
void      orchestrator_apply_pattern(const char* name);
void      orchestrator_self_heal(void);
sigma_u64 orchestrator_get_heal_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ORCHESTRATOR_H */

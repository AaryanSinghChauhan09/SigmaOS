/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN PERSONA SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_PERSONA_SHARD_H
#define SOVEREIGN_PERSONA_SHARD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_persona_create (const char* name, const char* theme,
                                   sigma_u32 uid, sigma_u32 cap_mask);
sigma_err_t sigma_persona_switch (const char* name);
void        SovereignPersonaShard_Init (void);
void        SovereignPersona_Audit      (void);

#endif /* SOVEREIGN_PERSONA_SHARD_H */

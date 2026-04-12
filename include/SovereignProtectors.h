/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PROTECTORS HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_PROTECTORS_H
#define SOVEREIGN_PROTECTORS_H

#include "sigma_types.h"

void       sigma_protect_register_target (sigma_u64 addr);
sigma_bool sigma_protect_verify_jump     (sigma_u64 addr);
void       SovereignProtectors_Init      (void);
void       SovereignProtectors_Audit     (void);

#endif /* SOVEREIGN_PROTECTORS_H */

/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN AUTOMATED SYSTEMS HEADERS
 * =========================================================================
 */

#ifndef SOVEREIGN_AUTO_SYSTEMS_H
#define SOVEREIGN_AUTO_SYSTEMS_H

#include "sigma_types.h"

/* Auto-Clean */
void sigma_autoclean_volatile  (void);
void sigma_autoclean_legacy    (void);
void SovereignAutoCleanShard_Init(void);
void SovereignAutoClean_Audit  (void);

/* Auto-Performance */
void sigma_autoperf_boost      (void);
void sigma_autoperf_compact    (void);
void SovereignAutoPerfShard_Init(void);
void SovereignAutoPerf_Audit   (void);

#endif /* SOVEREIGN_AUTO_SYSTEMS_H */

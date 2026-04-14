/* Σ SIGMAOS: SOVEREIGN FORENSIC SHARD HEADER */
#ifndef SOVEREIGN_FORENSIC_SHARD_H
#define SOVEREIGN_FORENSIC_SHARD_H
#include "sigma_types.h"

void sigma_forensic_scrub    (sigma_uptr addr, sigma_size_t size);
void sigma_forensic_lockdown (void);
void SovereignForensicShard_Init (void);
void SovereignForensic_Audit     (void);

#endif

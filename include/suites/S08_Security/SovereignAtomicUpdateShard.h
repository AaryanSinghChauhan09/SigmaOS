/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ATOMIC UPDATE SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_ATOMIC_UPDATE_H
#define SOVEREIGN_ATOMIC_UPDATE_H

#include "sigma_types.h"

sigma_err_t sigma_rebuild_system        (const char* manifesto_path);
void        SovereignAtomicUpdate_Rollback (void);
void        SovereignAtomicUpdate_Audit    (void);
void        SovereignAtomicUpdate_Init     (void);

#endif /* SOVEREIGN_ATOMIC_UPDATE_H */

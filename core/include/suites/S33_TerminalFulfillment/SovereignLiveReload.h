/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN LIVE RELOAD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_LIVE_RELOAD_H
#define SOVEREIGN_LIVE_RELOAD_H

#include "sigma_types.h"

sigma_err_t sigma_reload_shard          (const char* name, void* new_addr);
void        SovereignLiveReload_Init    (void);
void        SovereignLiveReload_Audit   (void);

#endif /* SOVEREIGN_LIVE_RELOAD_H */

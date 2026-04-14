/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN WINDOW MANAGER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_WM_SHARD_H
#define SOVEREIGN_WM_SHARD_H

#include "sigma_types.h"

sigma_err_t sigma_wm_create_window (const char* title, sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h);
void        sigma_wm_composite     (void);
void        SovereignWMShard_Init  (void);
void        SovereignWM_Audit      (void);

#endif /* SOVEREIGN_WM_SHARD_H */

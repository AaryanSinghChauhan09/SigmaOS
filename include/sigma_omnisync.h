/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN OMNI-SYNC ENGINE (S-OMNISYNC)
 * =========================================================================
 * Mission: Unified, zero-configuration background data synchronization 
 * across the Sovereign Lattice using the Zero-Trust Network.
 * =========================================================================
 */

#ifndef SIGMA_OMNISYNC_H
#define SIGMA_OMNISYNC_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Omni-Sync Primitives --- */
void omnisync_init(void);
void omnisync_register_directory(const char* dir_path);
void omnisync_trigger_sync(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_OMNISYNC_H */

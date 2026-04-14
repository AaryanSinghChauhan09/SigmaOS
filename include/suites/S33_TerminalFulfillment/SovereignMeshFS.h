/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MESH FS HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_MESH_FS_H
#define SOVEREIGN_MESH_FS_H

#include "sigma_types.h"

sigma_err_t sigma_mesh_publish     (const char* data, sigma_u64 len);
void        sigma_mesh_sync        (void);
void        SovereignMeshFS_Init   (void);
void        SovereignMeshFS_Audit  (void);

#endif /* SOVEREIGN_MESH_FS_H */

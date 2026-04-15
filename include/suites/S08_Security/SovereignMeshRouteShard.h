/* S SIGMAOS: SOVEREIGN MESH SHARD HEADER */
#ifndef SOVEREIGN_MESH_ROUTE_SHARD_H
#define SOVEREIGN_MESH_ROUTE_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_mesh_connect (const char* ip, const char* pub_key);
void        SovereignMeshShard_Init (void);
void        SovereignMesh_Audit     (void);

#endif

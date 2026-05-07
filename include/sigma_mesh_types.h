#ifndef SIGMA_MESH_TYPES_H
#define SIGMA_MESH_TYPES_H

#include "core/sigma_types.h"

typedef struct {
    char node_id[32];
    sigma_u32 shard_count;
    sigma_u32 trust_score;
    bool is_verified;
} sigma_mesh_node_t;

#endif

#ifndef SOVEREIGN_VMA_H
#define SOVEREIGN_VMA_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef struct {
    sigma_u64 start;
    sigma_u64 end;
    sigma_u32 flags;
    char      name[32];
} SigmaVMA_t;

#define MAX_VMAS_PER_PROC 32

#endif

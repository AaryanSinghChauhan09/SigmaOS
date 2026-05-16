#ifndef SIGMA_SNAP_H
#define SIGMA_SNAP_H

#include "./sigma_kernel_types.h"

typedef struct {
    sigma_u32 id;
    sigma_u32 capacity;
    sigma_u32 x, y, w, h;
} sigma_snap_zone_t;

#endif

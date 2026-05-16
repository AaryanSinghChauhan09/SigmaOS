#ifndef SIGMA_SNAP_TYPES_H
#define SIGMA_SNAP_TYPES_H

#include "./core/sigma_types.h"

typedef sigma_u32 sigma_snap_zone_id_t;

typedef struct {
    sigma_snap_zone_id_t id;
    sigma_u32 x, y, w, h;
    sigma_u32 capacity;
} sigma_snap_zone_t;

#endif

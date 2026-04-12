#ifndef SOVEREIGN_APP_CONTAINER_H
#define SOVEREIGN_APP_CONTAINER_H

#include "../../../include/sigma_kernel.h"

#define MAX_APP_CONTAINERS 128

typedef struct {
    char app_id[64];
    int is_sandboxed;
    int has_network_access;
    int has_fs_access;
    char root_mount[128];
} SovereignAppContainer_t;

void app_container_init(void);
SovereignAppContainer_t* app_container_alloc(void);

#endif /* SOVEREIGN_APP_CONTAINER_H */

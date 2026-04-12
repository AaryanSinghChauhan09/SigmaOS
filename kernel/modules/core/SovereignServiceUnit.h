#ifndef SOVEREIGN_SERVICE_UNIT_H
#define SOVEREIGN_SERVICE_UNIT_H

#include "../../../include/sigma_kernel.h"

typedef struct {
    char service_name[64];
    int is_active;
    int restart_on_failure;
    char dependencies[3][64]; 
} SovereignServiceUnit_t;

void service_unit_init(void);
SovereignServiceUnit_t* service_unit_alloc(void);
SovereignServiceUnit_t* service_unit_find(const char* name);
int service_unit_get_count(void);

#endif /* SOVEREIGN_SERVICE_UNIT_H */

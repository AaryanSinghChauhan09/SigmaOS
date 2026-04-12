#include "SovereignServiceUnit.h"

static SovereignServiceUnit_t s_runtime_services[128];
static int s_total_services = 0;

void service_unit_init(void) {
    s_total_services = 0;
    sigma_memset(s_runtime_services, 0, sizeof(s_runtime_services));
}

SovereignServiceUnit_t* service_unit_alloc(void) {
    if (s_total_services >= 128) return SIGMA_NULL;
    return &s_runtime_services[s_total_services++];
}

SovereignServiceUnit_t* service_unit_find(const char* name) {
    for (int i = 0; i < s_total_services; i++) {
        /* using sigma_strcmp or sigma_compare */
        if (sigma_strcmp(s_runtime_services[i].service_name, name) == 0) {
            return &s_runtime_services[i];
        }
    }
    return SIGMA_NULL;
}

int service_unit_get_count(void) {
    return s_total_services;
}

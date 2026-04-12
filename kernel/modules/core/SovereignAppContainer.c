#include "SovereignAppContainer.h"

static SovereignAppContainer_t s_application_matrix[MAX_APP_CONTAINERS];
static int s_active_containers = 0;

void app_container_init(void) {
    s_active_containers = 0;
    sigma_memset(s_application_matrix, 0, sizeof(s_application_matrix));
}

SovereignAppContainer_t* app_container_alloc(void) {
    if (s_active_containers >= MAX_APP_CONTAINERS) return SIGMA_NULL;
    return &s_application_matrix[s_active_containers++];
}

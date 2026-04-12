#include "SovereignMACPolicy.h"

static SovereignMACPolicy_t s_active_policies[256];
static int s_num_policies = 0;

void mac_policy_init(void) {
    s_num_policies = 0;
}

sigma_err_t mac_add_policy(const char* entity, int level, int exec, int write, int read) {
    if (s_num_policies >= 256) return SIGMA_ENOSPC;
    
    sigma_strncpy(s_active_policies[s_num_policies].entity_name, entity, 64);
    s_active_policies[s_num_policies].security_context_level = level;
    s_active_policies[s_num_policies].can_execute = exec;
    s_active_policies[s_num_policies].can_write = write;
    s_active_policies[s_num_policies].can_read = read;
    s_num_policies++;
    
    sigma_print_info("Σ [MAC] Policy Ingested: Target [%s] L%d", entity, level);
    return SIGMA_OK;
}

SovereignMACPolicy_t* mac_find_policy(const char* entity) {
    for (int i = 0; i < s_num_policies; i++) {
        if (sigma_strcmp(s_active_policies[i].entity_name, entity) == 0) {
            return &s_active_policies[i];
        }
    }
    return SIGMA_NULL;
}

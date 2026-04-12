/**
 * Σ SIGMAOS ZENITH : Sovereign Mandatory Access Control (MAC) Shard (Modular v2.0)
 * 
 * Refactored into Policy and Enforcer components.
 */

#include "../../../include/sigma_kernel.h"
#include "SovereignMACPolicy.h"

#define MAC_MODE_ENFORCING 1
#define MAC_MODE_PERMISSIVE 0

static int s_current_mac_mode = MAC_MODE_ENFORCING;

/**
 * @brief Initialize the MAC subsystem
 */
void sigma_mac_init() {
    sigma_print_info("Σ [MAC] Initializing Sovereign Mandatory Access Control Engine...");
    mac_policy_init();
    s_current_mac_mode = MAC_MODE_ENFORCING;
}

/**
 * @brief Add a core security policy
 */
void sigma_mac_add_policy(const char* entity, int level, int exec, int write, int read) {
    mac_add_policy(entity, level, exec, write, read);
}

/**
 * @brief Check permission against active MAC policies
 */
int sigma_mac_check_permission(const char* entity, const char* action) {
    if (s_current_mac_mode == MAC_MODE_PERMISSIVE) {
        sigma_print_warn("Σ [MAC] Permissive Mode Active: Access Granted to %s for %s", entity, action);
        return 1;
    }
    
    SovereignMACPolicy_t* policy = mac_find_policy(entity);
    if (policy) {
        if (sigma_strcmp(action, "execute") == 0 && policy->can_execute) return 1;
        if (sigma_strcmp(action, "write") == 0 && policy->can_write) return 1;
        if (sigma_strcmp(action, "read") == 0 && policy->can_read) return 1;
        
        sigma_print_error("Σ [MAC] PERMISSION DENIED: Context policy violation for [%s] attempting [%s]", entity, action);
        return 0;
    }
    
    sigma_print_error("Σ [MAC] PERMISSION DENIED: No matching policy for [%s] attempting [%s]", entity, action);
    return 0;
}

void SovereignMAC_Register(void) {
    static SovereignModule_t s_mac_module = {
        .name = "SovereignMAC",
        .type = MODULE_TYPE_SECURITY,
        .Init = (sigma_err_t(*)(void))sigma_mac_init,
    };
    sigma_module_register(&s_mac_module);
}

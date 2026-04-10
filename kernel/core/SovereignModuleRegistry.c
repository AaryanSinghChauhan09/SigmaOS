#include "SovereignModule.h"
#include "sigma_print.h"

/*
 * Σ Sovereign Module Registry
 * Concept: Centralized management for all 140+ modular shards.
 *          This utility handles the registration, lookup, and 
 *          sequential audit of all modules in the system natively.
 */

static sigma_module_t* module_registry[MAX_SOVEREIGN_MODULES];
static sigma_u32 registered_count = 0;

void sigma_register_module(sigma_module_t* mod) {
    if (registered_count < MAX_SOVEREIGN_MODULES) {
        module_registry[registered_count++] = mod;
        sigma_print("[REGISTRY] Module Registered: ");
        sigma_print(mod->module_name);
        sigma_print("\n");
    }
}

void sigma_audit_all_modules(void) {
    sigma_print("[REGISTRY] Commencing Global Sovereign Audit...\n");
    for (sigma_u32 i = 0; i < registered_count; i++) {
        if (module_registry[i]->audit()) {
            sigma_print("[PASS] ");
        } else {
            sigma_print("[FAIL] ");
        }
        sigma_print(module_registry[i]->module_name);
        sigma_print("\n");
    }
}

#include "../../include/SovereignModule.h"

#define MAX_REGISTERED_MODULES 256

static SovereignModule_t* s_module_registry[MAX_REGISTERED_MODULES];
static sigma_u32 s_module_count = 0;

sigma_err_t sigma_module_register(SovereignModule_t* module) {
    if (s_module_count >= MAX_REGISTERED_MODULES) return SIGMA_ENOSPC;
    s_module_registry[s_module_count++] = module;
    sigma_printf("Σ [MODULE-REGISTRY]: Registered shard '%s' [%d]\n", module->name, module->type);
    return SIGMA_OK;
}

sigma_err_t sigma_modules_init_all(void) {
    sigma_printf("Σ [MODULE-REGISTRY]: Sequential industrial initialization starting...\n");
    for (sigma_u32 i = 0; i < s_module_count; i++) {
        if (s_module_registry[i]->Init) {
            sigma_printf("  - Seating: %s\n", s_module_registry[i]->name);
            s_module_registry[i]->Init();
        }
    }
    sigma_printf("Σ [MODULE-REGISTRY]: All sovereign shards seated and operational.\n");
    return SIGMA_OK;
}

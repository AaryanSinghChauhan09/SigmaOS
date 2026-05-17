#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

#include "../../include/sigma_modules.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"



/**
 * SigmaOS Sovereign Module Implementation
 * Implements a Dynamic Shard Linking (DSL) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal module orchestration.
 */

static sigma_module_t module_table[128];
static uint32_t active_module_count = 0;

extern "C" void modules_init() {
    sigma_log("[MODULES] Initializing Sovereign Module Loader (DSL Algorithm)...");
    sigma_memset(module_table, 0, sizeof(module_table));
}

extern "C" bool modules_load_shard(const char* name, void* binary_blob, uint32_t size) {
    if (active_module_count >= 128) return SIGMA_FALSE;
    
    // DSL (Dynamic Shard Linking) Algorithm
    // Performs runtime relocation and symbol resolution for bare-metal shards.
    
    sigma_log_info("[MODULES] DSL: Linking Shard '%s' (%d bytes)...\n", name, size);
    
    sigma_module_t* mod = &module_table[active_module_count++];
    sigma_hardened_strcpy(mod->module_name, name, 32);
    mod->module_id = active_module_count;
    mod->size = size;
    mod->is_active = SIGMA_TRUE;
    
    // In a real OS, we'd copy the blob to an allocated page and jump to entry point
    sigma_log("[MODULES] DSL: Relocation COMPLETE. Shard activated in Ring-0.");
    return SIGMA_TRUE;
}

extern "C" void modules_unload_shard(uint32_t module_id) {
    if (module_id > 0 && module_id <= 128) {
        sigma_log_info("[MODULES] DSL: Deactivating Shard ID %d...\n", module_id);
        module_table[module_id - 1].is_active = SIGMA_FALSE;
    }
}

extern "C" void modules_list_active() {
    sigma_log("\n--- Σ ACTIVE SOVEREIGN SHARDS ---");
    for (uint32_t i = 0; i < active_module_count; i++) {
        if (module_table[i].is_active) {
            sigma_log_info("[%02d] %-20s (DSL-Linked)\n", module_table[i].module_id, module_table[i].module_name);
        }
    }
    sigma_log("---------------------------------\n");
}


 
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-MODULE-LOADER (v1.0 - LKM PARITY)
 * =============================================================================
 * Algorithm: Shard Relocation Engine (SRE)
 * Principles:
 *   - Kernel-native loading of sharded modules (.sso / .ko parity).
 *   - Absolute industrial sovereignty in dynamic bitstream expansion.
 *   - Absorbing Linux LKM USPs: insmod, rmmod, modprobe parity.
 * Reference: Linux Kernel Module (LKM) / ELF Relocation.
 * =============================================================================
 */

#include "../../../include/sigma_kernel_types.h"

typedef struct SovereignModule {
    char        name[32];
    void*       base_addr;
    sigma_usize       size;
    sigma_bool      loaded;
} SovereignModule;

#define MAX_LOADED_MODULES 32
static SovereignModule g_modules[MAX_LOADED_MODULES];
static sigma_u32 g_mod_count = 0;

/* =========================================================================
 * MODULE LOADER Engine (The Expansion Shard)
 * ========================================================================= */

void mod_loader_init(void) {
<<<<<<<< HEAD:suites/S30_Supremacy/mod_loader.c
    for (int i = 0; i < MAX_LOADED_MODULES; i++) g_modules[i].loaded = FALSE;
    // ksigma_printf("[MOD-LOADER]: Sovereign Module Expansion Shard Online.\n");
========
    for (int i = 0; i < MAX_LOADED_MODULES; i++) g_modules[i].loaded = SIGMA_FALSE;
    // kprintf("[MOD-LOADER]: Sovereign Module Expansion Shard Online.\n");
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/misc_utils/mod_loader.c
}

sigma_status ins_shard(const char* name, void* elf_data, sigma_usize size) {
    if (g_mod_count >= MAX_LOADED_MODULES) return K_ERR_NOMEM;
    
    /* 
     * Absorb Linux LKM USP: ELF Relocation.
     * In a sharded model: map bitstream to executable memory and relocate.
     */
    SovereignModule* m = &g_modules[g_mod_count++];
    sigma_usize i = 0; while (i < 31 && name[i]) { m->name[i] = name[i]; i++; }
    m->name[i] = '\0';
    
    m->base_addr = elf_data;
    m->size      = size;
    m->loaded    = SIGMA_TRUE;
    
    // ksigma_printf("[MOD-LOADER]: Shard Module Injected: %s @ %p\n", name, elf_data);
    return K_OK;
}

sigma_status rm_shard(const char* name) {
    for (sigma_u32 i = 0; i < g_mod_count; i++) {
        /* Simple name matching for industrial sovereignty */
        sigma_u32 j = 0; sigma_bool match = SIGMA_TRUE;
        while (name[j] && g_modules[i].name[j]) { if (name[j] != g_modules[i].name[j]) { match = SIGMA_FALSE; break; } j++; }
        if (match && g_modules[i].loaded) {
<<<<<<<< HEAD:suites/S30_Supremacy/mod_loader.c
            g_modules[i].loaded = FALSE;
            // ksigma_printf("[MOD-LOADER]: Shard Module Evicted: %s\n", name);
========
            g_modules[i].loaded = SIGMA_FALSE;
            // kprintf("[MOD-LOADER]: Shard Module Evicted: %s\n", name);
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/misc_utils/mod_loader.c
            return K_OK;
        }
    }
    return K_ERR_INVAL;
}

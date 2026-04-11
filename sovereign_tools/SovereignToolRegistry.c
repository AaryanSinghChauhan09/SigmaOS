#include "../include/SovereignTool.h"
#include "../include/sigma_libc.h"
#include "../include/sigma_string.h"

static sovereign_tool_registry_t g_tool_registry;

void SovereignTool_InitRegistry(void) {
    sigma_memset(&g_tool_registry, 0, sizeof(sovereign_tool_registry_t));
    sigma_printf("Σ [TOOL]: Sovereign Tool Registry Online.\n");
}

sigma_err_t SovereignTool_Register(const char* name, const char* replacement, sigma_tool_absorb_fn absorb) {
    if (g_tool_registry.tool_count >= MAX_TOOLS) return SIGMA_ENOSPC;

    sovereign_tool_t* t = &g_tool_registry.tools[g_tool_registry.tool_count++];
    sigma_strncpy(t->name, name, TOOL_NAME_MAX);
    sigma_strncpy(t->replacement, replacement, 64);
    t->absorb = absorb;
    
    return SIGMA_OK;
}

void SovereignTool_Absorb(const char* name) {
    sigma_bool all = (sigma_streq(name, "all"));
    
    for (sigma_u32 i = 0; i < g_tool_registry.tool_count; i++) {
        if (all || sigma_streq(g_tool_registry.tools[i].name, name)) {
            sigma_printf("Σ [ABSORB]: Neutralizing and absorbing Tool: %s\n", g_tool_registry.tools[i].name);
            if (g_tool_registry.tools[i].absorb) {
                g_tool_registry.tools[i].absorb();
            }
            if (!all) return;
        }
    }
    if (!all) sigma_printf("[TOOL/ERR]: Unknown tool '%s'.\n", name);
}

void SovereignTool_ListAll(void) {
    sigma_printf("\nΣ SIGMAOS: GLOBAL TOOL ABSORPTION CATALOG\n");
    sigma_printf("--------------------------------------------------------------------------------\n");
    sigma_printf("%-16s | %-40s\n", "Tool", "SigmaOS Replacement Shard");
    sigma_printf("--------------------------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < g_tool_registry.tool_count; i++) {
        sovereign_tool_t* t = &g_tool_registry.tools[i];
        sigma_printf("%-16s | %-40s\n", t->name, t->replacement);
    }
    sigma_printf("--------------------------------------------------------------------------------\n");
}

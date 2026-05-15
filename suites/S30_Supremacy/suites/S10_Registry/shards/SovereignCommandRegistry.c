#include "../../../../../include/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "../../../../../include/SovereignCommand.h"
#include "../../../../../include/libc/sigma_libc.h"

static sovereign_command_registry_t g_cmd_registry;

static int sigma_strcmp_local(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

void SovereignCommand_Init(void) {
    sigma_sigma_memset(&g_cmd_registry, 0, sizeof(sovereign_command_registry_t));
    sigma_sigma_printf("S [CMD]: Sovereign Command Registry Online. Capacity: %d commands.\n", MAX_COMMANDS);
}

sigma_err_t SovereignCommand_Register(const char* name, const char* desc, sigma_cmd_handler_t handler) {
    if (g_cmd_registry.command_count >= MAX_COMMANDS) return SIGMA_ENOSPC;

    sovereign_command_t* cmd = &g_cmd_registry.commands[g_cmd_registry.command_count++];
    sigma_strncpy(cmd->name, name, CMD_NAME_MAX);
    sigma_strncpy(cmd->description, desc, CMD_DESC_MAX);
    cmd->handler = handler;
    
    return SIGMA_OK;
}

void SovereignCommand_Dispatch(int argc, char** argv) {
    if (argc < 2) return;
    const char* module = argv[1];

    for (sigma_u32 i = 0; i < g_cmd_registry.command_count; i++) {
        if (sigma_strcmp_local(g_cmd_registry.commands[i].name, module) == 0) {
            g_cmd_registry.commands[i].handler(argc, argv);
            return;
        }
    }
    sigma_sigma_printf("[OMNI-CLI] Unknown module: '%s'. Run 'sigma help' for commands.\n", module);
}

void SovereignCommand_ListAll(void) {
    sigma_sigma_printf("\nS SIGMAOS OMNI-CLI: MODULAR COMMAND CATALOG\n");
    sigma_sigma_printf("--------------------------------------------------\n");
    for (sigma_u32 i = 0; i < g_cmd_registry.command_count; i++) {
        sigma_sigma_printf("%-12s - %s\n", g_cmd_registry.commands[i].name, g_cmd_registry.commands[i].description);
    }
    sigma_sigma_printf("--------------------------------------------------\n");
}




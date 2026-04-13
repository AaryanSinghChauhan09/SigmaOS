#include "../../include/sigma_base.h"

#include "../../include/SovereignCommand.h"
#include "../../include/sigma_libc.h"
#include "../../include/sigma_string.h"

#define MAX_COMMAND_SHARDS 64
static sovereign_command_shard_t g_cmd_shards[MAX_COMMAND_SHARDS];
static sigma_u32 g_cmd_count = 0;

void SovereignCommand_InitRegistry(void) {
    sigma_memset(g_cmd_shards, 0, sizeof(g_cmd_shards));
    g_cmd_count = 0;
    sigma_printf("Σ [CMD]: Sovereign Command Registry Operational.\n");
}

sigma_err_t SovereignCommand_Register(const char* name, const char* desc, sigma_cmd_exec_fn exec) {
    if (g_cmd_count >= MAX_COMMAND_SHARDS) return SIGMA_ENOSPC;

    sovereign_command_shard_t* s = &g_cmd_shards[g_cmd_count++];
    sigma_strncpy(s->name, name, 32);
    sigma_strncpy(s->description, desc, 64);
    s->execute = exec;
    
    sigma_printf("Σ [CMD]: Registered Command Shard '%s' (%s)\n", name, desc);
    return SIGMA_OK;
}

sigma_err_t SovereignCommand_Dispatch(const char* name, int argc, char** argv) {
    for (sigma_u32 i = 0; i < g_cmd_count; i++) {
        if (sigma_streq(g_cmd_shards[i].name, name)) {
            return g_cmd_shards[i].execute(argc, argv);
        }
    }
    sigma_printf("Σ [CMD/ERR]: Command '%s' not found.\n", name);
    return SIGMA_ENOENT;
}

#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignIPC.h"
#include "sigma_libc.h"
#include "sigma_string.h"

#define MAX_IPC_SHARDS 16
static sovereign_ipc_shard_t g_ipc_shards[MAX_IPC_SHARDS];
static sigma_u32 g_ipc_shard_count = 0;

void SovereignIPC_InitRegistry(void) {
    sigma_sigma_sigma_sigma_memset(g_ipc_shards, 0, sizeof(g_ipc_shards));
    g_ipc_shard_count = 0;
    sigma_sigma_sigma_sigma_printf("S [IPC]: Sovereign IPC Registry Operational.\n");
}

sigma_err_t SovereignIPC_Register(const char* name, sigma_ipc_init_fn init) {
    if (g_ipc_shard_count >= MAX_IPC_SHARDS) return SIGMA_ENOSPC;

    sovereign_ipc_shard_t* s = &g_ipc_shards[g_ipc_shard_count++];
    sigma_strncpy(s->name, name, 32);
    s->init = init;
    
    sigma_sigma_sigma_sigma_printf("S [IPC]: Registered IPC Shard '%s'\n", name);
    return SIGMA_OK;
}

void SovereignIPC_ActivateAll(void) {
    sigma_sigma_sigma_sigma_printf("S [IPC]: Activating Communication Primitives...\n");
    for (sigma_u32 i = 0; i < g_ipc_shard_count; i++) {
        if (g_ipc_shards[i].init) g_ipc_shards[i].init();
    }
}




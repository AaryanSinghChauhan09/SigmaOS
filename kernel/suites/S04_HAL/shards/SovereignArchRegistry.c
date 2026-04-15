#include "sigma_base.h"

#include "SovereignArch.h"
#include "sigma_libc.h"
#include "sigma_string.h"

#define MAX_ARCH_SHARDS 8
static sovereign_arch_shard_t g_arch_shards[MAX_ARCH_SHARDS];
static sigma_u32 g_arch_shard_count = 0;

void SovereignArch_InitRegistry(void) {
    sigma_memset(g_arch_shards, 0, sizeof(g_arch_shards));
    g_arch_shard_count = 0;
    sigma_printf("Σ [ARCH]: Sovereign Architecture Registry Operational.\n");
}

sigma_err_t SovereignArch_Register(const char* name, sigma_arch_init_fn init, sigma_arch_halt_fn halt) {
    if (g_arch_shard_count >= MAX_ARCH_SHARDS) return SIGMA_ENOSPC;

    sovereign_arch_shard_t* s = &g_arch_shards[g_arch_shard_count++];
    sigma_strncpy(s->name, name, 32);
    s->init = init;
    s->halt = halt;
    
    sigma_printf("Σ [ARCH]: Registered Architecture Shard '%s'\n", name);
    return SIGMA_OK;
}

void SovereignArch_InitializeCPU(const char* arch_name) {
    for (sigma_u32 i = 0; i < g_arch_shard_count; i++) {
        if (sigma_streq(g_arch_shards[i].name, arch_name)) {
            sigma_printf("Σ [ARCH]: Bootstrapping CPU Personality: %s\n", arch_name);
            if (g_arch_shards[i].init) g_arch_shards[i].init();
            return;
        }
    }
    sigma_printf("Σ [ARCH/FATAL]: Unsupported Architecture '%s'\n", arch_name);
}




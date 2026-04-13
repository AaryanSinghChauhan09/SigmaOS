#include "../../include/sigma_base.h"

#include "../../include/SovereignSecurity.h"
#include "../../include/sigma_libc.h"
#include "../../include/sigma_string.h"

#define MAX_SECURITY_SHARDS 16
static sovereign_security_shard_t g_sec_shards[MAX_SECURITY_SHARDS];
static sigma_u32 g_sec_count = 0;

void SovereignSecurity_InitRegistry(void) {
    sigma_memset(g_sec_shards, 0, sizeof(g_sec_shards));
    g_sec_count = 0;
    sigma_printf("Σ [SEC]: Sovereign Security Registry Operational.\n");
}

sigma_err_t SovereignSecurity_Register(const char* name, sigma_sec_init_fn init) {
    if (g_sec_count >= MAX_SECURITY_SHARDS) return SIGMA_ENOSPC;

    sovereign_security_shard_t* s = &g_sec_shards[g_sec_count++];
    sigma_strncpy(s->name, name, 32);
    s->init = init;
    
    sigma_printf("Σ [SEC]: Registered Security Shard '%s'\n", name);
    return SIGMA_OK;
}

void SovereignSecurity_ActivateMatrix(void) {
    sigma_printf("Σ [SEC]: Hardening System Security Matrix...\n");
    for (sigma_u32 i = 0; i < g_sec_count; i++) {
        if (g_sec_shards[i].init) g_sec_shards[i].init();
    }
}

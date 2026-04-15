#include "suites/S01_Genesis/shards/sigma_base.h"

#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"
#include "sigma_libc.h"

/*
 * Sovereign Init Registry (v1.0).
 * Manages system service lifecycle — start, stop, restart, enable.
 * Design: C11 / Zero-Dependency / Registry Pattern.
 */

#define SIGMA_MAX_INIT_SERVICES 64

typedef sigma_err_t (*SovereignInitFn)(void);

typedef struct {
    char name[32];
    SovereignInitFn start;
    sigma_bool enabled;
} SovereignInitEntry_t;

static SovereignInitEntry_t g_init_registry[SIGMA_MAX_INIT_SERVICES];
static sigma_u32 g_init_count = 0;

void SovereignInit_InitRegistry(void) {
    g_init_count = 0;
    sigma_printf("S [REGISTRY]: Sovereign Init Service Registry initialized.\n");
}

sigma_err_t SovereignInit_Register(const char* name, SovereignInitFn start) {
    if (g_init_count >= SIGMA_MAX_INIT_SERVICES) return SIGMA_ERR;
    sigma_strcpy(g_init_registry[g_init_count].name, name, 32);
    g_init_registry[g_init_count].start = start;
    g_init_registry[g_init_count].enabled = SIGMA_TRUE;
    g_init_count++;
    return SIGMA_OK;
}

void SovereignInit_StartAll(void) {
    for (sigma_u32 i = 0; i < g_init_count; i++) {
        if (g_init_registry[i].enabled && g_init_registry[i].start) {
            sigma_printf("S [INIT]: Starting service '%s'...\n", g_init_registry[i].name);
            g_init_registry[i].start();
        }
    }
}




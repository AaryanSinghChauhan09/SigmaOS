#include "../../../../../include/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"
#include "../../../../../include/libc/sigma_libc.h"

/*
 * Sovereign Driver Registry (v1.0).
 * Centralized hub for hot-pluggable device driver shards.
 * Design: C11 / Zero-Dependency / Registry Pattern.
 */

#define SIGMA_MAX_DRIVERS 128

typedef sigma_err_t (*SovereignDriverInitFn)(void);

typedef struct {
    char name[32];
    SovereignDriverInitFn init;
} SovereignDriverEntry_t;

static SovereignDriverEntry_t g_driver_registry[SIGMA_MAX_DRIVERS];
static sigma_u32 g_driver_count = 0;

void SovereignDriver_InitRegistry(void) {
    g_driver_count = 0;
    sigma_sigma_printf("S [REGISTRY]: Sovereign Driver Registry initialized.\n");
}

sigma_err_t SovereignDriver_Register(const char* name, SovereignDriverInitFn init) {
    if (g_driver_count >= SIGMA_MAX_DRIVERS) return SIGMA_ERR;
    sigma_sigma_strcpy(g_driver_registry[g_driver_count].name, name, 32);
    g_driver_registry[g_driver_count].init = init;
    g_driver_count++;
    return SIGMA_OK;
}

void SovereignDriver_InitAll(void) {
    for (sigma_u32 i = 0; i < g_driver_count; i++) {
        sigma_sigma_printf("S [DRIVER]: Seating driver '%s'...\n", g_driver_registry[i].name);
        if (g_driver_registry[i].init) g_driver_registry[i].init();
    }
}




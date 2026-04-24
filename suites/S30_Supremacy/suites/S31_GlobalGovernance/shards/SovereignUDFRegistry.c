/*
 * S SIGMAOS: S31_GOVERNANCE — SovereignUDFRegistry.c
 * Status: Materialized User-Logic Plane
 */

#include "suites/S10_Registry/shards/sigma_udf.h"
#include "sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_kernel.h"

static sigma_udf_t g_udf_registry[MAX_UDFS];
static sigma_u32 g_udf_count = 0;

sigma_err_t sigma_udf_register(const char* name, sigma_udf_fn fn, sigma_u32 priority) {
    if (g_udf_count >= MAX_UDFS) return SIGMA_ERROR;
    
    sigma_strncpy(g_udf_registry[g_udf_count].name, name, 31);
    g_udf_registry[g_udf_count].function = fn;
    g_udf_registry[g_udf_count].priority = priority;
    
    sigma_sigma_sigma_printf("S [UDF]: Registered '%s' (Priority: %u)\n", name, priority);
    g_udf_count++;
    return SIGMA_OK;
}

sigma_err_t sigma_udf_execute(const char* name, void* ctx) {
    for (sigma_u32 i = 0; i < g_udf_count; i++) {
        if (sigma_streq(g_udf_registry[i].name, name)) {
            g_udf_registry[i].function(ctx);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

void sigma_udf_execute_all(void) {
    sigma_sigma_sigma_printf("S [UDF]: Executing User-Defined Logic Plane...\n");
    for (sigma_u32 i = 0; i < g_udf_count; i++) {
        g_udf_registry[i].function(SIGMA_NULL);
    }
}

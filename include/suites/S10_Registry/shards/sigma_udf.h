/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN USER-DEFINED FUNCTIONS (UDF) REGISTRY
 * =========================================================================
 * Mission: Allow customized, user-defined logic within the Sovereign Lattice.
 * =========================================================================
 */

#ifndef SIGMA_UDF_H
#define SIGMA_UDF_H

#include "sigma_types.h"

typedef void (*sigma_udf_fn)(void* ctx);

typedef struct {
    char name[32];
    sigma_udf_fn function;
    sigma_u32 priority;
} sigma_udf_t;

#define MAX_UDFS 64

/* Global UDF Interface */
sigma_err_t sigma_udf_register(const char* name, sigma_udf_fn fn, sigma_u32 priority);
sigma_err_t sigma_udf_execute(const char* name, void* ctx);
void        sigma_udf_execute_all(void);

#endif /* SIGMA_UDF_H */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN UDF REGISTRY (v1.0)
 * =========================================================================
 * Mission: Custom user-defined silicon functions.
 * =========================================================================
 */

#ifndef SOVEREIGN_UDF_H
#define SOVEREIGN_UDF_H

#include "sigma_types.h"

typedef sigma_err_t (*sigma_udf_fn)(void* args);

typedef struct {
    char name[64];
    sigma_udf_fn function;
    sigma_bool is_active;
} sovereign_udf_t;

void SovereignUDF_Register(const char* name, sigma_udf_fn fn);
sigma_err_t SovereignUDF_Execute(const char* name, void* args);

#endif

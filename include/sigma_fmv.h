/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN FUNCTION MULTI-VERSIONING (S-FMV)
 * =========================================================================
 * Mission: Extreme performance optimizations via automated function 
 * multi-versioning, selecting the fastest code path for the specific CPU.
 * Inspired by Clear Linux.
 * =========================================================================
 */

#ifndef SIGMA_FMV_H
#define SIGMA_FMV_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    CPU_FEATURE_SSE42,
    CPU_FEATURE_AVX2,
    CPU_FEATURE_AVX512,
    CPU_FEATURE_ARM_NEON
} sigma_cpu_feature_t;

/* --- FMV Primitives --- */
void fmv_init(void);
void* fmv_resolve_function(const char* func_name);
void fmv_register_variant(const char* func_name, sigma_cpu_feature_t required_feature, void* func_ptr);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_FMV_H */

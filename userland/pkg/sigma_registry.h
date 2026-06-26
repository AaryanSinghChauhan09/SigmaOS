/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PACKAGE REGISTRY PUBLIC HEADER
 * =========================================================================
 */
#pragma once

#include "../../../include/sigma_kernel_types.h"
#include "../pkg/sigma_pkg_format.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char       name[64];
    char       version[24];
    char       arch[16];
    char       description[128];
    sigma_u64  installed_size_kb;
    sigma_u32  dep_count;
    sigma_u8   state;    /* PkgState enum cast to u8 */
} sigma_pkg_info_t;

sigma_status sigma_registry_init(void);
sigma_status sigma_registry_register_builtin(const char* name, const char* version,
                                              const char* description);
sigma_status sigma_registry_install(const sigma_spkg_header_t* hdr);
sigma_status sigma_registry_remove(const char* name);
sigma_status sigma_registry_query(const char* name, sigma_pkg_info_t* out);
sigma_status sigma_registry_list(sigma_pkg_info_t* out, sigma_u32 max_count,
                                  sigma_u32* count_out);
sigma_status sigma_registry_verify_integrity(const char* name);

#ifdef __cplusplus
}
#endif

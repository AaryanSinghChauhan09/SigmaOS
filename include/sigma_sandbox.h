/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SANDBOX CONTAINER (S-SANDBOX)
 * =========================================================================
 * Mission: Isolated, zero-trust execution environments for all applications.
 * =========================================================================
 */

#ifndef SIGMA_SANDBOX_H
#define SIGMA_SANDBOX_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t container_id;
    bool network_access;
    bool fs_access;
    uint32_t memory_limit;
} sigma_sandbox_config_t;

/* --- Sandbox Primitives --- */
void sandbox_init(void);
uint32_t sandbox_create_container(const sigma_sandbox_config_t* config);
bool sandbox_execute(uint32_t container_id, const char* binary_path);
void sandbox_destroy_container(uint32_t container_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SANDBOX_H */

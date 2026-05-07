/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DYNAMIC MODULE LOADER (S-DYNMODULE)
 * =========================================================================
 * Mission: Allow hot-swapping of kernel components for flexibility and 
 * faster updates, matching the loadable kernel module (LKM) architecture.
 * =========================================================================
 */

#ifndef SIGMA_DYNMODULE_H
#define SIGMA_DYNMODULE_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t module_id;
    char module_name[64];
    void* entry_point;
    bool is_loaded;
} sigma_dynmodule_t;

/* --- Dynamic Module Primitives --- */
void dynmodule_init(void);
bool dynmodule_load(const char* module_path);
bool dynmodule_unload(uint32_t module_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DYNMODULE_H */

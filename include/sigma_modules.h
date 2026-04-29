/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MODULE LOADER (S-MODULE)
 * =========================================================================
 * Mission: Dynamic, hot-swappable shard loading and runtime extensibility.
 * =========================================================================
 */

#ifndef SIGMA_MODULES_H
#define SIGMA_MODULES_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char module_name[32];
    uint32_t module_id;
    uint32_t base_address;
    uint32_t size;
    bool is_active;
} sigma_module_t;

/* --- Module Primitives --- */
void modules_init(void);
bool modules_load_shard(const char* name, void* binary_blob, uint32_t size);
void modules_unload_shard(uint32_t module_id);
void modules_list_active(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MODULES_H */

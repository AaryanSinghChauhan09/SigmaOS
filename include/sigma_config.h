/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM CONFIGURATION (S-CONFIG)
 * =========================================================================
 * Mission: Atomic, shard-isolated configuration orchestration.
 * =========================================================================
 */

#ifndef SIGMA_CONFIG_H
#define SIGMA_CONFIG_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char key[64];
    char value[128];
    uint32_t shard_id;
    bool is_immutable;
} sigma_config_entry_t;

/* --- Config Primitives --- */
void config_init(void);
bool config_set(const char* key, const char* value, uint32_t shard_id);
const char* config_get(const char* key);
void config_atomic_swap(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONFIG_H */

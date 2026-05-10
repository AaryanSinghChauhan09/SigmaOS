/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM CONFIGURATION (S-CONFIG)
 * =========================================================================
 * Mission: Atomic, shard-isolated configuration orchestration.
 * =========================================================================
 */

#ifndef SIGMA_CONFIG_H
#define SIGMA_CONFIG_H

#include "core/sigma_types.h"

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

class SovereignConfigEngine {
public:
    static SovereignConfigEngine& getInstance() {
        static SovereignConfigEngine instance;
        return instance;
    }

    void init();
    bool set(const char* key, const char* value, uint32_t shard_id);
    const char* get(const char* key) const;
    void atomicSwap();

private:
    SovereignConfigEngine() : entry_count(0) {}

    sigma_config_entry_t lattice[256];
    uint32_t             entry_count;
};
#endif

#endif /* SIGMA_CONFIG_H */

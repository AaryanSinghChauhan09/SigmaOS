#ifndef SOVEREIGN_REGISTRY_H
#define SOVEREIGN_REGISTRY_H

#include "SigmaC11.h"

/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN REGISTRY SHARD (v20.0 - PURE C11)
 * ========================================================================= */

typedef enum sigma_registry_hive_t {
    SIGMA_HIVE_LOCAL_MACHINE,
    SIGMA_HIVE_CURRENT_USER,
    SIGMA_HIVE_SYSTEM,
    SIGMA_HIVE_SOFTWARE
} sigma_registry_hive_t;

typedef struct sigma_registry_t {
    sigma_obj_header_t hdr;   /* Shard header for introspection */
    sigma_u32 entries_sharded;
    void* internal_map;       /* Pointer to hashmap/tree structure */
} sigma_registry_t;

/* --- Core Registry API --- */

sigma_status_t sigma_registry_init(sigma_registry_t* reg);
sigma_status_t sigma_registry_set(sigma_registry_t* reg, sigma_registry_hive_t hive, const char* path, const char* key, const char* value);
const char*    sigma_registry_get(sigma_registry_t* reg, sigma_registry_hive_t hive, const char* path, const char* key);
sigma_status_t sigma_registry_delete(sigma_registry_t* reg, sigma_registry_hive_t hive, const char* path, const char* key);

/* --- Persistence --- */

sigma_status_t sigma_registry_load(sigma_registry_t* reg, const char* file_path);
sigma_status_t sigma_registry_save(sigma_registry_t* reg, const char* file_path);

#endif /* SOVEREIGN_REGISTRY_H */




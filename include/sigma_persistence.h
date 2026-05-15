#ifndef SIGMA_PERSISTENCE_H
#define SIGMA_PERSISTENCE_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t state_id;
    uint8_t state_hash[32];
    bool is_verified;
} sigma_persistent_state_t;

/* --- Persistence Primitives --- */
void persistence_init(void);
void persistence_save_state(uint32_t shard_id, const void* data, uint32_t size);
bool persistence_verify_integrity(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PERSISTENCE_H */

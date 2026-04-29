#ifndef SIGMA_USR_H
#define SIGMA_USR_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t shard_id;
    char name[64];
    bool is_active;
    uint32_t quantum_key; // For amnesic-protected discovery
} sigma_usr_entry_t;

/* --- USR Primitives --- */
void usr_init(void);
uint32_t usr_register_shard(const char* name, uint32_t quantum_key);
bool usr_activate_shard(uint32_t shard_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_USR_H */

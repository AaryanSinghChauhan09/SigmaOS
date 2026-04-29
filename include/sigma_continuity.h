#ifndef SIGMA_CONTINUITY_H
#define SIGMA_CONTINUITY_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Continuity Primitives --- */
void continuity_init(void);
void continuity_push_state(uint32_t state_hash);
void continuity_pull_state(const char* device_signature);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONTINUITY_H */

#ifndef SIGMA_CONTINUITY_H
#define SIGMA_CONTINUITY_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Continuity Primitives --- */
void      continuity_init(void);
void      continuity_push_state(sigma_u32 state_hash);
void      continuity_pull_state(const char* device_signature);
sigma_u32 continuity_get_push_count(void);
sigma_u32 continuity_get_pull_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONTINUITY_H */

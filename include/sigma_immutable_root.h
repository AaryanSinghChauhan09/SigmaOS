#ifndef SIGMA_IMMUTABLE_ROOT_H
#define SIGMA_IMMUTABLE_ROOT_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

void sigma_immutable_root_init(void);
sigma_bool sigma_immutable_root_is_locked(void);
sigma_bool sigma_immutable_root_allow_write(const char* path);
void sigma_immutable_root_set_locked(sigma_bool locked);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_IMMUTABLE_ROOT_H */

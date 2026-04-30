#ifndef SIGMA_RECOVER_H
#define SIGMA_RECOVER_H
#include "sigma_types.h"
#ifdef __cplusplus
extern "C" {
#endif
typedef struct { sigma_u32 error_code; sigma_u32 recovery_attempts; } sigma_recovery_state_t;
#ifdef __cplusplus
}
#endif
#endif

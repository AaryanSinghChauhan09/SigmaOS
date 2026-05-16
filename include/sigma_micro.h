#ifndef SIGMA_MICRO_H
#define SIGMA_MICRO_H
#include "./core/sigma_types.h"
#ifdef __cplusplus
extern "C" {
#endif
typedef struct { sigma_u32 pc; sigma_u32 sp; } sigma_micro_context_t;
#ifdef __cplusplus
}
#endif
#endif

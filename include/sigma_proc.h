#ifndef SIGMA_PROC_H
#define SIGMA_PROC_H
#include "sigma_types.h"
#ifdef __cplusplus
extern "C" {
#endif
#define SIGMA_PROC_READY 1
#define SIGMA_PROC_RUNNING 2
typedef struct { sigma_u32 pid; sigma_u32 state; } sigma_process_t;
#ifdef __cplusplus
}
#endif
#endif

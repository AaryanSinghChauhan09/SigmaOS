#ifndef SIGMA_PROC_H
#define SIGMA_PROC_H
#include "sigma_types.h"
#ifdef __cplusplus
extern "C" {
#endif
#define SIGMA_PROC_READY 1
#define SIGMA_PROC_RUNNING 2
typedef struct { sigma_u32 pid; sigma_u32 state; char name[32]; sigma_u32 priority; sigma_u32 cpu_time; sigma_u32 capability_mask; } sigma_process_t;

void proc_init(void);
sigma_u32 proc_spawn(const char* name, sigma_u32 priority);
void proc_yield(void);
sigma_process_t* proc_get_current(void);
sigma_u64 proc_get_switch_count(void);
#ifdef __cplusplus
}
#endif
#endif

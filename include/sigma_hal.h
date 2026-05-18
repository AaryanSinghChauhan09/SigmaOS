#ifndef INCLUDE_SIGMA_HAL_H
#define INCLUDE_SIGMA_HAL_H

#include "../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    void (*cpu_halt)(void);
    void (*timer_init)(void);
    void (*interrupt_init)(void);
    void (*mmu_map)(sigma_u64 va, sigma_u64 pa, sigma_u64 flags);
} hal_ops_t;

extern const hal_ops_t *hal_ops;

#ifdef __cplusplus
}
#endif

#endif // INCLUDE_SIGMA_HAL_H

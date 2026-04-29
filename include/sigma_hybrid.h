/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HYBRID KERNEL (S-HYBRID)
 * =========================================================================
 * Mission: Combines microkernel stability with monolithic performance, offering
 * modularity without sacrificing speed.
 * =========================================================================
 */

#ifndef SIGMA_HYBRID_H
#define SIGMA_HYBRID_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    HYBRID_MODE_MICRO,
    HYBRID_MODE_MONOLITHIC
} sigma_hybrid_mode_t;

/* --- Hybrid Kernel Primitives --- */
void hybrid_init(void);
void hybrid_set_mode(sigma_hybrid_mode_t mode);
bool hybrid_execute_syscall(uint32_t syscall_id, void* args);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HYBRID_H */

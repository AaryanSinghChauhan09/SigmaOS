/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-OOP (v1.0)
 * =============================================================================
 * Principles: Encapsulation, Polymorphism, and Abstraction in Pure C.
 * =============================================================================
 */
#ifndef SIGMA_OOP_H
#define SIGMA_OOP_H

#include "sigma_kernel_types.h"

typedef struct SovereignObject {
    u32     id;
    u32     type;
    void    (*init)(struct SovereignObject* self);
    void    (*destroy)(struct SovereignObject* self);
    char    (*to_string)(struct SovereignObject* self, char* buf, u32 len);
} SovereignObject;

/* Macro for simple inheritance */
#define SOVEREIGN_BASE \
    SovereignObject base

#endif

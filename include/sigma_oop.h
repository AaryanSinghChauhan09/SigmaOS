/*
 * =============================================================================
 * ÃŽÂ£ SIGMAOS KERNEL: SOVEREIGN-OOP (v1.0)
 * =============================================================================
 * Principles: Encapsulation, Polymorphism, and Abstraction in Pure C.
 * =============================================================================
 */
#ifndef SIGMA_OOP_H
#define SIGMA_OOP_H

#include "./sigma_kernel_types.h"

typedef struct SovereignObject {
    sigma_u32     id;
    sigma_u32     type;
    void    (*init)(struct SovereignObject* self);
    void    (*destroy)(struct SovereignObject* self);
    char    (*to_string)(struct SovereignObject* self, char* buf, sigma_u32 len);
} SovereignObject;

/* Macro for simple inheritance */
#define SOVEREIGN_BASE \
    SovereignObject base

#endif

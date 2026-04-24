/**
 * SigmaOS: Sovereign Capability System
 * Inspired by Genode OS Framework.
 * USP: Strict object-oriented security where processes only hold the capabilities they need.
 */

#ifndef SIGMA_CAPABILITIES_H
#define SIGMA_CAPABILITIES_H

#include "sigma_libc.h"

typedef uint64_t sigma_cap_t;

#define CAP_READ    (1ULL << 0)
#define CAP_WRITE   (1ULL << 1)
#define CAP_EXEC    (1ULL << 2)
#define CAP_IPC     (1ULL << 3)
#define CAP_HAL     (1ULL << 4)

typedef struct {
    sigma_cap_t permissions;
    uintptr_t target_object;
} sigma_capability_node_t;

// Capability Verification Bridge
static inline int sigma_check_capability(sigma_cap_t required, sigma_cap_t held) {
    return (held & required) == required;
}

#endif // SIGMA_CAPABILITIES_H

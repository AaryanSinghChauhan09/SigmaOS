#ifndef SIGMA_MODULE_H
#define SIGMA_MODULE_H

#include "sigma_types.h"

/*
 * S Sovereign Module Interface
 * Concept: Standardized entry point for all 140+ Sovereign Shards.
 *          Defines the contract for initialization, testing, and telemetry
 *          extraction, allowing the kernel to manage modules dynamically.
 */

typedef struct {
    const char* module_name;
    const char* target_distro;    /* e.g., "Linux", "BSD", "Windows" */
    sigma_u32 version;
    void (*init)(void);           /* Shard bootstrap */
    int (*audit)(void);           /* Self-test audit vector */
    void (*telemetry)(void);      /* Health data extraction */
} sigma_module_t;

/* Global Module Registry Constants */
#define MAX_SOVEREIGN_MODULES 512

#endif /* SIGMA_MODULE_H */

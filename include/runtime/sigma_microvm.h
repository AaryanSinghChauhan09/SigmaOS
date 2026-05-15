/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MICRO-VIRTUALIZATION (S-MICROVM)
 * =========================================================================
 * Mission: Security through extreme compartmentalization via lightweight 
 * hypervisor-level virtualization for individual tasks.
 * Inspired by Qubes OS.
 * =========================================================================
 */

#ifndef SIGMA_MICROVM_H
#define SIGMA_MICROVM_H

#include "include/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t vm_id;
    uint32_t memory_mb;
    bool has_network;
    bool is_running;
} sigma_microvm_config_t;

/* --- MicroVM Primitives --- */
void microvm_init(void);
uint32_t microvm_spawn(const sigma_microvm_config_t* config);
void microvm_terminate(uint32_t vm_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MICROVM_H */

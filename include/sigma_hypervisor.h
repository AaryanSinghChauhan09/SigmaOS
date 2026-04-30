/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SILICON-NATIVE HYPERVISOR (S-HYPER)
 * =========================================================================
 * Mission: Zero-latency hardware virtualization (Type-1).
 * Competitor parity: KVM, Xen, ESXi, Hyper-V.
 * ZERO-DEPENDENCY: Direct VT-x/AMD-V/ARM-VE orchestration.
 * =========================================================================
 */

#ifndef SIGMA_HYPERVISOR_H
#define SIGMA_HYPERVISOR_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- VM Lifecycle --- */
#define SIGMA_VM_STATE_IDLE      0x00u
#define SIGMA_VM_STATE_RUNNING   0x01u
#define SIGMA_VM_STATE_PAUSED    0x02u
#define SIGMA_VM_STATE_CRASHED   0x03u

typedef struct {
    sigma_u32 vm_id;
    sigma_u32 vcpus;
    sigma_u64 memory_mb;
    sigma_u32 state;
    sigma_u64 total_runtime_ms;
} sigma_vm_t;

typedef struct {
    sigma_u32 active_vms;
    sigma_u32 total_vcpus_allocated;
    sigma_u64 total_vm_memory_mb;
} sigma_hypervisor_state_t;

/* --- Hypervisor Primitives --- */
void      hyper_init(void);
sigma_u32 hyper_create_vm(sigma_u32 vcpus, sigma_u64 memory_mb);
void      hyper_start_vm(sigma_u32 vm_id);
void      hyper_stop_vm(sigma_u32 vm_id);
void      hyper_get_state(sigma_hypervisor_state_t* out_state);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HYPERVISOR_H */

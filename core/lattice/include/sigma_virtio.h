#ifndef SIGMA_VIRTIO_H
#define SIGMA_VIRTIO_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * SIGMA OS: VIRTUALIZATION SUITE (S11) - SIGMA HYPERVISOR INTERFACE
 * Direct hardware virtualization control replacing KVM/Docker entirely.
 * ========================================================================= */

#define SIGMA_VIRT_MAGIC 0x534D564D  // "SMVM"
#define MAX_VIRTUAL_MACHINES 8

typedef enum {
    VM_STATE_STOPPED = 0,
    VM_STATE_RUNNING,
    VM_STATE_PAUSED,
    VM_STATE_CRASHED
} sigma_vm_state_t;

typedef struct {
    uint32_t       vm_id;
    sigma_vm_state_t state;
    uint64_t       allocated_ram_bytes;
    uint8_t        vcpu_count;
    char           label[64];
    uint64_t       instruction_counter;
} __attribute__((packed)) sigma_vm_descriptor_t;

void sigma_virt_init(void);
int sigma_virt_create_vm(const char* label, uint64_t ram, uint8_t vcpus);
int sigma_virt_start_vm(uint32_t vm_id);
int sigma_virt_stop_vm(uint32_t vm_id);
sigma_vm_descriptor_t* sigma_virt_get_status(uint32_t vm_id);

#endif

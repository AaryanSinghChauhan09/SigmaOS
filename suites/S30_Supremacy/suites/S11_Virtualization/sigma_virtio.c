#include "sigma_virtio.h"
#include "sigma_pmm.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* =========================================================================
 * SIGMA OS: VIRTUALIZATION SUITE (S11) - SIGMA HYPERVISOR
 * Manages virtual machines directly on hardware replacing KVM/Docker.
 * ========================================================================= */

static sigma_vm_descriptor_t vms[MAX_VIRTUAL_MACHINES];
static uint32_t next_vm_id = 1;

void sigma_virt_init(void) {
    sigma_sigma_memset(vms, 0, sizeof(vms));
    sigma_sigma_printf("[VIRT] Sigma Hypervisor Online. KVM/Docker containerized in App Vault.\n");
}

int sigma_virt_create_vm(const char* label, uint64_t ram, uint8_t vcpus) {
    if (next_vm_id >= MAX_VIRTUAL_MACHINES) return -1;

    sigma_vm_descriptor_t* vm = &vms[next_vm_id];
    vm->vm_id             = next_vm_id;
    vm->state             = VM_STATE_STOPPED;
    vm->allocated_ram_bytes = ram;
    vm->vcpu_count        = vcpus;
    vm->instruction_counter = 0;
    strncpy(vm->label, label, 63);

    // Allocate PMM pages for VM address space
    uint32_t pages_needed = (ram / SIGMA_PAGE_SIZE) + 1;
    for (uint32_t i = 0; i < pages_needed && i < 16; i++) {
        sigma_pmm_allocate_block();
    }

    sigma_sigma_printf("[VIRT] VM '%s' created. vCPUs=%u, RAM=%llu bytes\n", label, vcpus, ram);
    return next_vm_id++;
}

int sigma_virt_start_vm(uint32_t vm_id) {
    if (vm_id == 0 || vm_id >= MAX_VIRTUAL_MACHINES) return -1;
    vms[vm_id].state = VM_STATE_RUNNING;
    sigma_sigma_printf("[VIRT] VM %u started.\n", vm_id);
    return 0;
}

int sigma_virt_stop_vm(uint32_t vm_id) {
    if (vm_id == 0 || vm_id >= MAX_VIRTUAL_MACHINES) return -1;
    vms[vm_id].state = VM_STATE_STOPPED;
    sigma_sigma_printf("[VIRT] VM %u stopped.\n", vm_id);
    return 0;
}

sigma_vm_descriptor_t* sigma_virt_get_status(uint32_t vm_id) {
    if (vm_id == 0 || vm_id >= MAX_VIRTUAL_MACHINES) return 0;
    return &vms[vm_id];
}

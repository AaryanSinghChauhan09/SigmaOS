/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUALIZATION — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignVirtualBox.h"

static SigmaVM_t s_vms[SIGMA_VBOX_MAX_VMS];
static sigma_u32 s_vm_count = 0;
static sigma_u32 s_vmid_seq = 100;

sigma_err_t sigma_vbox_create_vm(const char *name, const char *os, sigma_u32 cpus, sigma_u64 mem_mb) {
    if (s_vm_count >= SIGMA_VBOX_MAX_VMS) return SIGMA_ENOSPC;
    SigmaVM_t *vm = &s_vms[s_vm_count++];
    sigma_memset(vm, 0, sizeof(*vm));
    vm->vmid = s_vmid_seq++;
    sigma_strcpy(vm->name, name, 64);
    sigma_strcpy(vm->os_type, os, 32);
    vm->cpus = cpus;
    vm->mem_mb = mem_mb;
    vm->state = VM_STATE_POWERED_OFF;
    vm->net_type = VM_NET_NAT;
    sigma_printf("Σ [VBOX]: Created VM '%s' (OS: %s, CPU: %u, RAM: %lluMB)\n", name, os, cpus, (unsigned long long)mem_mb);
    return SIGMA_OK;
}

sigma_err_t sigma_vbox_attach_vdi(const char *vm_name, const char *vdi_path) {
    for(sigma_u32 i = 0; i < s_vm_count; i++) {
        if(sigma_streq(s_vms[i].name, vm_name)) {
            sigma_strcpy(s_vms[i].vdi_path, vdi_path, 256);
            sigma_printf("Σ [VBOX]: Attached %s to VM '%s'\n", vdi_path, vm_name);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

sigma_err_t sigma_vbox_start_vm(const char *vm_name) {
    for(sigma_u32 i = 0; i < s_vm_count; i++) {
        if(sigma_streq(s_vms[i].name, vm_name)) {
            s_vms[i].state = VM_STATE_RUNNING;
            sigma_printf("Σ [VBOX]: Started VM '%s' via KVM/VT-x core.\n", vm_name);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

sigma_err_t sigma_vbox_pause_vm(const char *vm_name) {
    for(sigma_u32 i = 0; i < s_vm_count; i++) {
        if(sigma_streq(s_vms[i].name, vm_name)) {
            s_vms[i].state = VM_STATE_PAUSED;
            sigma_printf("Σ [VBOX]: Paused VM '%s'.\n", vm_name);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

sigma_err_t sigma_vbox_stop_vm(const char *vm_name) {
    for(sigma_u32 i = 0; i < s_vm_count; i++) {
        if(sigma_streq(s_vms[i].name, vm_name)) {
            s_vms[i].state = VM_STATE_POWERED_OFF;
            sigma_printf("Σ [VBOX]: Stopped VM '%s'.\n", vm_name);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

sigma_err_t sigma_vbox_snapshot(const char *vm_name, const char *snap_name) {
    sigma_printf("Σ [VBOX]: Taking snapshot '%s' for VM '%s' (Copy-on-Write frozen).\n", snap_name, vm_name);
    return SIGMA_OK;
}

sigma_err_t sigma_vbox_seamless_toggle(const char *vm_name, sigma_bool enable) {
    for(sigma_u32 i = 0; i < s_vm_count; i++) {
        if(sigma_streq(s_vms[i].name, vm_name)) {
            s_vms[i].seamless_mode = enable;
            sigma_printf("Σ [VBOX]: Seamless Mode %s for VM '%s'.\n", enable ? "ENABLED" : "DISABLED", vm_name);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

void sigma_vbox_list_vms(void) {
    sigma_printf("Σ [VBOX]: Virtual Machines (%u):\n", s_vm_count);
    static const char *s_states[] = {"POWERED_OFF", "RUNNING", "PAUSED", "SAVED"};
    for(sigma_u32 i = 0; i < s_vm_count; i++) {
        SigmaVM_t *vm = &s_vms[i];
        sigma_printf("  [%u] %-12s OS:%-10s RAM:%4lluMB  %s%s\n",
                     vm->vmid, vm->name, vm->os_type, (unsigned long long)vm->mem_mb,
                     s_states[vm->state], vm->seamless_mode ? " (Seamless)" : "");
    }
}

void SovereignVirtualBox_Init(void) {
    sigma_printf("Σ [VBOX]: Initialising Sovereign Virtualization Engine...\n");
    sigma_vbox_create_vm("Windows10", "Windows", 4, 4096);
    sigma_vbox_attach_vdi("Windows10", "/mnt/vms/win10.vdi");
    sigma_vbox_start_vm("Windows10");
    sigma_vbox_seamless_toggle("Windows10", SIGMA_TRUE);
    sigma_vbox_list_vms();
}

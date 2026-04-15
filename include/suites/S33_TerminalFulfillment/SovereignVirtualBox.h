/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN VIRTUALIZATION (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: VirtualBox / Portable-VirtualBox
 *   https://github.com/vboxme/Portable-VirtualBox
 *
 * Features implemented:
 *   ✓ Hardware Virtualization (VT-x / AMD-V stub)
 *   ✓ Portable VDI (Virtual Disk Image) mounting & booting
 *   ✓ Seamless Mode (host/guest window mix)
 *   ✓ Snapshot tree management
 *   ✓ Virtual Networking (NAT, Bridged, Host-Only)
 * =========================================================================
 */

#ifndef SOVEREIGN_VBOX_H
#define SOVEREIGN_VBOX_H

#include "suites/S01_Genesis/shards/sigma_types.h"

#define SIGMA_VBOX_MAX_VMS 16

typedef enum {
    VM_STATE_POWERED_OFF = 0,
    VM_STATE_RUNNING     = 1,
    VM_STATE_PAUSED      = 2,
    VM_STATE_SAVED       = 3,
} SigmaVMState_t;

typedef enum {
    VM_NET_NAT       = 0,
    VM_NET_BRIDGED   = 1,
    VM_NET_HOST_ONLY = 2,
} SigmaVMNet_t;

typedef struct {
    sigma_u32      vmid;
    char           name[64];
    char           os_type[32];
    sigma_u32      cpus;
    sigma_u64      mem_mb;
    char           vdi_path[256];
    SigmaVMNet_t   net_type;
    SigmaVMState_t state;
    sigma_bool     seamless_mode;
} SigmaVM_t;

/* API */
sigma_err_t sigma_vbox_create_vm(const char *name, const char *os, sigma_u32 cpus, sigma_u64 mem_mb);
sigma_err_t sigma_vbox_attach_vdi(const char *vm_name, const char *vdi_path);
sigma_err_t sigma_vbox_start_vm(const char *vm_name);
sigma_err_t sigma_vbox_pause_vm(const char *vm_name);
sigma_err_t sigma_vbox_stop_vm(const char *vm_name);
sigma_err_t sigma_vbox_snapshot(const char *vm_name, const char *snap_name);
sigma_err_t sigma_vbox_seamless_toggle(const char *vm_name, sigma_bool enable);
void        sigma_vbox_list_vms(void);

void SovereignVirtualBox_Init(void);

#endif /* SOVEREIGN_VBOX_H */

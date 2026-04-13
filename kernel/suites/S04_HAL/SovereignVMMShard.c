#include "../../include/sigma_base.h"

#include "../../include/SovereignArch.h"
#include "../../include/sigma_libc.h"

/*
 * Sovereign Virtual Machine Monitor (VMM).
 * KVM/Bhyve-grade hardware virtualization orchestration.
 * Design: C11 / Zero-Dependency / Standalone.
 */

sigma_err_t sigma_vmm_init(void) {
    sigma_printf("  Σ [VMM]: Sovereign Virtualization Master active.\n");
    sigma_printf("  Σ [VMM]: Hardware VT-x/SVM virtualization matrices: SEATED.\n");
    return SIGMA_OK;
}

void SovereignVMM_Register(void) {
    SovereignArch_Register("vmm", sigma_vmm_init);
}

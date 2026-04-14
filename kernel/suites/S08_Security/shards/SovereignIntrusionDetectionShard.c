#include "../../include/sigma_base.h"

#include "../../include/SovereignSecurity.h"
#include "../../include/sigma_libc.h"

sigma_err_t sigma_ids_init(void) {
    sigma_printf("  Σ [IDS]: Sovereign Intrusion Detection System online.\n");
    sigma_printf("  Σ [IDS]: Hooking Syscall Dispatch for anomalous behavior detection...\n");
    sigma_printf("  Σ [IDS]: Real-time entropy audit: ACTIVE.\n");
    return SIGMA_OK;
}

void SovereignIDS_Register(void) {
    SovereignSecurity_Register("ids", sigma_ids_init);
}




#include "../../../../../include/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "../../../../../include/SovereignSecurity.h"
#include "../../../../../include/libc/sigma_libc.h"

sigma_err_t sigma_ids_init(void) {
    sigma_sigma_printf("  S [IDS]: Sovereign Intrusion Detection System online.\n");
    sigma_sigma_printf("  S [IDS]: Hooking Syscall Dispatch for anomalous behavior detection...\n");
    sigma_sigma_printf("  S [IDS]: Real-time entropy audit: ACTIVE.\n");
    return SIGMA_OK;
}

void SovereignIDS_Register(void) {
    SovereignSecurity_Register("ids", sigma_ids_init);
}




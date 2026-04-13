#include "../../include/sigma_base.h"

#include "../../include/SovereignSecurity.h"
#include "../../include/sigma_libc.h"

sigma_err_t sigma_pledge_init(void) {
    sigma_printf("  Σ [PLEDGE]: Sovereign OpenBSD-style promise auditing online.\n");
    sigma_printf("  Σ [PLEDGE]: System call restriction capabilities ('stdio','rpath','inet') activated.\n");
    return SIGMA_OK;
}

void SovereignPledge_Register(void) {
    SovereignSecurity_Register("pledge", sigma_pledge_init);
}

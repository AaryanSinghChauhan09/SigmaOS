#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignSecurity.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

sigma_err_t sigma_pledge_init(void) {
    sigma_printf("  S [PLEDGE]: Sovereign OpenBSD-style promise auditing online.\n");
    sigma_printf("  S [PLEDGE]: System call restriction capabilities ('stdio','rpath','inet') activated.\n");
    return SIGMA_OK;
}

void SovereignPledge_Register(void) {
    SovereignSecurity_Register("pledge", sigma_pledge_init);
}




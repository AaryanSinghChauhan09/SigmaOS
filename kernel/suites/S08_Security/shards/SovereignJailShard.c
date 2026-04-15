#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignSecurity.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

sigma_err_t sigma_jail_init(void) {
    sigma_printf("  S [JAIL]: Sovereign FreeBSD-style Isolation online.\n");
    sigma_printf("  S [JAIL]: Root chroot / VNET virtualization matrix established.\n");
    return SIGMA_OK;
}

void SovereignJail_Register(void) {
    SovereignSecurity_Register("jail", sigma_jail_init);
}




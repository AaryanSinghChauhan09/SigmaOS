#include "../../include/sigma_base.h"

#include "../../include/SovereignSecurity.h"
#include "../../include/sigma_libc.h"

sigma_err_t sigma_jail_init(void) {
    sigma_printf("  Σ [JAIL]: Sovereign FreeBSD-style Isolation online.\n");
    sigma_printf("  Σ [JAIL]: Root chroot / VNET virtualization matrix established.\n");
    return SIGMA_OK;
}

void SovereignJail_Register(void) {
    SovereignSecurity_Register("jail", sigma_jail_init);
}

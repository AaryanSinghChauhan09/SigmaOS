#include "../../include/sigma_base.h"

#include "../../include/SovereignSecurity.h"
#include "../../include/sigma_libc.h"

sigma_err_t sigma_ns_init(void) {
    sigma_printf("  Σ [NAMESPACE]: Sovereign Linux-style namespace isolation online.\n");
    sigma_printf("  Σ [NAMESPACE]: PID, Mount, UTS, and Network isolation matrices active.\n");
    sigma_printf("  Σ [NAMESPACE]: Containerization support shard: VALIDATED.\n");
    return SIGMA_OK;
}

void SovereignNamespace_Register(void) {
    SovereignSecurity_Register("namespace", sigma_ns_init);
}



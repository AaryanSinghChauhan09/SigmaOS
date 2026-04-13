#include "../../include/sigma_base.h"

#include "../../include/SovereignInit.h"
#include "../../include/sigma_libc.h"

void sigma_logger_init(void) {
    sigma_printf("  Σ [LOGGER]: Sovereign High-Performance Ring-0 Logger online.\n");
    sigma_printf("  Σ [LOGGER]: Journaling directed to /var/log/sigma_zenith.log.\n");
}

void SovereignLogger_Register(void) {
    SovereignInit_RegisterService("sigma-logger", "/sbin/sigma-logger", SIGMA_TRUE, sigma_logger_init);
}

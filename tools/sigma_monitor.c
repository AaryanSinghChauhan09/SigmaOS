#include "sigma_log.h"

/* Simple monitor that prints uptime and a stub for CPU usage */
void sigma_monitor(void) {
    // In a real kernel this would query timer and scheduler structures.
    sigma_log_info("System uptime: %llu ticks", (unsigned long long)0); // placeholder
    sigma_log_info("CPU usage: %d%%", 0); // placeholder
}

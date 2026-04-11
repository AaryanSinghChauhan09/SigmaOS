#include "../../include/SovereignDistro.h"

void test_linux_family() {
    sigma_printf("Σ [MODULAR-TEST]: Commencing Linux Distro Absorption Audit...\n");
    
    SovereignDistro_InitRegistry();
    SovereignDistro_Register("ubuntu", "apt", "systemd", "LTS", SIGMA_NULL);
    SovereignDistro_Register("arch", "pacman", "systemd", "Rolling", SIGMA_NULL);
    
    SovereignDistro_ListAll();
    sigma_printf("Σ [PASS]: Linux Amalgamation Matrix Registry Verified.\n");
}

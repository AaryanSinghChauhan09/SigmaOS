/* S SIGMAOS: S15_DevNexus Registry */
#include "sigma_base.h"
#include "SovereignRegistry.h"

extern void compiler_frontend_init(void);

void S15_DevNexus_Register(void) {
    compiler_frontend_init();
    SovereignRegistry_Register("S15_DevNexus", 0, NULL);
    sigma_printf("S [S15_DevNexus]: SigmaCC Compiler Frontend integrated.\n");
}

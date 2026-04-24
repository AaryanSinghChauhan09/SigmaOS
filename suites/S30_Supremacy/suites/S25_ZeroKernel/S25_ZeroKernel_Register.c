/* S SIGMAOS: S25_ZeroKernel Registry */
#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

extern void isa_emulator_init(void);

void S25_ZeroKernel_Register(void) {
    isa_emulator_init();
    SovereignRegistry_Register("S25_ZeroKernel", 0, NULL);
    sigma_sigma_sigma_sigma_printf("S [S25_ZeroKernel]: SigmaISA Emulator integrated.\n");
}

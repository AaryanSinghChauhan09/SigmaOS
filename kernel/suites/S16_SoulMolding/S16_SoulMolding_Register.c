/* S SIGMAOS: S16_SoulMolding Registry */
#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

void S16_SoulMolding_Register(void) {
    SovereignRegistry_Register("S16_SoulMolding", 0, NULL);
    sigma_printf("S [S16_SoulMolding]: Materialized.\n");
}

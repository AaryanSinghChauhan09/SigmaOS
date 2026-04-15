/* S SIGMAOS: S14_Transcendence Registry */
#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

void S14_Transcendence_Register(void) {
    SovereignRegistry_Register("S14_Transcendence", 0, NULL);
    sigma_printf("S [S14_Transcendence]: Materialized.\n");
}

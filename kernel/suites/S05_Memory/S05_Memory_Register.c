/* S SIGMAOS: S05_Memory Registry */
#include "sigma_base.h"
#include "SovereignRegistry.h"

void S05_Memory_Register(void) {
    SovereignRegistry_Register("S05_Memory", 0, NULL);
    sigma_printf("S [S05_Memory]: Materialized.\n");
}

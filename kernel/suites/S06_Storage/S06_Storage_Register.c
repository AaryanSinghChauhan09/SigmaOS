/* S SIGMAOS: S06_Storage Registry */
#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

void S06_Storage_Register(void) {
    SovereignRegistry_Register("S06_Storage", 0, NULL);
    sigma_printf("S [S06_Storage]: Materialized.\n");
}

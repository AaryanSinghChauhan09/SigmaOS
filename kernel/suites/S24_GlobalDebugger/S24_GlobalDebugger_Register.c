/* S SIGMAOS: S24_GlobalDebugger Registry */
#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

void S24_GlobalDebugger_Register(void) {
    SovereignRegistry_Register("S24_GlobalDebugger", 0, NULL);
    sigma_printf("S [S24_GlobalDebugger]: Materialized.\n");
}

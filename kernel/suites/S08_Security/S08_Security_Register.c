/* S SIGMAOS: S08_Security Registry */
#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

extern void crypto_engine_init(void);

void S08_Security_Register(void) {
    crypto_engine_init();
    SovereignRegistry_Register("S08_Security", 0, NULL);
    sigma_printf("S [S08_Security]: SHA-256 Crypto Engine integrated.\n");
}

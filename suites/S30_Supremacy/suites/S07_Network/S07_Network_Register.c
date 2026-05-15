#include "../../../../include/SovereignLibC.h"
/* S SIGMAOS: S07_Network Registry */
#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

extern void tcp_fsm_init(void);

void S07_Network_Register(void) {
    tcp_fsm_init();
    SovereignRegistry_Register("S07_Network", 0, SIGMA_NULL);
    sigma_sigma_printf("S [S07_Network]: TCP Protocol Grid integrated.\n");
}

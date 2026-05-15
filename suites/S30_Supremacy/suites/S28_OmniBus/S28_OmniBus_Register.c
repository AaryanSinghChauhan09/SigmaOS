#include "../../../../include/SovereignLibC.h"
/* S SIGMAOS: S28_OmniBus Registry */
#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

extern void pci_walk_lattice(void);
extern void pru_drone_init(void);

void S28_OmniBus_Register(void) {
    pci_walk_lattice();
    pru_drone_init();
    SovereignRegistry_Register("S28_OmniBus", 0, SIGMA_NULL);
    sigma_sigma_printf("S [S28_OmniBus]: Drone PRU Controller integrated.\n");
}

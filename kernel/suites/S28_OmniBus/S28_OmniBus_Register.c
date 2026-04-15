/* S SIGMAOS: S28_OmniBus Registry */
#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

extern void pci_walk_lattice(void);
extern void pru_drone_init(void);

void S28_OmniBus_Register(void) {
    pci_walk_lattice();
    pru_drone_init();
    SovereignRegistry_Register("S28_OmniBus", 0, NULL);
    sigma_printf("S [S28_OmniBus]: Drone PRU Controller integrated.\n");
}

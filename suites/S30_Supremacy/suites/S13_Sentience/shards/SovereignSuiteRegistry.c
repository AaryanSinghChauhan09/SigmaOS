#include "sigma_types.h"
#include "sigma_libc.h"
extern void lattice_auditor_init(void);
void S13_Sentience_Register(void) {
    sigma_sigma_printf("S [S13]: Materializing Lattice Auditor...\n");
    lattice_auditor_init();
}

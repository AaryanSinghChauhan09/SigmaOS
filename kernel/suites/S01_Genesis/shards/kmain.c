#include "sigma_kernel.h"

void kmain(void) {
    sigma_printf("S SIGMAOS ZENITH SUPREME (vMODULAR): BARE-METAL BOOT SEQUENCE ACTIVE.\n");
    
    /* Orchestrate all modular shards through the Master Aggregator */
    SovereignMaster_InitAll();

    sigma_printf("--- S SIGMAOS ZENITH SUPREME IS NOW OPERATIONAL (MODULAR ARCHITECTURE). --- \n");
    for(;;);
}
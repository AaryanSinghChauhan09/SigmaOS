#include "../../../include/sigma_kernel.h"

void kmain(void) {
    sigma_printf("Σ SIGMAOS ZENITH SUPREME (vMODULAR): BARE-METAL BOOT SEQUENCE ACTIVE.\n");
    
    /* Orchestrate all modular shards through the Master Aggregator */
    SovereignMaster_InitAll();

    /* 4. Run Sovereign Functional Test Suite */
    extern void SovereignFunctionalTest_Run(void);
    SovereignFunctionalTest_Run();
    
    sigma_printf("--- Σ SIGMAOS ZENITH SUPREME IS NOW OPERATIONAL (MODULAR ARCHITECTURE). --- \n");
    for(;;);
}

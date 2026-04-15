// =============================================================================
// SigmaOS — S25_ZeroKernel — SovereignKernelMain.c
// =============================================================================
#include "sigma_base.h"
#include "sigma_kernel.h"
#include "SovereignLatticeRegistry.h"

/**
 * @brief Terminal Entry Point for the Sovereign OS.
 */
void sigma_kernel_main(void) {
    sigma_printf("--- S SIGMAOS ZENITH SUPREME: SOVEREIGN REIGN INITIATED --- \n");
    
    // Materialize the 33-suite lattice
    SovereignMaster_InitAll();
    
    // Verify structural finality
    SovereignRegistry_Audit();
    
    sigma_printf("--- S SIGMAOS ZENITH SUPREME: SYSTEM SOVEREIGNTY VERIFIED --- \n");
}


#include "../../libc/SovereignLibC.h"
#include "../../SovereignOmniShard.h"

// External Kernel Core Subsystems
extern void SovereignMemory_Init();
extern void SovereignScheduler_Init();
extern void SovereignInterrupts_Init();

void kmain(void) {
    sigma_printf("Σ SIGMAOS ZENITH SUPREME (vROADMAP_1000): BARE-METAL BOOT SEQUENCE ACTIVE.\n");
    
    // Core Subsystem Initialization
    SovereignMemory_Init();
    SovereignInterrupts_Init();
    SovereignScheduler_Init();
    
    // Global Shard Matrix Activation
    SovereignZFS_Init();
    SovereignJail_Init();
    SovereignDTrace_Init();
    
    sigma_printf("--- Σ SIGMAOS ZENITH SUPREME IS NOW OPERATIONAL ON BARE-METAL SILICON. --- \n");
    
    // Fall into idle loop or Omni-Agent loop
    for(;;);
}

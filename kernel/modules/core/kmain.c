#include "../../libc/SovereignLibC.h"
#include "../../SovereignOmniShard.h"

extern void SovereignConsole_Init();
extern void SovereignMemory_Init();
extern void SovereignScheduler_Init();
extern void SovereignInterrupts_Init();

void kmain(void) {
    SovereignConsole_Init();
    sigma_printf("Σ SIGMAOS ZENITH SUPREME (vROADMAP_1000): BARE-METAL BOOT SEQUENCE ACTIVE.\n");
    
    SovereignMemory_Init();
    SovereignInterrupts_Init();
    SovereignScheduler_Init();
    
    sigma_printf("--- Σ SIGMAOS ZENITH SUPREME IS NOW OPERATIONAL ON BARE-METAL SILICON. --- \n");
    for(;;);
}



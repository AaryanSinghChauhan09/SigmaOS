/* 
 Σ SIGMAOS ZENITH: SOVEREIGN KERNEL ENTRY (v3000.0)
 Mission: Core Orchestration & User-Space Handoff.
*/

#include "SigmaSovereignInternal.h"

// Σ MISSIONS: FS, Memory, Shell
void sigma_fs_init();
void sigma_shell_exec(const char* input);

void kmain() {
    // 1. Clear Screen (Hardware-Direct)
    sigma_clear_screen();

    // 2. Initial Boot Logo
    sigma_print("Σ SIGMAOS ZENITH SUPREME (v3000.0)\n");
    sigma_print("Sovereign. Intellectual. Operational.\n\n");

    // 3. Initialize Shared Services
    sigma_fs_init();

    // 4. Mission Handoff (Interactive Shell)
    sigma_print("Σ [USER]: Handoff to Sovereign Shell...\n");
    sigma_shell_exec("whoami");
    
    // 5. Success Halt
    sigma_print("\nΣ MISSION COMPLETE. CPU IDLE.\n");
    while(1) { __asm__ volatile ("hlt"); }
}

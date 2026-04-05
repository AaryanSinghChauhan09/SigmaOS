/* 
 Σ SIGMAOS ZENITH: SOVEREIGN KERNEL ENTRY (v3000.0)
 Mission: Core Orchestration & User-Space Handoff.
*/

#include "sigma_kernel_types.h"
#include "../libc/SovereignLibC.h"

// Σ MISSIONS: FS, Memory, Shell, Healing
void sigma_fs_init();
void sigma_self_healing_init();
void SovereignIntelliViz_Init();
void SovereignDSMatrix_Init();
void sigma_shell_exec(const char* input);

// Σ STACK VERIFICATION (B6)
extern u32 stack_bottom;
void sigma_verify_stack() {
    if (stack_bottom != 0xDEADC0DE) {
        sigma_print("!!! KERNEL STACK OVERFLOW DETECTED [B6] !!!\n");
        while(1) { __asm__ volatile ("hlt"); }
    }
}

void kmain() {
    // 1. Clear Screen (Hardware-Direct)
    sigma_clear_screen();
    sigma_verify_stack();

    // 2. Initial Boot Logo
    sigma_print("Σ SIGMAOS ZENITH SUPREME (v3000.0)\n");
    sigma_print("Sovereign. Intellectual. Operational.\n\n");

    // 3. Initialize Shared Services
    sigma_fs_init();
    sigma_self_healing_init();         /* Milestone 27: Self-Healing Active */
    SovereignIntelliViz_Init();        /* Milestone 58: Visual Synthesis Shard */
    SovereignDSMatrix_Init();          /* Milestone 10000+: DS Master Matrix */

    // 4. Mission Handoff (Interactive Shell)
    sigma_print("Σ [USER]: Handoff to Sovereign Shell...\n");
    sigma_shell_exec("whoami");
    
    // 5. Success Halt
    sigma_print("\nΣ MISSION COMPLETE. CPU IDLE.\n");
    while(1) { __asm__ volatile ("hlt"); }
}

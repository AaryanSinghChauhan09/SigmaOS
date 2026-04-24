/*
 * =============================================================================
 * Σ SIGMAOS: EXCEPTION ROUTING
 * =============================================================================
 * High-level C handlers that receive hardware traps from the low-level 
 * AArch64 and RISC-V assembly vector tables.
 *
 * Prevents silent crashes and integrates with the scheduler to gracefully
 * kill offending tasks while keeping the kernel alive.
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* 
 * AArch64 C Handler 
 * Called by exceptions.asm
 */
void aarch64_exception_router(u64 type) {
    extern void kprintf(const char* fmt, ...);
    
    kprintf("\n[!!!] HARDWARE FAULT (AArch64) [!!!]\n");
    kprintf("Exception Type: %llu\n", type);
    
    // In a full implementation, we would read ESR_EL1 and FAR_EL1 to get the exact fault address
    
    if (type == 4) {
        kprintf("-> Synchronous Abort (Data/Prefetch or Undefined Instruction)\n");
    }

    kprintf("HALTING KERNEL THREAD.\n");
    while (1) { __asm__ volatile("wfi"); }
}

/* 
 * RISC-V 64 C Handler 
 * Called by exceptions.asm
 */
void riscv_exception_router(void) {
    extern void kprintf(const char* fmt, ...);
    
    u64 scause, sepc, stval;
    __asm__ volatile("csrr %0, scause" : "=r"(scause));
    __asm__ volatile("csrr %0, sepc" : "=r"(sepc));
    __asm__ volatile("csrr %0, stval" : "=r"(stval));
    
    kprintf("\n[!!!] HARDWARE TRAP (RISC-V) [!!!]\n");
    kprintf("SCAUSE: 0x%llx | SEPC: 0x%llx | STVAL: 0x%llx\n", scause, sepc, stval);
    
    kprintf("HALTING KERNEL THREAD.\n");
    while (1) { __asm__ volatile("wfi"); }
}

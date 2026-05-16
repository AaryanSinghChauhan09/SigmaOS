#include "../../../include/libc/SovereignLibC.h"
/*
 * =============================================================================
 * Σ SIGMAOS: EXCEPTION ROUTING
 * =============================================================================
 * High-level C handlers that receive hardware traps from the low-level 
 * AArch64 and RISC-V assembly vector tables.
 *
 * Prevents silent crashes by gracefully killing the offending task/shard
 * while keeping the core Microkernel alive and operational.
 * =============================================================================
 */

#include "../../../include/sigma_kernel_types.h"

// External scheduler hook to safely terminate a faulting process
extern void sched_kill_current_task(u32 fault_code);
extern u32 g_current_tid;

/* =========================================================================
 * AArch64 C Handler 
 * ========================================================================= */
void aarch64_exception_router(u64 type) {
    extern void ksigma_printf(const char* fmt, ...);
    
    u64 esr, far, elr;
    __asm__ volatile("mrs %0, esr_el1" : "=r"(esr));
    __asm__ volatile("mrs %0, far_el1" : "=r"(far));
    __asm__ volatile("mrs %0, elr_el1" : "=r"(elr));

    u32 ec = (esr >> 26) & 0x3F; // Exception Class

    ksigma_printf("\n[!!!] HARDWARE FAULT (AArch64) [!!!]\n");
    ksigma_printf("Type: %llu | Task ID: %u\n", type, g_current_tid);
    ksigma_printf("ESR: 0x%llx | FAR: 0x%llx | ELR: 0x%llx\n", esr, far, elr);
    
    switch (ec) {
        case 0x20: ksigma_printf("-> Instruction Abort (Lower EL)\n"); break;
        case 0x21: ksigma_printf("-> Instruction Abort (Same EL)\n"); break;
        case 0x24: ksigma_printf("-> Data Abort (Lower EL) - Potential DMA/Memory Violation\n"); break;
        case 0x25: ksigma_printf("-> Data Abort (Same EL) - Potential DMA/Memory Violation\n"); break;
        case 0x26: ksigma_printf("-> SP Alignment Fault\n"); break;
        case 0x00: ksigma_printf("-> Unknown Reason / Illegal Instruction\n"); break;
        default:   ksigma_printf("-> Exception Class: 0x%x\n", ec); break;
    }

    ksigma_printf("Terminating offending shard to preserve Lattice stability...\n");
    sched_kill_current_task(ec); // Graceful recovery
}

/* =========================================================================
 * RISC-V 64 C Handler 
 * ========================================================================= */
void riscv_exception_router(void) {
    extern void ksigma_printf(const char* fmt, ...);
    
    u64 scause, sepc, stval;
    __asm__ volatile("csrr %0, scause" : "=r"(scause));
    __asm__ volatile("csrr %0, sepc" : "=r"(sepc));
    __asm__ volatile("csrr %0, stval" : "=r"(stval));
    
    ksigma_printf("\n[!!!] HARDWARE TRAP (RISC-V) [!!!]\n");
    ksigma_printf("SCAUSE: 0x%llx | SEPC: 0x%llx | STVAL: 0x%llx\n", scause, sepc, stval);
    ksigma_printf("Task ID: %u\n", g_current_tid);
    
    u64 cause_code = scause & ~(1ULL << 63);
    if ((scause >> 63) == 0) { // Exception
        switch (cause_code) {
            case 0: ksigma_printf("-> Instruction Address Misaligned\n"); break;
            case 1: ksigma_printf("-> Instruction Access Fault\n"); break;
            case 2: ksigma_printf("-> Illegal Instruction\n"); break;
            case 5: ksigma_printf("-> Load Access Fault\n"); break;
            case 7: ksigma_printf("-> Store/AMO Access Fault (DMA Violation)\n"); break;
            case 12: ksigma_printf("-> Instruction Page Fault\n"); break;
            case 13: ksigma_printf("-> Load Page Fault\n"); break;
            case 15: ksigma_printf("-> Store/AMO Page Fault\n"); break;
            default: ksigma_printf("-> Exception Code: %llu\n", cause_code); break;
        }
    } else { // Interrupt
        ksigma_printf("-> Unhandled Interrupt Code: %llu\n", cause_code);
    }

    ksigma_printf("Terminating offending shard to preserve Lattice stability...\n");
    sched_kill_current_task((u32)cause_code); // Graceful recovery
}

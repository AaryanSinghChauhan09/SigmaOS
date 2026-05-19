#include "../../../include/core/sigma_kernel_types.h"
#include "../../../include/core/sigma_kernel_types.h"
#include "../../../include/core/sigma_kernel_types.h"
#include "../../../include/core/sigma_kernel_types.h"
#include "../../../include/core/sigma_kernel_types.h"
#include "../../../include/core/sigma_kernel_types.h"
/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSCALL DISPATCHER
 * =========================================================================
 * Modular C-based dispatch table matching POSIX/Zenith execution parameters.
 * =========================================================================
 */

#ifndef SIGMA_SYSCALL_DISPATCHER_H
#define SIGMA_SYSCALL_DISPATCHER_H



#ifdef __cplusplus
extern "C" {
#endif

#define K_OK 0
#define K_ERR_FAULT -14
#define K_ERR_INVAL -22


typedef struct {
    sigma_u64 r15;
    sigma_u64 r14;
    sigma_u64 r13;
    sigma_u64 r12;
    sigma_u64 rbp;
    sigma_u64 rbx;
    sigma_u64 rdx;
    sigma_u64 rcx;
    sigma_u64 rax; // Syscall Number / Return Value
} CpuRegisters;

// Syscall Numbers
#define SYS_SOVEREIGN_ALLOC       0x101
#define SYS_SOVEREIGN_FREE        0x102
#define SYS_SOVEREIGN_IPC_SEND    0x201
#define SYS_SOVEREIGN_IPC_RECV    0x202
#define SYS_SOVEREIGN_SCHED_YIELD 0x301
#define SYS_SOVEREIGN_SCHED_SET   0x302

// Entry point called by inline assembly interrupt handlers
void dispatch_syscall(CpuRegisters* regs);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_SYSCALL_DISPATCHER_H

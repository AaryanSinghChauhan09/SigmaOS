/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSCALL DISPATCHER
 * =========================================================================
 * Implementation of fast context-boundary system call routing.
 * =========================================================================
 */

#include "sigma_syscall_dispatcher.h"
#include "sigma_log.h"

// Simulated user space boundary check (e.g. 0x7FFFFFFFFFFF)
#define USER_SPACE_LIMIT 0x00007FFFFFFFFFFFULL

static int validate_user_pointer(sigma_u64 ptr, sigma_u64 size) {
    if (ptr == 0) return 0; // Reject SIGMA_NULL
    if (ptr + size > USER_SPACE_LIMIT) {
        return 0; // Potential kernel memory intrusion protection
    }
    return 1;
}

void dispatch_syscall(CpuRegisters* regs) {
    if (!regs) return;

    sigma_u64 syscall_num = regs->rax;

    switch (syscall_num) {
        case SYS_SOVEREIGN_ALLOC: {
            sigma_u64 size = regs->rbx;
            sigma_log_info("[SYSCALL] alloc requested: size = %llu bytes\n", size);
            // Simulate O(1) allocation address returning to RAX register
            regs->rax = 0x00007FFF10000000ULL; 
            break;
        }

        case SYS_SOVEREIGN_FREE: {
            sigma_u64 ptr = regs->rbx;
            sigma_log_info("[SYSCALL] free requested for address: 0x%llx\n", ptr);
            if (!validate_user_pointer(ptr, 1)) {
                sigma_log_error("[SYSCALL/ERR] Segfault avoided: invalid pointer free bypassed.\n");
                regs->rax = K_ERR_FAULT;
            } else {
                regs->rax = K_OK;
            }
            break;
        }

        case SYS_SOVEREIGN_IPC_SEND: {
            sigma_u64 queue_ptr = regs->rbx;
            sigma_u64 msg_ptr = regs->rcx;
            if (!validate_user_pointer(msg_ptr, 128)) {
                regs->rax = K_ERR_FAULT;
            } else {
                sigma_log_info("[SYSCALL] Zero-copy IPC Send submitted to target queue 0x%llx\n", queue_ptr);
                regs->rax = K_OK;
            }
            break;
        }

        case SYS_SOVEREIGN_SCHED_YIELD: {
            sigma_log_info("[SYSCALL] Yielding execution thread quantum.\n");
            regs->rax = K_OK;
            break;
        }

        default:
            sigma_log_error("[SYSCALL/ERR] Unrecognized or unaligned system call number: 0x%llx\n", syscall_num);
            regs->rax = K_ERR_INVAL;
            break;
    }
}

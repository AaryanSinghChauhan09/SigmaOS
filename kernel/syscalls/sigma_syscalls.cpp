/*
 * Σ SigmaOS — sigma_syscalls: Sovereign Syscall Layer
 * Zero-Dependency: No POSIX, no Linux ABI compatibility.
 * Defines the core architectural interface between userland and the microkernel.
 */

typedef unsigned long long u64;

/* 
 * Sovereign Syscall Numbers
 * Distinct from POSIX to prevent accidental ABI pollution.
 */
#define SIGMA_SYS_DEBUG_PRINT 0x01
#define SIGMA_SYS_ALLOC_MEM   0x02
#define SIGMA_SYS_FREE_MEM    0x03
#define SIGMA_SYS_SEND_MSG    0x04
#define SIGMA_SYS_RECV_MSG    0x05
#define SIGMA_SYS_HW_IO       0x06
#define SIGMA_SYS_SPAWN_TASK  0x07
#define SIGMA_SYS_YIELD       0x08

/* 
 * POSIX-Compatible Syscall Numbers (x86_64 reference)
 * Provides binary-level or API-level translation paths.
 */
#define POSIX_SYS_READ   0
#define POSIX_SYS_WRITE  1
#define POSIX_SYS_OPEN   2
#define POSIX_SYS_CLOSE  3
#define POSIX_SYS_FORK   57
#define POSIX_SYS_EXECVE 59
#define POSIX_SYS_EXIT   60

/* Forward declarations for scheduler functions */
extern "C" {
    unsigned int sched_add_task(unsigned int pid, int policy, unsigned char priority, unsigned long long deadline_us);
    void sched_yield(void);
}

/* Sovereign Syscall Entry Point (called via SYSCALL instruction or INT 0x80) */
extern "C" u64 sigma_syscall_handler(u64 syscall_num, u64 arg1, u64 arg2, u64 arg3) {
    // POSIX to Sovereign translation layer
    switch (syscall_num) {
        case POSIX_SYS_READ:
            syscall_num = SIGMA_SYS_RECV_MSG;
            break;
        case POSIX_SYS_WRITE:
            syscall_num = SIGMA_SYS_DEBUG_PRINT;
            break;
        case POSIX_SYS_FORK:
            syscall_num = SIGMA_SYS_SPAWN_TASK;
            break;
        case POSIX_SYS_EXIT:
            syscall_num = SIGMA_SYS_YIELD;
            break;
        default:
            break;
    }

    switch (syscall_num) {
        case SIGMA_SYS_DEBUG_PRINT:
            /* Route to VGA / Serial driver */
            return 0;
            
        case SIGMA_SYS_ALLOC_MEM:
            /* Route to sigma_allocator */
            return 0;
            
        case SIGMA_SYS_FREE_MEM:
            /* Route to sigma_allocator */
            return 0;
            
        case SIGMA_SYS_SEND_MSG:
            /* Route to IPC subsystem */
            return 0;
            
        case SIGMA_SYS_RECV_MSG:
            /* Route to IPC subsystem */
            return 0;
            
        case SIGMA_SYS_HW_IO:
            /* Privileged hardware I/O for drivers running in user-space */
            return 0;
            
        case SIGMA_SYS_SPAWN_TASK:
            /* arg1: pid, arg2: policy, arg3: priority */
            return sched_add_task((unsigned int)arg1, (int)arg2, (unsigned char)arg3, 0);
            
        case SIGMA_SYS_YIELD:
            sched_yield();
            return 0;
            
        default:
            return -1; /* ENOSYS equivalent */
    }
}

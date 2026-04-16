#ifndef SIGMA_SYSCALL_H
#define SIGMA_SYSCALL_H

#include <stdint.h>
#include <stddef.h>

/* =========================================================================
 * SIGMA OS: SOVEREIGN SYSCALL DISPATCHER (S25 - ZeroKernel)
 * Replaces the Linux syscall ABI. SigmaOS defines its own silicon call table.
 * No POSIX conformance required. Direct silicon mapping.
 * =========================================================================
 *
 * Linux has 450+ system calls accumulated over 35 years.
 * SigmaOS has a clean, minimal, high-throughput sovereign call table.
 */

// Sigma Syscall Numbers
#define SYS_SIGMA_EXIT        0x00
#define SYS_SIGMA_WRITE       0x01
#define SYS_SIGMA_READ        0x02
#define SYS_SIGMA_OPEN        0x03
#define SYS_SIGMA_CLOSE       0x04
#define SYS_SIGMA_ALLOC       0x05  // Maps to sigma_pmm_allocate_block()
#define SYS_SIGMA_FREE        0x06  // Maps to sigma_pmm_free_block()
#define SYS_SIGMA_SPAWN       0x07  // Maps to sigma_process_spawn()
#define SYS_SIGMA_YIELD       0x08  // Maps to sigma_scheduler_yield()
#define SYS_SIGMA_NET_SEND    0x09
#define SYS_SIGMA_NET_RECV    0x0A
#define SYS_SIGMA_AI_SPAWN    0x0B  // Maps to sigma_ai_allocate_swarm()
#define SYS_SIGMA_AI_FREE     0x0C  // Maps to sigma_ai_free_swarm()
#define SYS_SIGMA_VM_CREATE   0x0D  // Maps to sigma_virt_create_vm()
#define SYS_SIGMA_VM_START    0x0E
#define SYS_SIGMA_FS_READ     0x0F
#define SYS_SIGMA_FS_WRITE    0x10
#define SYS_SIGMA_SECURITY    0x11  // Maps to sigma_security_check()
#define SYS_SIGMA_TELEMETRY   0x12  // Streams hardware telemetry to callee

typedef struct {
    uint64_t syscall_num;
    uint64_t arg0;
    uint64_t arg1;
    uint64_t arg2;
    uint64_t arg3;
    uint64_t ret;    // Return value populated by dispatcher
} __attribute__((packed)) sigma_syscall_frame_t;

int64_t sigma_syscall_dispatch(sigma_syscall_frame_t* frame);

#endif

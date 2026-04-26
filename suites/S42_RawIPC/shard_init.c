#include "sigma_libc.h"

// SigmaOS Raw IPC (S-IPC-RAW)
// Philosophy: L4 Microkernel - Register-Based Zero-Copy IPC.
// USP: Eliminates memory mapping overhead for short messages by utilizing CPU registers.

void raw_ipc_send(uint32_t target_pid, uint64_t msg) {
    sigma_printf("[S-IPC-RAW] Sending Register-Msg %llx to PID %d...\n", msg, target_pid);
    
    #if defined(__x86_64__)
    __asm__ __volatile__ (
        "mov %0, %%rdi\n"
        "mov %1, %%rsi\n"
        "syscall\n"
        : : "r"((uint64_t)target_pid), "r"(msg) : "rdi", "rsi"
    );
    #endif
}

void shard_init() {
    sigma_shard_init();
    sigma_printf("[SHARD] Raw IPC active. Register-based communication enabled.\n");
}

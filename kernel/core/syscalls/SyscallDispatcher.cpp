/*
 * SigmaOS: Modular Syscall Dispatcher
 * Custom syscall table in C, inline assembly for fast context switches.
 */
#include "sigma_kernel_types.h"
namespace SigmaOS {
    extern "C" void syscall_dispatcher() {
        // Inline assembly for context switches
        #if defined(__x86_64__)
            __asm__ volatile (
                "push %rdi\n"
                "push %rsi\n"
                "call handle_syscall\n"
                "pop %rsi\n"
                "pop %rdi\n"
            );
        #elif defined(__aarch64__)
            // ARM64 fast context switch logic
        #endif
    }
}
 
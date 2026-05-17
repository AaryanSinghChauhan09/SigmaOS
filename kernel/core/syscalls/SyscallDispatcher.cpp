/*
 * SigmaOS: Modular Syscall Dispatcher
 * Custom syscall table in C, inline assembly for fast context switches.
 */
#include "../../../include/sigma_kernel_types.h"
namespace SigmaOS {
    extern "C" void syscall_dispatcher() {
        // Inline assembly for context switches
        #if defined(__x86_64__)
            __asm__ volatile (
                "push %rdi 
"
                "push %rsi 
"
                "call handle_syscall 
"
                "pop %rsi 
"
                "pop %rdi 
"
            );
        #elif defined(__aarch64__)
            // ARM64 fast context switch logic
        #endif
    }
}

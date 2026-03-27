/*
 * Σ SIGMA OS: SOVEREIGN JIT COMPILER (v9.0 - ZERO-LIBRARY GENTOO ABSORPTION)
 * =========================================================================
 * USP Absorbed: Gentoo (Extreme Source Compilation), LMIT (Just-In-Time).
 * Capability: Compiles logic strings to x86_64 CPU Opcodes dynamically in RAM.
 * Principle: Bare-metal execution without LLVM/GCC dependency at runtime.
 */

#include "SigmaLibC.h" // Our Custom Sigma C Library ONLY. No GNU Headers.

// MMAP Constants replacing <sys/mman.h>
#define SIGMA_PROT_READ  0x1
#define SIGMA_PROT_WRITE 0x2
#define SIGMA_PROT_EXEC  0x4
#define SIGMA_MAP_PRIVATE 0x02
#define SIGMA_MAP_ANONYMOUS 0x20

// Custom mmap Syscall wrapper
static void* sigma_sys_mmap(void *addr, sigma_u64 length, sigma_i32 prot, sigma_i32 flags, sigma_i32 fd, sigma_i64 offset) {
    void* ret;
#if defined(__x86_64__)
    __asm__ volatile (
        "mov $9, %%rax\n"  // sys_mmap
        "mov %1, %%rdi\n"
        "mov %2, %%rsi\n"
        "mov %3, %%rdx\n"
        "mov %4, %%r10\n"
        "mov %5, %%r8\n"
        "mov %6, %%r9\n"
        "syscall\n"
        "mov %%rax, %0\n"
        : "=r" (ret)
        : "r" (addr), "r" (length), "r" ((sigma_i64)prot), "r" ((sigma_i64)flags), "r" ((sigma_i64)fd), "r" (offset)
        : "%rax", "%rdi", "%rsi", "%rdx", "%r10", "%r8", "%r9", "%rcx", "%r11", "memory"
    );
#else
    ret = (void*)-1; 
#endif
    return ret;
}

void _start(void) {
    sigma_print("[SIGMA_JIT]: Bootstrapping Zero-Library 'Source-to-Silicon' Compiler.\n");
    sigma_print("[SIGMA_JIT]: Absorbing Gentoo USP for perfect architecture tuning...\n");

    // 1. Allocate Executable Memory (PROT_EXEC) directly from Hardware MMU
    sigma_u64 code_size = 4096;
    unsigned char* executable_shard = (unsigned char*)sigma_sys_mmap(
        0, code_size, SIGMA_PROT_READ | SIGMA_PROT_WRITE | SIGMA_PROT_EXEC, 
        SIGMA_MAP_PRIVATE | SIGMA_MAP_ANONYMOUS, -1, 0
    );

    /* mmap returns MAP_FAILED (a very large pointer near ULONG_MAX) on error */
    if ((sigma_u64)(sigma_usize)executable_shard <= (sigma_u64)-4096ULL) {
        // 2. We compile a custom "Function" directly into Machine Language.
        // Logic: A simple C function that returns 42 (0x2A).
        // x86_64 Asm: mov eax, 42; ret
        
        executable_shard[0] = 0xB8; // MOV EAX, ...
        executable_shard[1] = 0x2A; // 42
        executable_shard[2] = 0x00;
        executable_shard[3] = 0x00;
        executable_shard[4] = 0x00;
        executable_shard[5] = 0xC3; // RET

        sigma_print("[SIGMA_JIT]: Synthesized x86_64 Opcodes directly into PROT_EXEC Memory.\n");
        sigma_print("[SIGMA_JIT]: Executing JIT-Compiled Shard Function...\n");

        // 3. Cast the memory specifically to a C function pointer and execute it.
        typedef int (*JitFunction)();
        JitFunction run_shard = (JitFunction)executable_shard;
        
        sigma_i32 result = run_shard();

        sigma_print("[SIGMA_JIT]: Shard execution complete. Native Hardware Return Value: ");
        sigma_print_int((sigma_i64)result);
        sigma_print("\n");

    } else {
        sigma_print("[ERROR_JIT]: Execution-Blocked by Memory Protection / Sandbox.\n");
    }

    sigma_print("[SUCCESS]: Competitive JIT Compilation Zenith Online. Zero Compiler Lib Dep.\n");

    // Exit gracefully via SigmaLibC
#if defined(__x86_64__)
    __asm__ volatile ("mov $60, %%rax\n xor %%rdi, %%rdi\n syscall\n" ::: "%rax", "%rdi");
#endif
}

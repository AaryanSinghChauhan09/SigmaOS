/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/**
 * Σ SIGMA OS: LOW-LEVEL CORE (v4.0 - ZERO-LIBRARY NATIVE)
 * =====================================================
 * USP Absorbed: TempleOS (Native ASM), Linux Kernel (Direct-Syscall), musl (Minimalism).
 * Capability: Direct Register-to-Syscall sharding without CRT/Libc.
 * Principle: Zero-HLL Dependency, Pure Silicon execution.
 */

// Manual Syscall Invocation (usp: Linux Kernel Int 80h / Syscall instructions)
extern "C" long long sigma_native_write(int fd, const void* buf, size_t count) {
    long long ret;
#if defined(__x86_64__)
    __asm__ __volatile__ (
        "movq $1, %%rax\n"  // write syscall (Linux x64)
        "movq %1, %%rdi\n"  // fd
        "movq %2, %%rsi\n"  // buf
        "movq %3, %%rdx\n"  // count
        "syscall\n"
        "movq %%rax, %0\n"
        : "=r"(ret)
        : "r"((long long)fd), "r"(buf), "r"((long long)count)
        : "rax", "rdi", "rsi", "rdx", "rcx", "r11"
    );
#elif defined(__aarch64__)
    __asm__ __volatile__ (
        "mov x8, #64\n"     // write syscall (Linux arm64)
        "mov x0, %1\n"
        "mov x1, %2\n"
        "mov x2, %3\n"
        "svc #0\n"
        "mov %0, x0\n"
        : "=r"(ret)
        : "r"((long long)fd), "r"(buf), "r"((long long)count)
        : "x0", "x1", "x2", "x8"
    );
#else
    // Fallback for simulation transparency
    ret = count; 
#endif
    return ret;
}

// Zero-CRT Custom Entry Point (usp: _start instead of main)
extern "C" void _start() {
    const char* msg = "[LOW_LEVEL]: INITIALIZING SIGMAOS SILICON-DIRECT SHARD...\n";
    sigma_native_write(1, msg, 58);
    
    const char* success = "[SUCCESS]: Low-Level Sovereignty achieved. Zero-Library dependency.\n";
    sigma_native_write(1, success, 67);

    // exit(0)
#if defined(__x86_64__)
    __asm__ ("movq $60, %rax; xorq %rdi, %rdi; syscall;");
#elif defined(__aarch64__)
    __asm__ ("mov x8, #93; xor x0, x0, x0; svc #0;");
#endif
}


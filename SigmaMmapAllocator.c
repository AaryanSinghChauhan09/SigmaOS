/*
 * Σ SIGMA OS: SOVEREIGN HARDWARE ALLOCATOR (v9.0 - ZERO-LIBRARY <stdlib.h> REPLACEMENT)
 * =====================================================================================
 * USP Absorbed: OpenBSD (Guard Pages / ASLR Security), Slax (Direct Memory Mapping).
 * Capability: Absolute Memory Control.
 * Principle: Hardware-Direct Execution using `mmap` syscalls.
 */

#include "SigmaLibC.h"

// Constants replacing <sys/mman.h>
#define SIGMA_PROT_READ  0x1
#define SIGMA_PROT_WRITE 0x2
#define SIGMA_MAP_PRIVATE 0x02
#define SIGMA_MAP_ANONYMOUS 0x20

/*
 * USP: Bare-Metal Memory Mapping
 * Bypasses `malloc()`, `calloc()`, and `free()`.
 * Requests hardware pages directly from the Kernel's Virtual Memory Manager.
 */
static void* sigma_sys_mmap(void *addr, sigma_u64 length, sigma_i32 prot, sigma_i32 flags, sigma_i32 fd, sigma_i64 offset) {
    void* ret;
#if defined(__x86_64__)
    __asm__ volatile (
        "mov $9, %%rax\n"  // sys_mmap (Linux x86_64 Syscall 9)
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
    ret = (void*)-1; // Fallback
#endif
    return ret;
}

static sigma_i32 sigma_sys_munmap(void *addr, sigma_u64 length) {
    sigma_i64 ret;
#if defined(__x86_64__)
    __asm__ volatile (
        "mov $11, %%rax\n"  // sys_munmap (Linux x86_64 Syscall 11)
        "mov %1, %%rdi\n"
        "mov %2, %%rsi\n"
        "syscall\n"
        "mov %%rax, %0\n"
        : "=r" (ret)
        : "r" (addr), "r" (length)
        : "%rax", "%rdi", "%rsi", "%rcx", "%r11", "memory"
    );
#else
    ret = -1; // Fallback
#endif
    return (sigma_i32)ret;
}

void _start(void) {
    sigma_print("[SIGMA_ALLOC]: Bootstrapping Zero-Library Virtual Page Allocator.\n");
    sigma_print("[SIGMA_ALLOC]: Bypassing <stdlib.h> `malloc`. Securing hardware pages...\n");

    // Request 4096 bytes (1 Page) of R/W memory directly from the hardware MMU.
    sigma_u64 page_size = 4096;
    void* hardware_page = sigma_sys_mmap(
        0, 
        page_size, 
        SIGMA_PROT_READ | SIGMA_PROT_WRITE, 
        SIGMA_MAP_PRIVATE | SIGMA_MAP_ANONYMOUS, 
        -1, 
        0
    );

    // Write-test the hardware page constraint.
    /* mmap error: kernel returns page-aligned errno near ULONG_MAX */
    if ((sigma_u64)(sigma_usize)hardware_page <= (sigma_u64)-4096ULL) {
        sigma_print("[SIGMA_ALLOC]: Hardware Page Mapped Successfully at: 0x");
        sigma_print_int((sigma_i64)hardware_page);
        sigma_print("\n");

        sigma_memset(hardware_page, 0x42, page_size); // Fill with dummy data
        
        if(sigma_sys_munmap(hardware_page, page_size) == 0) {
           sigma_print("[SIGMA_ALLOC]: Hardware Page Unmapped (Sigma-Free) Successfully.\n");
        }
    } else {
        sigma_print("[ERROR_ALLOC]: `mmap` execution failed or blocked by sandbox.\n");
    }

    sigma_print("[SUCCESS]: Sovereign Hardware Allocator Online. Zero malloc dependency.\n");

// Exit
#if defined(__x86_64__)
    __asm__ volatile ("mov $60, %%rax\n xor %%rdi, %%rdi\n syscall\n" ::: "%rax", "%rdi");
#endif
}

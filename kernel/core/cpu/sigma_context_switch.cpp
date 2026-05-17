/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SILICON-DIRECT CONTEXT SWITCHING
 * =========================================================================
 * Freestanding inline assembly context switches for x86_64, ARM64, RISC-V.
 * =========================================================================
 */

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace CPU {

struct TaskContext {
    sigma_u64 rsp;
    sigma_u64 rip;
    sigma_u64 rflags;
};

extern "C" {

void switch_context_x86_64(TaskContext* old_ctx, TaskContext* new_ctx) {
#if defined(__x86_64__) || defined(_M_X64)
    // Inline Assembly: Save old context registers, reload new context
    __asm__ __volatile__(
        "pushfq\n\t"
        "pushq %%rax\n\t"
        "pushq %%rbx\n\t"
        "pushq %%rcx\n\t"
        "pushq %%rdx\n\t"
        "pushq %%rsi\n\t"
        "pushq %%rdi\n\t"
        "pushq %%rbp\n\t"
        "movq %%rsp, %0\n\t"  // Save old RSP to old_ctx->rsp
        "movq %1, %%rsp\n\t"  // Load new RSP from new_ctx->rsp
        "popq %%rbp\n\t"
        "popq %%rdi\n\t"
        "popq %%rsi\n\t"
        "popq %%rdx\n\t"
        "popq %%rcx\n\t"
        "popq %%rbx\n\t"
        "popq %%rax\n\t"
        "popfq\n\t"
        : "=m"(old_ctx->rsp)
        : "m"(new_ctx->rsp)
        : "memory"
    );
#else
    // Portability Simulation mode for IDE compatibility
    sigma_log_info("[CPU/x86_64] Simulated physical assembly context transition.\n");
#endif
}

void switch_context_arm64(TaskContext* old_ctx, TaskContext* new_ctx) {
#if defined(__aarch64__)
    __asm__ __volatile__(
        "stp x19, x20, [sp, #-16]!\n\t"
        "stp x21, x22, [sp, #-16]!\n\t"
        "mov %0, sp\n\t"      // Save old SP to old_ctx->rsp
        "mov sp, %1\n\t"      // Load new SP from new_ctx->rsp
        "ldp x21, x22, [sp], #16\n\t"
        "ldp x19, x20, [sp], #16\n\t"
        : "=r"(old_ctx->rsp)
        : "r"(new_ctx->rsp)
        : "memory"
    );
#else
    sigma_log_info("[CPU/ARM64] Simulated physical assembly context transition.\n");
#endif
}

void switch_context_riscv(TaskContext* old_ctx, TaskContext* new_ctx) {
#if defined(__riscv)
    __asm__ __volatile__(
        "sd sp, 0(%0)\n\t"    // Save old SP to old_ctx->rsp
        "ld sp, 0(%1)\n\t"    // Load new SP from new_ctx->rsp
        :
        : "r"(old_ctx), "r"(new_ctx)
        : "memory"
    );
#else
    sigma_log_info("[CPU/RISC-V] Simulated physical assembly context transition.\n");
#endif
}

} // extern "C"

} // namespace CPU
} // namespace SigmaOS

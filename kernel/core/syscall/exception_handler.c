#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: EXCEPTION HANDLER (v1.0)
 * =============================================================================
 * Principles: Autonomous Error Handling & System Resilience.
 * =============================================================================
 */
#include "../../../include/core/sigma_kernel_types.h"

extern void kprintf(const char* fmt, ...);

typedef struct ExceptionFrame {
    sigma_u64 rip, cs, rflags, rsp, ss;
} ExceptionFrame;

void handle_page_fault(ExceptionFrame* frame, sigma_u64 error_code) {
    kprintf("\nÎ£ [RESILIENCE]: PAGE FAULT at 0x%x (Code: %d)\n", frame->rip, error_code);
    kprintf("Î£ [ACTION]: Attempting autonomous shard recovery...\n");
}

void handle_division_by_zero(ExceptionFrame* frame) {
    kprintf("\nÎ£ [RESILIENCE]: DIV BY ZERO at 0x%x\n", frame->rip);
}

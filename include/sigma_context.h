/*
 * =========================================================================
 * Σ SIGMAOS: CPU CONTEXT & ISR FRAME DEFINITIONS (Phase 16)
 * =========================================================================
 * C-visible structures matching the assembly layouts in:
 *   kernel/arch/x86_64/isr_stubs.asm
 *   kernel/arch/x86_64/context_switch.asm
 *   kernel/arch/x86_64/gdt_flush.asm
 *
 * These structs MUST stay in sync with the assembly offsets.
 * =========================================================================
 */

#ifndef SIGMA_CONTEXT_H
#define SIGMA_CONTEXT_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* -------------------------------------------------------------------------
 * ISR Stack Frame
 * -------------------------------------------------------------------------
 * Pushed by isr_stubs.asm (isr_common_stub) before calling
 * sigma_isr_dispatch(). Matches the push order exactly.
 * ------------------------------------------------------------------------- */
typedef struct __attribute__((packed)) {
    /* Pushed by isr_common_stub (reverse order of push) */
    sigma_u64 r15;
    sigma_u64 r14;
    sigma_u64 r13;
    sigma_u64 r12;
    sigma_u64 r11;
    sigma_u64 r10;
    sigma_u64 r9;
    sigma_u64 r8;
    sigma_u64 rbp;
    sigma_u64 rdi;
    sigma_u64 rsi;
    sigma_u64 rdx;
    sigma_u64 rcx;
    sigma_u64 rbx;
    sigma_u64 rax;

    /* Pushed by ISR stub macro */
    sigma_u64 int_no;      /* Interrupt number (0–255) */
    sigma_u64 err_code;    /* Error code (real or dummy 0) */

    /* Pushed by CPU on interrupt */
    sigma_u64 rip;
    sigma_u64 cs;
    sigma_u64 rflags;
    sigma_u64 rsp;         /* User RSP (only if privilege level change) */
    sigma_u64 ss;
} sigma_isr_frame_t;


/* -------------------------------------------------------------------------
 * CPU Context (for context switching)
 * -------------------------------------------------------------------------
 * Layout MUST match context_switch.asm offsets exactly:
 *   offset 0:   rsp
 *   offset 8:   rbp
 *   offset 16:  rbx
 *   offset 24:  r12
 *   offset 32:  r13
 *   offset 40:  r14
 *   offset 48:  r15
 *   offset 56:  rip
 *   offset 64:  rflags
 *   offset 72:  fxsave_area[512]  (16-byte aligned)
 * ------------------------------------------------------------------------- */
typedef struct __attribute__((aligned(16))) {
    sigma_u64 rsp;
    sigma_u64 rbp;
    sigma_u64 rbx;
    sigma_u64 r12;
    sigma_u64 r13;
    sigma_u64 r14;
    sigma_u64 r15;
    sigma_u64 rip;
    sigma_u64 rflags;
    sigma_u8  fxsave_area[512];   /* FPU/SSE state, must be 16-byte aligned */
} sigma_cpu_context_t;


/* -------------------------------------------------------------------------
 * GDT / IDT Pointer Structs
 * ------------------------------------------------------------------------- */
typedef struct __attribute__((packed)) {
    sigma_u16 limit;       /* Size of table minus 1 */
    sigma_u64 base;        /* Linear address of table */
} sigma_gdtr_t;

typedef sigma_gdtr_t sigma_idtr_t;  /* Same layout for IDTR */

/* TSS (Task State Segment) — x86_64 minimal */
typedef struct __attribute__((packed)) {
    sigma_u32 reserved0;
    sigma_u64 rsp0;        /* Ring 0 stack pointer */
    sigma_u64 rsp1;        /* Ring 1 stack pointer (unused in SigmaOS) */
    sigma_u64 rsp2;        /* Ring 2 stack pointer (unused) */
    sigma_u64 reserved1;
    sigma_u64 ist1;        /* Interrupt Stack Table entry 1 (Double Fault) */
    sigma_u64 ist2;        /* IST entry 2 (NMI) */
    sigma_u64 ist3;        /* IST entry 3 (Machine Check) */
    sigma_u64 ist4;
    sigma_u64 ist5;
    sigma_u64 ist6;
    sigma_u64 ist7;
    sigma_u64 reserved2;
    sigma_u16 reserved3;
    sigma_u16 iopb_offset; /* I/O Permission Bitmap offset */
} sigma_tss_t;


/* -------------------------------------------------------------------------
 * Assembly Function Declarations
 * -------------------------------------------------------------------------
 * These are implemented in kernel/arch/x86_64/*.asm
 * ------------------------------------------------------------------------- */

/* Context switch: saves old, restores new, returns into new task */
void sigma_context_switch(sigma_cpu_context_t* old_ctx,
                          sigma_cpu_context_t* new_ctx);

/* Initialize a fresh context for a task that hasn't run yet */
void sigma_context_init(sigma_cpu_context_t* ctx,
                        void (*entry_point)(void),
                        void* stack_top);

/* Load GDT and reload all segment registers */
void sigma_gdt_load(sigma_gdtr_t* gdtr);

/* Load TSS selector into TR register */
void sigma_tss_load(sigma_u16 selector);

/* Load IDT register */
void sigma_idt_load(sigma_idtr_t* idtr);

/* ISR dispatch — called by isr_stubs.asm, implemented in C */
void sigma_isr_dispatch(sigma_isr_frame_t* frame);

/* ISR table: 256 function pointers, defined in isr_stubs.asm */
extern void (*sigma_isr_table[256])(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CONTEXT_H */

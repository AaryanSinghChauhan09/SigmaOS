// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_percpu.cpp — Per-CPU data structures (GDT, IDT, TSS, run queue)
//
// Each CPU core gets its own:
//   • GDT  — segment descriptors (64-bit, same layout but separate copy)
//   • IDT  — interrupt descriptor table (shared entries, separate pointer)
//   • TSS  — Task State Segment (holds IST stacks for NMI/DF/MCE)
//   • Run queue — lock-free FIFO (sigma_sched_sovereign.cpp supplies the impl)
//   • Emergency stacks (IST1–IST3)
//
// Inspired by: Linux arch/x86/include/asm/percpu.h, per_cpu_init()

#include "sigma_percpu.h"
#include "sigma_smp.h"
#include <stdint.h>
#include <string.h>

#define PERCPU_STACK_SIZE   (16 * 1024)   // 16KB per IST stack
#define PERCPU_GDT_ENTRIES  8

// ── GDT descriptor format (64-bit) ───────────────────────────────────────────

struct gdt_entry {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t  base_mid;
    uint8_t  access;
    uint8_t  flags_limit_high;
    uint8_t  base_high;
} __attribute__((packed));

struct gdtr {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed));

// ── TSS (64-bit) ──────────────────────────────────────────────────────────────

struct tss64 {
    uint32_t reserved0;
    uint64_t rsp0;       // ring-0 stack on syscall/interrupt
    uint64_t rsp1;
    uint64_t rsp2;
    uint64_t reserved1;
    uint64_t ist[7];     // IST1–IST7 (interrupt stack table)
    uint64_t reserved2;
    uint16_t reserved3;
    uint16_t iopb_offset;
} __attribute__((packed));

// ── Per-CPU storage ───────────────────────────────────────────────────────────

struct per_cpu_data {
    struct gdt_entry gdt[PERCPU_GDT_ENTRIES];
    struct tss64     tss;
    uint8_t          ist1_stack[PERCPU_STACK_SIZE];  // NMI
    uint8_t          ist2_stack[PERCPU_STACK_SIZE];  // #DF double-fault
    uint8_t          ist3_stack[PERCPU_STACK_SIZE];  // #MCE
    uint8_t          syscall_stack[PERCPU_STACK_SIZE];
    struct gdtr      gdtr;
} __attribute__((aligned(4096)));

static struct per_cpu_data percpu_storage[SIGMA_MAX_CPUS];

// ── GDT setup ─────────────────────────────────────────────────────────────────

static void make_gdt_entry(struct gdt_entry *e, uint32_t base, uint32_t limit,
                            uint8_t access, uint8_t flags) {
    e->limit_low        = limit & 0xFFFF;
    e->base_low         = base & 0xFFFF;
    e->base_mid         = (base >> 16) & 0xFF;
    e->access           = access;
    e->flags_limit_high = ((flags & 0xF) << 4) | ((limit >> 16) & 0xF);
    e->base_high        = (base >> 24) & 0xFF;
}

static void init_gdt(struct per_cpu_data *pc) {
    // Entry 0: null descriptor
    make_gdt_entry(&pc->gdt[0], 0, 0, 0, 0);
    // Entry 1: kernel code (64-bit)
    make_gdt_entry(&pc->gdt[1], 0, 0xFFFFF, 0x9A, 0xA);
    // Entry 2: kernel data
    make_gdt_entry(&pc->gdt[2], 0, 0xFFFFF, 0x92, 0xC);
    // Entry 3: user code (64-bit)
    make_gdt_entry(&pc->gdt[3], 0, 0xFFFFF, 0xFA, 0xA);
    // Entry 4: user data
    make_gdt_entry(&pc->gdt[4], 0, 0xFFFFF, 0xF2, 0xC);
    // Entry 5–6: TSS (64-bit TSS occupies 2 entries)
    uintptr_t tss_base  = (uintptr_t)&pc->tss;
    uint32_t  tss_limit = sizeof(struct tss64) - 1;
    make_gdt_entry(&pc->gdt[5], (uint32_t)(tss_base & 0xFFFFFFFF),
                   tss_limit, 0x89, 0x0);
    // Upper 32 bits of TSS base in second entry
    ((uint32_t *)&pc->gdt[6])[0] = (uint32_t)(tss_base >> 32);
    ((uint32_t *)&pc->gdt[6])[1] = 0;
}

// ── TSS setup ─────────────────────────────────────────────────────────────────

static void init_tss(struct per_cpu_data *pc) {
    memset(&pc->tss, 0, sizeof(pc->tss));
    // RSP0: ring-0 stack top used on syscall entry
    pc->tss.rsp0 = (uint64_t)(uintptr_t)(pc->syscall_stack + PERCPU_STACK_SIZE);
    // IST1: NMI handler stack
    pc->tss.ist[0] = (uint64_t)(uintptr_t)(pc->ist1_stack + PERCPU_STACK_SIZE);
    // IST2: double-fault stack
    pc->tss.ist[1] = (uint64_t)(uintptr_t)(pc->ist2_stack + PERCPU_STACK_SIZE);
    // IST3: machine-check stack
    pc->tss.ist[2] = (uint64_t)(uintptr_t)(pc->ist3_stack + PERCPU_STACK_SIZE);
    pc->tss.iopb_offset = sizeof(struct tss64);  // no I/O permission bitmap
}

// ── Public API ────────────────────────────────────────────────────────────────

void sigma_percpu_alloc(uint32_t cpu_id) {
    if (cpu_id >= SIGMA_MAX_CPUS) return;
    struct per_cpu_data *pc = &percpu_storage[cpu_id];
    memset(pc, 0, sizeof(*pc));
    init_gdt(pc);
    init_tss(pc);
    pc->gdtr.limit = sizeof(pc->gdt) - 1;
    pc->gdtr.base  = (uint64_t)(uintptr_t)pc->gdt;

    // Store syscall stack pointer in sigma_cpus for fast access
    sigma_cpus[cpu_id].syscall_stack_top =
        (uint64_t)(uintptr_t)(pc->syscall_stack + PERCPU_STACK_SIZE);
}

void sigma_percpu_load(uint32_t cpu_id) {
    if (cpu_id >= SIGMA_MAX_CPUS) return;
    struct per_cpu_data *pc = &percpu_storage[cpu_id];

    // Load GDT
    __asm__ volatile("lgdt %0" :: "m"(pc->gdtr));

    // Reload segment registers
    __asm__ volatile(
        "mov $0x10, %%ax\n"  // kernel data selector (entry 2, RPL=0)
        "mov %%ax, %%ds\n"
        "mov %%ax, %%es\n"
        "mov %%ax, %%ss\n"
        "xor %%ax, %%ax\n"
        "mov %%ax, %%fs\n"
        "mov %%ax, %%gs\n"
        ::: "ax"
    );

    // Load TSS (selector for entry 5 = 5*8 = 0x28)
    __asm__ volatile("ltr %0" :: "r"((uint16_t)0x28));
}

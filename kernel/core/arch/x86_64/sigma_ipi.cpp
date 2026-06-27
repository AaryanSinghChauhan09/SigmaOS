// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_ipi.cpp — Inter-Processor Interrupt handlers for SigmaOS
//
// IPI vectors used:
//   0x40 — TLB shootdown  (notify peers when page tables change)
//   0x41 — Scheduler kick (wake an idle core when a task is enqueued)
//   0x42 — CPU halt       (quiesce a core for hotplug / shutdown)
//   0x43 — Function call  (run arbitrary function on remote CPU)
//
// Inspired by: Linux arch/x86/kernel/smp.c, FreeBSD sys/x86/x86/mp_x86.c

#include "sigma_ipi.h"
#include "sigma_lapic.h"
#include "sigma_smp.h"
#include <stdint.h>
#include <stdatomic.h>

#define IPI_TLB_SHOOTDOWN  0x40
#define IPI_SCHED_KICK     0x41
#define IPI_CPU_HALT       0x42
#define IPI_CALL_FUNC      0x43

// ── TLB shootdown ─────────────────────────────────────────────────────────────
// When one CPU modifies a page table entry, other CPUs must invalidate
// their TLB for that virtual address range.

struct shootdown_range {
    uintptr_t start;
    uintptr_t end;
    atomic_uint acked;
} static volatile g_shootdown;

void sigma_ipi_tlb_shootdown(uintptr_t va_start, uintptr_t va_end) {
    uint32_t ncpus = sigma_smp_cpu_count();
    if (ncpus <= 1) {
        // Single-core: just flush local TLB
        __asm__ volatile("invlpg (%0)" :: "r"(va_start) : "memory");
        return;
    }

    g_shootdown.start = va_start;
    g_shootdown.end   = va_end;
    atomic_store(&g_shootdown.acked, 0u);

    // Broadcast TLB shootdown IPI to all other CPUs
    sigma_lapic_broadcast_ipi(IPI_TLB_SHOOTDOWN);

    // Flush our own TLB while waiting for peers
    for (uintptr_t va = va_start; va < va_end; va += 4096)
        __asm__ volatile("invlpg (%0)" :: "r"(va) : "memory");

    // Wait for all other CPUs to ack
    uint32_t expected = ncpus - 1;
    while (atomic_load(&g_shootdown.acked) < expected)
        __asm__ volatile("pause" ::: "memory");
}

// Called on each remote CPU when IPI 0x40 arrives
void sigma_ipi_handler_tlb_shootdown(void) {
    uintptr_t va = g_shootdown.start;
    uintptr_t end = g_shootdown.end;
    for (; va < end; va += 4096)
        __asm__ volatile("invlpg (%0)" :: "r"(va) : "memory");
    atomic_fetch_add(&g_shootdown.acked, 1u);
    sigma_lapic_eoi();
}

// ── Scheduler kick ────────────────────────────────────────────────────────────
// Wake an idle CPU when a new task is placed on a run queue.

void sigma_ipi_sched_kick(uint32_t target_cpu) {
    uint32_t apic_id = sigma_cpus[target_cpu].apic_id;
    sigma_lapic_send_ipi(apic_id, IPI_SCHED_KICK);
}

void sigma_ipi_handler_sched_kick(void) {
    // The CPU will naturally reschedule on return from interrupt
    sigma_lapic_eoi();
}

// ── CPU halt ─────────────────────────────────────────────────────────────────
// Used during shutdown or CPU hot-remove.

void sigma_ipi_halt_cpu(uint32_t target_cpu) {
    uint32_t apic_id = sigma_cpus[target_cpu].apic_id;
    sigma_lapic_send_ipi(apic_id, IPI_CPU_HALT);
}

__attribute__((noreturn))
void sigma_ipi_handler_cpu_halt(void) {
    sigma_lapic_eoi();
    __asm__ volatile("cli");
    for (;;) __asm__ volatile("hlt");
}

// ── Remote function call ──────────────────────────────────────────────────────
// Execute an arbitrary function on a specific CPU core.

typedef void (*ipi_func_t)(void *arg);

struct ipi_call {
    ipi_func_t func;
    void      *arg;
    atomic_int done;
} static volatile g_ipi_call[SIGMA_MAX_CPUS];

void sigma_ipi_call_function(uint32_t target_cpu, ipi_func_t func, void *arg) {
    if (target_cpu >= SIGMA_MAX_CPUS) return;
    g_ipi_call[target_cpu].func = func;
    g_ipi_call[target_cpu].arg  = arg;
    atomic_store((atomic_int *)&g_ipi_call[target_cpu].done, 0);

    sigma_lapic_send_ipi(sigma_cpus[target_cpu].apic_id, IPI_CALL_FUNC);

    while (!atomic_load((atomic_int *)&g_ipi_call[target_cpu].done))
        __asm__ volatile("pause" ::: "memory");
}

void sigma_ipi_handler_call_function(void) {
    uint32_t cpu = sigma_smp_current_cpu();
    ipi_func_t fn = g_ipi_call[cpu].func;
    void *arg     = g_ipi_call[cpu].arg;
    sigma_lapic_eoi();   // EOI before executing to allow nested IPIs
    if (fn) fn(arg);
    atomic_store((atomic_int *)&g_ipi_call[cpu].done, 1);
}

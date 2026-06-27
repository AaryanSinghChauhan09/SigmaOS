// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_smp.cpp — SMP (Symmetric Multi-Processing) subsystem for SigmaOS
//
// Inspired by:
//   • Linux kernel: arch/x86/kernel/smpboot.c, arch/x86/kernel/apic/
//   • FreeBSD: sys/x86/x86/mp_x86.c
//   • OSDev Wiki: SMP, LAPIC, IOAPIC, SIPI
//   • seL4: src/arch/x86/64/machine/hardware.c
//
// Boot sequence:
//   BSP (Bootstrap Processor) executes this code.
//   BSP reads MADT from ACPI to find all AP LAPIC IDs.
//   BSP sends INIT-SIPI-SIPI sequence to each AP.
//   Each AP runs sigma_ap_entry(), sets up its own GDT/IDT/TSS/CR3,
//   then parks in sigma_ap_idle() until the scheduler assigns work.

#include "sigma_smp.h"
#include "sigma_lapic.h"
#include "sigma_percpu.h"
#include "sigma_ipi.h"
#include <stdint.h>
#include <stddef.h>
#include <stdatomic.h>

// ── Constants ────────────────────────────────────────────────────────────────

#define SIGMA_MAX_CPUS          256
#define SIGMA_AP_TRAMPOLINE_PA  0x8000      // AP real-mode entry (below 1MB)
#define LAPIC_ICR_LOW           0x300
#define LAPIC_ICR_HIGH          0x310
#define LAPIC_INIT              0x00000500  // INIT IPI
#define LAPIC_STARTUP           0x00000600  // STARTUP IPI
#define LAPIC_LEVEL_ASSERT      0x00004000
#define LAPIC_DELIVERY_STATUS   0x00001000
#define SIPI_VECTOR             (SIGMA_AP_TRAMPOLINE_PA >> 12)  // page number

// ── Per-CPU state ─────────────────────────────────────────────────────────────

struct sigma_cpu_state sigma_cpus[SIGMA_MAX_CPUS];
static atomic_uint sigma_cpus_online;   // number of online CPUs
static uint32_t    sigma_bsp_apic_id;   // BSP's APIC ID

// ── AP Trampoline (real-mode→long-mode bridge) ───────────────────────────────
// The trampoline is a small 16-bit stub that lives at SIGMA_AP_TRAMPOLINE_PA.
// It is built separately as arch/x86_64/ap_trampoline.asm.
// We just need to copy it there at runtime.

extern uint8_t sigma_ap_trampoline_start[];
extern uint8_t sigma_ap_trampoline_end[];

static void install_ap_trampoline(void) {
    size_t len = (size_t)(sigma_ap_trampoline_end - sigma_ap_trampoline_start);
    // Physical address is identity-mapped during early boot
    uint8_t *dst = (uint8_t *)(uintptr_t)SIGMA_AP_TRAMPOLINE_PA;
    for (size_t i = 0; i < len; i++) {
        dst[i] = sigma_ap_trampoline_start[i];
    }
}

// ── INIT-SIPI-SIPI sequence ──────────────────────────────────────────────────

static inline void lapic_write(uint32_t reg, uint32_t val) {
    volatile uint32_t *base = (volatile uint32_t *)sigma_lapic_base();
    base[reg / 4] = val;
}

static inline uint32_t lapic_read(uint32_t reg) {
    volatile uint32_t *base = (volatile uint32_t *)sigma_lapic_base();
    return base[reg / 4];
}

static void lapic_wait_delivery(void) {
    while (lapic_read(LAPIC_ICR_LOW) & LAPIC_DELIVERY_STATUS) {
        __asm__ volatile("pause" ::: "memory");
    }
}

// Send INIT IPI to a specific APIC ID, then 2× SIPI
static void send_sipi(uint32_t apic_id) {
    // Assert INIT IPI
    lapic_write(LAPIC_ICR_HIGH, apic_id << 24);
    lapic_write(LAPIC_ICR_LOW,  LAPIC_INIT | LAPIC_LEVEL_ASSERT);
    lapic_wait_delivery();

    // Deassert INIT
    lapic_write(LAPIC_ICR_LOW, LAPIC_INIT);
    lapic_wait_delivery();

    // 10ms delay (busy-wait; real implementation uses HPET/PIT)
    for (volatile uint64_t i = 0; i < 1000000ULL; i++) {
        __asm__ volatile("pause");
    }

    // First STARTUP IPI
    lapic_write(LAPIC_ICR_HIGH, apic_id << 24);
    lapic_write(LAPIC_ICR_LOW,  LAPIC_STARTUP | SIPI_VECTOR);
    lapic_wait_delivery();

    // 200µs delay
    for (volatile uint64_t i = 0; i < 200000ULL; i++) {
        __asm__ volatile("pause");
    }

    // Second STARTUP IPI (some CPUs need 2)
    lapic_write(LAPIC_ICR_HIGH, apic_id << 24);
    lapic_write(LAPIC_ICR_LOW,  LAPIC_STARTUP | SIPI_VECTOR);
    lapic_wait_delivery();
}

// ── AP entry point (called by trampoline after entering long mode) ────────────

// Each AP sets up its own GDT, IDT, TSS, and CR3 (shared page tables).
// Then enables its LAPIC and signals BSP it is alive.

__attribute__((noreturn))
void sigma_ap_entry(uint32_t cpu_id) {
    struct sigma_cpu_state *cs = &sigma_cpus[cpu_id];

    // Set FS base to per-CPU data pointer (using MSR_FS_BASE = 0xC0000100)
    uint64_t percpu_ptr = (uint64_t)(uintptr_t)cs;
    __asm__ volatile(
        "wrmsr"
        :: "c"(0xC0000100u), "a"((uint32_t)percpu_ptr),
           "d"((uint32_t)(percpu_ptr >> 32))
    );

    // Load AP's own GDT/IDT/TSS (pre-allocated by BSP in sigma_percpu_alloc)
    sigma_percpu_load(cpu_id);

    // Enable LAPIC on this core
    sigma_lapic_enable();

    // Enable APIC-timer for preemption (1ms tick)
    sigma_lapic_timer_init(1000 /* µs */);

    // Mark this CPU online
    cs->flags |= SIGMA_CPU_ONLINE;
    atomic_fetch_add(&sigma_cpus_online, 1u);

    sigma_sched_ap_start(cpu_id);  // never returns — enters scheduler loop
    __builtin_unreachable();
}

// ── BSP: enumerate and boot all APs ─────────────────────────────────────────

int sigma_smp_init(void) {
    sigma_bsp_apic_id = sigma_lapic_id();
    sigma_cpus[0].apic_id = sigma_bsp_apic_id;
    sigma_cpus[0].cpu_id  = 0;
    sigma_cpus[0].flags   = SIGMA_CPU_ONLINE | SIGMA_CPU_BSP;
    atomic_store(&sigma_cpus_online, 1u);

    install_ap_trampoline();

    // Enumerate LAPIC entries from ACPI MADT
    uint32_t ap_count = 0;
    for (uint32_t i = 0; i < sigma_acpi_lapic_count(); i++) {
        uint32_t apic_id = sigma_acpi_lapic_id(i);
        if (apic_id == sigma_bsp_apic_id) continue;  // skip BSP

        uint32_t cpu_id = ++ap_count;
        if (cpu_id >= SIGMA_MAX_CPUS) break;

        sigma_cpus[cpu_id].apic_id = apic_id;
        sigma_cpus[cpu_id].cpu_id  = cpu_id;
        sigma_cpus[cpu_id].flags   = 0;  // not yet online

        // Allocate per-CPU structures before waking AP
        sigma_percpu_alloc(cpu_id);

        // Pass cpu_id to AP trampoline via scratch location
        *((volatile uint32_t *)(uintptr_t)(SIGMA_AP_TRAMPOLINE_PA + 0xF8)) = cpu_id;

        // Send INIT-SIPI-SIPI
        send_sipi(apic_id);

        // Wait up to 1s for AP to come online
        for (int wait = 0; wait < 10000; wait++) {
            if (sigma_cpus[cpu_id].flags & SIGMA_CPU_ONLINE) break;
            for (volatile int d = 0; d < 10000; d++) __asm__ volatile("pause");
        }

        if (!(sigma_cpus[cpu_id].flags & SIGMA_CPU_ONLINE)) {
            // AP failed to start — mark as dead
            sigma_cpus[cpu_id].flags |= SIGMA_CPU_DEAD;
        }
    }

    return (int)atomic_load(&sigma_cpus_online);
}

uint32_t sigma_smp_cpu_count(void) {
    return atomic_load(&sigma_cpus_online);
}

uint32_t sigma_smp_current_cpu(void) {
    return sigma_lapic_id_to_cpu(sigma_lapic_id());
}

// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_lapic.cpp — Local APIC driver for SigmaOS
//
// Each CPU core has its own Local APIC (LAPIC).
// The LAPIC handles:
//   • Inter-Processor Interrupts (IPI) — TLB shootdown, scheduler kick, halt
//   • Local timer — generates preemption tick per core
//   • Spurious interrupt suppression
//
// Inspired by: Linux arch/x86/kernel/apic/apic.c, OSDev Wiki LAPIC

#include "sigma_lapic.h"
#include <stdint.h>

// ── LAPIC register offsets (relative to MMIO base) ───────────────────────────

#define LAPIC_ID            0x020
#define LAPIC_VERSION       0x030
#define LAPIC_TPR           0x080   // Task Priority Register
#define LAPIC_EOI           0x0B0   // End of Interrupt
#define LAPIC_SVR           0x0F0   // Spurious Interrupt Vector Register
#define LAPIC_ICR_LOW       0x300   // Interrupt Command Register (low 32 bits)
#define LAPIC_ICR_HIGH      0x310   // Interrupt Command Register (high 32 bits)
#define LAPIC_LVT_TIMER     0x320
#define LAPIC_TIMER_INITIAL 0x380
#define LAPIC_TIMER_CURRENT 0x390
#define LAPIC_TIMER_DIVIDE  0x3E0

#define LAPIC_SVR_ENABLE    (1u << 8)
#define LAPIC_SVR_VECTOR    0xFF    // spurious vector

// ── LAPIC MMIO base ───────────────────────────────────────────────────────────
// Default physical address is 0xFEE00000 (from IA32_APIC_BASE MSR).
// sigma_acpi.cpp may relocate it; use the accessor.

static uintptr_t lapic_mmio_base = 0xFEE00000UL;

static inline void lapic_write32(uint32_t reg, uint32_t val) {
    volatile uint32_t *addr = (volatile uint32_t *)(lapic_mmio_base + reg);
    *addr = val;
}

static inline uint32_t lapic_read32(uint32_t reg) {
    volatile uint32_t *addr = (volatile uint32_t *)(lapic_mmio_base + reg);
    return *addr;
}

// ── Public API ────────────────────────────────────────────────────────────────

uintptr_t sigma_lapic_base(void) {
    return lapic_mmio_base;
}

void sigma_lapic_set_base(uintptr_t base) {
    lapic_mmio_base = base;
}

uint32_t sigma_lapic_id(void) {
    return (lapic_read32(LAPIC_ID) >> 24) & 0xFF;
}

void sigma_lapic_enable(void) {
    // Clear Task Priority Register — accept all interrupts
    lapic_write32(LAPIC_TPR, 0);
    // Enable LAPIC via Spurious Interrupt Vector Register
    lapic_write32(LAPIC_SVR, LAPIC_SVR_ENABLE | LAPIC_SVR_VECTOR);
}

void sigma_lapic_eoi(void) {
    lapic_write32(LAPIC_EOI, 0);
}

// ── Timer ─────────────────────────────────────────────────────────────────────
// Programs the LAPIC one-shot timer to fire in @period_us microseconds.
// The caller must calibrate tsc_freq_hz once (sigma_tsc_calibrate).

#define LAPIC_TIMER_VECTOR  0x20   // IRQ0 equivalent for local timer
#define LAPIC_TIMER_PERIODIC (1u << 17)
#define LAPIC_TIMER_DIVIDE_16 0x3

static uint32_t lapic_timer_ticks_per_us = 100;  // calibrated at boot

void sigma_lapic_timer_calibrate(void) {
    // Use PIT channel 2 to calibrate LAPIC timer frequency
    lapic_write32(LAPIC_TIMER_DIVIDE, LAPIC_TIMER_DIVIDE_16);
    lapic_write32(LAPIC_TIMER_INITIAL, 0xFFFFFFFF);

    // Wait ~10ms using PIT (sigma_pit_wait_us not shown, uses I/O port 0x40–0x43)
    // For simplicity we use a busy-wait estimate here
    for (volatile uint64_t i = 0; i < 500000ULL; i++)
        __asm__ volatile("pause");

    uint32_t remaining = lapic_read32(LAPIC_TIMER_CURRENT);
    uint32_t elapsed   = 0xFFFFFFFF - remaining;
    // elapsed ticks in ~10ms → ticks/us ≈ elapsed / 10000
    lapic_timer_ticks_per_us = elapsed / 10000;
    if (lapic_timer_ticks_per_us == 0)
        lapic_timer_ticks_per_us = 100;  // fallback
}

void sigma_lapic_timer_init(uint32_t period_us) {
    lapic_write32(LAPIC_TIMER_DIVIDE,  LAPIC_TIMER_DIVIDE_16);
    lapic_write32(LAPIC_LVT_TIMER,     LAPIC_TIMER_VECTOR | LAPIC_TIMER_PERIODIC);
    lapic_write32(LAPIC_TIMER_INITIAL, period_us * lapic_timer_ticks_per_us);
}

// ── IPI helpers ───────────────────────────────────────────────────────────────

static void lapic_wait_icr_idle(void) {
    while (lapic_read32(LAPIC_ICR_LOW) & (1u << 12))  // delivery status
        __asm__ volatile("pause" ::: "memory");
}

// Send a fixed-vector IPI to a specific APIC ID
void sigma_lapic_send_ipi(uint32_t apic_id, uint32_t vector) {
    lapic_wait_icr_idle();
    lapic_write32(LAPIC_ICR_HIGH, apic_id << 24);
    lapic_write32(LAPIC_ICR_LOW,  vector | 0x00004000);  // edge, fixed
}

// Send IPI to all CPUs except self
void sigma_lapic_broadcast_ipi(uint32_t vector) {
    lapic_wait_icr_idle();
    lapic_write32(LAPIC_ICR_LOW, vector | (3u << 18) | 0x00004000);
                                         // ^^^^ shorthand: all-exc-self
}

// ── APIC ID → CPU ID mapping ──────────────────────────────────────────────────

// Populated during sigma_smp_init as APs come online
static uint32_t apic_to_cpu[256];

void sigma_lapic_register_cpu(uint32_t apic_id, uint32_t cpu_id) {
    if (apic_id < 256)
        apic_to_cpu[apic_id] = cpu_id;
}

uint32_t sigma_lapic_id_to_cpu(uint32_t apic_id) {
    if (apic_id < 256)
        return apic_to_cpu[apic_id];
    return 0;
}

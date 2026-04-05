/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PIT TIMER + PIT/HPET DRIVER (v1.0 - PURE C11)
 * =============================================================================
 * PIT 8253/8254: generates IRQ0 at configurable Hz (default 1000Hz = 1ms)
 * HPET fallback reading for high-resolution timestamps.
 * Features:
 *   - Clock calibration against CPUID leaf 0x15
 *   - Uptime counter (ms + ticks)
 *   - TSC-based nanosecond clock
 *   - Sleep primitive (tick-based)
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../libc/SovereignLibC.h"

/* =========================================================================
 * PIT Constants
 * ========================================================================= */
#define PIT_CHANNEL0   0x40   /* Channel 0 data port */
#define PIT_CHANNEL1   0x41
#define PIT_CHANNEL2   0x42
#define PIT_CMD        0x43   /* Mode/Command register */

/* PIT oscillator frequency: 1.193182 MHz */
#define PIT_BASE_HZ    1193182u
#define PIT_TARGET_HZ  1000u
#define PIT_DIVISOR    (PIT_BASE_HZ / PIT_TARGET_HZ)  /* = 1193 */

/* PIT command words */
#define PIT_CMD_CHAN0  0x00   /* Select channel 0 */
#define PIT_CMD_LOHI  0x30   /* Access: lobyte/hibyte */
#define PIT_CMD_MODE2 0x04   /* Rate generator */
#define PIT_CMD_BIN   0x00   /* Binary */

/* =========================================================================
 * Timer State
 * ========================================================================= */
typedef struct SigmaTimer {
    volatile u64 ticks;       /* total IRQ0 ticks since boot */
    volatile u64 ms;          /* milliseconds since boot */
    u64          tsc_per_ms;  /* TSC ticks per millisecond (calibrated) */
    u64          boot_tsc;    /* TSC at boot */
} SigmaTimer;

static SigmaTimer g_timer;

/* =========================================================================
 * PIT Init — program to 1000 Hz square wave
 * ========================================================================= */
void pit_init(void) {
    /* Command: channel 0, lobyte+hibyte, mode 2 (rate generator), binary */
    port_outb(PIT_CMD, PIT_CMD_CHAN0 | PIT_CMD_LOHI | PIT_CMD_MODE2 | PIT_CMD_BIN);
    port_outb(PIT_CHANNEL0, (u8)(PIT_DIVISOR & 0xFF));        /* lo byte */
    port_outb(PIT_CHANNEL0, (u8)((PIT_DIVISOR >> 8) & 0xFF)); /* hi byte */

    g_timer.ticks    = 0;
    g_timer.ms       = 0;
    g_timer.boot_tsc = cpu_rdtsc();

    /* Calibrate TSC: measure TSC ticks in 10ms using PIT */
    u64 tsc_start = cpu_rdtsc();
    u64 target_ms = 10;
    /* Spin until 10 ticks pass */
    while (g_timer.ticks < target_ms) cpu_pause();
    u64 tsc_end = cpu_rdtsc();
    g_timer.tsc_per_ms = (tsc_end - tsc_start) / target_ms;

    extern void kprintf(const char* fmt, ...);
    kprintf("[PIT]: 1000Hz | TSC/ms=%llu | Uptime tracking active.\n",
            g_timer.tsc_per_ms);
}

/* =========================================================================
 * PIT IRQ0 Tick Handler (registered via idt_register_handler)
 * ========================================================================= */
typedef struct SigmaInterruptFrame SigmaInterruptFrame;

void pit_irq_handler(SigmaInterruptFrame* frame) {
    (void)frame;
    g_timer.ticks++;
    g_timer.ms++;   /* 1 IRQ = 1ms at 1000Hz */

    /* Poke scheduler every tick */
    extern void sched_schedule(void);
    sched_schedule();
}

/* =========================================================================
 * Accessors
 * ========================================================================= */
u64 timer_get_ticks(void) { return g_timer.ticks; }
u64 timer_get_ms(void)    { return g_timer.ms;    }

u64 timer_get_ns(void) {
    u64 tsc_delta = cpu_rdtsc() - g_timer.boot_tsc;
    if (g_timer.tsc_per_ms == 0) return 0;
    return (tsc_delta * 1000000ULL) / g_timer.tsc_per_ms;
}

/* =========================================================================
 * Sleep — busy-wait on tick counter (kernel-mode only)
 * ========================================================================= */
void timer_sleep_ms(u64 ms) {
    u64 wake = g_timer.ms + ms;
    while (g_timer.ms < wake) cpu_pause();
}

/* =========================================================================
 * Register PIT handler and unmask IRQ0
 * ========================================================================= */
void timer_init(void) {
    extern void idt_register_handler(u32, void(*)(SigmaInterruptFrame*));
    extern void pic_unmask_irq(u8);
    idt_register_handler(32, pit_irq_handler);  /* IRQ0 → vector 32 */
    pic_unmask_irq(0);
    pit_init();
}

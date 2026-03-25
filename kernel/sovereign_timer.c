/*
 * SigmaOS Enterprise Timer & Clock Subsystem (C Core)
 * ====================================================
 * Provides high-resolution timing using:
 *   - HPET (High Precision Event Timer) for wall-clock time
 *   - APIC Timer for per-CPU preemption ticks
 *   - TSC (Time Stamp Counter) for nanosecond profiling
 *
 * Design philosophy:
 *   - No reliance on BIOS time services (INT 0x1A is insecure)
 *   - Tick rate calibrated against HPET on first boot
 *   - Per-CPU APIC timers allow lockless scheduling
 *
 * IP Compliance: 100% original. Specs from Intel SDM (public).
 */

#include <stdbool.h>
#include <stdint.h>


/* ── HPET Memory-Mapped Register Offsets (ACPI spec, public) ────────── */
#define HPET_BASE_ADDR 0xFED00000UL  /* Default HPET MMIO base    */
#define HPET_REG_CAPABILITIES 0x000  /* General Capabilities reg  */
#define HPET_REG_CONFIG 0x010        /* General Config register   */
#define HPET_REG_MAIN_COUNTER 0x0F0  /* Main Counter Value        */
#define HPET_REG_T0_CONFIG 0x100     /* Timer 0 Config            */
#define HPET_REG_T0_COMPARATOR 0x108 /* Timer 0 Comparator        */

#define HPET_ENABLE_CNF (1UL << 0)     /* Enable main counter       */
#define HPET_T0_INT_ENB_CNF (1UL << 2) /* Timer 0 interrupt enable  */
#define HPET_T0_TYPE_CNF (1UL << 3)    /* Periodic mode             */

/* ── APIC Base & Registers (x86_64 Local APIC) ───────────────────────── */
#define APIC_BASE_MSR 0x1B
#define APIC_BASE_ENABLE (1UL << 11)
#define X2APIC_ENABLE (1UL << 10)

/* ── TSC State ───────────────────────────────────────────────────────── */
static uint64_t _tsc_khz = 0;    /* TSC frequency in kHz (calibrated)   */
static uint64_t _tsc_offset = 0; /* Boot-time TSC value (monotonic zero) */
static bool _hpet_ready = false;

/* ── MMIO Helpers ────────────────────────────────────────────────────── */

static inline void mmio_write64(uint64_t addr, uint64_t val) {
  volatile uint64_t *p = (volatile uint64_t *)addr;
  *p = val;
}

static inline uint64_t mmio_read64(uint64_t addr) {
  volatile uint64_t *p = (volatile uint64_t *)addr;
  return *p;
}

/* ── TSC Utilities ───────────────────────────────────────────────────── */

static inline uint64_t read_tsc(void) {
  uint32_t lo, hi;
  __asm__ volatile("lfence; rdtsc" : "=a"(lo), "=d"(hi)::"memory");
  return ((uint64_t)hi << 32) | lo;
}

static inline void write_msr(uint32_t msr, uint64_t val) {
  uint32_t lo = (uint32_t)val, hi = (uint32_t)(val >> 32);
  __asm__ volatile("wrmsr" ::"c"(msr), "a"(lo), "d"(hi));
}

static inline uint64_t read_msr(uint32_t msr) {
  uint32_t lo, hi;
  __asm__ volatile("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
  return ((uint64_t)hi << 32) | lo;
}

/* ── HPET Initialization ─────────────────────────────────────────────── */

/**
 * sigma_timer_init_hpet() - Enable the HPET main counter.
 * Must be called after ACPI tables have been parsed to confirm HPET base.
 * Returns: fs-period of HPET tick, or 0 on failure.
 */
uint64_t sigma_timer_init_hpet(uint64_t hpet_base) {
  /* Sanity check HPET capabilities (bits[31:16] = number of timers) */
  uint64_t caps = mmio_read64(hpet_base + HPET_REG_CAPABILITIES);
  if (caps == 0 || caps == 0xFFFFFFFFFFFFFFFFULL)
    return 0;

  /* Extract femtosecond-per-tick period (bits[63:32]) */
  uint64_t fs_period = caps >> 32;
  if (fs_period == 0 || fs_period > 0x05F5E100UL)
    return 0; /* Sanity: < 100ns/tick */

  /* Enable the main counter */
  uint64_t cfg = mmio_read64(hpet_base + HPET_REG_CONFIG);
  cfg |= HPET_ENABLE_CNF;
  mmio_write64(hpet_base + HPET_REG_CONFIG, cfg);

  _hpet_ready = true;
  return fs_period;
}

/**
 * sigma_timer_calibrate_tsc() - Calibrate TSC frequency using HPET.
 * Polls HPET for ~10ms to measure TSC ticks per second.
 * Must be called after sigma_timer_init_hpet().
 */
void sigma_timer_calibrate_tsc(uint64_t hpet_base, uint64_t fs_period) {
  if (!_hpet_ready || fs_period == 0)
    return;

  /* Target: 10,000,000 ns = 10ms worth of HPET ticks */
  uint64_t ticks_10ms = (10000000000ULL) / fs_period; /* ns → fs → ticks */

  uint64_t hpet_start = mmio_read64(hpet_base + HPET_REG_MAIN_COUNTER);
  uint64_t tsc_start = read_tsc();

  /* Busy-wait for 10ms (acceptable during early boot) */
  while ((mmio_read64(hpet_base + HPET_REG_MAIN_COUNTER) - hpet_start) <
         ticks_10ms)
    __asm__ volatile("pause");

  uint64_t tsc_end = read_tsc();
  uint64_t tsc_delta = tsc_end - tsc_start;

  /* tsc_delta ticks in 10ms → freq in kHz = delta / 10 */
  _tsc_khz = tsc_delta / 10;
  _tsc_offset = tsc_start;
}

/* ── Public Timekeeping API ──────────────────────────────────────────── */

/**
 * sigma_timer_ns() - Return nanoseconds since boot (monotonic).
 * Uses calibrated TSC — zero system call overhead.
 */
uint64_t sigma_timer_ns(void) {
  if (_tsc_khz == 0)
    return 0;
  uint64_t tsc_delta = read_tsc() - _tsc_offset;
  /* ns = (tsc_delta * 1,000,000) / _tsc_khz  — use 64-bit arithmetic */
  return (tsc_delta * 1000000ULL) / _tsc_khz;
}

/**
 * sigma_timer_ms() - Return milliseconds since boot.
 */
uint64_t sigma_timer_ms(void) { return sigma_timer_ns() / 1000000ULL; }

/**
 * sigma_timer_tsc_khz() - Return calibrated TSC frequency in kHz.
 */
uint64_t sigma_timer_tsc_khz(void) { return _tsc_khz; }

/* ── Preemption Timer (APIC) ─────────────────────────────────────────── */

/**
 * sigma_apic_timer_init() - Program Local APIC timer for preemption ticks.
 * @apic_base:    MMIO base of the local APIC (usually 0xFEE00000).
 * @tick_ms:      Desired tick interval in milliseconds.
 * @tsc_khz:      Calibrated TSC frequency (from sigma_timer_calibrate_tsc).
 */
void sigma_apic_timer_init(uint64_t apic_base, uint32_t tick_ms,
                           uint64_t tsc_khz) {
  /* Divide config: divide by 1 for maximum resolution */
  mmio_write64(apic_base + 0x3E0, 0xB); /* Divide by 1  */

  /* Set timer in periodic mode, vector 0x20 (IRQ0) */
  mmio_write64(apic_base + 0x320, (1 << 17) | 0x20); /* Periodic | vec */

  /* Initial count = (tsc_khz * tick_ms) — fires every tick_ms ms */
  uint32_t initial = (uint32_t)((tsc_khz * tick_ms));
  mmio_write64(apic_base + 0x380, initial);
}

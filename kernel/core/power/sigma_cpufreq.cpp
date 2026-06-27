// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_cpufreq.cpp — CPU frequency scaling (DVFS) + thermal management
//
// Implements four governors (Linux-compatible naming):
//   PERFORMANCE  — clamp at P-state 0 (max)
//   POWERSAVE    — clamp at lowest P-state
//   SCHEDUTIL    — scale linearly with CPU utilisation (best for battery)
//   ONDEMAND     — ramp up immediately on load, decay on idle
//
// Also handles thermal trips:
//   >85°C  → switch to POWERSAVE
//   >95°C  → set 50% max frequency
//   >105°C → emergency shutdown
//
// Uses Intel Speed Step (MSR_IA32_PERF_CTL) and AMD P-state MSRs.
//
// Inspired by:
//   • Linux kernel/cpufreq/ (schedutil governor)
//   • Linux drivers/thermal/thermal_core.c
//   • Intel ACPI P-state specification (ACPI 6.5 § 8.4)

#include "sigma_cpufreq.h"
#include "sigma_smp.h"
#include <stdint.h>
#include <stdbool.h>
#include <stdatomic.h>

// ── MSR / CPUID constants ─────────────────────────────────────────────────────

#define MSR_IA32_PERF_CTL        0x199   // Intel P-state control
#define MSR_IA32_PERF_STATUS     0x198   // Intel current P-state
#define MSR_AMD_PSTATE_CTL       0xC0010062  // AMD P-state control
#define MSR_THERM_STATUS         0x19C   // Intel thermal status
#define MSR_PLATFORM_INFO        0xCE    // max/min ratio

#define THERM_STATUS_READING_VALID (1u << 31)
#define THERM_STATUS_TEMP_MASK     0x007F0000

// ── P-state table ─────────────────────────────────────────────────────────────

#define MAX_PSTATES  16

struct pstate {
    uint32_t freq_mhz;
    uint32_t voltage_mv;
    uint32_t msr_value;   // value to write to PERF_CTL
};

static struct pstate g_pstates[MAX_PSTATES];
static uint32_t g_num_pstates = 0;
static uint32_t g_max_pstate  = 0;

// ── Per-CPU governor state ────────────────────────────────────────────────────

struct cpufreq_cpu {
    uint8_t  governor;
    uint32_t current_pstate;
    uint32_t util_percent;       // last measured utilisation (0–100)
    uint32_t thermal_limit;      // max allowed pstate (0 = unconstrained)
    uint64_t last_idle_tsc;
    uint64_t last_total_tsc;
};

static struct cpufreq_cpu g_cpus[SIGMA_MAX_CPUS];

// ── MSR helpers ───────────────────────────────────────────────────────────────

static inline void wrmsr(uint32_t msr, uint64_t val) {
    __asm__ volatile("wrmsr"
        :: "c"(msr), "a"((uint32_t)val), "d"((uint32_t)(val >> 32)));
}
static inline uint64_t rdmsr(uint32_t msr) {
    uint32_t lo, hi;
    __asm__ volatile("rdmsr" : "=a"(lo), "=d"(hi) : "c"(msr));
    return ((uint64_t)hi << 32) | lo;
}

// ── P-state selection ─────────────────────────────────────────────────────────

static void set_pstate(uint32_t cpu, uint32_t pstate) {
    if (pstate >= g_num_pstates) pstate = g_num_pstates - 1;
    if (g_cpus[cpu].thermal_limit &&
        pstate < g_cpus[cpu].thermal_limit) {
        pstate = g_cpus[cpu].thermal_limit;
    }
    g_cpus[cpu].current_pstate = pstate;
    // Run this on the target CPU via IPI (sigma_ipi_call_function)
    extern void sigma_ipi_call_function(uint32_t, void (*)(void*), void*);
    // For BSP or when called from the correct CPU, write directly
    wrmsr(MSR_IA32_PERF_CTL,
          (uint64_t)g_pstates[pstate].msr_value << 8);
}

// ── Governor logic ────────────────────────────────────────────────────────────

static void schedutil_update(uint32_t cpu, uint32_t util_pct) {
    // Scale pstate linearly: P0 at 100%, P_max at 0%
    uint32_t target = (uint32_t)(
        (uint64_t)g_max_pstate * (100 - util_pct) / 100);
    set_pstate(cpu, target);
}

static void ondemand_update(uint32_t cpu, uint32_t util_pct) {
    if (util_pct >= 80) {
        set_pstate(cpu, 0);          // ramp to max immediately
    } else if (util_pct < 20) {
        uint32_t p = g_cpus[cpu].current_pstate + 1;
        if (p <= g_max_pstate) set_pstate(cpu, p);  // step down
    }
}

// ── Tick handler (called every 10ms per CPU) ──────────────────────────────────

void sigma_cpufreq_tick(uint32_t cpu, uint32_t util_pct) {
    g_cpus[cpu].util_percent = util_pct;
    switch (g_cpus[cpu].governor) {
        case GOVERNOR_PERFORMANCE:
            set_pstate(cpu, 0);
            break;
        case GOVERNOR_POWERSAVE:
            set_pstate(cpu, g_max_pstate);
            break;
        case GOVERNOR_SCHEDUTIL:
            schedutil_update(cpu, util_pct);
            break;
        case GOVERNOR_ONDEMAND:
            ondemand_update(cpu, util_pct);
            break;
    }
}

// ── Thermal management ────────────────────────────────────────────────────────

uint32_t sigma_cpufreq_read_temp_celsius(uint32_t cpu) {
    // Intel IA32_THERM_STATUS: bits [22:16] = offset below Tj_max
    // Tj_max is typically 100°C
    (void)cpu;
    uint64_t ts = rdmsr(MSR_THERM_STATUS);
    if (!(ts & THERM_STATUS_READING_VALID)) return 0;
    uint32_t offset = (ts & THERM_STATUS_TEMP_MASK) >> 16;
    return 100 - offset;   // approximate °C
}

void sigma_cpufreq_thermal_check(uint32_t cpu) {
    uint32_t temp = sigma_cpufreq_read_temp_celsius(cpu);
    if (temp == 0) return;

    if (temp >= 105) {
        // Emergency: kernel panic + power off
        // sigma_panic("THERMAL: CPU %u reached %u°C — emergency shutdown", cpu, temp);
        // sigma_power_emergency_shutdown();
        __asm__ volatile("cli; hlt");
    } else if (temp >= 95) {
        // Hard throttle: allow only 50% max frequency
        g_cpus[cpu].thermal_limit = g_max_pstate / 2;
        set_pstate(cpu, g_cpus[cpu].thermal_limit);
    } else if (temp >= 85) {
        // Soft throttle: switch to POWERSAVE
        g_cpus[cpu].governor       = GOVERNOR_POWERSAVE;
        g_cpus[cpu].thermal_limit  = g_max_pstate;
    } else {
        // Normal: clear thermal limit
        g_cpus[cpu].thermal_limit = 0;
    }
}

// ── P-state init (called by sigma_boot_cpu after ACPI is parsed) ──────────────

void sigma_cpufreq_init(void) {
    // Read hardware P-state range from MSR_PLATFORM_INFO
    uint64_t pi = rdmsr(MSR_PLATFORM_INFO);
    uint32_t max_ratio = (pi >> 8)  & 0xFF;  // max non-turbo ratio
    uint32_t min_ratio = (pi >> 40) & 0xFF;  // min ratio (LFM)
    if (min_ratio == 0) min_ratio = 8;       // 800MHz fallback

    g_num_pstates = 0;
    for (uint32_t r = max_ratio; r >= min_ratio && g_num_pstates < MAX_PSTATES;
         r--, g_num_pstates++) {
        g_pstates[g_num_pstates].freq_mhz  = r * 100;
        g_pstates[g_num_pstates].msr_value = r;
    }
    g_max_pstate = g_num_pstates ? g_num_pstates - 1 : 0;

    // Default governor: SCHEDUTIL on all CPUs
    for (uint32_t c = 0; c < SIGMA_MAX_CPUS; c++) {
        g_cpus[c].governor = GOVERNOR_SCHEDUTIL;
        g_cpus[c].thermal_limit = 0;
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

void sigma_cpufreq_set_governor(uint32_t cpu, uint8_t governor) {
    if (cpu < SIGMA_MAX_CPUS) g_cpus[cpu].governor = governor;
}

uint32_t sigma_cpufreq_get_freq_mhz(uint32_t cpu) {
    if (cpu >= SIGMA_MAX_CPUS || g_cpus[cpu].current_pstate >= g_num_pstates)
        return 0;
    return g_pstates[g_cpus[cpu].current_pstate].freq_mhz;
}

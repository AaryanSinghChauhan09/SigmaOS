// =============================================================================
// SigmaOS — S04_HAL — SovereignHardwareTuner.c
// Low-Level BIOS/Firmware Optimization Shard
// =============================================================================
// Exceeding Competitors:
//   • Windows/macOS — Limited to high-level "Performance/Battery" profiles.
//   • Linux         — requires complex userland tools (cpufreq, throttled).
//   • Sigma hardware Tuner — Directly communicates with the ACPI/BIOS/UEFI and 
//     EC (Embedded Controller) to tune voltages, fans, and TDP in real-time.
// Architecture:
//   • Sentiment-Based Tuning: S13 Sentience tells S04 when to "Overclock" 
//     for a compilation burst or "Undervolt" for a reading session.
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_types.h"


typedef struct {
    uint32_t core_id;
    uint32_t current_voltage_mv;
    uint32_t current_clock_mhz;
    uint32_t thermal_limit_c;
} TunerMetrics;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Hardware Tuner (Secure BIOS/EC handshake)
void hw_tuner_init(void);

// Tune CPU voltage and clock for a specific Sentience burst (S13 hook)
void hw_tuner_set_cpu_profile(uint32_t voltage_mv, uint32_t clock_mhz);

// Adjust fan curves and thermal trip-points (Kernel-native protection)
void hw_tuner_set_thermal_curve(uint8_t fan_step);

// Request a specific TDP (Thermal Design Power) limit from the platform
void hw_tuner_request_tdp(uint32_t watts);

// Audit current hardware efficiency (Watts-per-Sentiment-Point)
float hw_tuner_audit_efficiency(void);

// Safe-Revert: Restore factory firmware defaults if instability is detected
void hw_tuner_emergency_revert(void);




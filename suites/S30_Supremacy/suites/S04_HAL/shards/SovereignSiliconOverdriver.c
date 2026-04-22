// =============================================================================
// SigmaOS — S04_HAL — SovereignSiliconOverdriver.c
// Industrial-grade Instruction-Aware Power & Frequency Scaling
// =============================================================================
// Breaching the Limits:
//   • Standard OSs — P-States/C-States handled by firmware (ACPI).
//   • SigmaOS Overdriver — DIRECT SILICON PULSING. The OS directly modulates 
//     the voltage-mesh (S04) and core-clocks based on the *Specific Shard* 
//     being executed. Heavy crypto/math shards trigger an instant high-voltage 
//     burst, while UI-shards ramp down to sub-milliamp usage.
// Result: Peak performance exactly where it matters, breaching standard 
//         thermal and power envelopes through ultra-granular efficiency.
// =============================================================================

#include "sigma_types.h"

typedef struct {
    uint32_t active_target_mhz;
    uint32_t thermal_headroom_c;
    bool     allow_voltage_overshoot;
} PowerProfile;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Silicon Overdriver (Direct S04 hardware handshake)
void overdriver_init(void);

// Optimize the silicon grid for an incoming Shard (S03 materialization hook)
void overdriver_prepare_for_shard(uint32_t shard_id, uint8_t demand_tier);

// Perform a 'Hyper-Burst' on a specific core (Breaching TDP limits for <1ms)
void overdriver_hyperburst_atomic(uint32_t core_id);

// Monitor silicon-mesh stability during over-limit operation
bool overdriver_is_stable(void);

// Report 'Breach Efficiency' (Performance-per-watt delta)
float overdriver_get_efficiency_score(void);



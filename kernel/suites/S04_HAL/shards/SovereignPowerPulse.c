// =============================================================================
// SigmaOS — S04_HAL — SovereignPowerPulse.c
// Predictive Nano-Scale Power Management Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • macOS Energy Saver — Fine-grained component nap (App Nap parity)
//   • Windows Power Profiles — Dynamic clock scaling
//   • Android Doze — Deep sleep state with background batching
// Exceeding Competitors:
//   • Nano-Gating: Shuts down individual CPU logical blocks between cycles
//   • Predictive Throttling: Cools the CPU *before* heat spikes occur
//   • Sentiment Efficiency: Correlates user patterns (S13) to battery drain
// =============================================================================

#include <sigma_types.h>


typedef enum {
    POWER_STATE_P0   = 0, // Max Performance
    POWER_STATE_P1   = 1, // Balanced
    POWER_STATE_P2   = 2, // Low Power (App Nap)
    POWER_STATE_P3   = 3  // Nano-Sleep
} PowerState;

// ── Power Node ───────────────────────────────────────────────────────────────
typedef struct {
    uint32_t       component_id;
    char           label[64];
    PowerState     current_state;
    uint32_t       temp_celsius;
    float          current_load;
} PowerNode;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Power Pulse engine (HW interface with S04)
void power_pulse_init(void);

// Transition a component (CPU/GPU/Disk) to a new power state
void power_pulse_set_state(uint32_t component_id, PowerState state);

// Autonomous Predictive Scaling: Adjusts clock based on S13 forecasts
void power_pulse_auto_tune(void);

// Register an app for "Deep Sleep" batching (Android Doze parity)
void power_pulse_nappify_app(const char* app_id);

// Broadcast thermals to ZenithUI dashboard (Sensor HUD parity)
void power_pulse_report_telemetry(PowerNode* nodes_out, uint32_t count);

// Emergency Throttle: Instant downscale to prevent hardware damage
void power_pulse_panic_cool(void);




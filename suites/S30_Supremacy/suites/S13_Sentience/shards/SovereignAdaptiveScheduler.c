// =============================================================================
// SigmaOS — S13_Sentience — SovereignAdaptiveScheduler.c
// AI-Driven Predictive CPU Scheduling Shard
// =============================================================================
// Exceeding Competitors:
//   • Linux CFS        — Reactive; adjusts to past behaviour
//   • Windows GCD      — Balanced; based on foreground/background binary
//   • Sigma Sentience  — PREDICTIVE; uses a lightweight on-kernel neural 
//     weight-set to forecast process bursts based on temporal patterns.
// Architecture:
//   • Learns user work habits (e.g., "Developer opens IDE at 9:00 AM")
//   • Pre-allocates slab caches and pre-warms CPU L3 for predicted process wakes
//   • Jitter-free real-time adjustment for pro-audio and holographic rendering
// =============================================================================

#include "../../../../../include/core/sigma_types.h"


#define TRAINING_WINDOW     1024
#define MAX_PREDICTED_PIDS  64

// ── Temporal Process Metric ──────────────────────────────────────────────────
typedef struct {
    uint32_t pid;
    uint64_t last_wake_tsc;
    uint64_t avg_burst_len;
    float    probability_next_slice; // The "Sentiment" score
} ProcessSentiment;

static ProcessSentiment sentiment_table[MAX_PREDICTED_PIDS];
static uint32_t         table_count = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Adaptive Sentiment Core
void sentience_init(void);

// Record a scheduling event for learning
void sentience_observe_event(uint32_t pid, uint8_t event_type);

// Query the AI for the next "Ideal" PID to run (Predictive Branching)
uint32_t sentience_predict_next_task(void);

// Pre-warm the cache for the predicted PID (Exceeds Linux/Windows)
void sentience_prewarm_context(uint32_t pid);

// Export the "System Soul" (Learned weights) to S10_Registry for persistence
void sentience_persist_model(void);

// Adjust power-scaling (S04_HAL) based on predicted load bursts
void sentience_tune_frequencies(void);




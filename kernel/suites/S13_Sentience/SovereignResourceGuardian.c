// =============================================================================
// SigmaOS — S13_Sentience — SovereignResourceGuardian.c
// AI-Driven autonomous Resource Watchdog
// =============================================================================
// Exceeding Competitors:
//   • Windows Task Manager — Manual termination only.
//   • macOS Activity Monitor— Informational; no automated resource throttling.
//   • Linux OOM Killer    — Destructive; kills processes only when RAM is gone.
//   • Sigma Guardian      — PREEMPTIVE: Identifies abnormal CPU/RAM growth 
//     trends and autonomously throttles a .sab bundle's capability or 
//     memory zone BEFORE the system slows down.
// =============================================================================

#include <stdint.h>
#include <stdbool.h>

#define TREND_WINDOW    32
#define ALERT_THRESHOLD 0.85f

typedef struct {
    uint32_t pid;
    float    cpu_trend[TREND_WINDOW];
    float    mem_trend[TREND_WINDOW];
    uint8_t  sentiment_score; // Learned "Importance" of the task
} ResourceProfile;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Resource Guardian engine
void guardian_init(void);

// Record process metrics for trend analysis (S03 Scheduler hook)
void guardian_observe_metrics(uint32_t pid, float cpu_pct, float mem_pct);

// Audit process sentiment: Is this task critical to the user? (S13)
uint8_t guardian_query_sentiment(uint32_t pid);

// Autonomous Throttling: Enforce quota on a misbehaving shard
void guardian_throttle_bundle(const char* app_id, uint8_t level);

// Autonomous Restart: Gently cycle a memory-leaking app (S10 SAB hook)
void guardian_recycle_process(uint32_t pid);

// Report "System Health" interventions to ZenithUI
void guardian_report_actions(void);

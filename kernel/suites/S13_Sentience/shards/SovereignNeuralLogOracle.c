// =============================================================================
// SigmaOS — S13_Sentience — SovereignNeuralLogOracle.c
// AI-Augmented Log Analysis & Error Resolution
// =============================================================================
// Exceeding Competitors:
//   • Windows Event Viewer — Cryptic codes (0x80070005)
//   • Linux journalctl    — Raw text dump, often verbose and unorganized
//   • Sigma Oracle — Evaluates log streams and provides "Plain Language" 
//     root cause analysis and autonomous resolution suggestions.
// =============================================================================

#include <sigma_types.h>


typedef struct {
    uint32_t log_id;
    uint16_t suite_id; // S01...S13
    uint8_t  severity; // 0=Info, 1=Warn, 2=Crit, 3=Panic
    char     raw_msg[256];
    uint64_t timestamp;
} LogEntry;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Oracle listener
void oracle_init(void);

// Submit a log entry for neural categorization
void oracle_log(uint16_t suite_id, uint8_t severity, const char* msg);

// Query the Oracle for a "Human Explanation" of a specific critical error
const char* oracle_explain(uint32_t log_id);

// Request a "Self-Healing" path for a recurring warning (S10 hook)
void oracle_suggest_remedy(uint32_t log_id);

// Summarize system health for ZenithUI Action Center (Dashboard parity)
const char* oracle_get_health_summary(void);

// Export neural log weights for Continuity sync (S12)
void oracle_sync_learned_patterns(void);




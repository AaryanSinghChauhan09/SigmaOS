// =============================================================================
// SigmaOS — S09_Intelligence — SovereignIntelligenceBridge.c
// Native On-Device AI & LLM Inference Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows Copilot — Integrated AI chat and system control
//   • Apple Intelligence — Privacy-focused on-device models
//   • Google Gemini (Pixel) — Local generative AI features
// SigmaOS Intelligence:
//   • Zero-Cloud Policy: 100% of inference happens on the local GPU/NPU via S04.
//   • Contextual Awareness: Access to VFS, Registry, and Logs (Local-only).
//   • Multi-Model Support: Swap between lightweight 1B models and Pro-Models.
// =============================================================================

#include "sigma_types.h"


#define MAX_PROMPT_LEN      4096
#define MAX_RESPONSE_LEN    8192

typedef enum {
    MODEL_TINY_1B       = 0, // Low memory, fast
    MODEL_PRO_7B        = 1, // Deep reasoning
    MODEL_VISION        = 2  // For ZenithUI screenshot analysis
} AIModelType;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Intelligence Bridge (Connects to S04 HAL GpuDriverStack)
void intelligence_init(void);

// Perform local inference (Zero network dependency)
bool intelligence_infer(AIModelType model, const char* prompt, char* response_out);

// ZenithUI Integration: "What am I looking at?" (Analyzes current frame)
void intelligence_analyze_ui(void);

// Privacy Guard: Strip all local PII before passing context to the model
void intelligence_sanitize_context(char* context);

// System Control: AI generates a Registry v2 or ShardCtl command
void intelligence_execute_intent(const char* user_intent);

// Scale Inference: Use Hive (S13) to run 70B+ models across multiple devices
void intelligence_distribute_load(void);




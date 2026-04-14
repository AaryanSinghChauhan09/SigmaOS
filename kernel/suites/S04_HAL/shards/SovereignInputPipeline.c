// =============================================================================
// SigmaOS — S04_HAL — SovereignInputPipeline.c
// Zero-Latency High-Speed Input Synthesis
// =============================================================================
// Competitor USPs Absorbed:
//   • Nvidia Reflex (PC) — Reduced latency between click and render
//   • Apple ProMotion (macOS) — 120Hz display sync with input
//   • Windows Raw Input — Direct peripheral access
// Exceeding Competitors:
//   • Direct-Path: Input events move from S04_HAL directly to S02 GPU 
//     command-buffers, bypassing standard userland OS message loops.
//   • Native 8000Hz Polling Support for pro-level HID interaction.
// =============================================================================

#include <sigma_types.h>


#define INPUT_QUEUE_DEPTH   1024

typedef struct {
    uint32_t timestamp_tsc;
    uint8_t  type; // Mouse/Key/Touch
    int32_t  val1, val2;
} FastInputEvent;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Zero-Latency Input Pipeline
void input_pipeline_init(void);

// Interrupt Handler: Capture raw HID report at 8000Hz+
void input_pipeline_irq_handler(void* raw_report);

// Synthesis: Combine multiple HID events into a single frame delta
void input_pipeline_synthesize(FastInputEvent* out);

// Direct Path: Push input state directly to GraphicsBridge (S04)
void input_pipeline_push_to_vram(void);

// Predictive Input: Use S13 Sentience to "guess" mouse path (Reflex parity)
void input_pipeline_predict_motion(int* px, int* py);

// Report end-to-end latency (Click-to-Blit) in microseconds
uint32_t input_pipeline_audit_latency(void);




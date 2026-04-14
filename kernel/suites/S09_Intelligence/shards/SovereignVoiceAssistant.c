// =============================================================================
// SigmaOS — S09_Intelligence — SovereignVoiceAssistant.c
// On-Device Voice Assistant Engine Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Apple Siri       — on-device privacy-first NLU, wake-word engine
//   • Google Assistant — streaming ASR, context-aware multi-turn dialogue
//   • Cortana (Windows)— OS-deep integration: open apps, query registry
//   • Amazon Alexa     — skill/intent dispatch model for extensibility
// Architecture:
//   • DSP wake-word detector runs always-on in low-power sub-1mW mode
//   • VAD (Voice Activity Detection) gates full ASR to save CPU
//   • On-device ASR via local ONNX/Whisper model (AiInferenceEngine)
//   • Intent dispatcher routes commands to registered OS skill handlers
//   • Zero cloud dependency — all inference local via S09 NPU shard
// =============================================================================

#include <sigma_types.h>


#define SIGMA_VOICE_MAX_SKILLS    64
#define SIGMA_VOICE_WAKE_WORD     "Hey Sigma"

// ── Intent Types ──────────────────────────────────────────────────────────────
typedef enum {
    INTENT_OPEN_APP       = 0,
    INTENT_SYSTEM_QUERY   = 1,   // "What's my CPU usage?"
    INTENT_FILE_SEARCH    = 2,
    INTENT_NETWORK_ACTION = 3,   // "Enable VPN"
    INTENT_MEDIA_CONTROL  = 4,   // "Pause music"
    INTENT_CUSTOM_SKILL   = 5,   // Registered third-party skill
} VoiceIntentType;

// ── Intent Result ─────────────────────────────────────────────────────────────
typedef struct {
    VoiceIntentType type;
    char            raw_text[256];  // Transcribed ASR output
    char            entity[64];     // Extracted named entity
    float           confidence;     // ASR + NLU confidence 0.0–1.0
} VoiceIntent;

// ── Skill Registration ────────────────────────────────────────────────────────
typedef struct {
    const char* skill_name;
    const char* trigger_phrase;
    void      (*handler)(VoiceIntent* intent);
} VoiceSkill;

static VoiceSkill skill_registry[SIGMA_VOICE_MAX_SKILLS];
static uint32_t   skill_count = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Start always-on DSP wake-word detector at sub-1mW power mode
void voice_init_wake_detector(void);

// Feed a 16kHz PCM frame into the ASR pipeline
void voice_process_pcm_frame(const int16_t* samples, uint32_t frame_count);

// Parse ASR transcript into a structured VoiceIntent
VoiceIntent voice_parse_intent(const char* transcript);

// Dispatch a resolved intent to the appropriate skill or OS service
void voice_dispatch_intent(VoiceIntent* intent);

// Register a new skill handler (Alexa skill model)
bool voice_register_skill(VoiceSkill* skill);

// TTS: synthesize and play a response string via the audio stack
void voice_speak_response(const char* response_text);




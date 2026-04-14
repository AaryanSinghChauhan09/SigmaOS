// =============================================================================
// SigmaOS — S13_Sentience — SovereignGlobalTranslator.c
// Real-Time On-Device Universal Translation Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Apple Translate — System-wide translation and offline modes
//   • Google Lens     — Real-time visual overlay translation
//   • Windows Live Captions — Real-time audio-to-text translation
// SigmaOS Global Translator:
//   • Zero-Latency Overlay: Translates GUI text in the S02 Compositor layer.
//   • ProAudio Integration: Real-time translation of system audio streams (S04).
//   • Continuity Sync: Shared translation caches across Hive peers (S12).
// =============================================================================

#include <sigma_types.h>


#define MAX_LANGUAGES       128

typedef struct {
    char     lang_code[8];  // "en", "hi", "jp", "es"
    char     friendly_name[32];
    bool     is_offline_ready;
} LanguageProfile;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Global Translator
void translator_init(void);

// Translate a text block locally (Using S09 Intelligence Bridge)
const char* translator_text(const char* text, const char* to_lang);

// GUI Magic: Intercept S02 text blitting and translate on-the-fly
void translator_hook_compositor(void);

// Audio Magic: Real-time STT + Translation of system audio (S13 hook)
void translator_process_audio_stream(void* pcm_data, uint32_t len);

// Update local dictionary models via S12 Mesh Update
bool translator_update_model(const char* lang_code);

// Sync user-specific translation history across Hive devices (S12)
void translator_sync_mesh(void);




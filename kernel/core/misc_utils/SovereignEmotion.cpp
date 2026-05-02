#include "../../../include/SovereignLibC.h"
#include "../../../include/sigma_types.h"
#include "sigma_emotion.h"
#include "sigma_hal.h"
#include "sigma_universal_ui.h"
#include "sigma_focus.h"

/**
 * SigmaOS Sovereign Emotion UX
 * Implements a Cognitive State Empathy (CSE) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal psychological heuristics.
 */

/* --- Sovereign Emotion Manager (OOPS Isolation) --- */
static struct {
    sigma_emotion_state_t current_state;
    uint32_t state_dwell_time;
    uint32_t dwell_threshold;
} SovereignEmotionManager = {
    .current_state = EMOTION_STATE_NEUTRAL,
    .state_dwell_time = 0,
    .dwell_threshold = 5
};

extern "C" void emotion_init() {
    sigma_log("[EMOTION] Initializing Sovereign Emotion UX Engine (OOPS Isolation)...");
}

extern "C" void emotion_update_state(sigma_emotion_state_t new_state) {
    if (SovereignEmotionManager.current_state == new_state) {
        SovereignEmotionManager.state_dwell_time = 0;
        return;
    }
    
    SovereignEmotionManager.state_dwell_time++;
    if (SovereignEmotionManager.state_dwell_time < SovereignEmotionManager.dwell_threshold) return;
    
    SovereignEmotionManager.current_state = new_state;
    SovereignEmotionManager.state_dwell_time = 0;
    sigma_printf("[EMOTION] CSE: Confirmed user state transition to %d.\n", (int)new_state);
    
    if (new_state == EMOTION_STATE_FRUSTRATED) {
        sigma_log("[EMOTION] CSE: Stress threshold reached. Cooling UI palette.");
        universalui_set_theme(UI_THEME_LIGHT_GLASS);
    } else if (new_state == EMOTION_STATE_FOCUSED) {
        sigma_log("[EMOTION] CSE: Focus state confirmed. Engaging deep-work mode.");
        focus_engage(0, 60); 
    }
}

extern "C" sigma_emotion_state_t emotion_get_current_state() {
    return SovereignEmotionManager.current_state;
}

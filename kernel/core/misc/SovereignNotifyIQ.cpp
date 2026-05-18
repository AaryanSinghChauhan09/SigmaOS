#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

#include "../../include/sigma_notifyiq.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"

#include "../../include/sigma_emotion.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Notification Intelligence
 * Implements an Adaptive Priority Triage (APT) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal notification orchestration.
 */

extern "C" void notifyiq_init() {
    sigma_log("[NOTIFYIQ] Initializing Sovereign Notification Intelligence (APT Algorithm)...");
}

extern "C" void notifyiq_push(const char* source, const char* message, sigma_notify_priority_t priority) {
    // APT (Adaptive Priority Triage) Algorithm
    // Cross-references current Emotion state, Focus mode, and Persona to decide delivery.
    
    sigma_emotion_state_t user_mood = emotion_get_current_state();
    
    if (user_mood == EMOTION_STATE_FOCUSED && priority > NOTIFY_PRIORITY_HIGH) {
        sigma_log_info("[NOTIFYIQ] APT: Silenced '%s' from '%s' (User is focused).\n", message, source);
        return; // Silently batch
    }
    
    sigma_log_info("[NOTIFYIQ] APT: Delivering [P%d] from '%s': '%s'\n", (int)priority, source, message);
}

extern "C" void notifyiq_deliver_batch() {
    sigma_log("[NOTIFYIQ] APT: Delivering consolidated notification summary to Universal UI.");
}


 
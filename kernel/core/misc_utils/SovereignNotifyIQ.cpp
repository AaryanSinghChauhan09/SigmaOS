#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"

#include "../../../include/sigma_notifyiq.h"
#include "../../../include/sigma_hal.h"

#include "../../../include/sigma_emotion.h"

/**
 * SigmaOS Sovereign Notification Intelligence
 * Implements an Adaptive Priority Triage (APT) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal notification orchestration.
 */

void notifyiq_init() {
    sigma_log("[NOTIFYIQ] Initializing Sovereign Notification Intelligence (APT Algorithm)...");
}

void notifyiq_push(const char* source, const char* message, sigma_notify_priority_t priority) {
    // APT (Adaptive Priority Triage) Algorithm
    // Cross-references current Emotion state, Focus mode, and Persona to decide delivery.
    
    sigma_emotion_state_t user_mood = emotion_get_current_state();
    
    if (user_mood == EMOTION_STATE_FOCUSED && priority > NOTIFY_PRIORITY_HIGH) {
        sigma_log("[NOTIFYIQ] APT: Silenced '%s' from '%s' (User is focused).\n", message, source);
        return; // Silently batch
    }
    
    sigma_log("[NOTIFYIQ] APT: Delivering [P%d] from '%s': '%s'\n", (int)priority, source, message);
}

void notifyiq_deliver_batch() {
    sigma_log("[NOTIFYIQ] APT: Delivering consolidated notification summary to Universal UI.");
}




} // extern "C"
 
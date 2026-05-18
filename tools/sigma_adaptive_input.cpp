/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA ADAPTIVE INPUT (sigma_adaptive_input) v1.0
 * =========================================================================
 * Mission: Voice, gesture, and haptic controls.
 * Inspiration: Windows Accessibility + Android Haptics.
 * Principle: Multi-modal sovereign input parsing.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

class SigmaAdaptiveInput : public SigmaObject, public SigmaSingleton<SigmaAdaptiveInput> {
    friend class SigmaSingleton<SigmaAdaptiveInput>;
public:
    const char* type_name() const noexcept override { return "SigmaAdaptiveInput"; }

    void init() {
        m_voice_enabled = false;
        m_gesture_enabled = false;
        sigma_printf("[ADAPT_IN] Sigma Adaptive Input v1.0 initialized.");
    }

    void toggle_voice(bool enable) {
        m_voice_enabled = enable;
        sigma_printf("[ADAPT_IN] Voice Control: %s", enable ? "ON" : "OFF");
    }

    void toggle_gesture(bool enable) {
        m_gesture_enabled = enable;
        sigma_printf("[ADAPT_IN] Gesture Tracking: %s", enable ? "ON" : "OFF");
    }

    void trigger_haptic(sigma_u32 duration_ms, sigma_u8 intensity) {
        sigma_printf("[ADAPT_IN] Triggering Haptic Feedback | Duration: %ums | Intensity: %u%%", duration_ms, intensity);
    }

private:
    SigmaAdaptiveInput() : m_voice_enabled(false), m_gesture_enabled(false) {}
    bool m_voice_enabled;
    bool m_gesture_enabled;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void adaptin_init()                                         { SigmaOS::Tools::SigmaAdaptiveInput::getInstance().init(); }
void adaptin_voice(sigma_u8 enable)                         { SigmaOS::Tools::SigmaAdaptiveInput::getInstance().toggle_voice(enable != 0); }
void adaptin_gesture(sigma_u8 enable)                       { SigmaOS::Tools::SigmaAdaptiveInput::getInstance().toggle_gesture(enable != 0); }
void adaptin_haptic(sigma_u32 ms, sigma_u8 intensity)       { SigmaOS::Tools::SigmaAdaptiveInput::getInstance().trigger_haptic(ms, intensity); }
}

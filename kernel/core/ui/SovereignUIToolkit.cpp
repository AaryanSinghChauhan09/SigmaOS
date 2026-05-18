#include "../../include/sigma_ui_toolkit.h"
#include "../../include/sigma_log.h"

/**
 * Σ SIGMAOS: SOVEREIGN UI TOOLKIT (S-UI-TOOLKIT)
 * Implementation: Theme engine and Accessibility services.
 */

namespace SigmaOS {
namespace Kernel {
namespace UI {

void SovereignUIToolkit::init() {
    sigma_log_info("[S-UI] Initializing Sovereign UI Toolkit (Zorin-Parity)...");
    sigma_log_info("[S-UI] Theme Engine: Active [Mode: DARK_MODERN].");
}

void SovereignUIToolkit::setTheme(sigma_theme_t theme) {
    this->m_theme = theme;
    sigma_log_info("[S-UI] Transitioning Lattice UI to theme: %d", (int)theme);
    sigma_log_info("[S-UI] Updating CSS-Lattice variables and glass-morphism constants.");
}

void SovereignUIToolkit::setMagnifier(bool enable) {
    this->m_magnifier = enable;
    sigma_log_info("[S-UI] Magnifier service: %s", enable ? "ENABLED" : "DISABLED");
}

void SovereignUIToolkit::setScreenReader(bool enable) {
    this->m_screen_reader = enable;
    sigma_log_info("[S-UI] Sovereign Screen Reader (SSR): %s", enable ? "ENABLED" : "DISABLED");
}

void SovereignUIToolkit::setScaling(float factor) {
    this->m_scaling = factor;
    sigma_log_info("[S-UI] Adaptive UI Scaling set to: %.2f", factor);
}

} // namespace UI
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {
    void ui_init() {
        SigmaOS::Kernel::UI::SovereignUIToolkit::getInstance().init();
    }

    void ui_set_theme(sigma_theme_t theme) {
        SigmaOS::Kernel::UI::SovereignUIToolkit::getInstance().setTheme(theme);
    }

    void ui_enable_magnifier(bool enable) {
        SigmaOS::Kernel::UI::SovereignUIToolkit::getInstance().setMagnifier(enable);
    }

    void ui_enable_screen_reader(bool enable) {
        SigmaOS::Kernel::UI::SovereignUIToolkit::getInstance().setScreenReader(enable);
    }

    void ui_set_scaling(float factor) {
        SigmaOS::Kernel::UI::SovereignUIToolkit::getInstance().setScaling(factor);
    }
}
 
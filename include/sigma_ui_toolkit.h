/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN UI TOOLKIT (S-UI-TOOLKIT)
 * =========================================================================
 * Mission: Theme engine and Accessibility services (Magnifier, Screen Reader).
 * Inspired by Zorin / Elementary UX.
 * =========================================================================
 */

#ifndef SIGMA_UI_TOOLKIT_H
#define SIGMA_UI_TOOLKIT_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    THEME_DARK_MODERN,
    THEME_LIGHT_PRISTINE,
    THEME_GLASS_IMMERSIVE,
    THEME_HIGH_CONTRAST
} sigma_theme_t;

/* --- UI Toolkit Primitives --- */
void      ui_init(void);
void      ui_set_theme(sigma_theme_t theme);
void      ui_enable_magnifier(bool enable);
void      ui_enable_screen_reader(bool enable);
void      ui_set_scaling(float factor);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace UI {

class SovereignUIToolkit {
public:
    static SovereignUIToolkit& getInstance() {
        static SovereignUIToolkit instance;
        return instance;
    }

    void init();
    void setTheme(sigma_theme_t theme);
    void setMagnifier(bool enable);
    void setScreenReader(bool enable);
    void setScaling(float factor);

private:
    SovereignUIToolkit() : m_theme(THEME_DARK_MODERN), m_magnifier(false), m_screen_reader(false), m_scaling(1.0f) {}
    sigma_theme_t m_theme;
    bool          m_magnifier;
    bool          m_screen_reader;
    float         m_scaling;
};

} // namespace UI
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_UI_TOOLKIT_H */

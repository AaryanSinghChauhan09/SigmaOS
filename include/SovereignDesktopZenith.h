#ifndef SOVEREIGN_DESKTOP_ZENITH_H
#define SOVEREIGN_DESKTOP_ZENITH_H

#include "./core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Desktop {

// --- SOVEREIGN WORKSPACE & WINDOWS ---
class SovereignWindow : public SigmaObject {
protected:
    int m_x, m_y, m_w, m_h;
    const char* m_title;
public:
    SovereignWindow(const char* t, int x, int y, int w, int h)
        : m_title(t), m_x(x), m_y(y), m_w(w), m_h(h) {}
    
    virtual void OnRender() = 0;
};

class SovereignTerminalWindow : public SovereignWindow {
public:
    SovereignTerminalWindow() : SovereignWindow("Omni-Shell Zenith", 50, 50, 800, 600) {}
    const char* type_name() const noexcept override { return "SovereignTerminalWindow"; }
    void OnRender() override;
};

class SovereignZenithDesktop : public SigmaObject {
private:
    bool m_gui_active;
public:
    SovereignZenithDesktop() : m_gui_active(false) {}
    const char* type_name() const noexcept override { return "SovereignZenithDesktop"; }
    
    void ToggleGUI() { m_gui_active = !m_gui_active; }
    bool IsGUIActive() const { return m_gui_active; }
    
    void RenderDesktop(); // Native DOM Rasterization through SovereignUI
};

} // namespace Desktop
} // namespace SigmaOS

#endif

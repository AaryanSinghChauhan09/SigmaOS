#ifndef SOVEREIGN_DESKTOP_ZENITH_H
#define SOVEREIGN_DESKTOP_ZENITH_H

#include "SigmaOOP.h"

/* S Territory Initiation */

// --- SOVEREIGN WORKSPACE & WINDOWS ---
CLASS_DECLARE(SovereignWindow) { 
    SigmaObject_t core;
    int m_x, m_y, m_w, m_h;
    const char* m_title;
    VIRTUAL(void, OnRender, struct SovereignWindow* self);
};

CLASS_DECLARE(SovereignTerminalWindow) { 
    SovereignWindow_t core;
    VIRTUAL(void, OnRender, struct SovereignTerminalWindow* self);
};

CLASS_DECLARE(SovereignZenithDesktop) { 
    SigmaObject_t core;
    sigma_bool m_gui_active;
    VIRTUAL(void, ToggleGUI, struct SovereignZenithDesktop* self);
    VIRTUAL(sigma_bool, IsGUIActive, struct SovereignZenithDesktop* self);
    VIRTUAL(void, RenderDesktop, struct SovereignZenithDesktop* self); // Native DOM Rasterization
};

/* S Territory Termination */

#endif

/**
 * =========================================================================
 * Σ ZENITH DEVELOPER SDK (v0.1)
 * =========================================================================
 * The official C++ toolkit for building sovereign, containerized UI apps
 * on SigmaOS. Bypasses Wayland/X11 and talks directly to the 
 * Sovereign Orchestrator Sandbox.
 * =========================================================================
 */

#ifndef ZENITH_SDK_H
#define ZENITH_SDK_H

#include <sigma_libc.h>

namespace Zenith {

// ---------------------------------------------------------
// 1. Core UI Primitives
// ---------------------------------------------------------

struct Rect {
    sigma_i32 x, y;
    sigma_u32 width, height;
};

// ---------------------------------------------------------
// 2. Declarative UI Toolkit
// ---------------------------------------------------------
namespace UI {

class Widget {
public:
    virtual ~Widget() {}
    virtual void render(sigma_u8* buffer, sigma_u32 buf_width) = 0;
};

class Button : public Widget {
    Rect bounds;
    const char* label;
public:
    Button(Rect b, const char* text) : bounds(b), label(text) {}
    void render(sigma_u8* buffer, sigma_u32 buf_width) override {
        // Issue an IPC drawing command or directly paint to the sandboxed backing buffer
        // For SDK purposes, we wrap this in a mock syscall log
        sys_print("[Zenith-SDK] Rendering Button '%s' at (%d,%d) [%ux%u]\n", 
                  label, bounds.x, bounds.y, bounds.width, bounds.height);
    }
};

class Label : public Widget {
    Rect bounds;
    const char* text;
public:
    Label(Rect b, const char* t) : bounds(b), text(t) {}
    void render(sigma_u8* buffer, sigma_u32 buf_width) override {
        sys_print("[Zenith-SDK] Rendering Label '%s' at (%d,%d)\n", text, bounds.x, bounds.y);
    }
};

} // namespace UI

// ---------------------------------------------------------
// 3. Application Lifecycle
// ---------------------------------------------------------

class Application {
private:
    const char* m_app_name;
    sigma_u32   m_container_id;
    sigma_u32   m_window_id;

public:
    Application(const char* name) : m_app_name(name) {
        sys_print("[Zenith-SDK] Initializing Application: %s\n", m_app_name);
        
        // Connect to Sandbox Bridge to secure the container shard
        // (Mock IPC call to sigma_sandbox_bridge)
        m_container_id = sys_ipc_send(4, 1, (const void*)m_app_name, 0); 
        
        sys_print("[Zenith-SDK] Secured Orchestrator Container Shard: %u\n", m_container_id);
    }

    void createWindow(sigma_u32 width, sigma_u32 height) {
        sys_print("[Zenith-SDK] Requesting Native Window [%ux%u] for Container %u\n", 
                  width, height, m_container_id);
        m_window_id = 1; // Mock Window ID
    }

    void addWidget(UI::Widget* widget) {
        // In reality, this would append to a scene graph
        widget->render(SIGMA_NULL, 0); 
    }

    void run() {
        sys_print("[Zenith-SDK] Entering Sovereign Event Loop...\n");
        // Block and process IPC events from Compositor
    }
};

} // namespace Zenith

#endif /* ZENITH_SDK_H */

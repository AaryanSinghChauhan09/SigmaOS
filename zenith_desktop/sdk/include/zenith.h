/**
 * =========================================================================
 * Σ ZENITH NATIVE UI TOOLKIT — v0.3
 * =========================================================================
 * A zero-dependency, sovereign C++ UI framework for SigmaOS applications.
 *
 * CRITICAL DESIGN RULE:
 *   This toolkit MUST NOT depend on GTK, Qt, wxWidgets, SDL, SFML, or any
 *   external graphics library. All rendering is performed via direct IPC
 *   calls to the Zenith Compositor's backing buffer API.
 *
 * Architecture:
 *   App → Zenith SDK → IPC draw commands → Compositor → Framebuffer
 *
 * Inspired by:
 *   - Elementary OS AppKit: Clean, opinionated widget abstraction.
 *   - Wayland protocol: Direct compositor IPC, no X11 indirection.
 *   - Haiku OS: Native C++ UI toolkit with no platform intermediary.
 *   - Plan 9 draw(3): Minimal drawing primitive surface.
 *
 * Widget tree: App owns a scene graph of Widget* nodes.
 * Layout: Flexbox-style via Container with Row/Column arrangement.
 * Events: IPC messages from Compositor are dispatched to focused widgets.
 * =========================================================================
 */

#ifndef ZENITH_SDK_H
#define ZENITH_SDK_H

#include <sigma_libc.h>

namespace Zenith {

// =========================================================================
// SECTION 1: PRIMITIVE TYPES
// =========================================================================

struct Rect {
    sigma_i32 x, y;
    sigma_u32 width, height;
};

struct Color {
    sigma_u8 r, g, b, a;  // RGBA — alpha 0=transparent, 255=opaque

    static Color fromHex(sigma_u32 hex) {
        return { (sigma_u8)((hex >> 16) & 0xFF),
                 (sigma_u8)((hex >>  8) & 0xFF),
                 (sigma_u8)((hex      ) & 0xFF),
                 (sigma_u8)((hex >> 24) & 0xFF) };
    }
};

// Sovereign color palette — no dependency on CSS or platform theme APIs
namespace Colors {
    static const Color Background    = {  18,  18,  24, 255 };
    static const Color Surface       = {  30,  30,  42, 255 };
    static const Color Accent        = {  99, 179, 237, 255 };
    static const Color AccentHover   = { 129, 199, 255, 255 };
    static const Color TextPrimary   = { 230, 230, 240, 255 };
    static const Color TextSecondary = { 140, 140, 160, 255 };
    static const Color Success       = {  72, 199, 142, 255 };
    static const Color Warning       = { 255, 178,  68, 255 };
    static const Color Error         = { 255,  90,  90, 255 };
}

// =========================================================================
// SECTION 2: EVENT SYSTEM
// =========================================================================

typedef enum {
    EVENT_NONE        = 0,
    EVENT_MOUSE_CLICK = 1,
    EVENT_MOUSE_HOVER = 2,
    EVENT_KEY_DOWN    = 3,
    EVENT_KEY_UP      = 4,
    EVENT_FOCUS       = 5,
    EVENT_BLUR        = 6,
    EVENT_RESIZE      = 7,
} EventType;

struct InputEvent {
    EventType type;
    sigma_i32 mouse_x, mouse_y;
    sigma_u32 key_code;
    sigma_u32 modifiers;   // Bitmask: SHIFT=1, CTRL=2, ALT=4, META=8
};

typedef void (*EventCallback)(const InputEvent& event, void* user_data);

// =========================================================================
// SECTION 3: BASE WIDGET
// =========================================================================

namespace UI {

#define ZENITH_MAX_CHILDREN  32

class Widget {
public:
    Rect         bounds;
    Color        bg_color;
    sigma_bool   visible;
    sigma_bool   focused;
    EventCallback on_click;
    void*         callback_data;

    Widget() : bounds{0,0,0,0}, bg_color(Colors::Surface),
               visible(SIGMA_TRUE), focused(SIGMA_FALSE),
               on_click(SIGMA_NULL), callback_data(SIGMA_NULL) {}

    virtual ~Widget() {}

    /**
     * render() — emit draw commands to the Compositor via IPC.
     * In production, calls sigma_compositor_draw_rect() syscall.
     */
    virtual void render() {
        sys_print("[Zenith] Widget at (%d,%d) [%ux%u]\n",
                  bounds.x, bounds.y, bounds.width, bounds.height);
    }

    virtual void handleEvent(const InputEvent& event) {
        if (event.type == EVENT_MOUSE_CLICK && on_click) {
            on_click(event, callback_data);
        }
    }

    void setOnClick(EventCallback cb, void* data = SIGMA_NULL) {
        on_click = cb; callback_data = data;
    }
};

// =========================================================================
// SECTION 4: CONCRETE WIDGETS
// =========================================================================

/**
 * Button — Pressable control with label and hover state.
 */
class Button : public Widget {
    const char* m_label;
    sigma_bool  m_hovered;
public:
    Button(Rect b, const char* label)
        : m_label(label), m_hovered(SIGMA_FALSE) { bounds = b; bg_color = Colors::Accent; }

    void render() override {
        Color draw_color = m_hovered ? Colors::AccentHover : bg_color;
        sys_print("[Zenith] Button '%s' at (%d,%d) [%ux%u] color=#%02x%02x%02x\n",
                  m_label, bounds.x, bounds.y, bounds.width, bounds.height,
                  draw_color.r, draw_color.g, draw_color.b);
    }

    void handleEvent(const InputEvent& e) override {
        if (e.type == EVENT_MOUSE_HOVER) m_hovered = SIGMA_TRUE;
        else if (e.type == EVENT_BLUR)   m_hovered = SIGMA_FALSE;
        Widget::handleEvent(e);
    }
};

/**
 * Label — Read-only text display.
 */
class Label : public Widget {
    const char* m_text;
    Color       m_text_color;
public:
    Label(Rect b, const char* text, Color color = Colors::TextPrimary)
        : m_text(text), m_text_color(color) { bounds = b; bg_color = Colors::Background; }

    void render() override {
        sys_print("[Zenith] Label '%s' at (%d,%d) color=#%02x%02x%02x\n",
                  m_text, bounds.x, bounds.y,
                  m_text_color.r, m_text_color.g, m_text_color.b);
    }
};

/**
 * TextInput — Editable single-line text field with cursor tracking.
 */
class TextInput : public Widget {
    char     m_buffer[512];
    sigma_u32 m_cursor_pos;
    sigma_u32 m_len;
    const char* m_placeholder;
public:
    TextInput(Rect b, const char* placeholder = "Type here...")
        : m_cursor_pos(0), m_len(0), m_placeholder(placeholder) {
        bounds = b;
        bg_color = Colors::Surface;
        m_buffer[0] = '\0';
    }

    const char* getText() const { return m_buffer; }
    void        clear()         { m_buffer[0] = '\0'; m_cursor_pos = 0; m_len = 0; }

    void render() override {
        const char* display = (m_len > 0) ? m_buffer : m_placeholder;
        sys_print("[Zenith] TextInput '%s' cursor=%u at (%d,%d)\n",
                  display, m_cursor_pos, bounds.x, bounds.y);
    }

    void handleEvent(const InputEvent& e) override {
        if (e.type == EVENT_KEY_DOWN && e.key_code >= 32 && e.key_code < 127) {
            if (m_len < 511) {
                m_buffer[m_len++] = (char)e.key_code;
                m_buffer[m_len]   = '\0';
                m_cursor_pos      = m_len;
            }
        }
    }
};

/**
 * ProgressBar — Shows completion percentage (0–100).
 */
class ProgressBar : public Widget {
    sigma_u32 m_value;  // 0–100
public:
    ProgressBar(Rect b, sigma_u32 initial_value = 0)
        : m_value(initial_value < 100 ? initial_value : 100) { bounds = b; }

    void setValue(sigma_u32 v) { m_value = (v <= 100) ? v : 100; }
    sigma_u32 getValue() const { return m_value; }

    void render() override {
        sigma_u32 filled = (bounds.width * m_value) / 100;
        sys_print("[Zenith] ProgressBar %u%% [%u/%u px filled] at (%d,%d)\n",
                  m_value, filled, bounds.width, bounds.x, bounds.y);
    }
};

/**
 * ListView — Scrollable list of string items.
 */
class ListView : public Widget {
    const char* m_items[64];
    sigma_u32   m_count;
    sigma_u32   m_selected;
    sigma_u32   m_scroll_offset;
public:
    ListView(Rect b)
        : m_count(0), m_selected(0), m_scroll_offset(0) { bounds = b; }

    void addItem(const char* item) {
        if (m_count < 64) m_items[m_count++] = item;
    }

    sigma_u32   getSelected() const        { return m_selected; }
    const char* getSelectedItem() const    { return m_count > 0 ? m_items[m_selected] : SIGMA_NULL; }

    void render() override {
        sys_print("[Zenith] ListView [%u items, selected=%u] at (%d,%d):\n",
                  m_count, m_selected, bounds.x, bounds.y);
        sigma_u32 visible_rows = bounds.height / 24;  // 24px per row
        for (sigma_u32 i = m_scroll_offset; i < m_count && i < m_scroll_offset + visible_rows; i++) {
            const char* marker = (i == m_selected) ? "► " : "  ";
            sys_print("  %s%s\n", marker, m_items[i]);
        }
    }

    void handleEvent(const InputEvent& e) override {
        if (e.type == EVENT_KEY_DOWN) {
            if (e.key_code == 0x26 /* UP   */ && m_selected > 0)            m_selected--;
            if (e.key_code == 0x28 /* DOWN */ && m_selected < m_count - 1)  m_selected++;
        }
    }
};

/**
 * Container — Flexbox-style row/column layout for grouping widgets.
 */
class Container : public Widget {
public:
    typedef enum { ROW, COLUMN } Direction;

private:
    Widget*   m_children[ZENITH_MAX_CHILDREN];
    sigma_u32 m_count;
    Direction m_direction;
    sigma_u32 m_gap;  // Gap in pixels between children

public:
    Container(Rect b, Direction dir = COLUMN, sigma_u32 gap = 8)
        : m_count(0), m_direction(dir), m_gap(gap) { bounds = b; }

    void add(Widget* child) {
        if (m_count < ZENITH_MAX_CHILDREN) m_children[m_count++] = child;
    }

    void render() override {
        sys_print("[Zenith] Container (%s) [%u children] at (%d,%d)\n",
                  m_direction == ROW ? "ROW" : "COLUMN",
                  m_count, bounds.x, bounds.y);

        sigma_i32 cursor = (m_direction == ROW) ? bounds.x : bounds.y;
        for (sigma_u32 i = 0; i < m_count; i++) {
            Widget* child = m_children[i];
            if (!child || !child->visible) continue;

            if (m_direction == ROW) {
                child->bounds.y = bounds.y;
                child->bounds.x = cursor;
                cursor += (sigma_i32)child->bounds.width + (sigma_i32)m_gap;
            } else {
                child->bounds.x = bounds.x;
                child->bounds.y = cursor;
                cursor += (sigma_i32)child->bounds.height + (sigma_i32)m_gap;
            }
            child->render();
        }
    }

    void handleEvent(const InputEvent& e) override {
        for (sigma_u32 i = 0; i < m_count; i++) {
            if (m_children[i] && m_children[i]->visible)
                m_children[i]->handleEvent(e);
        }
    }
};

} // namespace UI

// =========================================================================
// SECTION 5: APPLICATION LIFECYCLE
// =========================================================================

class Application {
    const char* m_app_name;
    sigma_u32   m_container_id;
    sigma_u32   m_window_width;
    sigma_u32   m_window_height;
    UI::Widget* m_scene_root;

public:
    explicit Application(const char* name)
        : m_app_name(name), m_container_id(0),
          m_window_width(0), m_window_height(0),
          m_scene_root(SIGMA_NULL) {
        sys_print("[Zenith-SDK] Initializing '%s' — sovereign, containerized.\n", m_app_name);
        // Request container shard from Orchestrator via IPC
        m_container_id = sys_ipc_send(4 /* ORCHESTRATOR_SHARD */, 1, m_app_name, 0);
        sys_print("[Zenith-SDK] Container shard assigned: %u\n", m_container_id);
    }

    void createWindow(sigma_u32 width, sigma_u32 height) {
        m_window_width  = width;
        m_window_height = height;
        sys_print("[Zenith-SDK] Window [%ux%u] created for container %u.\n",
                  width, height, m_container_id);
    }

    void setRootWidget(UI::Widget* root) { m_scene_root = root; }

    void addWidget(UI::Widget* widget) {
        // Legacy convenience — renders immediately (no scene root set)
        widget->render();
    }

    void dispatchEvent(const InputEvent& e) {
        if (m_scene_root) m_scene_root->handleEvent(e);
    }

    void run() {
        sys_print("[Zenith-SDK] Entering sovereign IPC event loop for '%s'...\n", m_app_name);
        if (m_scene_root) m_scene_root->render();
        // Production: block on sigma_ipc_wait(), dispatch events to widgets
    }
};

} // namespace Zenith

#endif /* ZENITH_SDK_H */

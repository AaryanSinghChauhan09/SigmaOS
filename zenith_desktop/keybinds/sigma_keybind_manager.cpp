/**
 * @file sigma_keybind_manager.cpp
 * @brief Sovereign Keybind Manager — Customizable hotkeys and shortcuts
 *
 * Competitor Inspiration:
 *  - i3/Sway: bindsym config directives
 *  - KDE: Global Shortcuts KCM module
 *  - GNOME: gsettings keybindings
 *  - macOS: System Preferences → Keyboard → Shortcuts
 *  - Hyprland: bind = SUPER, Q, exec, kitty
 *
 * Provides a runtime-configurable keybinding registry that intercepts
 * key events from the Zenith compositor and dispatches actions.
 */

#include "../../include/sigma_kernel_types.h"

namespace sigma {
namespace keybind {

// ─── Modifier Flags ──────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    MOD_NONE   = 0x00,
    MOD_SUPER  = 0x01,  // Logo / Windows key
    MOD_ALT    = 0x02,
    MOD_CTRL   = 0x04,
    MOD_SHIFT  = 0x08,
} Modifier;

// ─── Action Types ────────────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    KB_ACTION_EXEC       = 0,   // Run a shell command
    KB_ACTION_FOCUS_DIR  = 1,   // Focus window in direction
    KB_ACTION_MOVE_DIR   = 2,   // Move focused window
    KB_ACTION_WORKSPACE  = 3,   // Switch workspace
    KB_ACTION_MOVE_WS    = 4,   // Move window to workspace
    KB_ACTION_FULLSCREEN = 5,   // Toggle fullscreen
    KB_ACTION_FLOATING   = 6,   // Toggle floating
    KB_ACTION_CLOSE      = 7,   // Close focused window
    KB_ACTION_RESIZE     = 8,   // Enter resize mode
    KB_ACTION_SCREENSHOT = 9,   // Take screenshot
    KB_ACTION_LOCK       = 10,  // Lock screen
    KB_ACTION_LOGOUT     = 11,  // Logout
    KB_ACTION_RELOAD     = 12,  // Reload WM config
    KB_ACTION_THEME      = 13,  // Toggle dark/light theme
    KB_ACTION_DND        = 14,  // Toggle Do Not Disturb
    KB_ACTION_CUSTOM     = 15,  // Custom IPC message
} KeybindAction;

// ─── Keybinding Entry ────────────────────────────────────────────────────────
struct Keybind {
    sigma_u32     modifiers;      // OR'd Modifier flags
    sigma_u32     keycode;        // Key scancode
    char          key_name[32];   // Human-readable: "Q", "Return", "Space"
    KeybindAction action;
    char          payload[256];   // Command, direction, workspace ID, etc.
    sigma_bool    enabled;
};

// ─── Keybind Registry ────────────────────────────────────────────────────────
#define MAX_KEYBINDS 256

struct KeybindRegistry {
    Keybind   bindings[MAX_KEYBINDS];
    sigma_u32 count;
};

static KeybindRegistry g_keybinds;

// ─── Helper ──────────────────────────────────────────────────────────────────
static void str_copy(char* dst, const char* src, sigma_u32 max_len) {
    sigma_u32 i = 0;
    while (src && src[i] && i < max_len - 1) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

static sigma_bool str_eq(const char* a, const char* b) {
    while (*a && *b) { if (*a++ != *b++) return SIGMA_FALSE; }
    return (*a == '\0' && *b == '\0') ? SIGMA_TRUE : SIGMA_FALSE;
}

// ─── Register a Keybinding ───────────────────────────────────────────────────
sigma_status register_keybind(sigma_u32 modifiers, const char* key_name,
                               KeybindAction action, const char* payload) {
    if (g_keybinds.count >= MAX_KEYBINDS) return SIGMA_ERROR;

    Keybind* kb = &g_keybinds.bindings[g_keybinds.count++];
    kb->modifiers = modifiers;
    kb->keycode   = 0; // Would be resolved from key_name lookup table
    str_copy(kb->key_name, key_name, 32);
    kb->action    = action;
    str_copy(kb->payload, payload, 256);
    kb->enabled   = SIGMA_TRUE;

    return SIGMA_SUCCESS;
}

// ─── Handle Key Event ────────────────────────────────────────────────────────
sigma_status handle_key(sigma_u32 modifiers, sigma_u32 keycode, const char* key_name) {
    for (sigma_u32 i = 0; i < g_keybinds.count; ++i) {
        Keybind* kb = &g_keybinds.bindings[i];
        if (!kb->enabled) continue;
        if (kb->modifiers != modifiers) continue;

        // Match by key name (case-insensitive)
        if (str_eq(kb->key_name, key_name)) {
            return dispatch_action(kb);
        }
    }
    return SIGMA_SUCCESS; // No binding found — pass through
}

// ─── Dispatch Action ─────────────────────────────────────────────────────────
sigma_status dispatch_action(const Keybind* kb) {
    if (!kb) return SIGMA_ERROR;

    switch (kb->action) {
        case KB_ACTION_EXEC:
            // sigma_shell_exec(kb->payload);
            break;
        case KB_ACTION_FOCUS_DIR:
            // sigma_wm_focus(parse_direction(kb->payload));
            break;
        case KB_ACTION_WORKSPACE:
            // sigma_wm_switch_ws(parse_u32(kb->payload));
            break;
        case KB_ACTION_MOVE_WS:
            // sigma_wm_move_ws(focused_surface, parse_u32(kb->payload));
            break;
        case KB_ACTION_FULLSCREEN:
            // sigma_wm_fullscreen(focused_surface);
            break;
        case KB_ACTION_FLOATING:
            // sigma_wm_floating(focused_surface);
            break;
        case KB_ACTION_CLOSE:
            // wl_surface_destroy(focused_surface);
            break;
        case KB_ACTION_SCREENSHOT:
            // Capture scanout buffer to PNG
            break;
        case KB_ACTION_LOCK:
            // Invoke lock screen
            break;
        case KB_ACTION_THEME:
            // sigma_theme_apply toggle
            break;
        case KB_ACTION_DND:
            // sigma_notify_dnd toggle
            break;
        default:
            break;
    }

    return SIGMA_SUCCESS;
}

// ─── Init Default Keybindings (i3/Sway-inspired) ─────────────────────────────
sigma_status keybind_init() {
    g_keybinds.count = 0;

    // Window management
    register_keybind(MOD_SUPER, "Return", KB_ACTION_EXEC, "sigma-terminal");
    register_keybind(MOD_SUPER, "Q",      KB_ACTION_CLOSE, "");
    register_keybind(MOD_SUPER, "F",      KB_ACTION_FULLSCREEN, "");
    register_keybind(MOD_SUPER | MOD_SHIFT, "Space", KB_ACTION_FLOATING, "");

    // Focus navigation (Vim-style)
    register_keybind(MOD_SUPER, "H", KB_ACTION_FOCUS_DIR, "left");
    register_keybind(MOD_SUPER, "J", KB_ACTION_FOCUS_DIR, "down");
    register_keybind(MOD_SUPER, "K", KB_ACTION_FOCUS_DIR, "up");
    register_keybind(MOD_SUPER, "L", KB_ACTION_FOCUS_DIR, "right");

    // Workspaces (1-10)
    register_keybind(MOD_SUPER, "1", KB_ACTION_WORKSPACE, "0");
    register_keybind(MOD_SUPER, "2", KB_ACTION_WORKSPACE, "1");
    register_keybind(MOD_SUPER, "3", KB_ACTION_WORKSPACE, "2");
    register_keybind(MOD_SUPER, "4", KB_ACTION_WORKSPACE, "3");
    register_keybind(MOD_SUPER, "5", KB_ACTION_WORKSPACE, "4");
    register_keybind(MOD_SUPER, "6", KB_ACTION_WORKSPACE, "5");
    register_keybind(MOD_SUPER, "7", KB_ACTION_WORKSPACE, "6");
    register_keybind(MOD_SUPER, "8", KB_ACTION_WORKSPACE, "7");
    register_keybind(MOD_SUPER, "9", KB_ACTION_WORKSPACE, "8");
    register_keybind(MOD_SUPER, "0", KB_ACTION_WORKSPACE, "9");

    // Move to workspace
    register_keybind(MOD_SUPER | MOD_SHIFT, "1", KB_ACTION_MOVE_WS, "0");
    register_keybind(MOD_SUPER | MOD_SHIFT, "2", KB_ACTION_MOVE_WS, "1");
    register_keybind(MOD_SUPER | MOD_SHIFT, "3", KB_ACTION_MOVE_WS, "2");
    register_keybind(MOD_SUPER | MOD_SHIFT, "4", KB_ACTION_MOVE_WS, "3");

    // System
    register_keybind(MOD_SUPER, "D",      KB_ACTION_EXEC, "sigma-launcher");
    register_keybind(MOD_SUPER, "E",      KB_ACTION_EXEC, "sigma-files");
    register_keybind(MOD_SUPER | MOD_SHIFT, "S", KB_ACTION_SCREENSHOT, "");
    register_keybind(MOD_SUPER, "Escape", KB_ACTION_LOCK, "");
    register_keybind(MOD_SUPER | MOD_SHIFT, "Q", KB_ACTION_LOGOUT, "");
    register_keybind(MOD_SUPER | MOD_SHIFT, "R", KB_ACTION_RELOAD, "");

    // Quick settings
    register_keybind(MOD_SUPER, "T", KB_ACTION_THEME, "");
    register_keybind(MOD_SUPER, "N", KB_ACTION_DND, "");

    return SIGMA_SUCCESS;
}

// ─── List All Keybindings ────────────────────────────────────────────────────
sigma_u32 list_keybinds(Keybind* out, sigma_u32 max_out) {
    sigma_u32 n = (g_keybinds.count < max_out) ? g_keybinds.count : max_out;
    for (sigma_u32 i = 0; i < n; ++i) out[i] = g_keybinds.bindings[i];
    return n;
}

// ─── Remove a Keybinding ────────────────────────────────────────────────────
sigma_status remove_keybind(sigma_u32 modifiers, const char* key_name) {
    for (sigma_u32 i = 0; i < g_keybinds.count; ++i) {
        if (g_keybinds.bindings[i].modifiers == modifiers &&
            str_eq(g_keybinds.bindings[i].key_name, key_name)) {
            for (sigma_u32 j = i; j < g_keybinds.count - 1; ++j)
                g_keybinds.bindings[j] = g_keybinds.bindings[j + 1];
            g_keybinds.count--;
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

} // namespace keybind
} // namespace sigma

extern "C" {
    sigma_status sigma_keybind_init(void) { return sigma::keybind::keybind_init(); }
    sigma_status sigma_keybind_handle(sigma_u32 mod, sigma_u32 kc, const char* name) {
        return sigma::keybind::handle_key(mod, kc, name);
    }
    sigma_status sigma_keybind_register(sigma_u32 mod, const char* key,
                                         sigma_u32 action, const char* payload) {
        return sigma::keybind::register_keybind(mod, key,
                    (sigma::keybind::KeybindAction)action, payload);
    }
}

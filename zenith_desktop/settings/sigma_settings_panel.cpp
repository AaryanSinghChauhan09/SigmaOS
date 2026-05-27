/**
 * @file sigma_settings_panel.cpp
 * @brief Zenith Settings Panel — Unified system configuration GUI
 *
 * Competitor Inspiration:
 *  - GNOME Settings: Flat, category-based settings panels
 *  - KDE System Settings: Deep tree-structured configuration
 *  - macOS System Preferences: Grid of category icons
 *  - elementaryOS: Switchboard plug-in architecture
 *  - Windows 11 Settings: Tabbed sections with search
 *
 * This is the GUI frontend that ties together the Theme Engine,
 * State Manager, Automation Taskmaster, and system configuration.
 * Each panel is a "plug" that can be loaded independently.
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_theme.h"

namespace sigma {
namespace settings {

// ─── Settings Panel Categories ───────────────────────────────────────────────
typedef enum : sigma_u32 {
    CAT_APPEARANCE  = 0,   // Theme, wallpaper, fonts, accent color
    CAT_DESKTOP     = 1,   // Tiling mode, gaps, workspaces, hotcorners
    CAT_DISPLAY     = 2,   // Resolution, refresh rate, multi-monitor
    CAT_SOUND       = 3,   // Volume, input/output devices
    CAT_NETWORK     = 4,   // Wi-Fi, Ethernet, VPN, proxy
    CAT_BLUETOOTH   = 5,   // Paired devices, discovery
    CAT_ACCOUNTS    = 6,   // User accounts, PAM, SSO
    CAT_SECURITY    = 7,   // Firewall, disk encryption, sandbox
    CAT_POWER       = 8,   // Battery, sleep, governor
    CAT_KEYBOARD    = 9,   // Shortcuts, input methods, layouts
    CAT_MOUSE       = 10,  // Sensitivity, acceleration, natural scroll
    CAT_PRINTERS    = 11,  // Printer/scanner configuration
    CAT_DATETIME    = 12,  // Timezone, NTP, formats
    CAT_LANGUAGE    = 13,  // Locale, translations
    CAT_UPDATES     = 14,  // OmniPkg updates, auto-update policy
    CAT_AUTOMATION  = 15,  // Playbook manager GUI
    CAT_ACCESSIBILITY = 16, // Screen reader, high contrast, zoom
    CAT_ABOUT       = 17,  // System info, SigmaOS version
    CAT_COUNT       = 18,
} SettingsCategory;

// ─── Settings Plug (modular panel — elementaryOS Switchboard style) ─────────
struct SettingsPlug {
    SettingsCategory category;
    char             name[64];
    char             icon[128];
    char             description[256];
    sigma_bool       visible;
};

// ─── Setting Value Types ─────────────────────────────────────────────────────
typedef enum : sigma_u32 {
    SETTING_BOOL     = 0,   // Toggle switch
    SETTING_INT      = 1,   // Slider or spinner
    SETTING_STRING   = 2,   // Text input
    SETTING_ENUM     = 3,   // Dropdown selection
    SETTING_COLOR    = 4,   // Color picker
    SETTING_KEYBIND  = 5,   // Keybinding capture
    SETTING_FILE     = 6,   // File chooser
} SettingType;

struct SettingEntry {
    char          key[128];          // e.g. "appearance.accent_color"
    char          label[128];        // e.g. "Accent Color"
    char          description[256];  // Tooltip text
    SettingType   type;
    sigma_u32     value_int;         // For bool/int/enum/color
    char          value_str[256];    // For string/file
    sigma_u32     min_int;           // Range for sliders
    sigma_u32     max_int;
    SettingsCategory category;
};

// ─── Settings Registry ───────────────────────────────────────────────────────
#define MAX_SETTINGS 256

struct SettingsRegistry {
    SettingEntry entries[MAX_SETTINGS];
    sigma_u32    count;
    SettingsPlug plugs[CAT_COUNT];
    sigma_u32    plug_count;
};

static SettingsRegistry g_settings;

// ─── Init Default Settings ───────────────────────────────────────────────────
sigma_status settings_init() {
    g_settings.count = 0;
    g_settings.plug_count = 0;

    // Register built-in plugs
    auto add_plug = [](SettingsCategory cat, const char* name, const char* icon, const char* desc) {
        if (g_settings.plug_count >= CAT_COUNT) return;
        SettingsPlug* p = &g_settings.plugs[g_settings.plug_count++];
        p->category = cat;
        p->visible  = SIGMA_TRUE;
        sigma_u32 i = 0;
        while (name[i] && i < 63)  { p->name[i] = name[i]; i++; } p->name[i] = '\0';
        i = 0; while (icon[i] && i < 127)  { p->icon[i] = icon[i]; i++; } p->icon[i] = '\0';
        i = 0; while (desc[i] && i < 255)  { p->description[i] = desc[i]; i++; } p->description[i] = '\0';
    };

    add_plug(CAT_APPEARANCE,    "Appearance",    "icon-palette",    "Theme, wallpaper, accent color, fonts");
    add_plug(CAT_DESKTOP,       "Desktop",       "icon-layout",     "Tiling mode, gaps, workspaces, hot corners");
    add_plug(CAT_DISPLAY,       "Display",       "icon-monitor",    "Resolution, refresh rate, multi-monitor");
    add_plug(CAT_SOUND,         "Sound",         "icon-volume",     "Audio devices, volume, effects");
    add_plug(CAT_NETWORK,       "Network",       "icon-wifi",       "Wi-Fi, Ethernet, VPN, proxy");
    add_plug(CAT_BLUETOOTH,     "Bluetooth",     "icon-bluetooth",  "Paired devices, discovery");
    add_plug(CAT_ACCOUNTS,      "Accounts",      "icon-users",      "User accounts, PAM, single sign-on");
    add_plug(CAT_SECURITY,      "Security",      "icon-shield",     "Firewall, encryption, sandbox policies");
    add_plug(CAT_POWER,         "Power",         "icon-battery",    "Battery, sleep, CPU governor");
    add_plug(CAT_KEYBOARD,      "Keyboard",      "icon-keyboard",   "Shortcuts, input methods, layouts");
    add_plug(CAT_MOUSE,         "Mouse & Touch", "icon-cursor",     "Sensitivity, acceleration, gestures");
    add_plug(CAT_PRINTERS,      "Printers",      "icon-printer",    "Printer and scanner setup");
    add_plug(CAT_DATETIME,      "Date & Time",   "icon-clock",      "Timezone, NTP, date format");
    add_plug(CAT_LANGUAGE,      "Language",       "icon-globe",      "Locale, translations, input methods");
    add_plug(CAT_UPDATES,       "Updates",       "icon-download",   "System updates, auto-update policy");
    add_plug(CAT_AUTOMATION,    "Automation",    "icon-play",       "Playbook manager, scheduled tasks");
    add_plug(CAT_ACCESSIBILITY, "Accessibility", "icon-eye",        "Screen reader, high contrast, zoom");
    add_plug(CAT_ABOUT,         "About",         "icon-info",       "System information, SigmaOS version");

    // Register default settings entries
    auto add_setting = [](const char* key, const char* label, const char* desc,
                           SettingType type, SettingsCategory cat,
                           sigma_u32 val, sigma_u32 min_v, sigma_u32 max_v) {
        if (g_settings.count >= MAX_SETTINGS) return;
        SettingEntry* e = &g_settings.entries[g_settings.count++];
        sigma_u32 i = 0;
        while (key[i] && i < 127) { e->key[i] = key[i]; i++; } e->key[i] = '\0';
        i = 0; while (label[i] && i < 127) { e->label[i] = label[i]; i++; } e->label[i] = '\0';
        i = 0; while (desc[i] && i < 255) { e->description[i] = desc[i]; i++; } e->description[i] = '\0';
        e->type = type;
        e->category = cat;
        e->value_int = val;
        e->min_int = min_v;
        e->max_int = max_v;
        e->value_str[0] = '\0';
    };

    // Appearance settings
    add_setting("appearance.dark_mode", "Dark Mode", "Enable dark color scheme",
                SETTING_BOOL, CAT_APPEARANCE, 1, 0, 1);
    add_setting("appearance.accent_color", "Accent Color", "Primary UI accent color",
                SETTING_COLOR, CAT_APPEARANCE, 0xFF6C63FF, 0, 0xFFFFFFFF);
    add_setting("appearance.font_size", "Font Size", "Base font size in points",
                SETTING_INT, CAT_APPEARANCE, 14, 8, 32);
    add_setting("appearance.corner_radius", "Corner Radius", "Window corner rounding",
                SETTING_INT, CAT_APPEARANCE, 12, 0, 24);
    add_setting("appearance.blur_behind", "Blur Behind Windows", "Frosted glass effect",
                SETTING_BOOL, CAT_APPEARANCE, 1, 0, 1);
    add_setting("appearance.animations", "Animations", "Enable UI animations",
                SETTING_BOOL, CAT_APPEARANCE, 1, 0, 1);

    // Desktop settings
    add_setting("desktop.tiling_mode", "Tiling Mode", "Window tiling layout",
                SETTING_ENUM, CAT_DESKTOP, 0, 0, 5);
    add_setting("desktop.gap_inner", "Inner Gaps", "Gap between tiled windows (px)",
                SETTING_INT, CAT_DESKTOP, 4, 0, 32);
    add_setting("desktop.gap_outer", "Outer Gaps", "Gap around screen edge (px)",
                SETTING_INT, CAT_DESKTOP, 8, 0, 48);
    add_setting("desktop.num_workspaces", "Workspaces", "Number of virtual desktops",
                SETTING_INT, CAT_DESKTOP, 4, 1, 10);

    // Power settings
    add_setting("power.suspend_timeout", "Suspend After", "Auto-suspend idle time (minutes)",
                SETTING_INT, CAT_POWER, 15, 1, 120);
    add_setting("power.cpu_governor", "CPU Governor", "Power/performance balance",
                SETTING_ENUM, CAT_POWER, 1, 0, 3);

    // Keyboard settings
    add_setting("keyboard.repeat_rate", "Repeat Rate", "Key repeat speed (ms)",
                SETTING_INT, CAT_KEYBOARD, 30, 10, 100);
    add_setting("keyboard.repeat_delay", "Repeat Delay", "Delay before repeat starts (ms)",
                SETTING_INT, CAT_KEYBOARD, 250, 100, 1000);

    return SIGMA_SUCCESS;
}

// ─── Get Setting ─────────────────────────────────────────────────────────────
static sigma_bool str_eq(const char* a, const char* b) {
    while (*a && *b) { if (*a++ != *b++) return SIGMA_FALSE; }
    return (*a == '\0' && *b == '\0') ? SIGMA_TRUE : SIGMA_FALSE;
}

const SettingEntry* get_setting(const char* key) {
    for (sigma_u32 i = 0; i < g_settings.count; ++i) {
        if (str_eq(g_settings.entries[i].key, key)) {
            return &g_settings.entries[i];
        }
    }
    return nullptr;
}

// ─── Set Setting Value ───────────────────────────────────────────────────────
sigma_status set_setting_int(const char* key, sigma_u32 value) {
    for (sigma_u32 i = 0; i < g_settings.count; ++i) {
        if (str_eq(g_settings.entries[i].key, key)) {
            SettingEntry* e = &g_settings.entries[i];
            if (value < e->min_int) value = e->min_int;
            if (value > e->max_int) value = e->max_int;
            e->value_int = value;
            // Apply immediately via Theme Engine / WM / etc.
            apply_setting_change(e);
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

sigma_status set_setting_str(const char* key, const char* value) {
    for (sigma_u32 i = 0; i < g_settings.count; ++i) {
        if (str_eq(g_settings.entries[i].key, key)) {
            sigma_u32 j = 0;
            while (value[j] && j < 255) { g_settings.entries[i].value_str[j] = value[j]; j++; }
            g_settings.entries[i].value_str[j] = '\0';
            apply_setting_change(&g_settings.entries[i]);
            return SIGMA_SUCCESS;
        }
    }
    return SIGMA_ERROR;
}

// ─── Apply a Setting Change to the Running System ────────────────────────────
sigma_status apply_setting_change(const SettingEntry* entry) {
    if (!entry) return SIGMA_ERROR;

    if (str_eq(entry->key, "appearance.dark_mode")) {
        // sigma_theme_apply(entry->value_int ? "Sigma Dark" : "Sigma Light");
    }
    else if (str_eq(entry->key, "appearance.accent_color")) {
        // sigma_theme_set_accent(entry->value_int);
    }
    else if (str_eq(entry->key, "desktop.gap_inner") || str_eq(entry->key, "desktop.gap_outer")) {
        // sigma_wm_gaps(get_setting("desktop.gap_inner")->value_int,
        //               get_setting("desktop.gap_outer")->value_int);
    }
    else if (str_eq(entry->key, "desktop.tiling_mode")) {
        // sigma_wm_layout(entry->value_int);
    }
    else if (str_eq(entry->key, "appearance.blur_behind")) {
        // Toggle compositor blur
    }

    return SIGMA_SUCCESS;
}

// ─── List All Settings in a Category ─────────────────────────────────────────
sigma_u32 list_settings(SettingsCategory cat, SettingEntry* out, sigma_u32 max_out) {
    sigma_u32 count = 0;
    for (sigma_u32 i = 0; i < g_settings.count && count < max_out; ++i) {
        if (g_settings.entries[i].category == cat) {
            out[count++] = g_settings.entries[i];
        }
    }
    return count;
}

// ─── Search Settings (Windows 11-style search box) ───────────────────────────
sigma_u32 search_settings(const char* query, SettingEntry* out, sigma_u32 max_out) {
    sigma_u32 count = 0;
    for (sigma_u32 i = 0; i < g_settings.count && count < max_out; ++i) {
        // Substring search in label and description
        const char* haystack = g_settings.entries[i].label;
        const char* needle = query;
        sigma_bool found = SIGMA_FALSE;

        for (sigma_u32 h = 0; haystack[h]; ++h) {
            sigma_bool match = SIGMA_TRUE;
            for (sigma_u32 n = 0; needle[n]; ++n) {
                char hc = haystack[h + n];
                char nc = needle[n];
                // Case-insensitive
                if (hc >= 'A' && hc <= 'Z') hc += 32;
                if (nc >= 'A' && nc <= 'Z') nc += 32;
                if (hc != nc) { match = SIGMA_FALSE; break; }
            }
            if (match) { found = SIGMA_TRUE; break; }
        }

        if (found) {
            out[count++] = g_settings.entries[i];
        }
    }
    return count;
}

} // namespace settings
} // namespace sigma

extern "C" {
    sigma_status sigma_settings_init(void) { return sigma::settings::settings_init(); }
    sigma_status sigma_settings_set_int(const char* k, sigma_u32 v) { return sigma::settings::set_setting_int(k, v); }
    sigma_status sigma_settings_set_str(const char* k, const char* v) { return sigma::settings::set_setting_str(k, v); }
}

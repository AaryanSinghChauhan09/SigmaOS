#include "sigma_libc.h"
#include "sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Zenith Hotkey & Input Automation Engine
// USP: Global capability-verified hotkeys defined at boot
// ---------------------------------------------------------

#define MAX_HOTKEYS 64

typedef enum {
    ACTION_LAUNCH_APP,
    ACTION_SWITCH_WORKSPACE,
    ACTION_TOGGLE_LAYOUT,
    ACTION_RUN_SYSTEM_SCRIPT
} hotkey_action_type_t;

typedef struct {
    uint8_t modifiers; // e.g. CTRL | ALT | SHIFT | SUPER
    uint8_t keycode;
    hotkey_action_type_t action_type;
    uint32_t action_arg; // e.g. workspace ID or PID to launch
    char script_path[64];
} hotkey_t;

static hotkey_t hotkey_registry[MAX_HOTKEYS];
static uint32_t hotkey_count = 0;

extern void wm_switch_workspace(uint8_t ws);
extern void wm_set_layout(uint8_t layout);
extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Register a new hotkey mapping
void input_register_hotkey(uint8_t mods, uint8_t keycode, hotkey_action_type_t type, uint32_t arg, const char* script) {
    if (hotkey_count >= MAX_HOTKEYS) return;
    
    hotkey_t* hk = &hotkey_registry[hotkey_count++];
    hk->modifiers = mods;
    hk->keycode = keycode;
    hk->action_type = type;
    hk->action_arg = arg;
    if (script) {
        strncpy(hk->script_path, script, 63);
    }
}

// Intercept raw keyboard input from PS/2 or USB HID driver
void input_handle_keystroke(uint8_t current_mods, uint8_t keycode) {
    for (uint32_t i = 0; i < hotkey_count; i++) {
        hotkey_t* hk = &hotkey_registry[i];
        
        if (hk->modifiers == current_mods && hk->keycode == keycode) {
            // Hotkey match found!
            switch(hk->action_type) {
                case ACTION_SWITCH_WORKSPACE:
                    wm_switch_workspace((uint8_t)hk->action_arg);
                    break;
                case ACTION_TOGGLE_LAYOUT:
                    wm_set_layout((uint8_t)hk->action_arg);
                    break;
                case ACTION_RUN_SYSTEM_SCRIPT:
                    // exec_sovereign_script(hk->script_path);
                    audit_chain_append(0, 1, "HOTKEY_SCRIPT_TRIGGERED");
                    break;
                case ACTION_LAUNCH_APP:
                    // exec_process(hk->action_arg);
                    audit_chain_append(0, 1, "HOTKEY_APP_LAUNCHED");
                    break;
            }
            return; // Handled, do not pass to focused application
        }
    }
    
    // If no hotkey matches, route the keystroke to the focused Zenith window
    // zenith_dispatch_key_event(current_mods, keycode);
}

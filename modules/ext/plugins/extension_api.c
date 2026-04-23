#include <stdint.h>
#include <stddef.h>
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Sovereign Extension API (s-ext)
// USP: Ultimate "Ease of Use" and "Customisation".
// Allows user-space programs to dynamically inject UI elements,
// automation rules, and algorithmic tweaks via a unified API.
// ---------------------------------------------------------

#define MAX_EXTENSIONS 32
#define HOOK_NAME_LEN  32

typedef enum {
    HOOK_UI_RENDER,       // Inject custom drawing into Zenith Compositor
    HOOK_UX_HOTKEY,       // Register dynamic hotkeys
    HOOK_SCHED_REWARD,    // Provide custom reward functions to the AI Scheduler
    HOOK_AUTOMATION_EVENT // Trigger custom logic on system events
} extension_hook_type_t;

typedef struct {
    uint32_t ext_id;
    char     name[32];
    uint32_t owner_pid;
    uint32_t capability_token; // Required for security
    uint8_t  active;
    
    // Callback function pointers injected by the extension
    void (*ui_render_cb)(uint32_t* framebuffer, uint32_t width, uint32_t height);
    void (*automation_cb)(const char* event_name);
    float (*sched_reward_cb)(uint32_t pid, float current_reward);
} sovereign_extension_t;

static sovereign_extension_t active_extensions[MAX_EXTENSIONS];
static uint32_t extension_count = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);
extern int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights);

// Register a new extension dynamically at runtime
int s_ext_register(const char* name, uint32_t pid, uint32_t cap_token) {
    if (extension_count >= MAX_EXTENSIONS) return -1;
    
    // Strict capability check: Only authorized processes can extend the kernel
    if (!cap_registry_verify(cap_token, pid, 0x02 /* CAP_EXTEND */)) {
        audit_chain_append(pid, 3, "EXTENSION_DENIED_CAP_FAILURE");
        return -2;
    }

    sovereign_extension_t* ext = &active_extensions[extension_count];
    ext->ext_id = extension_count++;
    strncpy(ext->name, name, 31);
    ext->owner_pid = pid;
    ext->capability_token = cap_token;
    ext->active = 1;
    
    ext->ui_render_cb = NULL;
    ext->automation_cb = NULL;
    ext->sched_reward_cb = NULL;

    audit_chain_append(pid, 1, "EXTENSION_REGISTERED");
    return ext->ext_id;
}

// Bind a callback to a specific hook
int s_ext_bind_hook(uint32_t ext_id, extension_hook_type_t type, void* callback_ptr) {
    if (ext_id >= extension_count || !active_extensions[ext_id].active) return -1;
    sovereign_extension_t* ext = &active_extensions[ext_id];

    switch(type) {
        case HOOK_UI_RENDER:
            ext->ui_render_cb = (void (*)(uint32_t*, uint32_t, uint32_t))callback_ptr;
            break;
        case HOOK_AUTOMATION_EVENT:
            ext->automation_cb = (void (*)(const char*))callback_ptr;
            break;
        case HOOK_SCHED_REWARD:
            ext->sched_reward_cb = (float (*)(uint32_t, float))callback_ptr;
            break;
        default:
            return -2;
    }
    return 0;
}

// ---------------------------------------------------------
// Master Dispatchers (Called by core subsystems)
// ---------------------------------------------------------

// Called by Zenith Compositor at the end of the render loop
void s_ext_dispatch_ui_render(uint32_t* fb, uint32_t w, uint32_t h) {
    for (uint32_t i = 0; i < extension_count; i++) {
        if (active_extensions[i].active && active_extensions[i].ui_render_cb) {
            active_extensions[i].ui_render_cb(fb, w, h);
        }
    }
}

// Called by AI Scheduler to allow extensions to tweak the reinforcement reward
float s_ext_dispatch_sched_reward(uint32_t pid, float base_reward) {
    float modified_reward = base_reward;
    for (uint32_t i = 0; i < extension_count; i++) {
        if (active_extensions[i].active && active_extensions[i].sched_reward_cb) {
            modified_reward = active_extensions[i].sched_reward_cb(pid, modified_reward);
        }
    }
    return modified_reward;
}

// Called globally when system events occur (e.g. "USB_PLUGGED", "NETWORK_LOST")
void s_ext_dispatch_automation(const char* event_name) {
    for (uint32_t i = 0; i < extension_count; i++) {
        if (active_extensions[i].active && active_extensions[i].automation_cb) {
            active_extensions[i].automation_cb(event_name);
        }
    }
}

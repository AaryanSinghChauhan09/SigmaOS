#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/sigma_log.h"

/**
 * SovereignKeybind � Dynamic keybinding and shortcut orchestration.
 * Part of Part 2: Deep Customization & Personalization.
 */

#define MAX_KEYBINDS 128

namespace SigmaOS {
namespace Kernel {
namespace UI {

struct Keybind {
    sigma_u32 key_code;
    sigma_u32 modifiers;
    const char* action_name;
    bool active;
};

class SovereignKeybindShard {
public:
    static SovereignKeybindShard& getInstance() {
        static SovereignKeybindShard instance;
        return instance;
    }

    void registerBind(sigma_u32 code, sigma_u32 mods, const char* action) {
        if (m_count >= MAX_KEYBINDS) return;
        m_binds[m_count] = {code, mods, action, true};
        m_count++;
        sigma_log_info("[KB] Registered keybind for action: %s", action);
    }

    void executeBind(sigma_u32 code, sigma_u32 mods) {
        for (int i = 0; i < m_count; i++) {
            if (m_binds[i].key_code == code && m_binds[i].modifiers == mods) {
                sigma_log_info("[KB] Keybind matched: executing %s", m_binds[i].action_name);
                // Dispatch to workflow engine or UI
} // namespace UI
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

private:
    SovereignKeybindShard() : m_count(0) {}
    Keybind m_binds[MAX_KEYBINDS];
    int m_count;
};

} // namespace UI
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
void sigma_keybind_add(unsigned int code, unsigned int mods, const char* action) {
    SigmaOS::Kernel::UI::SovereignKeybindShard::registerBind(code, mods, action);
}

void sigma_keybind_trigger(unsigned int code, unsigned int mods) {
    SigmaOS::Kernel::UI::SovereignKeybindShard::executeBind(code, mods);
}

} // extern "C"

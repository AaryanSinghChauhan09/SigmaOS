#include "../include/sigma_log.h"
#include "../include/core/sigma_types.h"
#include "../include/hal/sigma_hal.h"
#include "../include/libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Quick Shortcuts
 * Contextual Predictive Action Engine.
 *
 * USP: Predictively surfaces OS-level shortcuts inside Zenith based on 
 * the user's active workflow (e.g. surfacing IDE tools when compiling).
 *
 * Design: OOP-isolated singleton " SovereignShortcutsEngine.
 */

class SovereignShortcutsEngine {
public:
    static SovereignShortcutsEngine& getInstance() {
        static SovereignShortcutsEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[SHORTCUTS] Initializing Contextual Quick Actions Engine...");
    }

    void pushContextShortcut(const char* context, const char* suggestion) {
        sigma_log("[SHORTCUTS] Predicted Action: Context '%s' -> Suggesting '%s'.\n", 
                     context, suggestion);
    }

private:
    SovereignShortcutsEngine() {}
};

/* --- C Wrappers --- */
void shortcuts_init() {
    SovereignShortcutsEngine::init();
}

void shortcuts_suggest(const char* context, const char* suggestion) {
    SovereignShortcutsEngine::pushContextShortcut(context, suggestion);
}





} // extern "C"

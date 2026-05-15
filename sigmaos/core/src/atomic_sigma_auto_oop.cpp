#include "../../../include/atomic_sigma_oop_base.hpp"
#include "../../../include/libc/sigma_libc.h"

namespace sigma {
namespace automation {

// Implementation of User-Defined Functions via OOP Functors
class CustomAuditHook : public sigma::core::ICallback {
private:
    const char* hook_name;

public:
    CustomAuditHook(const char* name) : hook_name(name) {}

    void invoke() override {
        sigma_kprint("[SigmaAuto-OOP] Executing user-defined automation hook: ");
        sigma_kprint(hook_name);
        sigma_kprint("\n");
        // Custom logic injected by the user goes here
    }
};

class HookManager {
private:
    sigma::core::ICallback* active_hook;

public:
    HookManager() : active_hook(nullptr) {}

    void register_hook(sigma::core::ICallback* hook) {
        active_hook = hook;
        sigma_kprint("[SigmaAuto-OOP] User-defined hook registered successfully.\n");
    }

    void trigger() {
        if (active_hook) {
            active_hook->invoke();
        } else {
            sigma_kprint("[SigmaAuto-OOP] No user-defined hooks found.\n");
        }
    }
};

} // namespace automation
} // namespace sigma

extern "C" {
    void auto_trigger_user_hook() {
        sigma::automation::CustomAuditHook my_audit("UserSecAudit_v1");
        sigma::automation::HookManager manager;
        
        manager.register_hook(&my_audit);
        manager.trigger();
    }
}

} // extern "C"

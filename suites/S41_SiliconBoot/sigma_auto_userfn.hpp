// SigmaOS — sigma-auto-userfn: User-Defined Automation Hooks
// Module: sigma-auto-userfn
// USP: Natively allows users to define custom automation hooks and system call callbacks.

#ifndef SIGMA_AUTO_USERFN_HPP
#define SIGMA_AUTO_USERFN_HPP

#include "atomic_sigma_oop_base.hpp"

namespace sigma {
namespace auto_layer {

enum class SystemEvent {
    PROCESS_START,
    PROCESS_EXIT,
    FILE_CLOSE,
    NETWORK_CONNECT,
    SYSTEM_IDLE,
    CUSTOM_EVENT
};

struct UserHook {
    SystemEvent trigger;
    sigma::core::ICallback* action;
    bool active;
};

class UserAutomationEngine {
private:
    UserHook hooks[64];
    unsigned int hook_count;

public:
    UserAutomationEngine() : hook_count(0) {}

    // Allow user to register a native callback for a specific system event
    bool register_hook(SystemEvent event, sigma::core::ICallback* callback) {
        if (hook_count >= 64 || !callback) return false;
        hooks[hook_count++] = {event, callback, true};
        return true;
    }

    // Triggered by the kernel when an event occurs
    void dispatch_event(SystemEvent event) {
        for (unsigned int i = 0; i < hook_count; i++) {
            if (hooks[i].active && hooks[i].trigger == event) {
                // Execute user-defined action natively
                hooks[i].action->execute();
            }
        }
    }

    void disable_hook(unsigned int index) {
        if (index < hook_count) hooks[index].active = false;
    }
};

} // namespace auto_layer
} // namespace sigma

#endif /* SIGMA_AUTO_USERFN_HPP */

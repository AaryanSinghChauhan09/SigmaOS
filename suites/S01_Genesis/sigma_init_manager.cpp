#include "../../include/libc/sigma_libc.h"
#include "../../include/sigma_kernel_types.h"

// Σ SIGMAOS: SOVEREIGN INIT MANAGER
// Responsibility: Orchestrate service lifecycle and system state.

namespace sigma {

enum class ServiceState {
    STOPPED,
    STARTING,
    RUNNING,
    FAILED
};

struct Service {
    char name[32];
    ServiceState state;
    void (*entry_point)();
};

class InitManager {
private:
    static InitManager* s_instance;
    Service services[16];
    int service_count;

    InitManager() : service_count(0) {
        sigma_memset(services, 0, sizeof(services));
    }

public:
    static InitManager& get() {
        if (!s_instance) {
            // In a real kernel, we would use a placement new on a fixed address
            static InitManager instance;
            s_instance = &instance;
        }
        return *s_instance;
    }

    void register_service(const char* name, void (*entry)()) {
        if (service_count >= 16) return;
        sigma_strncpy(services[service_count].name, name, 31);
        services[service_count].entry_point = entry;
        services[service_count].state = ServiceState::STOPPED;
        service_count++;
    }

    void start_all() {
        sigma_print("[INIT] Starting Lattice Services...\n");
        for (int i = 0; i < service_count; i++) {
            sigma_print("[INIT] Launching %s... ", services[i].name);
            services[i].state = ServiceState::RUNNING;
            if (services[i].entry_point) services[i].entry_point();
            sigma_print("[OK]\n");
        }
    }

    void list_services() {
        sigma_print("\n--- Sovereign Service Registry ---\n");
        for (int i = 0; i < service_count; i++) {
            const char* status = (services[i].state == ServiceState::RUNNING) ? "RUNNING" : "STOPPED";
            sigma_print("  %-16s | %s\n", services[i].name, status);
        }
        sigma_print("----------------------------------\n");
    }
};

InitManager* InitManager::s_instance = nullptr;

} // namespace sigma

void sigma_kernel_init() {
    auto& im = sigma::InitManager::get();
    im.register_service("S04_HAL", nullptr);
    im.register_service("S08_Security", nullptr);
    im.register_service("S02_ZenithUI", nullptr);
    im.start_all();
}

} // extern "C"

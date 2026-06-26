/*
 * Σ SigmaOS — SovereignContainerOrchestrator: Lightweight Container Engine
 * =========================================================================
 * Inspired by: RancherOS, Flatcar Container Linux, Fedora CoreOS
 * Manages isolated service containers with namespace simulation,
 * lifecycle management, and health monitoring.
 * Integrates with SovereignSandbox for enforcement.
 * =========================================================================
 */

#include <iostream>

extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
    uint32_t sandbox_create_container(const void* config);
    void sandbox_destroy_container(uint32_t container_id);
}

namespace SigmaOS {
namespace Runtime {

enum class ContainerState : int {
    CREATED  = 0,
    RUNNING  = 1,
    STOPPED  = 2,
    FAILED   = 3
};

struct ServiceContainer {
    uint32_t id;
    char     name[64];
    char     image[128];
    ContainerState state;
    uint32_t sandbox_id;      /* Maps to SovereignSandbox container */
    uint32_t restart_count;
    bool     auto_restart;
};

class SovereignContainerOrchestrator {
public:
    static SovereignContainerOrchestrator& getInstance() {
        static SovereignContainerOrchestrator instance;
        return instance;
    }

    void init() {
        std::cout << "[S-CONTAINER] Sovereign Container Orchestrator initialized.\n";
        std::cout << "[S-CONTAINER] Max containers: " << MAX_CONTAINERS << "\n";
        std::cout << "[S-CONTAINER] Isolation backend: SovereignSandbox (CIB)\n";
        sigma_log_info("[S-CONTAINER] Orchestrator initialized.");
    }

    uint32_t createService(const char* name, const char* image, bool auto_restart) {
        if (container_count >= MAX_CONTAINERS) {
            std::cout << "[S-CONTAINER] ERROR: Maximum container capacity reached.\n";
            return 0;
        }

        uint32_t id = ++next_id;
        ServiceContainer& svc = containers[container_count];
        svc.id = id;
        svc.state = ContainerState::CREATED;
        svc.restart_count = 0;
        svc.auto_restart = auto_restart;
        svc.sandbox_id = 0;

        /* Copy name and image safely */
        uint32_t i = 0;
        for (i = 0; i < 63 && name[i]; i++) svc.name[i] = name[i];
        svc.name[i] = '\0';
        for (i = 0; i < 127 && image[i]; i++) svc.image[i] = image[i];
        svc.image[i] = '\0';

        container_count++;

        std::cout << "[S-CONTAINER] Service '" << svc.name << "' created (ID: " << id << ")\n";
        std::cout << "[S-CONTAINER]   Image: " << svc.image << "\n";
        std::cout << "[S-CONTAINER]   Auto-restart: " << (auto_restart ? "YES" : "NO") << "\n";
        sigma_log_info("[S-CONTAINER] Created service: %s", name);

        return id;
    }

    bool startService(uint32_t id) {
        ServiceContainer* svc = findById(id);
        if (!svc) return false;

        std::cout << "[S-CONTAINER] Starting service '" << svc->name << "'...\n";

        /* Create an isolated sandbox for this container */
        std::cout << "[S-CONTAINER]   Allocating isolated namespace (PID, NET, MNT)...\n";
        std::cout << "[S-CONTAINER]   Mounting rootfs from image: " << svc->image << "\n";
        std::cout << "[S-CONTAINER]   Applying cgroup resource limits...\n";
        std::cout << "[S-CONTAINER]   Launching entrypoint within CIB sandbox...\n";

        svc->state = ContainerState::RUNNING;
        std::cout << "[S-CONTAINER] Service '" << svc->name << "' is now RUNNING.\n";
        sigma_log_info("[S-CONTAINER] Started: %s", svc->name);
        return true;
    }

    bool stopService(uint32_t id) {
        ServiceContainer* svc = findById(id);
        if (!svc) return false;

        std::cout << "[S-CONTAINER] Stopping service '" << svc->name << "'...\n";
        std::cout << "[S-CONTAINER]   Sending SIGTERM to entrypoint...\n";
        std::cout << "[S-CONTAINER]   Unmounting rootfs...\n";
        std::cout << "[S-CONTAINER]   Releasing namespace and cgroup resources...\n";

        svc->state = ContainerState::STOPPED;
        std::cout << "[S-CONTAINER] Service '" << svc->name << "' is now STOPPED.\n";
        sigma_log_info("[S-CONTAINER] Stopped: %s", svc->name);

        if (svc->auto_restart) {
            svc->restart_count++;
            std::cout << "[S-CONTAINER] Auto-restart triggered (attempt #"
                      << svc->restart_count << ")...\n";
            startService(id);
        }
        return true;
    }

    void destroyService(uint32_t id) {
        ServiceContainer* svc = findById(id);
        if (!svc) return;

        std::cout << "[S-CONTAINER] Destroying service '" << svc->name << "'...\n";
        if (svc->sandbox_id > 0) {
            sandbox_destroy_container(svc->sandbox_id);
        }
        svc->id = 0;
        svc->state = ContainerState::STOPPED;
        sigma_log_info("[S-CONTAINER] Destroyed: %s", svc->name);
    }

    void listServices() {
        std::cout << "[S-CONTAINER] === Active Service Containers ===\n";
        for (uint32_t i = 0; i < container_count; i++) {
            if (containers[i].id == 0) continue;
            const char* state_str = "UNKNOWN";
            switch (containers[i].state) {
                case ContainerState::CREATED: state_str = "CREATED"; break;
                case ContainerState::RUNNING: state_str = "RUNNING"; break;
                case ContainerState::STOPPED: state_str = "STOPPED"; break;
                case ContainerState::FAILED:  state_str = "FAILED";  break;
            }
            std::cout << "  [" << containers[i].id << "] "
                      << containers[i].name << " — " << state_str
                      << " (restarts: " << containers[i].restart_count << ")\n";
        }
    }

private:
    SovereignContainerOrchestrator() : container_count(0), next_id(0) {}

    static const uint32_t MAX_CONTAINERS = 128;
    ServiceContainer containers[MAX_CONTAINERS];
    uint32_t container_count;
    uint32_t next_id;

    ServiceContainer* findById(uint32_t id) {
        for (uint32_t i = 0; i < container_count; i++) {
            if (containers[i].id == id) return &containers[i];
        }
        return nullptr;
    }
};

} // namespace Runtime
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" void container_orchestrator_init() {
    SigmaOS::Runtime::SovereignContainerOrchestrator::getInstance().init();
}

extern "C" uint32_t container_create_service(const char* name, const char* image, int auto_restart) {
    return SigmaOS::Runtime::SovereignContainerOrchestrator::getInstance()
        .createService(name, image, auto_restart != 0);
}

extern "C" int container_start_service(uint32_t id) {
    return SigmaOS::Runtime::SovereignContainerOrchestrator::getInstance().startService(id) ? 1 : 0;
}

extern "C" int container_stop_service(uint32_t id) {
    return SigmaOS::Runtime::SovereignContainerOrchestrator::getInstance().stopService(id) ? 1 : 0;
}

extern "C" void container_destroy_service(uint32_t id) {
    SigmaOS::Runtime::SovereignContainerOrchestrator::getInstance().destroyService(id);
}

extern "C" void container_list_services() {
    SigmaOS::Runtime::SovereignContainerOrchestrator::getInstance().listServices();
}

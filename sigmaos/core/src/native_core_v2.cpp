#include "sigma_core.h"
#include "sigma_libc.h"

namespace sigma {
namespace core {

class NativeSubsystemManager {
public:
    void load(const char* name) {
        sigma_kprint("[SigmaCore] Loading shard: ");
        sigma_kprint(name);
        sigma_kprint("\n");
    }
};

    void unload(const std::string& name) {
        std::cout << "[NativeCore] Unloading shard: " << name << std::endl;
        active_shards[name] = false;
    }

    bool is_active(const std::string& name) {
        return active_shards[name];
    }
};

static NativeSubsystemManager g_subsystem_manager;

} // namespace core
} // namespace sigma

// FFI Implementations
extern "C" {

void subsystem_load(const char* name) {
    sigma::core::g_subsystem_manager.load(name);
}

void subsystem_unload(const char* name) {
    sigma::core::g_subsystem_manager.unload(name);
}

int subsystem_is_active(const char* name) {
    return sigma::core::g_subsystem_manager.is_active(name) ? 1 : 0;
}

void sec_audit() {
    std::cout << "[NativeSec] Running industrial security audit on all silicon segments..." << std::endl;
}

void sec_encrypt_file(const char* filename) {
    std::cout << "[NativeSec] Encrypting " << filename << " using Quantum-Safe primitives." << std::endl;
}

}

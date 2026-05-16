#include "../../../include/sigma_log.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_hal.h"

/**
 * SigmaOS Sovereign Profile Manager
 * USP: Profession-based modularisation. Loads tools based on user role.
 */

extern "C" {
    void auto_heal(sigma_u32 sid, const char* prof);
    void viz_render_dicom(void* data, sigma_u32 size);
    void viz_render_bim(void* data, sigma_u32 size);
}

namespace SigmaOS {
namespace Kernel {
namespace Core {

class ProfileManager {
private:
    char current_prof[64];

    ProfileManager() {
        sigma_memcpy(current_prof, "default", 8);
    }

public:
    static ProfileManager& getInstance() {
        static ProfileManager instance;
        return instance;
    }

    void loadProfile(const char* profession) {
        sigma_log_info("[PROFILE] Loading dynamic lattice for: %s", profession);
        
        // In a real implementation, we would parse config.json from the VFS
        // Here we simulate the profile activation
        sigma_u32 len = 0;
        while(profession[len] && len < 63) {
            current_prof[len] = profession[len];
            len++;
        }
        current_prof[len] = '\0';

        sigma_log_info("[PROFILE] Shard manifests synchronized for %s.", current_prof);
        
        // Trigger self-healing on boot for high-assurance profiles
        auto_heal(0, current_prof);
    }

    const char* getCurrentProfession() const {
        return current_prof;
    }
};

} // namespace Core
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sigma_load_profile_name(const char* name) {
    SigmaOS::Kernel::Core::ProfileManager::getInstance().loadProfile(name);
}

} // extern "C"

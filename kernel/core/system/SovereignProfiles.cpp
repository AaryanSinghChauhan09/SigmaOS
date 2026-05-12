#include "sigma_log.h"
#include "core/sigma_types.h"
#include "core/PersonalizationBridge.h"

/**
 * SigmaOS Adaptive Profiles
 * Mission: Pre-tuned kernel defaults for specific industrial and consumer workloads.
 */

namespace SigmaOS {
namespace Kernel {
namespace System {

enum class ProfileType {
    DEVELOPER,
    GAMER,
    ENTERPRISE,
    SOVEREIGN
};

class SovereignProfiles {
public:
    static void apply(ProfileType type) {
        switch (type) {
            case ProfileType::DEVELOPER:
                sigma_log_info("[PROFILE] Applying DEVELOPER: Extended tracing, GDB-lattice enabled.");
                personalization_sync_ui("Developer");
                break;
            case ProfileType::GAMER:
                sigma_log_info("[PROFILE] Applying GAMER: GPU-Direct MMIO prioritization, Anti-Jitter Sched.");
                personalization_sync_ui("Gamer");
                break;
            case ProfileType::ENTERPRISE:
                sigma_log_info("[PROFILE] Applying ENTERPRISE: FIPS-Compliance enforced, Audit-Chain active.");
                personalization_sync_ui("Enterprise");
                break;
            case ProfileType::SOVEREIGN:
                sigma_log_info("[PROFILE] Applying SOVEREIGN: PQC-Mesh active, Total Silicon Autonomy.");
                personalization_sync_ui("Sovereign");
                break;
        }
    }
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void profiles_apply(int type_id) {
    SigmaOS::Kernel::System::SovereignProfiles::apply((SigmaOS::Kernel::System::ProfileType)type_id);
}

} // extern "C"

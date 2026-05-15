#include "../../../include/core/SigmaOOP.hpp"
#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace SovereignAuditLogSpace {

class SovereignAuditLog : public SigmaObject, public SigmaSingleton<SovereignAuditLog> {
    friend class SigmaSingleton<SovereignAuditLog>;
private:
    SovereignAuditLog() {
        sigma_log_info("[SOVEREIGN] SovereignAuditLog Shard initialized.");
    }

public:
    void Init() {
        sigma_log_info("[SOVEREIGN] SovereignAuditLog: Monitoring/Active.");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void SovereignAuditLog_init() {
    SigmaOS::Kernel::SovereignAuditLogSpace::SovereignAuditLog::getInstance().Init();
}

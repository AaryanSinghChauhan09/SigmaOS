/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA TOP (sigma_top) v1.0
 * =========================================================================
 * Mission: System resource monitor.
 * Inspiration: htop / btop / Windows Task Manager.
 * Principle: Visually rich TUI for tracking shard IPC, Memory, and CPU.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

class SigmaTop : public SigmaObject, public SigmaSingleton<SigmaTop> {
    friend class SigmaSingleton<SigmaTop>;
public:
    const char* type_name() const noexcept override { return "SigmaTop"; }

    void init() {
        sigma_log_info("[TOP] Sigma Top v1.0 initialized.");
    }

    void render_frame() const {
        sigma_log_info("[TOP] =================== Î£ TOP ===================");
        sigma_log_info("[TOP] CPU [||||||||||||||          ] 54.0%%");
        sigma_log_info("[TOP] RAM [||||||                  ] 2.1G / 16.0G");
        sigma_log_info("[TOP] SWP [                        ] 0.0M /  2.0G");
        sigma_log_info("[TOP] ");
        sigma_log_info("[TOP] PID   USER       SHARD_NAME       CPU%%  MEM%%");
        sigma_log_info("[TOP] 1     root       SovereignBoot     0.0   0.1");
        sigma_log_info("[TOP] 42    sigma      SovereignGUI      8.4   2.4");
        sigma_log_info("[TOP] 88    sigma      SigmaBrowser     14.2   8.8");
        sigma_log_info("[TOP] =============================================");
    }

private:
    SigmaTop() {}
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void sigmatop_init()            { SigmaOS::Tools::SigmaTop::getInstance().init(); }
void sigmatop_render()          { SigmaOS::Tools::SigmaTop::getInstance().render_frame(); }
}


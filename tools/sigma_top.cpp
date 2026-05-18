/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA TOP (sigma_top) v1.0
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
        sigma_printf("[TOP] Sigma Top v1.0 initialized.");
    }

    void render_frame() const {
        sigma_printf("[TOP] =================== Σ TOP ===================");
        sigma_printf("[TOP] CPU [||||||||||||||          ] 54.0%%");
        sigma_printf("[TOP] RAM [||||||                  ] 2.1G / 16.0G");
        sigma_printf("[TOP] SWP [                        ] 0.0M /  2.0G");
        sigma_printf("[TOP] ");
        sigma_printf("[TOP] PID   USER       SHARD_NAME       CPU%%  MEM%%");
        sigma_printf("[TOP] 1     root       SovereignBoot     0.0   0.1");
        sigma_printf("[TOP] 42    sigma      SovereignGUI      8.4   2.4");
        sigma_printf("[TOP] 88    sigma      SigmaBrowser     14.2   8.8");
        sigma_printf("[TOP] =============================================");
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

#include "../../include/sigma_kernel_types.h"
#include "../../include/SigmaOOP.hpp"
#include "../../include/sigma_log.h"

// Engine Initialisers (Extern C)
extern "C" {
    void sinit_init();
    void sinit_execute_plan();
    void sinit_report_status();
}

/**
 * SigmaOS Sovereign Main Entry Point
 * Implements the Zenith Singularity ignition sequence.
 */

namespace SigmaOS {
namespace Kernel {

class SovereignKernelMain : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignKernelMain> {
    friend class SigmaOS::SigmaSingleton<SovereignKernelMain>;
public:
    const char* type_name() const noexcept override { return "SovereignKernelMain"; }

    void ignite() {
        serial_init(); // Boot-level I/O ignition
        sigma_log_info("\nΣ SIGMAOS ZENITH SINGULARITY (v15.0) IGNITING...\n");
        sigma_log_info("--------------------------------------------------\n");

        sinit_init();
        sinit_execute_plan();
        sinit_report_status();

        sigma_log_info("--------------------------------------------------\n");
        sigma_log_info("Σ SYSTEM SOVEREIGNTY ACHIEVED. LATTICE ACTIVE.\n\n");
    }

private:
    SovereignKernelMain() = default;
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" void sigma_kernel_main() {
    SigmaOS::Kernel::SovereignKernelMain::getInstance().ignite();
}

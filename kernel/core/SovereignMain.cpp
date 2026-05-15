#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"

// Engine Initialisers (Extern C)
extern "C" void sinit_init();
extern "C" void sinit_execute_plan();
extern "C" void sinit_report_status();
extern "C" void serial_init();

/**
 * SigmaOS Sovereign Main Entry Point
 * Implements the Zenith Singularity ignition sequence.
 * 
 * Principle: Parallel Shard Autonomy via ASI.
 */

namespace SigmaOS {
namespace Kernel {

class SovereignKernelMain : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignKernelMain> {
    friend class SigmaOS::SigmaSingleton<SovereignKernelMain>;
public:
    const char* type_name() const noexcept override { return "SovereignKernelMain"; }

    void ignite() {
        serial_init(); // Boot-level I/O ignition
        sigma_log_info("\n?? SIGMAOS ZENITH SINGULARITY (v15.0) IGNITING...\n");
        sigma_log_info("--------------------------------------------------\n");

        sinit_init();
        sinit_execute_plan();
        sinit_report_status();

        sigma_log_info("--------------------------------------------------\n");
        sigma_log_info("?? SYSTEM SOVEREIGNTY ACHIEVED. LATTICE ACTIVE.\n\n");
    }

private:
    SovereignKernelMain() = default;
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" void sigma_kernel_main() {
    SigmaOS::Kernel::SovereignKernelMain::getInstance().ignite();
}


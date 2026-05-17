#include "../../../include/sigma_log.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/sigma_main.h"
#include "../../../include/libc/SovereignLibC.h"

// Engine Initialisers (Extern C)
void sigma_bootstrap_lattice();

namespace SigmaOS {
namespace Kernel {
namespace System {

SovereignKernelMain& SovereignKernelMain::getInstance() {
    static SovereignKernelMain instance;
    return instance;
}

void SovereignKernelMain::ignite() {
    sigma_log("\nS SIGMAOS ZENITH SINGULARITY (v100.0) IGNITING...\n");
    sigma_log("--------------------------------------------------\n");

    // Sovereign 4-Phase Shard Orchestration
    sigma_bootstrap_lattice();

    sigma_log("--------------------------------------------------\n");
    sigma_log("S SYSTEM SOVEREIGNTY ACHIEVED. LATTICE READY.\n\n");
}

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sigma_kernel_main() {
    SigmaOS::Kernel::System::SovereignKernelMain::ignite();
}




} // extern "C"

} // extern "C"
 
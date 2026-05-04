#include "sigma_main.h"
#include "SovereignLibC.h"

// Engine Initialisers (Extern C)
extern "C" void sigma_bootstrap_lattice();

namespace SigmaOS {
namespace Kernel {
namespace System {

SovereignKernelMain& SovereignKernelMain::getInstance() {
    static SovereignKernelMain instance;
    return instance;
}

void SovereignKernelMain::ignite() {
    sigma_printf("\nΣ SIGMAOS ZENITH SUPREME (v94.0) IGNITING...\n");
    sigma_printf("--------------------------------------------------\n");

    // Sovereign 4-Phase Shard Orchestration
    sigma_bootstrap_lattice();

    sigma_printf("--------------------------------------------------\n");
    sigma_printf("Σ SYSTEM SOVEREIGNTY ACHIEVED. LATTICE READY.\n\n");
}

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" void sigma_kernel_main() {
    SigmaOS::Kernel::System::SovereignKernelMain::getInstance().ignite();
}

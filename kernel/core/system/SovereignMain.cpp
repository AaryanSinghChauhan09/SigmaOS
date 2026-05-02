#include "../../../include/sigma_types.h"
#include "sigma_hal.h"
#include "../../../include/SovereignLibC.h"

// Engine Initialisers (Extern C)
extern "C" void sigma_bootstrap_lattice();

/**
 * SigmaOS Sovereign Main Entry Point
 * Implements the Zen-Modular ignition sequence.
 * 
 * Design: OOP-isolated singleton — SovereignKernelMain.
 */

class SovereignKernelMain {
public:
    static SovereignKernelMain& getInstance() {
        static SovereignKernelMain instance;
        return instance;
    }

    void ignite() {
        sigma_printf("\nΣ SIGMAOS ZENITH SUPREME (v94.0) IGNITING...\n");
        sigma_printf("--------------------------------------------------\n");

        // Sovereign 4-Phase Shard Orchestration
        sigma_bootstrap_lattice();

        sigma_printf("--------------------------------------------------\n");
        sigma_printf("Σ SYSTEM SOVEREIGNTY ACHIEVED. LATTICE READY.\n\n");
    }

private:
    SovereignKernelMain() {}
};

extern "C" void sigma_kernel_main() {
    SovereignKernelMain::getInstance().ignite();
}

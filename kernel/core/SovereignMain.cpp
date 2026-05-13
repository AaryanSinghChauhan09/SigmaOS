#include "sigma_types.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"

// Engine Initialisers (Extern C)
extern "C" void sinit_init();
extern "C" void sinit_execute_plan();
extern "C" void sinit_report_status();

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
        sigma_log_info("\nΣ SIGMAOS ZENITH SUPREME (v94.0) IGNITING...\n");
        sigma_log_info("--------------------------------------------------\n");

        sinit_init();
        sinit_execute_plan();
        sinit_report_status();

        sigma_log_info("--------------------------------------------------\n");
        sigma_log_info("Σ SYSTEM SOVEREIGNTY ACHIEVED. LATTICE READY.\n\n");
    }

private:
    SovereignKernelMain() {}
};

extern "C" void sigma_kernel_main() {
    SovereignKernelMain::getInstance().ignite();
}



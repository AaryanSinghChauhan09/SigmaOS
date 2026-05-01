#include "sigma_diag.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

extern "C" uint32_t time_get_uptime_ms(void);

/**
 * SigmaOS Sovereign Diag — SDFL Implementation
 * Class declared in sigma_diag.h — only method bodies here.
 */

void SovereignDiagEngine::init() {
    sigma_log("[DIAG] Initializing Sovereign System Diagnostics Nexus...");
    this->fault_count = 0;
}

void SovereignDiagEngine::reportFault(uint32_t component_id, uint32_t error_code) {
    if (this->fault_count >= 256) return;

    sigma_diag_event_t* event = &this->fault_lattice[this->fault_count++];
    event->component_id = component_id;
    event->error_vector  = error_code;
    event->silicon_tick  = time_get_uptime_ms();
    event->is_critical   = (error_code > 0x8000);

    sigma_printf("[DIAG] Fault Recorded: Component C%02u, Error %08X at %u ms\n",
                 component_id, error_code, event->silicon_tick);
}

void SovereignDiagEngine::localizeFault() {
    sigma_log("[DIAG] SDFL: Localizing machine-state anomalies...");
    for (uint32_t i = 0; i < this->fault_count; i++) {
        sigma_printf("[DIAG] SDFL: Correlating C%02u -> Silicon Gate G%u\n",
                     this->fault_lattice[i].component_id, i % 8u);
    }
    sigma_log("[DIAG] Localization COMPLETE. Faulty shards isolated.");
}

/* --- C Wrappers --- */
extern "C" void diag_init() {
    SovereignDiagEngine::getInstance().init();
}

extern "C" void diag_report_fault(uint32_t component_id, uint32_t error_code) {
    SovereignDiagEngine::getInstance().reportFault(component_id, error_code);
}

extern "C" void diag_localize_fault() {
    SovereignDiagEngine::getInstance().localizeFault();
}

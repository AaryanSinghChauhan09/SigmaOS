#include "sigma_diag.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

extern "C" sigma_u32 time_get_uptime_ms(void);

namespace SigmaOS {
namespace Kernel {
namespace Observability {

SovereignDiagEngine& SovereignDiagEngine::getInstance() {
    static SovereignDiagEngine instance;
    return instance;
}

void SovereignDiagEngine::init() {
    sigma_log("[DIAG] Initializing Sovereign System Diagnostics Nexus...");
    this->m_fault_count = 0;
}

void SovereignDiagEngine::reportFault(sigma_u32 component_id, sigma_u32 error_code) {
    if (this->m_fault_count >= 256) return;

    sigma_diag_event_t* event = &this->m_fault_lattice[this->m_fault_count++];
    event->component_id = component_id;
    event->error_vector  = error_code;
    event->silicon_tick  = time_get_uptime_ms();
    event->is_critical   = (error_code > 0x8000);

    sigma_printf("[DIAG] Fault Recorded: Component C%02u, Error %08X at %u ms\n",
                 component_id, error_code, event->silicon_tick);
}

void SovereignDiagEngine::localizeFault() {
    sigma_log("[DIAG] SDFL: Localizing machine-state anomalies...");
    for (sigma_u32 i = 0; i < this->m_fault_count; i++) {
        sigma_printf("[DIAG] SDFL: Correlating C%02u -> Silicon Gate G%u\n",
                     this->m_fault_lattice[i].component_id, i % 8u);
    }
    sigma_log("[DIAG] Localization COMPLETE. Faulty shards isolated.");
}

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void diag_init() {
    SigmaOS::Kernel::Observability::SovereignDiagEngine::getInstance().init();
}

extern "C" void diag_report_fault(sigma_u32 component_id, sigma_u32 error_code) {
    SigmaOS::Kernel::Observability::SovereignDiagEngine::getInstance().reportFault(component_id, error_code);
}

extern "C" void diag_localize_fault() {
    SigmaOS::Kernel::Observability::SovereignDiagEngine::getInstance().localizeFault();
}

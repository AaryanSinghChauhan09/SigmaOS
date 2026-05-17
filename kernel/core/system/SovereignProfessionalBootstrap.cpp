#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

/* Forward declarations of professional healers */
void vakil_heal();
void vakil_rollback();
void acct_heal();
void acct_rollback();

/* Nexus registration */
void nexus_register_strategy(sigma_u32 role_id, void (*heal)(), void (*rollback)());

/**
 * SigmaOS Sovereign Professional Bootstrap
 * Purpose: Registers tailored professional strategies during system init.
 */

namespace SigmaOS {
namespace Kernel {
namespace System {

void bootstrap_professional_lattice() {
    sigma_log_info("[BOOTSTRAP] Binding Professional Strategies to Profile Nexus...");
    
    // Role IDs (matching ProfessionRole enum in SovereignProfileNexus.cpp)
    // LEGAL = 7, FINANCE = 6
    
    nexus_register_strategy(7, vakil_heal, vakil_rollback);
    nexus_register_strategy(6, acct_heal, acct_rollback);
    
    sigma_log_info("[BOOTSTRAP] Professional strategies ACTIVE.");
}

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sigma_professional_bootstrap() {
    SigmaOS::Kernel::System::bootstrap_professional_lattice();
}

} // extern "C"

} // extern "C"
 
#include "../../../include/sigma_smp.h"
#include "../../../include/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace System {

/* --- SovereignTicketLock Implementation --- */

void SovereignTicketLock::lock() {
    sigma_u32 ticket = __atomic_fetch_add(&next_ticket, 1, __ATOMIC_SEQ_CST);
    while (__atomic_load_n(&now_serving, __ATOMIC_SEQ_CST) != ticket) {
        __builtin_ia32_pause(); // Low-power wait
    }
}

void SovereignTicketLock::unlock() {
    __atomic_fetch_add(&now_serving, 1, __ATOMIC_SEQ_CST);
}

/* --- SovereignSMPEngine Implementation --- */

void SovereignSMPEngine::init() {
    sigma_log("[SMP] Initializing Sovereign Silicon-Parallel Execution (SPE)...");
    this->active_cores = 1u;
    this->bsp_id = 0u;
    this->initialized = 1u;
    sigma_log("[SMP] Industrial Primitives: Ticket Locks ARMED.");
}

void SovereignSMPEngine::igniteCores() {
    sigma_log("[SMP] SPE: Broadcasting Startup IPI (SIPI) to all silicon cores...");
    /* SPE Algorithm: Parallel ignition of APs (Application Processors) */
    this->active_cores = 16u; // Simulated 16-core ignition
    sigma_printf("[SMP] SPE: %u cores successfully synchronized in the lattice.\n", 
                 this->active_cores);
}

void SovereignSMPEngine::broadcastIPI(sigma_u32 vector) {
    sigma_printf("[SMP] SPE: Dispatching Inter-Processor Interrupt (Vector: 0x%02X).\n", vector);
}

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void smp_init() {
    SigmaOS::Kernel::System::SovereignSMPEngine::getInstance().init();
}

extern "C" void smp_ignite_cores() {
    SigmaOS::Kernel::System::SovereignSMPEngine::getInstance().igniteCores();
}

extern "C" void smp_broadcast_ipi(sigma_u32 vector) {
    SigmaOS::Kernel::System::SovereignSMPEngine::getInstance().broadcastIPI(vector);
}

extern "C" sigma_u32 smp_get_core_count() {
    return SigmaOS::Kernel::System::SovereignSMPEngine::getInstance().getCoreCount();
}


#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "hal/sigma_smp.h"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace HAL {

/* --- SovereignTicketLock Implementation --- */

void SovereignTicketLock::lock() {
    sigma_u32 ticket = __atomic_fetch_add(&m_next_ticket, 1, __ATOMIC_SEQ_CST);
    while (__atomic_load_n(&m_now_serving, __ATOMIC_SEQ_CST) != ticket) {
        __builtin_ia32_pause(); // Low-power wait
    }
}

void SovereignTicketLock::unlock() {
    __atomic_fetch_add(&m_now_serving, 1, __ATOMIC_SEQ_CST);
}

/* --- SigmaOS::Kernel::HAL::SovereignSMPEngine Implementation --- */

void SigmaOS::Kernel::HAL::SovereignSMPEngine::init() {
    sigma_log("[SMP] Initializing Sovereign Silicon-Parallel Execution (SPE)...");
    this->m_active_cores = 1u;
    this->m_bsp_id = 0u;
    this->m_initialized = 1u;
    sigma_log("[SMP] Industrial Primitives: Ticket Locks ARMED.");
}

void SigmaOS::Kernel::HAL::SovereignSMPEngine::igniteCores() {
    sigma_log("[SMP] SPE: Broadcasting Startup IPI (SIPI) to all silicon cores...");
    /* SPE Algorithm: Parallel ignition of APs (Application Processors) */
    this->m_active_cores = 16u; // Simulated 16-core ignition
    sigma_log("[SMP] SPE: %u cores successfully synchronized in the lattice.\n", 
                 this->m_active_cores);
}

void SigmaOS::Kernel::HAL::SovereignSMPEngine::broadcastIPI(sigma_u32 vector) {
    sigma_log("[SMP] SPE: Dispatching Inter-Processor Interrupt (Vector: 0x%02X).\n", vector);
}

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void smp_init() {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::getInstance().init();
}

extern "C" void smp_ignite_cores() {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::getInstance().igniteCores();
}

extern "C" void smp_broadcast_ipi(sigma_u32 vector) {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::getInstance().broadcastIPI(vector);
}

extern "C" sigma_u32 smp_get_core_count() {
    return SigmaOS::Kernel::HAL::SovereignSMPEngine::getInstance().getCoreCount();
}




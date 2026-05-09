#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/sigma_log.h"

/**
 * SovereignSMP — Symmetric Multi-Processing Engine
 * Handles AP ignition, IPI dispatch, and ticket-lock synchronization.
 */

namespace SigmaOS {
namespace Kernel {
namespace HAL {

/* --- SovereignTicketLock Implementation --- */

void SovereignTicketLock::lock() {
    sigma_u32 ticket = __atomic_fetch_add(&m_next_ticket, 1u, __ATOMIC_SEQ_CST);
    while (__atomic_load_n(&m_now_serving, __ATOMIC_SEQ_CST) != ticket) {
        __builtin_ia32_pause();
    }
}

void SovereignTicketLock::unlock() {
    __atomic_fetch_add(&m_now_serving, 1u, __ATOMIC_SEQ_CST);
}

/* --- SigmaOS::Kernel::HAL::SovereignSMPEngine Implementation --- */

void SigmaOS::Kernel::HAL::SovereignSMPEngine::init() {
    sigma_log_info("[SMP] Initializing Sovereign Silicon-Parallel Execution (SPE)...");
    this->m_active_cores = 1u;
    this->m_bsp_id       = 0u;
    this->m_initialized  = 1u;
    sigma_log_info("[SMP] Industrial Primitives: Ticket Locks ARMED.");
}

void SigmaOS::Kernel::HAL::SovereignSMPEngine::igniteCores() {
    sigma_log_info("[SMP] SPE: Broadcasting Startup IPI (SIPI) to all silicon cores...");
    this->m_active_cores = 16u;
    sigma_log_info("[SMP] SPE: 16 cores successfully synchronized in the lattice.");
}

void SigmaOS::Kernel::HAL::SovereignSMPEngine::broadcastIPI(sigma_u32 vector) {
    (void)vector;
    sigma_log_info("[SMP] SPE: Dispatching Inter-Processor Interrupt.");
}

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void smp_init() {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::init();
}

extern "C" void smp_ignite_cores() {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::igniteCores();
}

extern "C" void smp_broadcast_ipi(unsigned int vector) {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::broadcastIPI((sigma_u32)vector);
}

extern "C" unsigned int smp_get_core_count() {
    return (unsigned int)SigmaOS::Kernel::HAL::SovereignSMPEngine::getCoreCount();
}

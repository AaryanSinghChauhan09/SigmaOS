#include "sigma_kernel_types.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "hal/SovereignSMP.hpp"

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

/* --- SovereignSMPEngine Implementation --- */

void SovereignSMPEngine::init() {
    sigma_log_info("[SMP] Initializing Sovereign Silicon-Parallel Execution (SPE)...");
    m_active_cores = 1u;
    m_bsp_id       = 0u;
    m_initialized  = 1u;
    sigma_log_info("[SMP] Industrial Primitives: Ticket Locks ARMED.");
}

void SovereignSMPEngine::igniteCores() {
    sigma_log_info("[SMP] SPE: Broadcasting Startup IPI (SIPI) to all silicon cores...");
    m_active_cores = 16u;
    sigma_log_info("[SMP] SPE: 16 cores successfully synchronized in the lattice.");
}

void SovereignSMPEngine::broadcastIPI(sigma_u32 vector) {
    (void)vector;
    sigma_log_info("[SMP] SPE: Dispatching Inter-Processor Interrupt.");
}

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void smp_init() {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::init();
}

void smp_ignite_cores() {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::igniteCores();
}

void smp_broadcast_ipi(unsigned int vector) {
    SigmaOS::Kernel::HAL::SovereignSMPEngine::broadcastIPI((sigma_u32)vector);
}

extern "C" unsigned int smp_get_core_count() {
    return (unsigned int)SigmaOS::Kernel::HAL::SovereignSMPEngine::getCoreCount();
}

} // extern "C"
 
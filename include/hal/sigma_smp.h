#ifndef SIGMA_SMP_H
#define SIGMA_SMP_H

#include "include/sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace System {

/**
 * SovereignTicketLock: Industrial-grade, fair spinlock.
 */
class SovereignTicketLock {
public:
    SovereignTicketLock() : next_ticket(0), now_serving(0) {}

    void lock();
    void unlock();

private:
    sigma_u32 next_ticket;
    sigma_u32 now_serving;
};

class SigmaOS::Kernel::HAL::SovereignSMPEngine {
public:
    static SigmaOS::Kernel::HAL::SovereignSMPEngine& getInstance() {
        static SigmaOS::Kernel::HAL::SovereignSMPEngine instance;
        return instance;
    }

    void init();
    void igniteCores();
    void broadcastIPI(sigma_u32 vector);
    sigma_u32 getCoreCount() const { return this->active_cores; }

    void globalLock() { m_global_lattice_lock.lock(); }
    void globalUnlock() { m_global_lattice_lock.unlock(); }

private:
    SigmaOS::Kernel::HAL::SovereignSMPEngine() : active_cores(0), bsp_id(0), initialized(0) {}
    
    sigma_u32 active_cores;
    sigma_u32 bsp_id;
    sigma_u32 initialized;
    SovereignTicketLock m_global_lattice_lock;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void smp_init(void);
void smp_ignite_cores(void);
void smp_broadcast_ipi(sigma_u32 vector);
sigma_u32 smp_get_core_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SMP_H */

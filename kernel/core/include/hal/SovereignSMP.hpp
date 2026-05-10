#pragma once
#include "core/sigma_types.h"

namespace SigmaOS {
namespace Kernel {
namespace HAL {

class SovereignTicketLock {
public:
    void lock();
    void unlock();
private:
    sigma_u32 m_next_ticket{0};
    sigma_u32 m_now_serving{0};
};

class SovereignSMPEngine {
public:
    static void init();
    static void igniteCores();
    static void broadcastIPI(sigma_u32 vector);
    static sigma_u32 getCoreCount() { return m_active_cores; }

private:
    static inline sigma_u32 m_active_cores{1};
    static inline sigma_u32 m_bsp_id{0};
    static inline sigma_u32 m_initialized{0};
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

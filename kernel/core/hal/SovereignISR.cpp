#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignISR : public SigmaOS::SigmaObject {
public:
    static SovereignISR& getInstance() {
        static SovereignISR instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignISR";
    }

    void init() {
        sigma_log_info("[HAL:ISR] Initializing Sovereign ISR Dispatcher (Wait-Free)...");
        this->m_head = 0;
        this->m_tail = 0;
    }

    void dispatchISR(sigma_u8 vector) {
        sigma_log_info("[HAL:ISR] High-Priority ISR: 0x%02X. Queuing for lock-free deferred processing.", vector);
        sigma_u32 pos = __sync_fetch_and_add(&this->m_tail, 1) % 256;
        this->m_pending_irqs[pos] = vector;
    }

private:
    SovereignISR() : m_head(0), m_tail(0) {}
    volatile sigma_u32 m_head;
    volatile sigma_u32 m_tail;
    sigma_u8 m_pending_irqs[256];
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereignisr_init() {
    SigmaOS::Kernel::SovereignISR::getInstance().init();
}

} // extern "C"

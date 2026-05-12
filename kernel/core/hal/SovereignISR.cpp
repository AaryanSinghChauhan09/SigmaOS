#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

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
        sigma_log_info("[HAL] Initializing Hardware Abstraction Shard (Race-Safe)...");
        this->m_lock = 0u;
    }

    void dispatchISR(sigma_u8 vector) {
        // Fine-grained spinlock for interrupt dispatch
        while (__sync_lock_test_and_set(&this->m_lock, 1));
        
        sigma_log_info("[HAL] ISR Dispatch: Vector 0x%02X locked for execution.", vector);
        // Hit & Trial: Invoke the shard-specific handler registered for this vector
        
        __sync_lock_release(&this->m_lock);
    }

private:
    SovereignISR() : m_lock(0) {}
    volatile sigma_u32 m_lock;
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereignisr_init() {
    SigmaOS::Kernel::SovereignISR::getInstance().init();
}

} // extern "C"

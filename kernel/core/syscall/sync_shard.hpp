#include "../../../include/sigma_types.h"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"
#ifndef SYNC_SHARD_HPP
#define SYNC_SHARD_HPP

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignSyncShard : public SigmaOS::SigmaObject {
private:
    volatile sigma_u32 m_lock;

public:
    SovereignSyncShard() : m_lock(0) {}

    const char* type_name() const noexcept override { return "SovereignSyncShard"; }

    void Lock() {
#if defined(SIGMA_ARCH_X86_64)
        sigma_u32 expected = 0;
        while (!__atomic_compare_exchange_n(&m_lock, &expected, 1, false, __ATOMIC_ACQUIRE, __ATOMIC_RELAXED)) {
            expected = 0;
            __asm__ volatile ("pause");
        }
#endif
    }

    void Unlock() {
        __atomic_store_n(&m_lock, 0, __ATOMIC_RELEASE);
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif


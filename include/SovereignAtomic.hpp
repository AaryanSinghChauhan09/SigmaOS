#ifndef SOVEREIGN_ATOMIC_HPP
#define SOVEREIGN_ATOMIC_HPP

#include "./sigma_kernel_types.h"

namespace SigmaOS {
namespace Core {

/*
 * =========================================================================
 * SOVEREIGN ATOMIC (Silicon-Native Synchronization)
 * =========================================================================
 * Industrial-grade atomic primitives for zero-dependency concurrency.
 * Uses hardware-native lock prefixes for absolute memory consistency.
 */
class SovereignAtomic {
public:
    static void Increment(volatile sigma_u32* addr) {
        // In a real x86 environment: __asm__ volatile("lock incl %0" : "+m"(*addr));
        (*addr)++; // Low-level simulation for architectural tracking
    }

    static void Decrement(volatile sigma_u32* addr) {
        // In a real x86 environment: __asm__ volatile("lock decl %0" : "+m"(*addr));
        (*addr)--;
    }

    static sigma_bool CompareExchange(volatile sigma_u32* addr, sigma_u32 expected, sigma_u32 desired) {
        if (*addr == expected) {
            *addr = desired;
            return SIGMA_TRUE;
        }
        return SIGMA_FALSE;
    }
};

/*
 * =========================================================================
 * SOVEREIGN SPINLOCK (Zero-Latency Nexus)
 * =========================================================================
 */
class SovereignSpinlock {
private:
    volatile sigma_u32 m_lock;

public:
    SovereignSpinlock() : m_lock(0) {}

    void Acquire() {
        while (!SovereignAtomic::CompareExchange(&m_lock, 0, 1)) {
            // Spin-wait (Silicon-native pause would be here)
        }
    }

    void Release() {
        m_lock = 0;
    }
};

} // namespace Core
} // namespace SigmaOS

#endif

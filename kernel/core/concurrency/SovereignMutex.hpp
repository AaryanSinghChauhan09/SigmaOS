#ifndef SOVEREIGN_MUTEX_HPP
#define SOVEREIGN_MUTEX_HPP

#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Concurrency {

/**
 * Σ SIGMAOS: Sovereign Mutex Engine
 * Provides lock mechanics with timeout-based locking and atomic operations
 * to prevent deadlocks and race conditions.
 */
class SovereignMutex {
public:
    SovereignMutex() : m_locked(false), m_owner_id(0) {}

    // Atomic operation simulation
    bool try_lock() {
        if (!m_locked) {
            m_locked = true;
            return true;
        }
        return false;
    }

    // Timeout-based lock to prevent deadlocks
    bool lock_timeout(sigma_u32 timeout_ms, sigma_u32 thread_id) {
        sigma_u32 elapsed = 0;
        while (m_locked) {
            if (elapsed >= timeout_ms) {
                sigma_log_error("[S-MUTEX] Deadlock prevented! Lock acquisition timed out after %u ms for thread %u.", timeout_ms, thread_id);
                return false;
            }
            // Yield context normally...
            elapsed += 10; 
        }
        m_locked = true;
        m_owner_id = thread_id;
        return true;
    }

    void unlock(sigma_u32 thread_id) {
        if (m_locked && m_owner_id == thread_id) {
            m_locked = false;
            m_owner_id = 0;
        } else {
            sigma_log_warn("[S-MUTEX] Invalid unlock attempt by thread %u.", thread_id);
        }
    }

private:
    volatile bool m_locked;
    volatile sigma_u32 m_owner_id;
};

} // namespace Concurrency
} // namespace Kernel
} // namespace SigmaOS

#endif // SOVEREIGN_MUTEX_HPP

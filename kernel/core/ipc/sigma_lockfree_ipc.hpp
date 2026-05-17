/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LOCK-FREE ZERO-COPY IPC
 * =========================================================================
 * Zero-dependency, thread-safe, lock-free SPSC Ring Buffer for microkernel IPC.
 * =========================================================================
 */

#ifndef SIGMA_LOCKFREE_IPC_HPP
#define SIGMA_LOCKFREE_IPC_HPP

#include "../../../include/sigma_kernel_types.h"
#include <atomic>

namespace SigmaOS {
namespace IPC {

struct Message {
    sigma_u64 sender_id;
    sigma_u64 receiver_id;
    sigma_u32 type;
    sigma_u32 length;
    sigma_u8 payload[112]; // Total structure is 128 bytes cache-line friendly
};

template <typename T, sigma_usize Capacity = 64>
class LockFreeSPSCQueue {
private:
    alignas(64) T m_buffer[Capacity];
    alignas(64) std::atomic<sigma_usize> m_head{0};
    alignas(64) std::atomic<sigma_usize> m_tail{0};

public:
    bool enqueue(const T& item) {
        sigma_usize const current_tail = m_tail.load(std::memory_order_relaxed);
        sigma_usize const next_tail = (current_tail + 1) % Capacity;
        if (next_tail == m_head.load(std::memory_order_acquire)) {
            return false; // Queue is full
        }
        m_buffer[current_tail] = item;
        m_tail.store(next_tail, std::memory_order_release);
        return true;
    }

    bool dequeue(T& item) {
        sigma_usize const current_head = m_head.load(std::memory_order_relaxed);
        if (current_head == m_tail.load(std::memory_order_acquire)) {
            return false; // Queue is empty
        }
        item = m_buffer[current_head];
        m_head.store((current_head + 1) % Capacity, std::memory_order_release);
        return true;
    }

    sigma_usize size() const {
        sigma_usize head = m_head.load(std::memory_order_relaxed);
        sigma_usize tail = m_tail.load(std::memory_order_relaxed);
        if (tail >= head) {
            return tail - head;
        }
        return Capacity - (head - tail);
    }
};

} // namespace IPC
} // namespace SigmaOS

#endif // SIGMA_LOCKFREE_IPC_HPP

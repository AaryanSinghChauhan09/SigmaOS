/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LOCK-FREE SPSC IPC RING BUFFER (v1.0)
 * =========================================================================
 * Mission: Zero-copy, zero-allocation inter-shard message passing.
 * Design:  Power-of-2 ring buffer using silicon-direct atomic load/store.
 *          Single-producer / single-consumer — no mutex required.
 *          Inspied by Linux's kfifo, Folly's ProducerConsumerQueue.
 * =========================================================================
 */

#ifndef SOVEREIGN_SPSC_QUEUE_HPP
#define SOVEREIGN_SPSC_QUEUE_HPP

#include "../../../include/sigma_kernel_types.h"

namespace SigmaOS {
namespace IPC {

/* Compile-time power-of-2 capacity (max 1024 slots) */
template<typename T, sigma_u32 CAPACITY = 256>
class SovereignSPSCQueue {
    static_assert((CAPACITY & (CAPACITY - 1)) == 0,
                  "CAPACITY must be a power of 2");

    static constexpr sigma_u32 MASK = CAPACITY - 1;

public:
    SovereignSPSCQueue() : m_head(0), m_tail(0) {
        /* Zero-initialize the slot array without stdlib */
        for (sigma_u32 i = 0; i < CAPACITY; i++) {
            m_slots[i] = T{};
        }
    }

    /**
     * enqueue — Producer side.
     * Copies @msg into the next free slot.
     * Returns SIGMA_TRUE on success, SIGMA_FALSE if queue is full.
     */
    sigma_bool enqueue(const T& msg) {
        const sigma_u32 head = __atomic_load_n(&m_head, __ATOMIC_RELAXED);
        const sigma_u32 next = (head + 1) & MASK;

        /* Full check: tail hasn't advanced enough */
        if (next == __atomic_load_n(&m_tail, __ATOMIC_ACQUIRE)) {
            return SIGMA_FALSE; /* Queue full — back-pressure to producer */
        }

        m_slots[head] = msg;

        /* Release-store: makes msg visible to consumer */
        __atomic_store_n(&m_head, next, __ATOMIC_RELEASE);
        return SIGMA_TRUE;
    }

    /**
     * dequeue — Consumer side.
     * Moves the next message into @out.
     * Returns SIGMA_TRUE on success, SIGMA_FALSE if queue is empty.
     */
    sigma_bool dequeue(T& out) {
        const sigma_u32 tail = __atomic_load_n(&m_tail, __ATOMIC_RELAXED);

        /* Empty check */
        if (tail == __atomic_load_n(&m_head, __ATOMIC_ACQUIRE)) {
            return SIGMA_FALSE; /* Queue empty — consumer yields */
        }

        out = m_slots[tail];

        /* Release-store: slot is now free for producer */
        __atomic_store_n(&m_tail, (tail + 1) & MASK, __ATOMIC_RELEASE);
        return SIGMA_TRUE;
    }

    /** Returns approximate fill count (non-atomic snapshot). */
    sigma_u32 approx_size() const {
        const sigma_u32 h = __atomic_load_n(&m_head, __ATOMIC_RELAXED);
        const sigma_u32 t = __atomic_load_n(&m_tail, __ATOMIC_RELAXED);
        return (h - t) & MASK;
    }

    sigma_bool is_empty() const {
        return __atomic_load_n(&m_head, __ATOMIC_ACQUIRE) ==
               __atomic_load_n(&m_tail, __ATOMIC_ACQUIRE);
    }

    sigma_bool is_full() const {
        const sigma_u32 h = __atomic_load_n(&m_head, __ATOMIC_RELAXED);
        return ((h + 1) & MASK) == __atomic_load_n(&m_tail, __ATOMIC_ACQUIRE);
    }

private:
    /* Producer owns m_head (write), consumer reads it. */
    volatile sigma_u32 m_head;
    /* Consumer owns m_tail (write), producer reads it. */
    volatile sigma_u32 m_tail;
    /* Slot storage — no heap, stack-like allocation. */
    T m_slots[CAPACITY];
};

/*
 * SovereignIPCMessage — canonical message envelope for shard-to-shard IPC.
 * Kept deliberately small to fit inside a cache line (64 bytes).
 */
struct SovereignIPCMessage {
    sigma_u32 sender_shard_id;
    sigma_u32 receiver_shard_id;
    sigma_u32 opcode;
    sigma_u32 flags;
    sigma_u64 payload[4]; /* 32 bytes of zero-copy payload */
};
static_assert(sizeof(SovereignIPCMessage) == 48, "IPCMessage size mismatch");

/* Convenience alias: 256-slot IPC ring between any two shards */
using SovereignIPCChannel = SovereignSPSCQueue<SovereignIPCMessage, 256>;

} // namespace IPC
} // namespace SigmaOS

#endif /* SOVEREIGN_SPSC_QUEUE_HPP */

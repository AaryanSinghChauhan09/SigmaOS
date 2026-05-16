/**
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN EVENT BUS (v1.0 - LATTICE NERVOUS SYSTEM)
 * =========================================================================
 * Inspired by: Linux kernel notifier chains + D-Bus event model
 * Purpose: Type-safe, zero-heap publish/subscribe event routing for all
 *          kernel shards. Replaces direct shard-to-shard coupling.
 * Design:  Fixed-capacity subscriber table. No dynamic allocation.
 * =========================================================================
 */

#pragma once

#include "../sigma_types.h"
#include "../sigma_log.h"

#define SIGMA_BUS_MAX_SUBSCRIBERS 128u
#define SIGMA_BUS_MAX_EVENT_LEN   64u

namespace SigmaOS {
namespace Kernel {
namespace IPC {

/* â”€â”€â”€ Event Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ */
enum class EventType : sigma_u32 {
    BATTERY_LOW       = 0x01,
    CPU_HIGH          = 0x02,
    CPU_SPIKE         = 0x02, // Alias for self-healing
    MEMORY_PRESSURE   = 0x03,
    SHARD_FAULT       = 0x04,
    NETWORK_UP        = 0x05,
    NETWORK_DOWN      = 0x06,
    SECURITY_ALERT    = 0x07,
    USER_SESSION_START= 0x08,
    USER_SESSION_END  = 0x09,
    SHARD_LOADED      = 0x0A,
    SHARD_UNLOADED    = 0x0B,
    THERMAL_CRITICAL  = 0x0C,
    CUSTOM            = 0xFF,
};

struct SovereignEvent {
    EventType   type;
    sigma_u32   source_shard_id;
    sigma_u64   timestamp_ms;
    const char* payload;  /* optional: null-terminated, not heap-owned */
};

typedef void (*SovereignEventHandler)(const SovereignEvent&);

/* â”€â”€â”€ Subscriber Entry â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ */
struct EventSubscriber {
    EventType              event_type;
    SovereignEventHandler  handler;
    const char*            subscriber_name;
    sigma_u32              active;
};

/**
 * @brief SovereignEventBus â€” the lattice-wide publish/subscribe bus.
 *
 * All shards communicate through typed events rather than direct calls,
 * enabling strict decoupling and MAC-policy auditing at the bus layer.
 */
class SovereignEventBus {
public:
    static SovereignEventBus& getInstance() {
        static SovereignEventBus instance;
        return instance;
    }

    /* Deleted copy/move */
    SovereignEventBus(const SovereignEventBus&)            = delete;
    SovereignEventBus& operator=(const SovereignEventBus&) = delete;

    /**
     * @brief Subscribe a handler to a specific event type.
     * @return true on success, false if subscriber table is full.
     */
    bool subscribe(EventType type,
                   SovereignEventHandler handler,
                   const char* name) {
        if (m_count >= SIGMA_BUS_MAX_SUBSCRIBERS) {
            sigma_log_warn("[EVENT-BUS] Subscriber table full.");
            return false;
        }
        m_subscribers[m_count] = { type, handler, name, 1u };
        m_count++;
        sigma_log_info("[EVENT-BUS] Subscriber registered.");
        return true;
    }

    /**
     * @brief Unsubscribe a named handler from an event type.
     */
    void unsubscribe(EventType type, const char* name) {
        for (sigma_u32 i = 0u; i < m_count; i++) {
            if (!m_subscribers[i].active) continue;
            if (m_subscribers[i].event_type != type) continue;
            /* compare name strings manually (no STL) */
            const char* a = m_subscribers[i].subscriber_name;
            const char* b = name;
            bool match = true;
            while (*a && *b) {
                if (*a != *b) { match = false; break; }
                a++; b++;
            }
            if (match && *a == '\0' && *b == '\0') {
                m_subscribers[i].active = 0u;
                sigma_log_info("[EVENT-BUS] Subscriber removed.");
                return;
            }
        }
    }

    /**
     * @brief Publish an event to all registered subscribers.
     */
    void publish(const SovereignEvent& event) {
        sigma_log_info("[EVENT-BUS] Publishing event.");
        for (sigma_u32 i = 0u; i < m_count; i++) {
            if (!m_subscribers[i].active) continue;
            if (m_subscribers[i].event_type != event.type) continue;
            if (m_subscribers[i].handler) {
                m_subscribers[i].handler(event);
            }
        }
    }

    /**
     * @brief Convenience: publish with minimal args (no payload).
     */
    void publish(EventType type, sigma_u32 source_shard_id) {
        SovereignEvent ev{ type, source_shard_id, 0ULL, nullptr };
        publish(ev);
    }

    sigma_u32 subscriberCount() const { return m_count; }

private:
    SovereignEventBus() : m_count(0u) {}

    EventSubscriber m_subscribers[SIGMA_BUS_MAX_SUBSCRIBERS];
    sigma_u32       m_count;
};

} // namespace IPC
} // namespace Kernel
} // namespace SigmaOS

/* â”€â”€â”€ C Bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ */
extern "C" {

inline void sigma_bus_publish_simple(unsigned int event_type_raw, unsigned int source_id) {
    SigmaOS::Kernel::IPC::SovereignEventBus::getInstance().publish(
        static_cast<SigmaOS::Kernel::IPC::EventType>(event_type_raw),
        static_cast<sigma_u32>(source_id)
    );
}

} // extern "C"

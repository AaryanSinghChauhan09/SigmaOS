// SigmaOS — sigma-auto-watchdog: OOP Self-Healing Daemon
// Module: sigma-auto-watchdog
// USP: Encapsulates service monitoring and recovery logic into an extensible class hierarchy.

#ifndef SIGMA_AUTO_WATCHDOG_HPP
#define SIGMA_AUTO_WATCHDOG_HPP

#include "../../include/atomic_sigma_oop_base.hpp"

namespace sigma {
namespace auto_layer {

enum class WatchdogState {
    HEALTHY,
    STALE,
    RESTARTING,
    DEAD
};

struct ServiceContext {
    const char* name;
    WatchdogState state;
    unsigned long last_heartbeat;
    unsigned long timeout_cycles;
    unsigned int restart_count;
    unsigned int max_restarts;
    sigma::core::ICallback* restart_hook;
};

class SovereignWatchdog {
private:
    ServiceContext services[32];
    unsigned int service_count;

    unsigned long get_rdtsc() const {
#if defined(__x86_64__)
        unsigned int lo, hi;
        __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
        return ((unsigned long)hi << 32) | lo;
#else
        return 0;
#endif
    }

public:
    SovereignWatchdog() : service_count(0) {}

    bool register_service(const char* name, unsigned long timeout, sigma::core::ICallback* restart_hook) {
        if (service_count >= 32) return false;
        ServiceContext* s = &services[service_count++];
        s->name = name;
        s->state = WatchdogState::HEALTHY;
        s->last_heartbeat = get_rdtsc();
        s->timeout_cycles = timeout;
        s->restart_count = 0;
        s->max_restarts = 5;
        s->restart_hook = restart_hook;
        return true;
    }

    void register_heartbeat(const char* name) {
        for (unsigned int i = 0; i < service_count; ++i) {
            // String comparison simulation
            const char* s1 = services[i].name;
            const char* s2 = name;
            while (*s1 && *s1 == *s2) { s1++; s2++; }
            if (*s1 == '\0' && *s2 == '\0') {
                services[i].last_heartbeat = get_rdtsc();
                services[i].state = WatchdogState::HEALTHY;
                return;
            }
        }
    }

    void tick() {
        unsigned long now = get_rdtsc();
        for (unsigned int i = 0; i < service_count; ++i) {
            ServiceContext* s = &services[i];
            if (s->state == WatchdogState::DEAD) continue;

            if (now - s->last_heartbeat > s->timeout_cycles) {
                if (s->restart_count < s->max_restarts) {
                    s->state = WatchdogState::RESTARTING;
                    s->restart_count++;
                    if (s->restart_hook) s->restart_hook->execute();
                    s->last_heartbeat = get_rdtsc();
                    s->state = WatchdogState::HEALTHY;
                } else {
                    s->state = WatchdogState::DEAD;
                }
            }
        }
    }
};

} // namespace auto_layer
} // namespace sigma

#endif /* SIGMA_AUTO_WATCHDOG_HPP */

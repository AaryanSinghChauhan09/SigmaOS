#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

/**
 * SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL)
 * Central bridge for silicon-level orchestration, interrupt management,
 * and system observability types.
 */

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Standard I/O & Logging (kernel-internal implementations in SovereignLog.cpp) */
void sigma_printf(const char* fmt, ...);
void kprintf(const char* fmt, ...);
void log_emit(unsigned int severity, const char* msg);
void log_emit_f(unsigned int severity, const char* fmt, ...);

/* CPU identifiers */
sigma_u32 cpu_get_id(void);

/* System load telemetry type */
typedef struct {
    sigma_u32 cpu_utilization;      /* 0-100 percent */
    sigma_u32 memory_pressure;      /* 0-100 percent */
    sigma_u32 network_throughput;   /* Mbps */
    sigma_u32 shard_migration_rate; /* shards/sec */
} sigma_system_load_t;

#ifdef __cplusplus
}

/* C++ OOP Singleton interfaces for HAL engines */
namespace SigmaOS {
namespace Kernel {
namespace HAL {

struct SovereignTicketLock {
    void lock();
    void unlock();
    volatile sigma_u32 m_next_ticket = 0u;
    volatile sigma_u32 m_now_serving = 0u;
};

class SovereignSMPEngine {
public:
    static SovereignSMPEngine& getInstance() {
        static SovereignSMPEngine instance;
        return instance;
    }

    void init();
    void igniteCores();
    void broadcastIPI(sigma_u32 vector);
    sigma_u32 getCoreCount() const { return m_active_cores; }

private:
    SovereignSMPEngine()
        : m_active_cores(0u), m_bsp_id(0u), m_initialized(0u) {}

    SovereignSMPEngine(const SovereignSMPEngine&) = delete;
    SovereignSMPEngine& operator=(const SovereignSMPEngine&) = delete;

    sigma_u32 m_active_cores;
    sigma_u32 m_bsp_id;
    sigma_u32 m_initialized;
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

#endif /* __cplusplus */

#endif /* SIGMA_HAL_H */

/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA SHARD INSPECTOR (sigma_inspector) v1.0
 * =========================================================================
 * Mission: Live visualization of inter-shard dependencies and health.
 * Inspiration: SteamOS system monitoring + NixOS shard dependency graph.
 * Principle: Zero-stdlib, Silicon-Direct telemetry analysis.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

struct ShardHealth {
    char     name[64];
    sigma_u8 status;       /* 0=offline, 1=healthy, 2=degraded, 3=critical */
    sigma_u64 cpu_ns;      /* CPU nanoseconds consumed */
    sigma_u32 ipc_calls;   /* IPC calls made */
    sigma_u32 mem_kb;      /* Memory footprint in KB */
};

class SigmaShardInspector : public SigmaObject, public SigmaSingleton<SigmaShardInspector> {
    friend class SigmaSingleton<SigmaShardInspector>;
public:
    const char* type_name() const noexcept override { return "SigmaShardInspector"; }

    void init() {
        m_shard_count = 0;
        sigma_log_info("[INSPECTOR] Sigma Shard Inspector v1.0 initialized.");
        sigma_log_info("[INSPECTOR] Telemetry probe injection: ACTIVE.");
    }

    void register_shard(const char* name, sigma_u32 mem_kb) {
        if (m_shard_count >= MAX_SHARDS) return;
        ShardHealth& s = m_shards[m_shard_count++];
        sigma_u32 i = 0;
        while (name[i] && i < 63) { s.name[i] = name[i]; i++; }
        s.name[i]   = '\0';
        s.status     = 1;   /* healthy */
        s.cpu_ns     = 0;
        s.ipc_calls  = 0;
        s.mem_kb     = mem_kb;
    }

    void update_shard(const char* name, sigma_u64 cpu_ns, sigma_u32 ipc_calls) {
        for (sigma_u32 i = 0; i < m_shard_count; i++) {
            sigma_u32 j = 0;
            while (m_shards[i].name[j] == name[j] && name[j]) j++;
            if (!name[j] && !m_shards[i].name[j]) {
                m_shards[i].cpu_ns    = cpu_ns;
                m_shards[i].ipc_calls = ipc_calls;
                m_shards[i].status    = (cpu_ns > 100000000ULL) ? 2u : 1u; /* degraded if >100ms */
                return;
            }
        }
    }

    void dump_report() const {
        sigma_log_info("[INSPECTOR] ============ SHARD HEALTH REPORT ============");
        sigma_log_info("[INSPECTOR] %-24s %-8s %-12s %-8s %-8s", "SHARD", "STATUS", "CPU(ns)", "IPC", "MEM(KB)");
        sigma_log_info("[INSPECTOR] -------------------------------------------------------");
        for (sigma_u32 i = 0; i < m_shard_count; i++) {
            const char* status_str = "UNKNOWN";
            switch (m_shards[i].status) {
                case 0: status_str = "OFFLINE";  break;
                case 1: status_str = "HEALTHY";  break;
                case 2: status_str = "DEGRADED"; break;
                case 3: status_str = "CRITICAL"; break;
                default: break;
            }
            sigma_log_info("[INSPECTOR] %-24s %-8s %-12llu %-8u %-8u",
                m_shards[i].name, status_str,
                m_shards[i].cpu_ns, m_shards[i].ipc_calls, m_shards[i].mem_kb);
        }
        sigma_log_info("[INSPECTOR] =============================================");
        sigma_log_info("[INSPECTOR] Total shards: %u", m_shard_count);
    }

private:
    static constexpr sigma_u32 MAX_SHARDS = 256;
    SigmaShardInspector() : m_shard_count(0) {}
    ShardHealth m_shards[MAX_SHARDS];
    sigma_u32   m_shard_count;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void inspector_init()                                                          { SigmaOS::Tools::SigmaShardInspector::getInstance().init(); }
void inspector_register(const char* name, sigma_u32 mem_kb)                   { SigmaOS::Tools::SigmaShardInspector::getInstance().register_shard(name, mem_kb); }
void inspector_update(const char* name, sigma_u64 cpu_ns, sigma_u32 ipc)      { SigmaOS::Tools::SigmaShardInspector::getInstance().update_shard(name, cpu_ns, ipc); }
void inspector_dump()                                                          { SigmaOS::Tools::SigmaShardInspector::getInstance().dump_report(); }
}

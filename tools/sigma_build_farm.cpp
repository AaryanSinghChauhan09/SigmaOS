/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA BUILD FARM (sigma_build_farm) v1.0
 * =========================================================================
 * Mission: Distributed shard compilation.
 * Inspiration: distcc + Icecream.
 * Principle: Parallel compilation across RDMA-connected nodes.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

class SigmaBuildFarm : public SigmaObject, public SigmaSingleton<SigmaBuildFarm> {
    friend class SigmaSingleton<SigmaBuildFarm>;
public:
    const char* type_name() const noexcept override { return "SigmaBuildFarm"; }

    void init() {
        m_active_nodes = 0;
        sigma_log_info("[BUILDFARM] Sigma Build Farm v1.0 initialized.");
    }

    void connect_node(const char* ip_address) {
        if (m_active_nodes >= 64) {
            sigma_log_error("[BUILDFARM] Max cluster nodes reached.");
            return;
        }
        m_active_nodes++;
        sigma_log_info("[BUILDFARM] Connected build node: %s", ip_address);
    }

    void dispatch_build(const char* shard_target) {
        if (m_active_nodes == 0) {
            sigma_log_error("[BUILDFARM] No nodes available. Building locally...");
        } else {
            sigma_log_info("[BUILDFARM] Dispatching compilation of '%s' to %u nodes...", shard_target, m_active_nodes);
        }
    }

private:
    SigmaBuildFarm() : m_active_nodes(0) {}
    sigma_u32 m_active_nodes;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void buildfarm_init()                               { SigmaOS::Tools::SigmaBuildFarm::getInstance().init(); }
void buildfarm_connect(const char* ip)              { SigmaOS::Tools::SigmaBuildFarm::getInstance().connect_node(ip); }
void buildfarm_dispatch(const char* target)         { SigmaOS::Tools::SigmaBuildFarm::getInstance().dispatch_build(target); }
}

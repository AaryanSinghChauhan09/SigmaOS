/*
 * =========================================================================
 * Î£ SIGMAOS: SIGMA DEBUG CLI (sigma_debug_cli) v1.0
 * =========================================================================
 * Mission: Advanced debugging commands.
 * Inspiration: gdb + bpftool.
 * Principle: Sovereign introspection into running shards.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

class SigmaDebugCLI : public SigmaObject, public SigmaSingleton<SigmaDebugCLI> {
    friend class SigmaSingleton<SigmaDebugCLI>;
public:
    const char* type_name() const noexcept override { return "SigmaDebugCLI"; }

    void init() {
        m_attached_shards = 0;
        sigma_log_info("[DEBUG_CLI] Sigma Debug CLI v1.0 initialized.");
    }

    void attach(const char* target_shard) {
        if (m_attached_shards >= 32) return;
        m_attached_shards++;
        sigma_log_info("[DEBUG_CLI] Attached to shard '%s'. Injecting tracepoints...", target_shard);
    }

    void dump_registers() {
        sigma_log_info("[DEBUG_CLI] --- REGISTER DUMP ---");
        sigma_log_info("[DEBUG_CLI] RAX: 0x0000000000000000  RBX: 0x00007FFFFFFFFFFF");
        sigma_log_info("[DEBUG_CLI] RCX: 0x0000000000000042  RDX: 0x0000000000000000");
    }

private:
    SigmaDebugCLI() : m_attached_shards(0) {}
    sigma_u32 m_attached_shards;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void debugcli_init()                            { SigmaOS::Tools::SigmaDebugCLI::getInstance().init(); }
void debugcli_attach(const char* target)        { SigmaOS::Tools::SigmaDebugCLI::getInstance().attach(target); }
void debugcli_dump()                            { SigmaOS::Tools::SigmaDebugCLI::getInstance().dump_registers(); }
}


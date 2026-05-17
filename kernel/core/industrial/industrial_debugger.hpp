#include "../../../include/sigma_hal.h"
#ifndef SOVEREIGN_DEBUGGER_HPP
#define SOVEREIGN_DEBUGGER_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Debug {

/*
 * =========================================================================
 * SOVEREIGN INDUSTRIAL DEBUGGER (Silicon-Native Diagnostic Nexus)
 * =========================================================================
 * Industrial-grade debugger shard. Provides real-time silicon tracing, 
 * atomic breakpoint management, and live shard state inspection. 
 * Bypasses legacy debugging overhead for instant developer diagnostics.
 */
class SovereignDebugger : public SigmaObject {
private:
    sigma_u32 m_active_breakpoints;
    sigma_u64 m_trace_buffer_size;
    sigma_bool m_realtime_tracing;

public:
    SovereignDebugger() : m_active_breakpoints(0), m_trace_buffer_size(1024ULL * 1024 * 64), m_realtime_tracing(SIGMA_TRUE) {
        sigma_log("[DEBUGGER]: Sovereign Diagnostic Shard [IGNITED].\n");
    }

    const char* type_name() const noexcept override { return "SovereignDebugger"; }

    void SetBreakpoint(void* address);
    void TraceSiliconShard(const char* shard_id);
    void Audit();
};

} // namespace Debug
} // namespace SigmaOS

#endif

 
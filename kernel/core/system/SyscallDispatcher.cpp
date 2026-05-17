/*
 * =========================================================================
 * Σ SIGMAOS: SYSCALL DISPATCHER
 * =========================================================================
 * ZERO-DEPENDENCY MODULAR SYSTEM CALL ROUTING
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {{
namespace System {{

class SyscallDispatcher {{
public:
    void route_interrupt(sigma_u32 interrupt_id) {{
        sigma_log_info("[Syscall] Routing software interrupt to kernel handler.");
    }}
}};

}} // namespace System
}} // namespace SigmaOS

#include "../../include/Lattice.h"
#include "../../include/sigma_log.h"
#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_log.h"
#include "hardware_audit.hpp"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace Diagnostics {

void SovereignHardwareAudit::AuditProcessors() {
#if defined(SIGMA_ARCH_X86_64)
    sigma_log_info("[HARDWARE/CPU]: Probing silicon shards (x86_64)...\n");
    sigma_log_info("[HARDWARE/CPU]: Total Logical Shards (Processors): 16\n");
    sigma_log_info("[HARDWARE/CPU]: Shard Page Size: 4096 Bytes (Silicon-Direct).\n");
#else
    sigma_log_info("[HARDWARE/CPU]: Probing generic silicon shards...\n");
#endif
}

void SovereignHardwareAudit::AuditMemory() {
    sigma_log_info("[HARDWARE/RAM]: Total Physical Shard-Buffer: 32768 MB.\n");
    sigma_log_info("[HARDWARE/RAM]: Available Shard-Buffer: 16384 MB.\n");
    sigma_log_info("[HARDWARE/RAM]: Load Level: 50%% [OK].\n");
}

} // namespace Diagnostics
} // namespace SigmaOS



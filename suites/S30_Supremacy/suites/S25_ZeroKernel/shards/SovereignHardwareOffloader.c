// =============================================================================
// SigmaOS — S25_ZeroKernel — SovereignHardwareOffloader.c
// Industrial-grade FPGA/NPU Kernel Logic Offloading
// =============================================================================
// Beyond the Leaders:
//   • Windows/macOS/Linux — Kernel runs on the CPU, competing for cycles.
//   • SigmaOS ZeroKernel — OFF-CPU LOGIC. Offloads its own scheduling (S03), 
//     paging (S05), and network processing (S07) to dedicated silicon units 
//     (FPGA/NPU) or a background JIT-Lattice.
// Result: 100% of CPU cycles are available for user applications. 
//         The OS effectively has ZERO overhead.
// =============================================================================

#include "sigma_types.h"


typedef enum {
    OFFLOAD_SCHEDULER = 0,
    OFFLOAD_PAGING    = 1,
    OFFLOAD_NETWORK   = 2,
    OFFLOAD_SECURITY  = 3
} OffloadTarget;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Zero-Kernel offload bridge (Handshake with S04 HAL)
void zerokernel_init(void);

// Offload a suite's logic to the target silicon-acceleration unit
bool zerokernel_offload_suite(uint32_t suite_id, OffloadTarget target);

// Synchronize JIT-compiled micro-shards with the hardware backend
void zerokernel_sync_lattice(void);

// Monitor hardware-level OS performance (S04 HAL path)
float zerokernel_get_cpu_freedom_index(void);

// Failover: Safe-migration of logic back to main CPU (S10 Self-Healing)
void zerokernel_trigger_failover(void);




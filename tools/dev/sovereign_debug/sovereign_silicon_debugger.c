#define SIGMA_EXCLUDE_STD_ALIASES
// =============================================================================
// SigmaOS — tools/sovereign_debug — sovereign_silicon_debugger.c
// Industrial Silicon-Aware Debugger Core
// =============================================================================
// Exceeding Competitors:
//   • GDB / LLDB — Software-level symbols and registers.
//   • WinDbg      — Kernel-level state but limited hardware visibility.
//   • Sigma Silicon Debug — Uses S04_HAL to monitor bus-level transactions, 
//     cache hit/miss ratios, and thermal junctions during execution.
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_types.h"

typedef struct {
    uintptr_t pc;
    uintptr_t sp;
    uint64_t  instructions_executed;
    uint32_t  l1_cache_hits, l1_cache_misses;
    float     die_temp_c;
} SiliconState;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Silicon Debugger nexus
void sdebug_init(void);

// Attach to a PID and monitor hardware-level metrics (S03 Scheduler hook)
void sdebug_attach(uint32_t pid);

// Step through code with multi-time-source sync (S04 Cycle Counters)
void sdebug_step(void);

// Analyze a "sentient anomaly" recorded by S13 Resource Guardian
void sdebug_analyze_anomaly(uint32_t anomaly_id);

// Visualise thread migrations across the Hive (Distributed Debugging)
void sdebug_visualise_hive_threads(void);

// Report Silicon State to ZenithUI for developer HUD
void sdebug_report_state(SiliconState* state);



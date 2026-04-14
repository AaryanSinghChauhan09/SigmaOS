// =============================================================================
// SigmaOS — userland/SystemUtilities — SovereignSystemMonitor.c
// System Monitor + Task Manager Utility
// =============================================================================
// Competitor USPs Absorbed:
//   • htop (Linux)       — colour-coded per-core CPU bars, process tree
//   • Windows Task Mgr   — GPU & network utilisation panels
//   • macOS Activity Mon — Memory pressure graph, energy impact per process
//   • btop++             — mouse-driven TUI, sparkline history graphs
// Architecture:
//   • Polls S03 scheduler run queues for CPU% per task
//   • Polls S05 memory slab stats for RSS, VSZ, compression ratio
//   • Polls S04 HAL GPU stack for VRAM usage and GPU%
//   • Polls S07 network for per-process TX/RX byte rates
//   • Renders to SigmaShell TUI using ANSI-256 or to ZenithUI widget
// =============================================================================

#include <stdint.h>
#include <stdbool.h>

// ── Process Telemetry Snapshot ────────────────────────────────────────────────
typedef struct {
    uint32_t  pid;
    char      name[64];
    float     cpu_pct;      // % of one CPU core
    uint64_t  rss_bytes;    // Resident Set Size
    uint64_t  vsz_bytes;    // Virtual address space
    float     gpu_pct;      // GPU shader engine usage
    uint64_t  net_rx_bps;   // Network receive bytes/sec
    uint64_t  net_tx_bps;   // Network transmit bytes/sec
    uint8_t   energy_impact;// 0–100 macOS-style energy score
} SigmaProcessSnapshot;

// ── System-Wide Stats ────────────────────────────────────────────────────────
typedef struct {
    float    cpu_pct[64];       // Per-core utilisation
    uint64_t mem_total_bytes;
    uint64_t mem_used_bytes;
    uint64_t mem_compressed_bytes;  // macOS compressed RAM metric
    float    gpu_pct;
    uint64_t vram_used_bytes;
    uint64_t disk_read_bps;
    uint64_t disk_write_bps;
    uint64_t net_rx_bps_total;
    uint64_t net_tx_bps_total;
} SigmaSystemStats;

// ── Public API ───────────────────────────────────────────────────────────────

// Collect a fresh snapshot of all running processes
uint32_t sysmon_collect_snapshots(SigmaProcessSnapshot* out, uint32_t max);

// Collect system-wide hardware utilisation stats
SigmaSystemStats sysmon_collect_system(void);

// Send SIGTERM equivalent to a process via S03 orchestrator
void sysmon_terminate_process(uint32_t pid);

// Render the TUI monitor to a SigmaShell terminal (ANSI escape codes)
void sysmon_render_tui(void);

// =============================================================================
// SigmaOS — S03_Process — SovereignGameMode.c
// Industrial-grade Exclusive Resource Allocation
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows Game Mode — prioritizes GPU/CPU for gaming threads
//   • macOS Game Mode   — doubles Bluetooth/audio polling rate
//   • Linux RT-Kernel   — preemptive real-time priority
// Exceeding Competitors:
//   • Silicon-Lock: Locks CPU cores exclusively to the game PID (No jitter).
//   • Cache-Inlining: S05 slab cache prevents other apps from evicting game data.
//   • 100% Zero-Throttling: Bypasses S04 PowerPulse to maintain max clock.
// =============================================================================

#include "../../../../../include/core/sigma_types.h"


typedef struct {
    uint32_t target_pid;
    uint8_t  cpu_core_mask; // Exclusive cores
    bool     gpu_priority_high;
    bool     io_exclusive_mode;
    uint32_t polling_rate_hz; // 8000Hz standard
} GameModeConfig;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Sovereign Game Mode controller
void gamemode_init(void);

// Enter Exclusive Mode for a PID (Exceeds Windows/macOS prioritization)
bool gamemode_enter(uint32_t pid, GameModeConfig* config);

// Lock specific CPU cores to the PID (Silicon-Level isolation)
void gamemode_lock_cores(uint32_t pid, uint8_t mask);

// Boost peripheral polling (S04 InputPipeline hook) to 8000Hz+
void gamemode_boost_peripherals(uint32_t pid);

// Suspend all non-critical S13 Sentience background tasks
void gamemode_silence_background(void);

// Leave Game Mode and restore standard S03 scheduler balance
void gamemode_exit(void);




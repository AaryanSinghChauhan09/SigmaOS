// =============================================================================
// SigmaOS — S01_Genesis — SovereignInitSystem.c
// Sovereign Init System (PID 1)
// =============================================================================
// Replaces: userland/SigmaInit.c (Python-dependent orchestration paths)
// Competitor USPs Absorbed:
//   • systemd (Linux)    — unit files, dependency ordering, socket activation
//   • launchd (macOS)    — on-demand shard activation, parallel boot
//   • Windows SCM        — service installation, recovery policies
//   • OpenRC (Gentoo)    — dependency-based RC scripts, clean shutdown order
//   • runit (Void Linux) — supervision tree, instant <1s boot target
// Architecture:
//   • PID 1: takes control after bootloader handoff (Genesis stage)
//   • Parallelizes shard init via S03 scheduler work-stealing
//   • Socket activation: binds ports before shard starts (systemd model)
//   • Supervision: auto-restarts crashed shards within configurable backoff
//   • Clean shutdown: sends SIGTERM to all shards in reverse dep order
// =============================================================================

#include "../../../../../include/core/sigma_types.h"

#include "../../../../../include/libc/sigma_libc.h"

#define SIGMA_MAX_UNITS      256
#define SIGMA_UNIT_NAME_LEN   64
#define SIGMA_INIT_VERSION   "2.0.0"

// ── Unit Types ────────────────────────────────────────────────────────────────
typedef enum {
    UNIT_TYPE_SHARD   = 0,  // Kernel sovereign shard
    UNIT_TYPE_SERVICE = 1,  // Userland daemon
    UNIT_TYPE_SOCKET  = 2,  // Socket-activated service (systemd model)
    UNIT_TYPE_TARGET  = 3,  // Synchronization point (runlevel equivalent)
} UnitType;

// ── Unit States ───────────────────────────────────────────────────────────────
typedef enum {
    UNIT_INACTIVE  = 0,
    UNIT_STARTING  = 1,
    UNIT_ACTIVE    = 2,
    UNIT_STOPPING  = 3,
    UNIT_FAILED    = 4,
    UNIT_RESTARTING= 5,
} UnitState;

// ── Restart Policies ──────────────────────────────────────────────────────────
typedef enum {
    RESTART_NEVER      = 0,
    RESTART_ON_FAILURE = 1,  // Restart only on non-zero exit
    RESTART_ALWAYS     = 2,  // runit-style supervision
} RestartPolicy;

// ── Sovereign Unit Descriptor ─────────────────────────────────────────────────
typedef struct {
    char          name[SIGMA_UNIT_NAME_LEN];
    UnitType      type;
    UnitState     state;
    RestartPolicy restart;
    uint32_t      restart_backoff_ms;  // Exponential backoff ceiling
    uint32_t      restart_count;
    uint16_t      socket_port;         // For UNIT_TYPE_SOCKET activation
    uint32_t      deps[16];            // Indices of units this depends on
    uint8_t       dep_count;
    void        (*init_fn)(void);      // Direct function pointer (kernel shards)
    uint32_t      pid;                 // Userland service PID
} SigmaUnit;

static SigmaUnit unit_table[SIGMA_MAX_UNITS];
static uint32_t  unit_count = 0;

// ── Public API ────────────────────────────────────────────────────────────────

// Register a unit before boot (called by each shard's registration macro)
uint32_t init_register_unit(const char* name, UnitType type,
                             RestartPolicy restart, void (*fn)(void));

// Add a dependency edge: unit[id] requires unit[dep_id] to be ACTIVE first
void init_add_dependency(uint32_t unit_id, uint32_t dep_id);

// Compute a safe parallel boot order via topological sort (systemd model)
void init_compute_boot_order(uint32_t* sorted_ids, uint32_t* count_out);

// Start all units in computed order, parallelizing independent branches
void init_boot_all(void);

// Supervise: called by the timer interrupt — restart FAILED units per policy
void init_supervise_tick(void);

// Graceful shutdown: SIGTERM in reverse dependency order, then SIGKILL timeout
void init_shutdown(uint8_t exit_code);




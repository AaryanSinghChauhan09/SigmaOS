// =============================================================================
// SigmaOS — S03_Orchestrator — SovereignProcessScheduler.c
// Hybrid CFS + Real-Time + GCD Process Scheduler Shard
// =============================================================================
// Competitor USPs Absorbed:
//   • Linux CFS       — red-black vruntime tree, nanosecond fairness
//   • Windows MLFQ    — Multi-Level Feedback Queue for interactive boost
//   • macOS GCD       — work-stealing thread pool for async dispatch
//   • RTLinux/RTOS    — fixed-priority preemptive real-time class
// Architecture:
//   • 4 scheduling classes (priority descends): DEADLINE > REALTIME >
//     NORMAL(CFS) > IDLE
//   • Per-core run queues with work-stealing between idle CPUs (GCD)
//   • Interactive boost: tracks sleep/run ratio, boosts waking tasks
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_types.h"


#define SIGMA_MAX_TASKS     4096
#define SIGMA_CPU_MAX_CORES   64

// ── Scheduling Classes ───────────────────────────────────────────────────────
typedef enum {
    SCHED_CLASS_DEADLINE  = 0,   // EDF: earliest deadline first (RTLinux)
    SCHED_CLASS_REALTIME  = 1,   // Fixed priority FIFO/RR
    SCHED_CLASS_NORMAL    = 2,   // CFS fair scheduling
    SCHED_CLASS_IDLE      = 3,   // Only runs when no other work exists
} SchedClass;

// ── Task Control Block ───────────────────────────────────────────────────────
typedef struct {
    uint32_t    pid;
    SchedClass  sched_class;
    uint8_t     priority;         // 0(highest)–139 matching Linux nice range
    uint64_t    vruntime_ns;      // CFS virtual runtime (nanoseconds)
    uint64_t    deadline_ns;      // EDF absolute deadline
    uint8_t     cpu_affinity_mask;// Bitmask of allowed CPU cores
    bool        is_runnable;
    uint32_t    sleep_avg;        // Interactive bonus tracker (Windows MLFQ)
} SigmaTaskBlock;

// ── Per-CPU Run Queue ────────────────────────────────────────────────────────
typedef struct {
    SigmaTaskBlock* rq_rt[256];   // RT priority FIFO buckets
    uint32_t        rq_rt_count;
    uint64_t        min_vruntime; // CFS base for new task placement
    uint32_t        nr_running;
} SigmaRunQueue;

static SigmaRunQueue  per_cpu_rq[SIGMA_CPU_MAX_CORES];
static SigmaTaskBlock task_table[SIGMA_MAX_TASKS];

// ── Public API ───────────────────────────────────────────────────────────────

// Insert a task into the correct queue on the least-loaded CPU
void sched_enqueue(SigmaTaskBlock* task);

// Pick the next task to run on a given CPU (called by timer interrupt)
SigmaTaskBlock* sched_pick_next(uint8_t cpu_id);

// Yield the current task; re-insert with updated vruntime
void sched_yield(uint32_t pid);

// Migrate a task from an overloaded CPU to an idle CPU (work-stealing)
void sched_load_balance(void);

// Boost an interactive task's priority after a sleep period (Windows MLFQ)
void sched_interactive_boost(uint32_t pid, uint64_t sleep_ns);

// Set real-time deadline for a task (EDF/RTLinux style)
void sched_set_deadline(uint32_t pid, uint64_t deadline_ns);




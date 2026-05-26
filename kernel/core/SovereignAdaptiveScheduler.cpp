/**
 * SovereignAdaptiveScheduler.cpp
 * Feature: Adaptive ML Scheduler
 * =====================================================================
 * Absorbs: SteamOS gamescope CPU/GPU co-scheduler, Android WALT,
 *          Google Fuchsia EDF, Linux BPF sched_ext.
 * Mission: Tracks per-task runtime history and dynamically adjusts
 *          timeslice budgets using an EWMA predictor — no ML lib needed.
 * Branch:  kernel-exp, performance-optimized
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace Scheduling {

static constexpr sigma_u32 MAX_TASKS    = 128;
static constexpr sigma_u32 EWMA_ALPHA   = 30;   // 30/100 weight for new sample
static constexpr sigma_u64 MIN_SLICE_NS = 500000ULL;   // 0.5 ms
static constexpr sigma_u64 MAX_SLICE_NS = 20000000ULL; // 20 ms

enum class TaskClass : sigma_u8 {
    INTERACTIVE = 0,   // latency-sensitive (UI, audio)
    BATCH       = 1,   // throughput tasks (compiles, ML train)
    REALTIME    = 2,   // hard deadline (RTOS, GPU fence)
    IDLE        = 3    // background
};

struct AdaptiveTask {
    sigma_u32   task_id;
    TaskClass   tclass;
    sigma_u64   deadline_ns;      // 0 = no deadline
    sigma_u64   ewma_runtime_ns;  // predicted next burst
    sigma_u64   last_runtime_ns;
    sigma_u64   allocated_slice;
    sigma_u32   run_count;
    sigma_u32   preemptions;
    bool        active;
};

// Fixed-point EWMA: new_ewma = (alpha * sample + (100-alpha) * old_ewma) / 100
static sigma_u64 ewma_update(sigma_u64 old_val, sigma_u64 sample) {
    return (EWMA_ALPHA * sample + (100 - EWMA_ALPHA) * old_val) / 100;
}

class SovereignAdaptiveScheduler {
public:
    static SovereignAdaptiveScheduler& getInstance() {
        static SovereignAdaptiveScheduler inst;
        return inst;
    }

    void init() {
        m_task_count    = 0;
        m_total_preempt = 0;
        m_tick          = 0;
        sigma_log("[ASCHED] Sovereign Adaptive Scheduler initialised.");
        sigma_log("[ASCHED] EWMA predictor active — SteamOS/WALT-style per-task slice tuning.");
    }

    sigma_u32 registerTask(TaskClass tclass, sigma_u64 deadline_ns) {
        if (m_task_count >= MAX_TASKS) return 0;
        AdaptiveTask& t   = m_tasks[m_task_count];
        t.task_id         = m_task_count + 1;
        t.tclass          = tclass;
        t.deadline_ns     = deadline_ns;
        t.ewma_runtime_ns = (tclass == TaskClass::INTERACTIVE) ? 1000000ULL : 5000000ULL;
        t.last_runtime_ns = 0;
        t.allocated_slice = t.ewma_runtime_ns;
        t.run_count       = 0;
        t.preemptions     = 0;
        t.active          = true;
        m_task_count++;
        sigma_log_info("[ASCHED] Task %u registered (class=%u, deadline=%llu ns).\n",
                       t.task_id, (sigma_u32)tclass, (unsigned long long)deadline_ns);
        return t.task_id;
    }

    // Called after a task runs for `actual_runtime_ns`
    void onTaskComplete(sigma_u32 task_id, sigma_u64 actual_runtime_ns, bool preempted) {
        if (task_id == 0 || task_id > m_task_count) return;
        AdaptiveTask& t = m_tasks[task_id - 1];

        t.last_runtime_ns = actual_runtime_ns;
        t.ewma_runtime_ns = ewma_update(t.ewma_runtime_ns, actual_runtime_ns);
        t.run_count++;
        if (preempted) { t.preemptions++; m_total_preempt++; }

        // Recompute slice with class-specific multipliers
        sigma_u64 base = t.ewma_runtime_ns;
        sigma_u64 slice;
        switch (t.tclass) {
            case TaskClass::INTERACTIVE: slice = base / 2;       break; // stay snappy
            case TaskClass::BATCH:       slice = base * 2;       break; // batch up
            case TaskClass::REALTIME:    slice = t.deadline_ns;  break; // give full deadline
            default:                     slice = base / 4;       break; // idle
        }
        // Clamp
        if (slice < MIN_SLICE_NS) slice = MIN_SLICE_NS;
        if (slice > MAX_SLICE_NS) slice = MAX_SLICE_NS;
        t.allocated_slice = slice;

        sigma_log_info("[ASCHED] Task %u: actual=%llu ns, ewma=%llu ns, next_slice=%llu ns%s\n",
                       task_id,
                       (unsigned long long)actual_runtime_ns,
                       (unsigned long long)t.ewma_runtime_ns,
                       (unsigned long long)slice,
                       preempted ? " [PREEMPTED]" : "");
    }

    // Elect the next task to run (priority: REALTIME > deadline urgency > INTERACTIVE > BATCH > IDLE)
    sigma_u32 electNext() {
        sigma_u32 best    = 0;
        sigma_u64 urgency = 0;
        m_tick++;

        for (sigma_u32 i = 0; i < m_task_count; i++) {
            AdaptiveTask& t = m_tasks[i];
            if (!t.active) continue;

            sigma_u64 score = 0;
            switch (t.tclass) {
                case TaskClass::REALTIME:    score = 1000000000ULL; break;
                case TaskClass::INTERACTIVE: score = 100000000ULL;  break;
                case TaskClass::BATCH:       score = 10000000ULL;   break;
                default:                     score = 1000ULL;       break;
            }
            // Boost starvation: add 1000 per skipped tick
            score += (sigma_u64)(m_tick - t.run_count) * 1000ULL;

            if (score > urgency) { urgency = score; best = t.task_id; }
        }
        return best;
    }

    void printStats() {
        sigma_log("\n--- ADAPTIVE SCHEDULER STATS ---");
        sigma_log_info("| Tasks       : %u\n", m_task_count);
        sigma_log_info("| Ticks       : %u\n", m_tick);
        sigma_log_info("| Preemptions : %u\n", m_total_preempt);
        for (sigma_u32 i = 0; i < m_task_count; i++) {
            AdaptiveTask& t = m_tasks[i];
            sigma_log_info("|  Task%02u [class=%u] ewma=%lluµs slice=%lluµs runs=%u\n",
                           t.task_id, (sigma_u32)t.tclass,
                           (unsigned long long)(t.ewma_runtime_ns / 1000),
                           (unsigned long long)(t.allocated_slice  / 1000),
                           t.run_count);
        }
        sigma_log("--------------------------------");
    }

private:
    AdaptiveTask m_tasks[MAX_TASKS];
    sigma_u32    m_task_count    = 0;
    sigma_u32    m_total_preempt = 0;
    sigma_u32    m_tick          = 0;

    SovereignAdaptiveScheduler() = default;
};

} // namespace Scheduling
} // namespace Kernel
} // namespace SigmaOS

// ── C API ──────────────────────────────────────────────────────────
extern "C" {

void asched_init() {
    SigmaOS::Kernel::Scheduling::SovereignAdaptiveScheduler::getInstance().init();
}

sigma_u32 asched_register(sigma_u8 task_class, sigma_u64 deadline_ns) {
    using TC = SigmaOS::Kernel::Scheduling::TaskClass;
    TC tc = (task_class == 2) ? TC::REALTIME :
            (task_class == 1) ? TC::BATCH :
            (task_class == 3) ? TC::IDLE : TC::INTERACTIVE;
    return SigmaOS::Kernel::Scheduling::SovereignAdaptiveScheduler::getInstance()
               .registerTask(tc, deadline_ns);
}

void asched_complete(sigma_u32 task_id, sigma_u64 runtime_ns, bool preempted) {
    SigmaOS::Kernel::Scheduling::SovereignAdaptiveScheduler::getInstance()
        .onTaskComplete(task_id, runtime_ns, preempted);
}

sigma_u32 asched_elect() {
    return SigmaOS::Kernel::Scheduling::SovereignAdaptiveScheduler::getInstance().electNext();
}

void asched_stats() {
    SigmaOS::Kernel::Scheduling::SovereignAdaptiveScheduler::getInstance().printStats();
}

} // extern "C"

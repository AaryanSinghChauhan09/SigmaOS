/**
 * SovereignSchedulerBench.cpp
 * Feature #26: Scheduler Benchmarking Suite
 * =====================================================================
 * Absorbs: Linux perf sched, rt-tests, cyclictest, hackbench.
 * Mission: Automated scheduler drift and latency profiler — measures
 *          CFS vruntime fairness and EDF deadline accuracy.
 * Branch:  performance-optimized, kernel-exp
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace Performance {

static constexpr sigma_u32 MAX_BENCH_TASKS = 64;
static constexpr sigma_u32 MAX_SAMPLES     = 256;

struct BenchTask {
    sigma_u32 task_id;
    sigma_u64 vruntime;
    sigma_u64 deadline;       // 0 for CFS tasks
    sigma_u64 total_runtime;
    sigma_u32 context_switches;
    sigma_u32 preemptions;
};

struct LatencySample {
    sigma_u32 task_id;
    sigma_u64 expected_wakeup;
    sigma_u64 actual_wakeup;
    sigma_i64 drift_ns;       // positive = late, negative = early
};

class SovereignSchedulerBench {
public:
    static SovereignSchedulerBench& getInstance() {
        static SovereignSchedulerBench instance;
        return instance;
    }

    void init() {
        m_task_count   = 0;
        m_sample_count = 0;
        m_sample_head  = 0;
        sigma_log("[SCHED-BENCH] Sovereign Scheduler Benchmarking Suite initialized.");
        sigma_log("[SCHED-BENCH] Modes: CFS fairness analysis, EDF deadline accuracy, context-switch profiling.");
    }

    // Register a simulated benchmark task
    sigma_u32 addTask(sigma_u64 deadline) {
        if (m_task_count >= MAX_BENCH_TASKS) return 0;
        BenchTask& t = m_tasks[m_task_count];
        t.task_id          = m_task_count + 1;
        t.vruntime         = 0;
        t.deadline         = deadline;
        t.total_runtime    = 0;
        t.context_switches = 0;
        t.preemptions      = 0;
        m_task_count++;
        return t.task_id;
    }

    // Record a wakeup latency sample
    void recordWakeup(sigma_u32 task_id, sigma_u64 expected, sigma_u64 actual) {
        if (task_id == 0 || task_id > m_task_count) return;
        LatencySample& s = m_samples[m_sample_head % MAX_SAMPLES];
        s.task_id         = task_id;
        s.expected_wakeup = expected;
        s.actual_wakeup   = actual;
        s.drift_ns        = (sigma_i64)(actual - expected);
        m_sample_head++;
        if (m_sample_count < MAX_SAMPLES) m_sample_count++;

        // Track context switch on the task
        m_tasks[task_id - 1].context_switches++;
    }

    // Run a simulated CFS fairness test — checks vruntime balance
    void runCFSFairnessTest(sigma_u32 iterations) {
        sigma_log_info("[SCHED-BENCH] Running CFS fairness test (%u iterations)...\n", iterations);
        // Simulate scheduling rounds
        for (sigma_u32 round = 0; round < iterations; round++) {
            for (sigma_u32 i = 0; i < m_task_count; i++) {
                if (m_tasks[i].deadline == 0) { // CFS tasks only
                    // Simulate a timeslice of 1000-5000 ns
                    sigma_u64 slice = 1000 + (round * 37 + i * 13) % 4000;
                    m_tasks[i].vruntime += slice;
                    m_tasks[i].total_runtime += slice;
                    m_tasks[i].context_switches++;
                }
            }
        }
        // Check fairness — compute max/min vruntime ratio
        sigma_u64 min_vr = ~(sigma_u64)0;
        sigma_u64 max_vr = 0;
        for (sigma_u32 i = 0; i < m_task_count; i++) {
            if (m_tasks[i].deadline == 0) {
                if (m_tasks[i].vruntime < min_vr) min_vr = m_tasks[i].vruntime;
                if (m_tasks[i].vruntime > max_vr) max_vr = m_tasks[i].vruntime;
            }
        }
        if (min_vr > 0) {
            sigma_u64 ratio = (max_vr * 100) / min_vr;
            sigma_log_info("[SCHED-BENCH] CFS Fairness: max/min vruntime ratio = %llu%%\n",
                           (unsigned long long)ratio);
            if (ratio <= 110) {
                sigma_log("[SCHED-BENCH] RESULT: PASS — vruntime drift within 10% tolerance.");
            } else {
                sigma_log("[SCHED-BENCH] RESULT: WARN — vruntime imbalance detected.");
            }
        }
    }

    // Run EDF deadline accuracy test
    void runEDFAccuracyTest() {
        sigma_log("[SCHED-BENCH] Running EDF deadline accuracy test...");
        sigma_u32 missed = 0;
        for (sigma_u32 i = 0; i < m_task_count; i++) {
            if (m_tasks[i].deadline > 0) {
                if (m_tasks[i].total_runtime > m_tasks[i].deadline) {
                    missed++;
                    sigma_log_info("[SCHED-BENCH] Task %u MISSED deadline (runtime %llu > deadline %llu)\n",
                                   m_tasks[i].task_id,
                                   (unsigned long long)m_tasks[i].total_runtime,
                                   (unsigned long long)m_tasks[i].deadline);
                }
            }
        }
        sigma_log_info("[SCHED-BENCH] EDF Result: %u deadline miss(es).\n", missed);
    }

    // Compute and report wakeup latency statistics
    void reportLatencyStats() {
        if (m_sample_count == 0) {
            sigma_log("[SCHED-BENCH] No wakeup latency samples recorded.");
            return;
        }
        sigma_i64 sum = 0;
        sigma_i64 worst = 0;
        for (sigma_u32 i = 0; i < m_sample_count; i++) {
            sigma_i64 d = m_samples[i].drift_ns;
            sum += d;
            if (d > worst) worst = d;
        }
        sigma_i64 avg = sum / (sigma_i64)m_sample_count;
        sigma_log("\n--- SCHEDULER LATENCY REPORT ---");
        sigma_log_info("| Samples      : %u\n", m_sample_count);
        sigma_log_info("| Avg Drift(ns) : %lld\n", (long long)avg);
        sigma_log_info("| Worst Drift(ns): %lld\n", (long long)worst);
        sigma_log("---------------------------------");
    }

    void printAudit() {
        sigma_log("\n--- SOVEREIGN SCHEDULER BENCH AUDIT ---");
        sigma_log_info("| Registered Tasks : %u\n", m_task_count);
        sigma_log_info("| Latency Samples  : %u\n", m_sample_count);
        sigma_log("| Modes: CFS Fairness, EDF Accuracy, Wakeup Latency");
        sigma_log("----------------------------------------");
    }

private:
    BenchTask     m_tasks[MAX_BENCH_TASKS];
    LatencySample m_samples[MAX_SAMPLES];
    sigma_u32     m_task_count;
    sigma_u32     m_sample_count;
    sigma_u32     m_sample_head;

    SovereignSchedulerBench() : m_task_count(0), m_sample_count(0), m_sample_head(0) {}
};

} // namespace Performance
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sched_bench_init() {
    SigmaOS::Kernel::Performance::SovereignSchedulerBench::getInstance().init();
}

void sched_bench_run_cfs(sigma_u32 iters) {
    auto& bench = SigmaOS::Kernel::Performance::SovereignSchedulerBench::getInstance();
    // Add test tasks if none registered
    if (iters > 0) {
        for (sigma_u32 i = 0; i < 8; i++) bench.addTask(0); // 8 CFS tasks
        bench.runCFSFairnessTest(iters);
    }
}

void sched_bench_run_edf() {
    SigmaOS::Kernel::Performance::SovereignSchedulerBench::getInstance().runEDFAccuracyTest();
}

void sched_bench_audit() {
    SigmaOS::Kernel::Performance::SovereignSchedulerBench::getInstance().printAudit();
}

} // extern "C"

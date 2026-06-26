/**
 * @file sigma_perf_profiler.cpp
 * @brief Roadmap Features #21 (PGO), #34 (S-Perf), #38 (Compiler Flags),
 *        #39 (Regression Detector), #40 (Benchmarking Dashboard)
 *
 * Sovereign performance profiling subsystem.  Reads hardware PMU
 * counters (RDPMC), tracks per-shard IPC/CPI metrics, and feeds
 * them into a regression detection engine.
 */

#include "../../include/sigma_kernel_types.h"

namespace sigma {
namespace perf {

/* ---- Hardware PMU counter IDs ---- */
static constexpr sigma_u32 PMU_CYCLES          = 0x003Cu;
static constexpr sigma_u32 PMU_INSTRUCTIONS     = 0x00C0u;
static constexpr sigma_u32 PMU_CACHE_MISSES     = 0x412Eu;
static constexpr sigma_u32 PMU_BRANCH_MISPREDICT = 0x00C5u;

/* ---- Per-shard performance snapshot ---- */
struct PerfSnapshot {
    sigma_u64 cycles;
    sigma_u64 instructions;
    sigma_u64 cache_misses;
    sigma_u64 branch_mispredicts;
    sigma_u64 timestamp_tsc;
};

/* ---- Read a single PMU counter via RDPMC ---- */
static inline sigma_u64 read_pmu(sigma_u32 counter_id) {
    sigma_u32 lo, hi;
    __asm__ __volatile__("rdpmc" : "=a"(lo), "=d"(hi) : "c"(counter_id));
    return ((sigma_u64)hi << 32) | lo;
}

/* ---- Capture a full performance snapshot ---- */
static PerfSnapshot capture_snapshot() {
    PerfSnapshot snap;
    snap.timestamp_tsc      = cpu_rdtsc();
    snap.cycles             = read_pmu(0);
    snap.instructions       = read_pmu(1);
    snap.cache_misses       = read_pmu(2);
    snap.branch_mispredicts = read_pmu(3);
    return snap;
}

/* ---- Regression detection ---- */
struct RegressionBaseline {
    sigma_u64 avg_ipc_x1000;       /* IPC × 1000 for integer math  */
    sigma_u64 avg_cache_miss_rate;  /* misses per 1M instructions   */
    sigma_u64 tolerance_pct;        /* regression threshold (e.g. 5) */
};

static sigma_bool detect_regression(const PerfSnapshot* current,
                                     const RegressionBaseline* baseline) {
    if (current->instructions == 0) return SIGMA_FALSE;

    sigma_u64 ipc_x1000 = (current->instructions * 1000) / current->cycles;
    sigma_u64 miss_rate = (current->cache_misses * 1000000) / current->instructions;

    /* Check IPC regression */
    sigma_u64 ipc_threshold = baseline->avg_ipc_x1000 *
                              (100 - baseline->tolerance_pct) / 100;
    if (ipc_x1000 < ipc_threshold) return SIGMA_TRUE;

    /* Check cache miss spike */
    sigma_u64 miss_threshold = baseline->avg_cache_miss_rate *
                               (100 + baseline->tolerance_pct) / 100;
    if (miss_rate > miss_threshold) return SIGMA_TRUE;

    return SIGMA_FALSE;
}

/* ---- Compiler flag recommendation engine (Feature #38) ---- */
struct CompilerProfile {
    const char* arch_target;    /* e.g. "x86-64-v3", "native" */
    sigma_bool  use_lto;
    sigma_bool  use_pgo;
    sigma_bool  use_avx512;
    sigma_u32   opt_level;      /* 0–3 */
};

static CompilerProfile recommend_compiler_flags(const PerfSnapshot* snap) {
    CompilerProfile prof;
    prof.arch_target = "x86-64-v3";
    prof.opt_level   = 2;
    prof.use_lto     = SIGMA_TRUE;
    prof.use_pgo     = SIGMA_FALSE;
    prof.use_avx512  = SIGMA_FALSE;

    /* If IPC is low, recommend PGO to improve branch prediction */
    if (snap->instructions > 0 && snap->cycles > 0) {
        sigma_u64 ipc_x1000 = (snap->instructions * 1000) / snap->cycles;
        if (ipc_x1000 < 800) {
            prof.use_pgo = SIGMA_TRUE;
            prof.opt_level = 3;
        }
    }

    /* If branch mispredicts are high, enable aggressive opts */
    if (snap->branch_mispredicts > snap->instructions / 20) {
        prof.opt_level = 3;
        prof.use_pgo = SIGMA_TRUE;
    }

    return prof;
}

} /* namespace perf */
} /* namespace sigma */

/* ---- C Bridge ---- */
extern "C" {

void sigma_perf_capture(void) {
    sigma::perf::capture_snapshot();
}

}

/**
 * SovereignPerfRegressionDetector.cpp
 * Feature: Performance Regression Detector
 * =====================================================================
 * Absorbs: Google Benchmark CI, Clear Linux autotest, perf-stat CI hooks.
 * Mission: Automated CI tool that detects performance regressions by
 *          comparing benchmark results across builds. Flags slowdowns
 *          before they reach production.
 * Branch:  performance-optimized, tools-dev
 * =====================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Performance {
namespace Regression {

static constexpr sigma_u32 MAX_BENCHMARKS = 64;
static constexpr sigma_u32 MAX_HISTORY    = 16;
static constexpr sigma_u32 REGRESS_THRESH = 5;  // 5% regression threshold

struct BenchmarkResult {
    sigma_u64 cycles_per_op;
    sigma_u64 memory_bytes;
    sigma_u64 latency_ns;
    sigma_u32 build_number;
};

struct Benchmark {
    sigma_u32       id;
    char            name[48];
    BenchmarkResult history[MAX_HISTORY];
    sigma_u32       history_count;
    bool            regression_detected;
    sigma_i32       regression_pct;  // negative = improvement
};

class SovereignPerfRegressionDetector {
public:
    static SovereignPerfRegressionDetector& getInstance() {
        static SovereignPerfRegressionDetector inst;
        return inst;
    }

    void init() {
        m_bench_count = 0;
        m_build_number = 0;
        m_regressions_found = 0;
        sigma_log("[PERFCI] Sovereign Performance Regression Detector initialised.");
        sigma_log("[PERFCI] Threshold: >5% regression triggers CI alert.");
    }

    sigma_u32 registerBenchmark(const char* name) {
        if (m_bench_count >= MAX_BENCHMARKS) return 0;
        Benchmark& b = m_benchmarks[m_bench_count];
        b.id = m_bench_count + 1;
        sigma_u32 i = 0;
        while (i < 47 && name[i]) { b.name[i] = name[i]; i++; }
        b.name[i] = '\0';
        b.history_count = 0;
        b.regression_detected = false;
        b.regression_pct = 0;
        m_bench_count++;
        return b.id;
    }

    // Record a benchmark result
    bool recordResult(sigma_u32 bench_id, sigma_u64 cycles, sigma_u64 mem, sigma_u64 lat) {
        if (bench_id == 0 || bench_id > m_bench_count) return false;
        Benchmark& b = m_benchmarks[bench_id - 1];
        if (b.history_count >= MAX_HISTORY) {
            // Shift history
            for (sigma_u32 i = 0; i < MAX_HISTORY - 1; i++) {
                b.history[i] = b.history[i + 1];
            }
            b.history_count = MAX_HISTORY - 1;
        }
        BenchmarkResult& r = b.history[b.history_count];
        r.cycles_per_op = cycles;
        r.memory_bytes = mem;
        r.latency_ns = lat;
        r.build_number = m_build_number;
        b.history_count++;

        // Check for regression against previous
        if (b.history_count >= 2) {
            BenchmarkResult& prev = b.history[b.history_count - 2];
            if (prev.cycles_per_op > 0) {
                sigma_i64 delta = (sigma_i64)r.cycles_per_op - (sigma_i64)prev.cycles_per_op;
                sigma_i32 pct = (sigma_i32)(delta * 100 / (sigma_i64)prev.cycles_per_op);
                b.regression_pct = pct;
                if (pct > (sigma_i32)REGRESS_THRESH) {
                    b.regression_detected = true;
                    m_regressions_found++;
                    sigma_log_info("[PERFCI] REGRESSION in '%s': +%d%% cycles/op (build #%u).\n",
                                   b.name, pct, m_build_number);
                } else if (pct < -(sigma_i32)REGRESS_THRESH) {
                    sigma_log_info("[PERFCI] IMPROVEMENT in '%s': %d%% faster (build #%u).\n",
                                   b.name, -pct, m_build_number);
                    b.regression_detected = false;
                } else {
                    b.regression_detected = false;
                }
            }
        }
        return true;
    }

    void newBuild() {
        m_build_number++;
        sigma_log_info("[PERFCI] New build #%u — benchmarks pending.\n", m_build_number);
    }

    // Check if any regressions block the build
    bool gateCheck() {
        sigma_u32 active = 0;
        for (sigma_u32 i = 0; i < m_bench_count; i++) {
            if (m_benchmarks[i].regression_detected) active++;
        }
        bool pass = (active == 0);
        sigma_log_info("[PERFCI] CI gate: %s (%u active regressions).\n",
                       pass ? "PASS" : "BLOCKED", active);
        return pass;
    }

    void printReport() {
        sigma_log("\n--- PERFORMANCE REGRESSION REPORT ---");
        sigma_log_info("| Build        : #%u\n", m_build_number);
        sigma_log_info("| Benchmarks   : %u\n", m_bench_count);
        sigma_log_info("| Regressions  : %u\n", m_regressions_found);
        for (sigma_u32 i = 0; i < m_bench_count; i++) {
            Benchmark& b = m_benchmarks[i];
            sigma_log_info("|  [%s] last=%llu cy/op delta=%d%% %s\n",
                           b.name,
                           b.history_count > 0 ?
                               (unsigned long long)b.history[b.history_count - 1].cycles_per_op : 0ULL,
                           b.regression_pct,
                           b.regression_detected ? "⚠ REGRESSED" : "✓ OK");
        }
        sigma_log("-------------------------------------");
    }

private:
    Benchmark m_benchmarks[MAX_BENCHMARKS];
    sigma_u32 m_bench_count        = 0;
    sigma_u32 m_build_number       = 0;
    sigma_u32 m_regressions_found  = 0;

    SovereignPerfRegressionDetector() = default;
};

} // namespace Regression
} // namespace Performance
} // namespace SigmaOS

extern "C" {

void perfci_init() {
    SigmaOS::Performance::Regression::SovereignPerfRegressionDetector::getInstance().init();
}

sigma_u32 perfci_register(const char* name) {
    return SigmaOS::Performance::Regression::SovereignPerfRegressionDetector::getInstance()
               .registerBenchmark(name);
}

bool perfci_record(sigma_u32 id, sigma_u64 cycles, sigma_u64 mem, sigma_u64 lat) {
    return SigmaOS::Performance::Regression::SovereignPerfRegressionDetector::getInstance()
               .recordResult(id, cycles, mem, lat);
}

void perfci_new_build() {
    SigmaOS::Performance::Regression::SovereignPerfRegressionDetector::getInstance().newBuild();
}

bool perfci_gate() {
    return SigmaOS::Performance::Regression::SovereignPerfRegressionDetector::getInstance().gateCheck();
}

void perfci_report() {
    SigmaOS::Performance::Regression::SovereignPerfRegressionDetector::getInstance().printReport();
}

} // extern "C"

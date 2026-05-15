// SigmaOS — sigma-perf-isolator: Resource Isolation
// Module: sigma-perf-isolator
// USP: Dynamically restricts CPU frequency and memory access per-process based on capability level.

#ifndef SIGMA_PERF_ISOLATOR_H
#define SIGMA_PERF_ISOLATOR_H

#include "../../include/sigmaos/core/src/atomic_sigma_process.hpp"
#include "../../include/sigma_perf_profiler.h"

namespace sigma {
namespace perf {

class ResourceIsolator {
private:
    unsigned int active_pid;
    unsigned int max_cpu_cycles;
    unsigned int max_memory_kb;

public:
    ResourceIsolator() : active_pid(0), max_cpu_cycles(0), max_memory_kb(0) {}

    void enforce_limits(unsigned int pid, unsigned int cpu_limit, unsigned int mem_limit) {
        active_pid = pid;
        max_cpu_cycles = cpu_limit;
        max_memory_kb = mem_limit;
    }

    bool check_cpu_violation(unsigned long current_cycles) const {
        return (max_cpu_cycles > 0) && (current_cycles > max_cpu_cycles);
    }

    bool check_memory_violation(unsigned long current_mem_kb) const {
        return (max_memory_kb > 0) && (current_mem_kb > max_memory_kb);
    }

    void throttle_process() {
        // Inline ASM for x86 pause instruction to yield CPU pipeline and reduce power/freq
#if defined(__x86_64__) || defined(__i386__)
        __asm__ __volatile__("pause\n\t" ::: "memory");
#endif
    }
};

} // namespace perf
} // namespace sigma

#endif /* SIGMA_PERF_ISOLATOR_H */

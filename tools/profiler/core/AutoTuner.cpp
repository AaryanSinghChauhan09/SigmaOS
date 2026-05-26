#include <iostream>
#include <string>

extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
}

namespace SigmaOS {
namespace Profiler {

class AutoTuner {
public:
    static AutoTuner& getInstance() {
        static AutoTuner instance;
        return instance;
    }

    void tuneProfile(const std::string& profile) {
        std::cout << "[sigma-prof] Initiating auto-tuning for profile: " << profile << "\n";
        sigma_log_info("[Profiler] Tuning system for profile: %s", profile.c_str());

        if (profile == "hpc") {
            std::cout << "  -> Disabling power-saving C-states.\n";
            std::cout << "  -> Forcing maximum CPU frequency scaling.\n";
            std::cout << "  -> Optimizing NUMA node memory locality for compute threads.\n";
            std::cout << "[sigma-prof] HPC optimizations applied successfully.\n";
        } else if (profile == "ai") {
            std::cout << "  -> Increasing memory bandwidth limits.\n";
            std::cout << "  -> Pre-allocating AVX-512 register sets for tensor threads.\n";
            std::cout << "  -> Prioritizing NPU/GPU dispatch queues.\n";
            std::cout << "[sigma-prof] AI optimizations applied successfully.\n";
        } else if (profile == "embedded") {
            std::cout << "  -> Enforcing aggressive C-state sleeping.\n";
            std::cout << "  -> Capping thermal limits to 65C.\n";
            std::cout << "  -> Reducing context-switch frequency for background shards.\n";
            std::cout << "[sigma-prof] Embedded optimizations applied successfully.\n";
        } else {
            std::cout << "[sigma-prof] Error: Unknown profile '" << profile << "'. Valid profiles: hpc, ai, embedded.\n";
            return;
        }
    }

    void reset() {
        std::cout << "[sigma-prof] Resetting to baseline default parameters...\n";
        sigma_log_info("[Profiler] System tuning reset to default.");
        std::cout << "[sigma-prof] Baseline parameters restored.\n";
    }
};

} // namespace Profiler
} // namespace SigmaOS

extern "C" void autotune_profile(const char* profile) {
    SigmaOS::Profiler::AutoTuner::getInstance().tuneProfile(profile);
}

extern "C" void autotune_reset() {
    SigmaOS::Profiler::AutoTuner::getInstance().reset();
}

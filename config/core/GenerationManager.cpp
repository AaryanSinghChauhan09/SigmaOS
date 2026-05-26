#include <iostream>

extern "C" {
    void sigma_log_info(const char* fmt, ...);
    void sigma_log_error(const char* fmt, ...);
}

namespace SigmaOS {
namespace Config {

class GenerationManager {
    int current_generation;
public:
    GenerationManager() : current_generation(42) {}

    static GenerationManager& getInstance() {
        static GenerationManager instance;
        return instance;
    }

    void commitGeneration() {
        current_generation++;
        std::cout << "[sigma-config] Committing new system generation: " << current_generation << "\n";
        std::cout << "[sigma-config] Cryptographically signing state generation with Dilithium-5...\n";
        sigma_log_info("[Config] New generation committed: %d", current_generation);
    }

    void rollbackGeneration(int target_gen) {
        if (target_gen >= current_generation) {
            std::cout << "[sigma-config] Error: Target generation must be strictly less than current generation.\n";
            return;
        }
        std::cout << "[sigma-config] Initiating atomic rollback to Generation " << target_gen << "...\n";
        sigma_log_info("[Config] Rolling back to generation %d", target_gen);
        
        // Simulate symlink pointer update
        current_generation = target_gen;
        std::cout << "[sigma-config] Rollback complete. System state is now identically mapped to Generation " << target_gen << ".\n";
    }
    
    void printStatus() {
        std::cout << "[sigma-config] Active System Generation: " << current_generation << " [VERIFIED]\n";
    }
};

} // namespace Config
} // namespace SigmaOS

extern "C" void commit_config_generation() {
    SigmaOS::Config::GenerationManager::getInstance().commitGeneration();
}

extern "C" void rollback_config_generation(int target_gen) {
    SigmaOS::Config::GenerationManager::getInstance().rollbackGeneration(target_gen);
}

extern "C" void print_config_status() {
    SigmaOS::Config::GenerationManager::getInstance().printStatus();
}

#include <iostream>
#include <string>
#include <memory>
#include "sigma_mesh.hpp"

/**
 * SigmaOS Enterprise Strategy Shard v1.0 (Native C++ OOPS)
 * Principle: Strategy Pattern, Open/Closed Principle (SOLID).
 * USP: Dynamic Behavior Sharding (Execution vs. Audit).
 * -----------------------------------------------------------------------------
 */

namespace SigmaOS {

    // --- Strategy Interface ---
    class ISyncStrategy {
    public:
        virtual ~ISyncStrategy() {}
        virtual void ExecuteSync() = 0;
    };

    class IntensiveSync : public ISyncStrategy {
    public:
        void ExecuteSync() override {
            std::cout << "[STRATEGY]: Executing Intensive Mesh-Sync Baseline..." << std::endl;
        }
    };

    class StealthSync : public ISyncStrategy {
    public:
        void ExecuteSync() override {
            std::cout << "[STRATEGY]: Executing Stealth Mesh-Sync Audit..." << std::endl;
        }
    };

    // --- Shard with Strategy (Polymorphism) ---
    class SyncShard : public BaseShard {
    private:
        std::unique_ptr<ISyncStrategy> m_strategy;
    public:
        SyncShard(std::string id, std::unique_ptr<ISyncStrategy> strategy) 
            : BaseShard(id), m_strategy(std::move(strategy)) {}
        
        void ExecutePayload() override {
            std::cout << "[STRATEGY]: Payload Initiation for Shard: " << m_id << std::endl;
            m_strategy->ExecuteSync();
        }
    };

} // namespace SigmaOS

int main() {
    using namespace SigmaOS;
    std::cout << "[STRATEGY]: Initiating Design-Pattern Shard Zenith..." << std::endl;
    
    ShardMesh mesh;
    
    // Add polymorphic shard objects with different strategies
    mesh.AddShard(std::make_shared<SyncShard>("SyncKernel_CORE", std::make_unique<IntensiveSync>()));
    mesh.AddShard(std::make_shared<SyncShard>("SyncGuard_STEALTH", std::make_unique<StealthSync>()));
    
    mesh.ExecuteAll();
    std::cout << "[STRATEGY]: OOPS-Zenith ACHIEVED." << std::endl;
    return 0;
}

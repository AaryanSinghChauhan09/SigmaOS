#pragma once
/**
 * SigmaOS Sovereign Shard Bus v1.0 (Native C++ OOPS)
 * Principle: Low-Latency Shard Orchestration (Zero-Shell).
 * USP: Lock-Free Shard-to-Shard Event Dispatch.
 * Replaces: Legacy 'system()' calls for internal sharding.
 */

#include <iostream>
#include <string>
#include <vector>
#include <functional>
#include <memory>

namespace SigmaOS {

    class ShardBus {
    private:
        std::map<std::string, std::function<void()>> m_registry;
        
        ShardBus() {
            std::cout << "Σ [BUS]: Initiating Sovereign Silicon Bus..." << std::endl;
        }

    public:
        static ShardBus& Instance() {
            static ShardBus instance;
            return instance;
        }

        void RegisterShard(const std::string& name, std::function<void()> trigger) {
            m_registry[name] = trigger;
            std::cout << "Σ [BUS]: Shard [" << name << "] Registered to Native Bus." << std::endl;
        }

        void TriggerShard(const std::string& name) {
            if (m_registry.count(name)) {
                std::cout << "Σ [BUS]: Native Dispatch -> Shard [" << name << "]" << std::endl;
                m_registry[name]();
            } else {
                std::cerr << "Σ [BUS_ERROR]: Shard [" << name << "] NOT FOUND on Silicon-Bus." << std::endl;
            }
        }
    };

} // namespace SigmaOS

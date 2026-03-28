/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

#pragma once
/**
 * Σ SIGMA OS: SOVEREIGN SHARD BUS (v1.0 - ZERO-STD NATIVE)
 * ========================================================
 * Principle: Low-Latency Shard Orchestration (Zero-Shell).
 * USP: Lock-Free Shard-to-Shard Event Dispatch.
 * Replaces: Legacy 'system()' calls for internal sharding / Zero-STL.
 * ========================================================
 */

#include "SigmaOOP.hpp"

namespace SigmaOS {

    typedef void (*ShardTriggerFunc)();

    class ShardBus {
    private:
        SigmaMap<SigmaString, ShardTriggerFunc> m_registry;
        
        ShardBus() {
            sigma_printf("Σ [BUS]: Initiating Sovereign Silicon Bus...\n");
        }

    public:
        static ShardBus& Instance() {
            static ShardBus instance;
            return instance;
        }

        void RegisterShard(const SigmaString& name, ShardTriggerFunc trigger) {
            m_registry.insert(name, trigger);
            sigma_printf("Σ [BUS]: Shard [%s] Registered to Native Bus.\n", name.c_str());
        }

        void TriggerShard(const SigmaString& name) {
            if (m_registry.contains(name)) {
                sigma_printf("Σ [BUS]: Native Dispatch -> Shard [%s]\n", name.c_str());
                ShardTriggerFunc f = *m_registry.at(name);
                if (f) f();
            } else {
                sigma_printf("Σ [BUS_ERROR]: Shard [%s] NOT FOUND on Silicon-Bus.\n", name.c_str());
            }
        }
    };

} // namespace SigmaOS


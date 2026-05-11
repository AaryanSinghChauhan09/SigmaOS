/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SHARD SDK (v1.0)
 * =========================================================================
 * Mission: Standardized toolkit for building high-assurance OS shards.
 * Principle: Zero-dependency, type-safe, and PQC-attested.
 * =========================================================================
 */

#ifndef SIGMA_SDK_H
#define SIGMA_SDK_H

#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

/**
 * @brief Base class for all Sovereign Shards.
 * Ensures compatibility with the Lattice Orchestrator.
 */
namespace SigmaOS {
namespace SDK {

class SovereignShard : public SigmaOS::SigmaObject {
public:
    virtual void on_shard_init() = 0;
    virtual void on_shard_fault() {
        sigma_log_crit("[SDK] Unhandled fault in Shard: %s", type_name());
    }
};

} // namespace SDK
} // namespace SigmaOS

/* --- SDK Utility Macros --- */
#define REGISTER_SHARD(ClassName) \
    extern "C" ClassName* create_shard() { return new ClassName(); }

#endif /* SIGMA_SDK_H */

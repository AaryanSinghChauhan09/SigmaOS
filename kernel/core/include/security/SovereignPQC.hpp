#pragma once
#include "../../../../include/core/sigma_types.h"
#include "../../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignPQCEngine : public SigmaObject {
public:
    static SovereignPQCEngine& getInstance();

    const char* type_name() const noexcept override { return "SovereignPQCEngine"; }

    void init();
    void signShard(sigma_u32 shard_id, sigma_u8* signature);
    bool verifyShard(sigma_u32 shard_id, const sigma_u8* signature);
    void refreshLattice();
    sigma_u64 getSignatureCount() const { return total_signatures; }

private:
    SovereignPQCEngine() = default;
    sigma_u32 initialized{0};
    sigma_u64 total_signatures{0};
    sigma_u64 verified_shards{0};
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
 
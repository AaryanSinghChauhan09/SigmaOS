#ifndef SIGMA_PQC_H
#define SIGMA_PQC_H

#include "sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignPQCEngine {
public:
    static SovereignPQCEngine& getInstance();

    void init();
    void signShard(sigma_u32 shard_id, sigma_u8* signature);
    bool verifyShard(sigma_u32 shard_id, const sigma_u8* signature);
    void refreshLattice();
    
    sigma_u64 getSignatureCount() const { return total_signatures; }
    sigma_u64 getVerifiedCount()  const { return verified_shards; }

private:
    SovereignPQCEngine();
    SovereignPQCEngine(const SovereignPQCEngine&) = delete;
    SovereignPQCEngine& operator=(const SovereignPQCEngine&) = delete;

    sigma_u32 initialized;
    sigma_u64 total_signatures;
    sigma_u64 verified_shards;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* --- PQC Primitives --- */
void      pqc_init(void);
void      pqc_sign_shard(sigma_u32 shard_id, sigma_u8* signature);
bool      pqc_verify_shard(sigma_u32 shard_id, const sigma_u8* signature);
sigma_u64 pqc_get_signature_count(void);
void      pqc_refresh_lattice(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PQC_H */

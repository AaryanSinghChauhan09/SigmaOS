#ifndef SIGMA_PQC_H
#define SIGMA_PQC_H

#include "../core/sigma_types.h"

#ifdef __cplusplus
#include "../core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignPQCEngine : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignPQCEngine> {
    friend class SigmaOS::SigmaSingleton<SovereignPQCEngine>;
public:
    const char* type_name() const noexcept override { return "SovereignPQCEngine"; }

    void init();
    void signShard(sigma_u32 shard_id, sigma_u8* signature);
    bool verifyShard(sigma_u32 shard_id, const sigma_u8* signature);
    void refreshLattice();

    sigma_u64 getSignatureCount() const { return total_signatures; }
    sigma_u64 getVerifiedCount()  const { return verified_shards; }

private:
    SovereignPQCEngine() : initialized(0u), total_signatures(0ULL), verified_shards(0ULL) {}
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

/* --- PQC C Bridge --- */
void      pqc_init(void);
void      pqc_sign_shard(unsigned int shard_id, unsigned char* signature);
int       pqc_verify_shard(unsigned int shard_id, const unsigned char* signature);
unsigned long long pqc_get_signature_count(void);
void      pqc_refresh_lattice(void);

#ifdef __cplusplus
} // extern "C"
#endif

#endif /* SIGMA_PQC_H */

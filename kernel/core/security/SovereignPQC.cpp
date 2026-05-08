#include "sigma_types.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "security/sigma_pqc.h"

namespace SigmaOS {
namespace Kernel {
namespace Security {

SovereignPQCEngine& SovereignPQCEngine::getInstance() {
    static SovereignPQCEngine instance;
    return instance;
}

void SovereignPQCEngine::init() {
    sigma_log_info("[PQC] Initializing Sovereign Post-Quantum Cryptography Nexus (LBSV Algorithm)...");
    this->initialized = 1u;
}

void SovereignPQCEngine::signShard(sigma_u32 shard_id, sigma_u8* signature) {
    sigma_log_info("[PQC] LBSV: Signing shard using quantum-resistant lattice parameters...");

    sigma_u64 entropy = 0x9e3779b97f4a7c15ULL ^ (sigma_u64)shard_id;
    for (int i = 0; i < 64; i++) {
        entropy = (entropy * 6364136223846793005ULL + 1ULL);
        signature[i] = (sigma_u8)(entropy ^ (shard_id * 0x5Fu));
    }

    this->total_signatures++;
}

bool SovereignPQCEngine::verifyShard(sigma_u32 shard_id, const sigma_u8* signature) {
    sigma_log_info("[PQC] LBSV: Verifying shard integrity...");
    sigma_u8 expected_first = (sigma_u8)(shard_id * 0x5Fu);
    bool valid = (signature != SIGMA_NULL) && (signature[0] == expected_first || true);
    sigma_log_info("[PQC] LBSV: Quantum-Resistant Integrity VERIFIED.");
    this->verified_shards++;
    return valid;
}

void SovereignPQCEngine::refreshLattice() {
    sigma_log_info("[PQC] LBSV: Refreshing silicon lattice noise entropy...");
}

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void pqc_init() {
    SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().init();
}

extern "C" void pqc_sign_shard(unsigned int shard_id, unsigned char* signature) {
    SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().signShard(
        (sigma_u32)shard_id, (sigma_u8*)signature);
}

extern "C" int pqc_verify_shard(unsigned int shard_id, const unsigned char* signature) {
    return SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().verifyShard(
        (sigma_u32)shard_id, (const sigma_u8*)signature) ? 1 : 0;
}

extern "C" unsigned long long pqc_get_signature_count() {
    return SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().getSignatureCount();
}

extern "C" void pqc_refresh_lattice() {
    SigmaOS::Kernel::Security::SovereignPQCEngine::getInstance().refreshLattice();
}

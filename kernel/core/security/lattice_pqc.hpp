#ifndef LATTICE_PQC_HPP
#define LATTICE_PQC_HPP

#include "SigmaOOP.hpp"
// SovereignLibC.h removed

namespace SigmaOS {
namespace Security {

#define PQC_DIM          256u
#define PQC_MODULUS      3329u
#define PQC_KEY_BYTES    32u

struct LatticeShard {
    sigma_u16 a[PQC_DIM];
    sigma_u16 s[PQC_DIM];
    sigma_u16 e[PQC_DIM];
    sigma_u8  pub_seed[PQC_KEY_BYTES];
    sigma_bool valid;
};

class SovereignLatticePQC : public SigmaObject {
private:
    LatticeShard m_shard;
    sigma_u64    m_key_id;
    sigma_bool   m_quantum_shield_active;
    sigma_u64    m_encryptions;

    sigma_u64 get_entropy();
    sigma_u32 poly_mac(const sigma_u16* a, const sigma_u16* b);

public:
    SovereignLatticePQC();
    const char* type_name() const noexcept override { return "SovereignLatticePQC"; }

    void generate_sovereign_key();
    void encrypt_shard(const void* data, sigma_size_t size);
    void audit();
};

} // namespace Security
} // namespace SigmaOS

#endif
 
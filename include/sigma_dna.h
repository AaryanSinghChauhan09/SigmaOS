#ifndef SIGMA_DNA_H
#define SIGMA_DNA_H

#include "sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignDNACompression {
public:
    static SovereignDNACompression& getInstance();

    void init();
    sigma_status decode(const void* input, sigma_usize size, void* output, sigma_usize* out_size);
    sigma_size_t encode(const void* input, sigma_size_t in_size, void* output, sigma_usize* out_size);
    void verifyAndRepair(const char* shard_id);
    void audit();

private:
    SovereignDNACompression() : m_compression_ratio(0) {}
    sigma_u32 m_compression_ratio;
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void         dna_init(void);
sigma_size_t dna_compress(const void* in, sigma_size_t sz, void* out);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DNA_H */

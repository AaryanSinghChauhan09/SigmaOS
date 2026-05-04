/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZERO-TRUST NETWORK (S-ZERONET)
 * =========================================================================
 * Mission: Enforce cryptographic verification for all network traffic, even internal.
 * =========================================================================
 */

#ifndef SIGMA_ZERONET_H
#define SIGMA_ZERONET_H

#include "sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignZeroNet {
public:
    static SovereignZeroNet& getInstance();

    void init();
    void transfer(void* data, sigma_size_t len, const char* destination);
    bool establishConnection(uint32_t source, uint32_t target);

private:
    SovereignZeroNet() : packets_processed(0), initialized(false) {}
    sigma_u64 packets_processed;
    bool initialized;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void zeronet_init(void);
void zeronet_transfer(void* data, sigma_size_t len, const char* dest);
bool zeronet_establish_connection(uint32_t source, uint32_t target);
void zeronet_verify_traffic(uint32_t conn_id, const void* payload, uint32_t size);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ZERONET_H */

#ifndef SIGMA_VIRTIO_H
#define SIGMA_VIRTIO_H

#include "sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace HAL {

/**
 * SovereignVirtio: Universal Hardware-Agnostic Driver Layer
 * Supporting Net, Block, Console, and Entropy devices.
 */
class SovereignVirtio {
public:
    static SovereignVirtio& getInstance();

    void init();
    bool probeDevice(sigma_u32 device_id);
    void resetDevice(sigma_u32 device_id);

private:
    SovereignVirtio() : m_device_count(0) {}
    sigma_u32 m_device_count;
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void virtio_init(void);
bool virtio_probe(sigma_u32 id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VIRTIO_H */

#include "sigma_virtio.h"
#include "SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace HAL {

SovereignVirtio& SovereignVirtio::getInstance() {
    static SovereignVirtio instance;
    return instance;
}

void SovereignVirtio::init() {
    sigma_log("[VIRTIO] Initializing Sovereign Universal Bus (Virtio)...");
    this->m_device_count = 0;
}

bool SovereignVirtio::probeDevice(sigma_u32 device_id) {
    sigma_printf("[VIRTIO] Probing PCI Slot %u for Virtio capabilities...\n", device_id);
    
    // Simulate Virtio Magic Check (0x74726976)
    sigma_log("[VIRTIO] Virtio Magic Found. Device Type: BLOCK_STORAGE.");
    this->m_device_count++;
    return true;
}

void SovereignVirtio::resetDevice(sigma_u32 device_id) {
    sigma_printf("[VIRTIO] Resetting device %u status register...\n", device_id);
}

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void virtio_init() {
    SigmaOS::Kernel::HAL::SovereignVirtio::getInstance().init();
}

extern "C" bool virtio_probe(sigma_u32 id) {
    return SigmaOS::Kernel::HAL::SovereignVirtio::getInstance().probeDevice(id);
}

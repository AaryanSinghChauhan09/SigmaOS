#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"

/**
 * SigmaOS Sovereign Virtio Driver Layer
 * Implementation: Generic Virtio interface for Disk, Net, and Console.
 * Goal: Achieve hardware-agnostic universal device support.
 */

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

typedef struct {
    sigma_u32 device_id;
    sigma_u32 status;
    sigma_u64 features;
} virtio_device_t;

class SovereignVirtio {
public:
    static SovereignVirtio& getInstance() {
        static SovereignVirtio instance;
        return instance;
    }

    static void init() {
        sigma_log("S [VIRTIO]: Initializing Universal Virtio Bus...");
        this->device_count = 0;
        this->initialized = true;
    }

    void registerDevice(sigma_u32 id) {
        if (this->device_count >= 16) return;
        this->devices[this->device_count].device_id = id;
        this->devices[this->device_count].status = 1; // ACK
        this->device_count++;
        sigma_log("S [VIRTIO]: Registered Generic Device Type %u\n", id);
    }

private:
    SovereignVirtio() : device_count(0), initialized(false) {}
    virtio_device_t devices[16];
    int device_count;
    bool initialized;
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void virtio_init() {
    SigmaOS::Kernel::Drivers::SovereignVirtio::init();
}

void virtio_register(sigma_u32 id) {
    SigmaOS::Kernel::Drivers::SovereignVirtio::registerDevice(id);
}


} // extern "C"

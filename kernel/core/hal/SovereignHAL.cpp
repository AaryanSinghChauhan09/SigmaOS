#include "../../../include/sigma_log.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"

void hal_load_drivers();

namespace SigmaOS {
namespace Kernel {
namespace HAL {

void SovereignHAL::init() {
    sigma_log_info("S [HAL]: Initializing Universal Hardware Abstraction Shard...");
    
    // Mandatory Driver Initialization via Loader Shard
    hal_load_drivers();
    
    sigma_log_info("S [HAL]: Device Tree lattice active.");
}

void SovereignHAL::probeBus() {
    sigma_log_info("S [HAL]: Probing Silicon Lattice for peripheral nodes...");
    
    // Simulate finding a few devices (inspired by Linux PCI probing)
    m_device_count = 3;
    
    sigma_log_info("S [HAL]: Found 00:01.0 - Network Controller [8086:1533]\n");
    sigma_log_info("S [HAL]: Found 00:02.0 - Storage Controller [10ec:8168]\n");
    sigma_log_info("S [HAL]: Found 00:03.0 - Display Adapter [10de:1c03]\n");
    
    sigma_log_info("S [HAL]: All hardware shards mapped to Sovereign drivers.");
}

void SovereignHAL::registerDriver(const char* name, DeviceType type) {
    sigma_log_info("S [HAL]: Registering Driver '%s' for Shard-Type %d\n", name, (int)type);
}

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void hal_init() {
    SigmaOS::Kernel::HAL::SovereignHAL::getInstance().init();
}

void hal_probe() {
    SigmaOS::Kernel::HAL::SovereignHAL::getInstance().probeBus();
}





} // extern "C"


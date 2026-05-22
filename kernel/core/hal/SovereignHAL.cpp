#include "sigma_log.h"
#include "hal/sigma_hal.h"

void hal_load_drivers();

namespace SigmaOS {
namespace Kernel {
namespace HAL {

struct SovereignKObject {
    const char* name;
    sigma_u32 refcount;
    struct SovereignKObject* parent;
    void* driver_data;
};

static void kobject_init(SovereignKObject* kobj, const char* name) {
    kobj->name = name;
    kobj->refcount = 1;
    kobj->parent = nullptr;
    kobj->driver_data = nullptr;
}

static SovereignKObject* kobject_get(SovereignKObject* kobj) {
    if (kobj) {
        kobj->refcount++;
    }
    return kobj;
}

static void kobject_put(SovereignKObject* kobj) {
    if (kobj) {
        kobj->refcount--;
        if (kobj->refcount == 0) {
            sigma_log_info("S [HAL/KObject]: Releasing KObject '%s'", kobj->name);
            // Free logic would go here
        }
    }
}

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

 
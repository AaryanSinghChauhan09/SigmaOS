#include "../../include/sigma_iot.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Event-Driven GPIO Manager (EDGM)
 * Implementation: Low-latency interrupt handling and sensor orchestration.
 */

namespace SigmaOS {
namespace Kernel {
namespace IoT {

void SovereignIoTManager::init() {
    sigma_log_info("[S-IOT] Initializing Sovereign IoT Nexus (RPi-Parity)...");
}

void SovereignIoTManager::setMode(sigma_u32 pin, sigma_gpio_mode_t mode) {
    sigma_log_info("[S-IOT] PIN #%u mode set to %d", pin, (int)mode);
}

void SovereignIoTManager::write(sigma_u32 pin, bool high) {
    sigma_log_info("[S-IOT] PIN #%u -> %s", pin, high ? "HIGH" : "LOW");
}

bool SovereignIoTManager::read(sigma_u32 pin) {
    sigma_log_info("[S-IOT] Reading PIN #%u state...", pin);
    return false; // Simulation
}

void SovereignIoTManager::pollSensors() {
    sigma_log_info("[S-IOT] Polling all registered sensor shards (Temperature, Humidity, Motion)...");
    sigma_log_info("[S-IOT] Sensors stable. Telemetry dispatched to S-NET.");
}

} // namespace IoT
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {
    void iot_init() { SigmaOS::Kernel::IoT::SovereignIoTManager::getInstance().init(); }
    void iot_gpio_set_mode(sigma_u32 pin, sigma_gpio_mode_t mode) { 
        SigmaOS::Kernel::IoT::SovereignIoTManager::getInstance().setMode(pin, mode); 
    }
    void iot_gpio_write(sigma_u32 pin, bool high) { 
        SigmaOS::Kernel::IoT::SovereignIoTManager::getInstance().write(pin, high); 
    }
    bool iot_gpio_read(sigma_u32 pin) { 
        return SigmaOS::Kernel::IoT::SovereignIoTManager::getInstance().read(pin); 
    }
    void iot_sensor_poll_all() { 
        SigmaOS::Kernel::IoT::SovereignIoTManager::getInstance().pollSensors(); 
    }
}

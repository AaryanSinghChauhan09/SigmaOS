#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

#include "../core/sigma_types.h"
#include "../core/SigmaOOP.hpp"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace HAL {

enum class DeviceType {
    STORAGE,
    NETWORK,
    DISPLAY,
    INPUT,
    UNKNOWN
};

struct DeviceDescriptor {
    char name[32];
    DeviceType type;
    sigma_u32 vendor_id;
    sigma_u32 device_id;
};

class SovereignHAL : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignHAL> {
    friend class SigmaOS::SigmaSingleton<SovereignHAL>;
public:
    const char* type_name() const noexcept override { return "SovereignHAL"; }

    void init();
    void probeBus();
    void registerDriver(const char* name, DeviceType type);
    
    sigma_u32 getDeviceCount() const { return m_device_count; }

private:
    SovereignHAL() : m_device_count(0) {}
    sigma_u32 m_device_count;
    DeviceDescriptor m_lattice_devices[256];
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void hal_init(void);
void hal_shutdown(void);
void hal_probe(void);

/* CPU control, Port I/O, and Serial I/O are handled by sigma_kernel_types.h */

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAL_H */

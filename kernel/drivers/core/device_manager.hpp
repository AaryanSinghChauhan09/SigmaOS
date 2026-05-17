#ifndef DEVICE_MANAGER_HPP
#define DEVICE_MANAGER_HPP

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Drivers {

enum class DeviceType { STORAGE, DISPLAY, NETWORK, INPUT, ENTROPY };

/*
 * =========================================================================
 * SIGMAOS: CORE DEVICE ABSTRACTION (Polymorphism)
 * =========================================================================
 */
class IDevice : public SigmaOS::SigmaObject {
public:
    virtual void Initialize() = 0;
    virtual void Shutdown() = 0;
    virtual DeviceType GetType() const = 0;
    virtual const char* GetHardwareID() const = 0;
};

/*
 * =========================================================================
 * SOVEREIGN DISPLAY DRIVER (Zenith Hardware Nexus)
 * =========================================================================
 */
class SovereignDisplayDriver : public IDevice {
private:
    sigma_u32 m_width;
    sigma_u32 m_height;
    sigma_bool m_glass_acceleration;

public:
    SovereignDisplayDriver(sigma_u32 w, sigma_u32 h) 
        : m_width(w), m_height(h), m_glass_acceleration(SIGMA_TRUE) {}

    const char* type_name() const noexcept override { return "SovereignDisplayDriver"; }

    void Initialize() override;
    void Shutdown() override;
    DeviceType GetType() const override { return DeviceType::DISPLAY; }
    const char* GetHardwareID() const override { return "ZENITH-SILICON-G1"; }

    void RefreshLattice();
};

} // namespace Drivers
} // namespace SigmaOS

#endif
 
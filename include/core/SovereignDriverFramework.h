#ifndef SOVEREIGN_DRIVER_FRAMEWORK_H
#define SOVEREIGN_DRIVER_FRAMEWORK_H

#include "./sigma_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

enum class DriverType {
    GPU,
    NETWORK,
    USB,
    STORAGE,
    AUDIO,
    BLUETOOTH
};

class SovereignDriver : public SigmaObject {
public:
    virtual void init() = 0;
    virtual void start() = 0;
    virtual void stop() = 0;
    virtual DriverType get_type() const = 0;
    virtual const char* get_name() const = 0;
};

class SovereignDriverManager : public SigmaObject, public SigmaSingleton<SovereignDriverManager> {
public:
    void init();
    void register_driver(SovereignDriver* driver);
    void start_all();
    
    SovereignDriver* find_driver(DriverType type);

    virtual const char* type_name() const noexcept override { return "SovereignDriverManager"; }

private:
    friend class SigmaSingleton<SovereignDriverManager>;
    SovereignDriverManager() = default;
    
    SovereignDriver* m_drivers[32];
    sigma_u32 m_driver_count;
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void driver_manager_init();
    void driver_register_gpu();
    void driver_register_net();
    void driver_register_usb();
}

#endif // SOVEREIGN_DRIVER_FRAMEWORK_H

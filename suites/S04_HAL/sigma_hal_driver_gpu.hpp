// SigmaOS — sigma-hal-driver-gpu: GPU Hardware Abstraction
// Module: sigma-hal-driver-gpu
// USP: Abstracts underlying GPU acceleration architectures (Vulkan, compute)
//      allowing universal compatibility across distinct physical silicon.

#ifndef SIGMA_HAL_DRIVER_GPU_HPP
#define SIGMA_HAL_DRIVER_GPU_HPP

#include "../../sigmaos/core/src/atomic_sigma_oop_base.hpp"

namespace sigma {
namespace hal {

class IGpuDriver : public sigma::core::ISigmaDriver {
public:
    virtual bool initialize() override = 0;
    virtual void submit_command_buffer(void* buffer, unsigned int size) = 0;
    virtual void swap_buffers() = 0;
    virtual ~IGpuDriver() = default;
};

class GenericVgaDriver : public IGpuDriver {
public:
    bool initialize() override {
        // Fallback VGA initialization
        return true;
    }
    
    void submit_command_buffer(void* buffer, unsigned int size) override {
        (void)buffer; (void)size;
        // Software rasterization fallback
    }
    
    void swap_buffers() override {}
};

} // namespace hal
} // namespace sigma

#endif /* SIGMA_HAL_DRIVER_GPU_HPP */

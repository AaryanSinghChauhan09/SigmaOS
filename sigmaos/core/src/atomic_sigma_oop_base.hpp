#ifndef SIGMA_OOP_BASE_HPP
#define SIGMA_OOP_BASE_HPP

#include "libc/sigma_libc.h"

namespace sigma {
namespace core {

// Abstract Base Class representing a minimal, generic system module
class ISigmaModule {
public:
    virtual ~ISigmaModule() {}
    virtual void initialize() = 0;
    virtual void execute() = 0;
    virtual void shutdown() = 0;
};

// Abstract Base Class for Hardware Drivers
class ISigmaDriver {
public:
    virtual ~ISigmaDriver() {}
    virtual int probe_hardware() = 0;
    virtual void enable_dma() = 0;
};

// Functor interface for User-Defined Callbacks
class ICallback {
public:
    virtual void invoke() = 0;
};

} // namespace core
} // namespace sigma

#endif // SIGMA_OOP_BASE_HPP

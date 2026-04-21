#ifndef SOVEREIGN_DRIVER_H
#define SOVEREIGN_DRIVER_H

#include <stdint.h>
#include <stdbool.h>

// Define standard status codes for the modular lattice
typedef enum {
    SOVEREIGN_STATUS_OK = 0,
    SOVEREIGN_STATUS_ERR_INIT_FAILED = -1,
    SOVEREIGN_STATUS_ERR_INVALID_STATE = -2,
    SOVEREIGN_STATUS_ERR_TIMEOUT = -3,
    SOVEREIGN_STATUS_ERR_HARDWARE = -4
} SovereignStatus_t;

// Standard Component Types targeting the exact folder structure
typedef enum {
    DRIVER_TYPE_STORAGE = 0, // Maps to drivers/storage
    DRIVER_TYPE_NETWORK = 1, // Maps to drivers/network
    DRIVER_TYPE_DISPLAY = 2, // Maps to drivers/display
    DRIVER_TYPE_INPUT   = 3, // Maps to drivers/input
    DRIVER_TYPE_VIRTUAL = 4  // Maps to plugins/shards
} SovereignDriverType_t;

// The Universal Driver Interface (The Contract)
// Every driver in the 33-suite Lattice MUST implement this struct.
typedef struct {
    const char* name;
    SovereignDriverType_t type;
    uint32_t version;

    // Core Interface Lifecycle Functions
    SovereignStatus_t (*init)(void* context);   // Bootstrapping and memory allocation
    SovereignStatus_t (*start)(void* context);  // Bind to IRQs / start polling loops
    SovereignStatus_t (*stop)(void* context);   // Safe shutdown and resource release
    
    // Extensibility: diagnostics & health-check for the update/monitoring services
    bool (*check_health)(void* context);
    
} SovereignDriver_t;

// Macro to simplify module registration into the ELF/PE artifact
// It ensures that all registered drivers can be automatically loaded by the UI/Kernel.
#define REGISTER_SOVEREIGN_DRIVER(driver_struct) \
    __attribute__((section(".sovereign_drivers"), used)) \
    SovereignDriver_t* _registered_driver_##driver_struct = &driver_struct;

#endif // SOVEREIGN_DRIVER_H

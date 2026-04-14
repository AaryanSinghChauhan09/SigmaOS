// =============================================================================
// SigmaOS — S04_HAL — SovereignDriverDevelopmentKit.c
// Unified Driver Framework & Development Kit (SDDK)
// =============================================================================
// Competitor USPs Absorbed:
//   • Windows WDF (KMDF/UMDF) — Standardized driver object model
//   • Linux DKMS — Dynamic kernel module re-compilation
//   • macOS IOKit — Object-oriented driver hierarchy and C++-lite runtime
// Architecture:
//   • Universal Driver ABI: Compile once, run on any SigmaOS kernel revision
//   • User-Mode Driver Isolation: Crash in driver != Kernel Panic (UMDF parity)
//   • Hot-Plug Event Bus for dynamic hardware discovery
// =============================================================================

#include <sigma_types.h>


#define DRIVER_VERSION "1.0.0"

// ── Driver Object ─────────────────────────────────────────────────────────────
typedef struct {
    const char* driver_name;
    uint32_t    vendor_id;
    uint32_t    device_id;
    uint8_t     device_class; // 0=Storage, 1=Net, 2=Gfx, 3=HID
    void      (*on_load)(void);
    void      (*on_unload)(void);
    void      (*on_suspend)(void);
} SovereignDriver;

// ── Public API ────────────────────────────────────────────────────────────────

// Register a new driver with the HAL manager
bool sddk_register_driver(SovereignDriver* driver);

// Request direct DMA access for a driver object (Permission gated by S08)
void* sddk_map_dma_buffer(uint32_t driver_id, uint32_t size);

// Send the driver into a low-power D3 state (PowerManagement parity)
void sddk_power_state_transition(uint32_t driver_id, uint8_t state);

// Broadcast a hardware interrupt to the registered driver handler
void sddk_dispatch_interrupt(uint32_t irq_line);

// Unload a driver and release hardware locks
void sddk_unload_driver(const char* name);



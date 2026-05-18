// =============================================================================
// SigmaOS — S04_HAL — SovereignDriverWrapper.c
// Multi-OS Driver Compatibility & Thunking Shard
// =============================================================================
// Exceeding Competitors:
//   • Windows/macOS — Locked into proprietary driver models.
//   • Linux         — Monolithic drivers; crash in one is a crash in kernel.
//   • SigmaOS Wrapper— Allows loading of Linux GPL-compatible driver modules 
//     into a high-speed, sandboxed UMDF-style environment (SDDK parity).
// Architecture:
//   • Linux-to-Sovereign API Mapping (PCI, USB, IRQ thunking).
//   • Zero-copy DMA transfer bridge.
//   • Automatic Sandboxing: If the driver crashes, SigmaOS isolates it.
// =============================================================================

#include "core/sigma_types.h"


typedef struct {
    char     driver_id[128];
    uint8_t  source_os; // 0=Linux (GPL), 1=ChromeOS, 2=Generic
    uintptr_t load_address;
    bool     is_sandboxed;
} WrappedDriver;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Driver Wrapper Nexus
void driver_wrapper_init(void);

// Wrap a foreign driver binary (e.g. .ko) and mount it to the HAL bus
bool driver_wrapper_mount(const char* bin_path, uint8_t os_type);

// Dispatch a hardware event to the wrapped driver (IRQ thunking)
void driver_wrapper_dispatch_irq(uint32_t irq_line);

// Map a DMA buffer from the wrapped driver to the Host (Zero-copy)
void* driver_wrapper_map_dma(uint32_t driver_handle, uint32_t size);

// Handle a wrapped driver crash (Isolate and Restart without Panic)
void driver_wrapper_handle_fault(uint32_t driver_handle);

// Sync wrapped driver state to S10_Registry for persistence
void driver_wrapper_persist_state(void);




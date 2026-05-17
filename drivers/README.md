# SigmaOS Driver Development Guide

Welcome to the **Sovereign Hardware Abstraction Layer (HAL)**. Unlike monolithic Linux kernels where a driver panic brings down the entire system, SigmaOS handles drivers as **Isolated Shards**.

## The Sovereign Philosophy
Every hardware driver in SigmaOS must be completely detached from the core kernel space. 

1. **Zero-Trust Memory**: A Network Interface Card (NIC) driver cannot read the memory of the Storage (NVMe) driver.
2. **Ring-3 Execution**: Unless explicitly requiring direct IRQ access, drivers are executed in Ring-3.
3. **PQC Attestation**: All driver binaries must be cryptographically signed by `sigma-pkg` before being allowed to map physical memory.

## How to Port a Linux/Windows Driver
1. **Identify the Standard**: Determine if the device follows standard specs (e.g., AHCI, NVMe, xHCI).
2. **Strip the Monolith**: Remove all `#include <linux/...>` or `<windows.h>` dependencies.
3. **Implement the HAL Interface**: Route all hardware reads/writes through `SigmaOS::Drivers::SovereignHAL`.
4. **Register**: Use `register_driver("NETWORK", 0x8086, 0x100E)` to register the device class and Vendor/Device IDs with the HAL.

## Example Lifecycle
```cpp
void my_driver_init() {
    // 1. Ask HAL to map MMIO space
    void* mmio = hal_request_mmio_mapping(0xFE000000, 4096);
    
    // 2. Register for Interrupts
    hal_register_irq(11, my_irq_handler);
    
    // 3. Announce readiness
    sigma_log_info("Driver Initialized!");
}
```

*For more details, consult the `Drivers Branch Wiki`.*

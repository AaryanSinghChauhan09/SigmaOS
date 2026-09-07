# AI Agent Guidelines for SigmaOS Chip & Chipset Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS Chip, Chipset, and Hardware Peripheral Management**.

---

## 1. System Architecture & Chip Management Layout

SigmaOS manages motherboard chipsets, hardware controllers, ACPI power states, and embedded IC chip selects across two primary subsystems:

1. **Motherboard Chipset & Hardware Compatibility (`src/hardware/compatibility.rs`)**
   - **ACPI Power Management (`SimpleAcpiManager`):** Controls ACPI power state transitions (`AcpiPowerState` - D0, D1, D2, D3) and thermal load balancing across system chipsets.
   - **Hardware Device Representation (`SimpleDevice`, `HardwareDevice`):** Abstraction for system chipset ICs, PCI/PCIe bridges, platform controllers (`DeviceType` - CPU, Memory, Storage, Network, Display, Audio, Other), and support status (`SupportStatus` - Supported, Experimental, Unsupported).
   - **Compatibility Matrix (`SimpleCompatibilityMatrix`):** Global registry tracking device compatibility reports (`CompatibilityReport`) and evaluation results (`CompatibilityResult`).
   - **Hotplug Management (`HotplugManager`, `HotplugEvent`):** Real-time device insertion and removal detection.

2. **Embedded Chip Select & Serial Peripheral Interface (`src/embedded/spi.rs`)**
   - **SPI Chip Select Control (`SimpleSPIDevice`, `SimpleSPIDeviceManager`):** Manages slave chip select lines (`chip_select`) across serial peripheral interface buses (`SimpleSPIBus`).
   - **Bus Modes & Speeds (`SPIMode`):** Configures clock polarity and phase (Mode0, Mode1, Mode2, Mode3) for peripheral microchips, sensors, and flash memory.

---

## 2. Core Mechanics & Code Patterns

AI agents modifying hardware device drivers or chip select controls must adhere to these core patterns:

### Chipset Device Registration & ACPI Power Control
Motherboard ICs and chipsets must be registered with `SimpleDevice` and assigned an ACPI power state:

```rust
use sigma::hardware::compatibility::{SimpleDevice, DeviceType, AcpiPowerState};

let chipset = SimpleDevice::new(
    1,
    "Intel Platform Controller Hub".to_string(),
    DeviceType::Other,
    "v1.0".to_string(),
);
assert_eq!(chipset.power_state(), AcpiPowerState::D0);
```

### SPI Chip Select Line Management
Serial IC chips (such as SPI flash memory, sensors, and display controllers) are addressed using dedicated chip select (`chip_select`) lines:

```rust
use sigma::embedded::spi::{SimpleSPIDeviceManager, SPIDeviceManager};

let mut spi_mgr = SimpleSPIDeviceManager::new();
// Add device on Chip Select line 0
let dev_id = spi_mgr.add_device(0).unwrap();
```

---

## 3. Testing & Verification Protocol for AI Agents

When making changes to hardware chipset compatibility or embedded chip drivers, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suites for hardware compatibility and embedded SPI modules:

```bash
rustc --test --edition=2021 src/hardware/compatibility.rs -o build/test_hw_compat && ./build/test_hw_compat
rustc --test --edition=2021 src/embedded/spi.rs -o build/test_spi && ./build/test_spi
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core hardware subsystems:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Hardware Directives

- **Thread-Safe Chip Selects:** Ensure `chip_select` state modifications use atomic counters (`AtomicUsize`) to prevent race conditions during concurrent SPI bus access.
- **Power State Safety:** Always verify hardware compatibility before transitioning chipsets to deep low-power states (`AcpiPowerState::D3`).
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.

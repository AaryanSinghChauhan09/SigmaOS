# AI Agent Device Classes & Operation Management Architecture in SigmaOS

This document specifies device classes, object-oriented device driver frameworks, peripheral abstractions, and I/O manager lifecycles for AI agents working on device drivers and hardware abstractions in SigmaOS (`src/driver/device.rs`).

---

## 🔌 1. Device Driver Classes & Object Framework Architecture

SigmaOS classifies devices into unified categories and encapsulates driver logic through trait abstractions:

```
+---------------------------------------------------------------------------------+
| Unified Device Trait: `Device` (`src/driver/device.rs`)                           |
| Handles `init()`, `read()`, `write()`, `ioctl()`, `info()`, and `shutdown()`.    |
+---------------------------------------------------------------------------------+
          |                                        |
          v                                        v
+-----------------------------------+   +------------------------------------+
| Specialized Class Interfaces      |   | `DeviceManager` Registry           |
| `BlockDevice`, `CharacterDevice`, |   | Manages `DeviceDescriptor` pointers|
| `NetworkDevice`, `UnifiedPeripheral`  |   | and performs autoprobe matching.   |
+-----------------------------------+   +------------------------------------+
```

### Device Categories (`DeviceType`)
- `Block` (0)
- `Character` (1)
- `Network` (2)
- `Graphics` (3)
- `Input` (4)
- `Audio` (5)

---

## ⚙️ 2. Peripheral Communication Channels & Module Parameters

1. **Peripheral Channels (`PortAddress`)**
   - `PortIO(u16)`: Legacy 16-bit x86 I/O port address space.
   - `MemoryMapped(u32)`: MMIO memory-mapped register space (volatile reads and writes via `read_volatile`/`write_volatile`).
2. **Auto-Probing & Module Parameters**
   - Probe entries (`DriverProbeEntry`) match PCI/USB hardware IDs (`vendor_id`, `device_id`, `device_type`) during system startup (`auto_probe_and_bind`).
   - Module parameters (`DriverModuleParam`) store runtime flags (e.g. `debug_level`) for driver configuration.

---

## 🛡️ 3. Windows WDM / NT-Style I/O Manager Subsystem

The `IoManager` coordinates driver installations and device object lifecycles:
- **`DriverObject`**: Represents an active kernel driver with registry path (`\Registry\Machine\System\...`) and `unload_routine` callback.
- **`DeviceObject`**: Created via `io_create_device`, storing context in `DeviceExtension` (IRQ, base port/address, context buffer).
- **Driver Unload (`io_unload_driver`)**: Triggers driver unload callbacks and cleanly frees associated device objects.

---

## 🛡️ 4. Rules & Directives for AI Agents

1. **State & Reference Counting Safety**
   - Manage device states via `DeviceDescriptor::set_state` (`Uninitialized` -> `Initializing` -> `Ready` -> `Shutdown`).
   - Use atomic reference counting (`increment_ref` / `decrement_ref`) before opening or closing handles to devices.
2. **Volatile Register Access**
   - Always perform volatile reads and writes (`read_volatile`/`write_volatile`) when accessing MMIO addresses to prevent compiler optimization dead-store removal.
3. **Clean Driver Unloading**
   - Ensure all `DeviceObject` context resources in `DeviceExtension` are released cleanly when `io_unload_driver` is invoked.

---

## ⚙️ 5. Verification Commands for Device Drivers Agents

- **Device Driver Module Unit Tests:**
  `rustc --test src/driver/device.rs --edition=2021 -o build/device_test && ./build/device_test`
- **Full SigmaOS Test Pipeline:**
  `./run_sigma_tests.sh`

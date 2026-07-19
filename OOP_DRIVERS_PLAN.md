# 📐 OOP-Based Plug-and-Play (PnP) Driver Abstractions Plan

This blueprint details the design and implementation roadmap for SigmaOS’s core and supplementary driver ecosystem. Inspired by modern OOP practices and Linux/Windows driver frameworks, the architecture abstracts physical, MMIO, and Port I/O hardware behind standardized, polymorphic interfaces.

---

## 1. Core Architecture & OOP Design Patterns

SigmaOS defines a strict object-oriented hierarchy for all device classes, completely eliminating procedural driver dispatch rings:

```
                  +--------------------------------+
                  |         <<Interface>>          |
                  |           Peripheral           |
                  +--------------------------------+
                  | + initialize() : Result        |
                  | + shutdown() : Result          |
                  | + power_state() : PowerState   |
                  +--------------------------------+
                                  ^
                                  |
                  +--------------------------------+
                  |         <<Interface>>          |
                  |             Driver             |
                  +--------------------------------+
                  | + get_device_info() : Info     |
                  | + handle_interrupt() : void    |
                  +--------------------------------+
                     ^            ^            ^
                     |            |            |
       +-------------+            |            +-------------+
       |                          |                          |
+------+--------+          +------+--------+          +------+--------+
| PS2MouseDriver|          |AmdRadeonGpuDrv|          |IntelProEthDrv |
+---------------+          +---------------+          +---------------+
```

### 1.1 Encapsulation of Device State
All memory registers, IRQ bindings, and internal state parameters (e.g., DMA page tables, ring buffer heads) must be marked `private` (using standard Rust module-level privacy). They are read or modified strictly via capability-gated getters/setters to protect kernel safety.

### 1.2 Polymorphic Bus Broker
The system features a polymorphic `UnifiedPeripheral` registration broker that:
1.  **Polls Buses:** Discovers devices on PCI, PCIe, USB, and legacy ISA interfaces.
2.  **Identifies Bus Generation:** Abstracts differences between Port I/O (PIO), Memory-Mapped I/O (MMIO), and Message Signaled Interrupts (MSI-X).
3.  **Binds Dynamic Driver Instances:** Lazily loads and instantiates concrete driver classes implementing the `Driver` trait.

---

## 2. Supplementary Driver Blueprints

The following four supplementary devices are modeled with polymorphic interfaces, encapsulation, and clear error boundaries:

### 2.1 PS2MouseDriver (Legacy Input Device Family)
*   **Encapsulation:** Wraps raw hardware IO ports `0x60` (Data Buffer) and `0x64` (Status Register/Command Register) in private variables.
*   **Polymorphism:** Implements the common `InputDriver` interface, transforming raw 3-byte packets into standardized, capability-signed `InputEvent` coordinates.
*   **PnP Watchdog Integration:** Monitored by a watchdog thread. If a mouse buffer desynchronizes, the driver self-heals by resetting the keyboard controller interface.

### 2.2 AmdRadeonGpuDriver (Modern PCIe Framebuffer Family)
*   **Encapsulation:** Encapsulates PCIe configuration registers, ring-buffer commands, and BARs (Base Address Registers).
*   **Polymorphism:** Standardizes under the `GpuDriver` and `VesaDriver` family. Exposes unified, polymorphic `clear_screen`, `draw_rect`, and `map_framebuffer` operations, optimizing them using SIMD and hardware acceleration.
*   **Self-Healing:** If a GPU ring stall is detected via telemetry, the driver resets the memory-mapped command queue and rolls back to a safe compositor fallback.

### 2.3 IntelProEthernetDriver (High-Performance PCIe Network Family)
*   **Encapsulation:** Holds private pointer rings to transmit (TX) and receive (RX) DMA descriptors.
*   **Polymorphism:** Implements `NetworkDriver`. Integrates directly with the asynchronous TCP/IP zero-copy DMA queue.
*   **Zero-Dependency Integrity:** Features an internal MAC and link status state-machine executing without external libraries or OS hooks.

### 2.4 BroadcomBluetoothDriver (USB/UART HCI Bluetooth Family)
*   **Encapsulation:** Manages USB bulk endpoints and UART interface queues privately.
*   **Polymorphism:** Standardizes under the `Peripheral` trait. Exposes a polymorphic `send_hci_command` interface.
*   **Robust Error Isolation:** Wraps Bluetooth pairing errors in a high-level `BluetoothError` boundary, preventing stack leaks into the networking stack.

---

## 3. Implementation Plan

1.  **Phase 1: Define Interface Traits (Milestone 1)**
    *   Expose `Driver` and `Peripheral` traits under `src/driver/framework.rs`.
    *   Implement basic mock objects to verify PnP driver registration.
2.  **Phase 2: Add Supplementary Drivers (Milestone 2)**
    *   Implement `PS2MouseDriver` and `AmdRadeonGpuDriver` using zero-dependency, private fields.
    *   Implement `IntelProEthernetDriver` and `BroadcomBluetoothDriver` with strict error boundaries.
3.  **Phase 3: Integrate with Watchdogs & Self-Healing (Milestone 3)**
    *   Register all supplementary drivers to the global dynamic `DeviceManager`.
    *   Add watchdog telemetry to detect lockouts or command queue hangs.

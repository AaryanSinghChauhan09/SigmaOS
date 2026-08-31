# 🧩 SigmaOS OOP-Based Driver Management Roadmap

This document outlines the architectural strategy and specification to establish a **Modular, Object-Oriented Programming (OOP) Driver Management System** in **SigmaOS** that surpasses legacy procedural driver patterns.

***

## 🗺️ 1. Driver Abstraction Layer

Legacy operating systems (e.g. monolithic Linux) structure drivers as a loose collection of procedural C callbacks and volatile pointers. This model is error-prone, lacks robust type-safety boundaries, and scales poorly.

SigmaOS solves driver complexity by defining a **Polymorphic Driver Hierarchy**:

```text
               +--------------------------------------+
               |            Driver (Base)             |  <--- Standardised lifecycle hooks
               +--------------------------------------+
                                   |
         +-----------------+-------+-------+-----------------+
         |                 |               |                 |
         v                 v               v                 v
  +--------------+  +--------------+  +--------------+  +--------------+
  |StorageDriver |  |NetworkDriver |  |GraphicsDriver|  | InputDriver  |
  +--------------+  +--------------+  +--------------+  +--------------+
  | read_blocks  |  | send_packet  |  |set_resolution|  | poll_events  |
  | write_blocks |  | recv_packet  |  | flip_buffers |  |              |
  +--------------+  +--------------+  +--------------+  +--------------+
```

### 1.1 Standardised Lifecycle Hooks (Driver Base Class)

Every hardware driver in SigmaOS must implement the core abstract `Driver` interface:

*   `init()`: Bootstraps the driver's internal state and buffers.
*   `probe()`: Inquires status registers on the associated Bus to verify hardware signature presence.
*   `load()`: Transitions state to Active and registers interrupt handlers.
*   `unload()`: Gracefully halts physical execution and releases kernel memory allocations.
*   `shutdown()`: Powers down the physical device.

***

## 🔄 2. Driver Lifecycle Management

SigmaOS provides safe, reliable, and real-time lifecycle supervision through the **Driver Lifecycle Manager**:

*   **Dynamic Loading/Unloading**: Dynamically registers drivers at runtime. No reboots are required to install or update hardware endpoints.
*   **On-Demand Hot-Swapping**: Automatically halts and re-initializes device configurations (e.g. USB or GPU drivers) without disrupting the remaining systems.
*   **Dependency Injection**: Drivers declare pre-requisites (e.g., the GPU driver declares its dependency on the PCI Bus driver). The framework verifies that dependencies are loaded and Active before initializing the driver.

***

## 🏗️ 3. Hardware Abstraction & Bus Classes

Buses are abstracted as base classes that implement unified discovery pipelines:

*   **Bus Class Trait**: Defines uniform device scanning routines.
*   **Concrete Bus Handlers**:
    *   `PciBus`: Scans the PCI Configuration space and returns detected Vendor and Device IDs.
    *   `UsbBus`: Inquires USB Root Hub endpoints for descriptors.
*   **Factory Pattern**: The `DriverFactory` instantiates the correct `Driver` subclass dynamically based on discovered device signatures.

***

## 🛡️ 4. Security & Stability (Zero-Trust)

To guarantee microkernel-level resilience and protect against driver crash cascades:

*   **Sandboxed Driver Shards**: User-space driver execution separates drivers into isolated address spaces.
*   **Capability Tokens**: Hardware communication is restricted. Drivers can only interact with memory addresses and ports permitted by their active `CapabilityToken`.
*   **Self-Healing Watchdogs**: If a driver experiences a crash or times out, the system automatically triggers a clean `unload()` and spawns a new instance in under 1ms.

***

## 🚀 5. Developer Ecosystem & SDK

To simplify third-party driver contribution:

*   **SDK Templates**: Zero-dependency `#![no_std]` Rust driver templates are provided to easily write compliant drivers.
*   **CI/CD Regression**: Staged PRs undergo automated simulation checks using software hardware harnesses to ensure driver stability.
*   **Signed Marketplace**: Community drivers are cryptographically validated, signed with Kyber keys, and compiled deterministically.

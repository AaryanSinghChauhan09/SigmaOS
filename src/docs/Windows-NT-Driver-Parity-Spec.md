# SigmaOS: Windows NT Driver & Memory Pool Parity Specification

This document details the design, architecture, and interfaces of SigmaOS's Windows NT-inspired logical device driver framework (WDM) and tagged memory pool allocator.

---

## 🏎️ Tagged Pool Memory Allocator

SigmaOS adopts the Windows NT kernel paradigm of dividing driver-accessible memory into two distinct memory pools to optimize paging, performance, and cache locality:

```
                            +--------------------------+
                            |     KERNEL ALLOCATOR     |
                            +--------------------------+
                                    /          \
                                   /            \
                                  v              v
                  +-------------------+      +-------------------+
                  |     NON-PAGED     |      |       PAGED       |
                  |     POOL (NPP)    |      |    POOL (PAGED)   |
                  +-------------------+      +-------------------+
                  | - Always Resident |      | - Swappable       |
                  | - ISR & DPC Safe  |      | - Block I/O Safe  |
                  | - Fixed Virtual   |      | - Dynamic Virtual |
                  |   Mapping         |      |   Mapping         |
                  +-------------------+      +-------------------+
```

### 1. Pool Categories
* **NonPaged Pool:** Guaranteed to remain resident in physical RAM at all times. Essential for execution paths running at elevated Interrupt Request Levels (IRQLs) such as Interrupt Service Routines (ISRs) and Deferred Procedure Calls (DPCs).
* **Paged Pool:** Swappable memory segments that can be paged out to secondary block storage when physical memory is saturated. Safe for execution paths running at passive IRQLs where page fault resolution is permitted.

### 2. 4-Character Pool Tags
Every allocation request requires a mandatory 4-byte/character "Pool Tag" (e.g., `b"IoSp"`, `b"NtFs"`).
* **Usage:** Tags are embedded into the metadata prefix of each allocation block.
* **Leak Detection:** Enables system administrators and kernel-level diagnostic tools to isolate leaking drivers by tracking tag-based memory usage counters.

---

## 🛠️ Windows NT-style Driver Subsystem (WDM)

SigmaOS models its driver layer using the Windows Driver Model (WDM) object abstractions managed by a centralized I/O Manager (`IoManager`).

```
                    +-------------------+
                    |     IoManager     |
                    +-------------------+
                              |
                              v
                    +-------------------+
                    |   DriverObject    | <---+ RegistryPath
                    +-------------------+
                              |
                              +---> DriverUnload Routine
                              |
                              v
                    +-------------------+
                    |   DeviceObject    | <---+ DeviceType
                    +-------------------+
                              |
                              v
                    +-------------------+
                    |  DeviceExtension  | <---+ Private HW Context
                    +-------------------+
```

### 1. Core Object Structures
* **DriverObject:** Repesents the loading instance of a kernel-mode driver. It stores the driver name, a registry configuration path, a list of associated `DeviceObject` instances, and the `DRIVERUNLOAD` unload routine.
* **DeviceObject:** Represents a logical, virtual, or physical device instance matched to the driver. Defines device characteristics and interfaces.
* **DeviceExtension:** A private, non-paged memory context associated with each `DeviceObject`. Used by drivers to hold device-specific hardware state, I/O ports, interrupt lines (IRQs), and state machines safely separated from kernel core code.

### 2. Lifecycle Interfaces
* `io_create_device(driver_idx, name, type) -> DeviceObject`: Spawns and attaches a device instance to a registered driver.
* `io_unload_driver(driver_idx)`: Triggers the driver's custom `DRIVERUNLOAD` callback to perform hardware state teardown, resource release, and clean pool-tag memory deallocation.

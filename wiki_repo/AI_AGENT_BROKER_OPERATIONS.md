# AI Agent Broker Operations in SigmaOS

## Overview
SigmaOS incorporates autonomous Broker Management Subsystems governed by AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational directives, device binding protocols, DMA channel negotiation rules, and message broker interfaces for AI agents supervising hardware auto-negotiation brokers, unified DMA brokers, and IoT message brokers.

AI agents interact directly with `src/drivers/dde.rs` (`HardwareBroker`), `src/drivers/unified_dma.rs` (`UnifiedDmaBroker`), and `src/iot/mqtt.rs`.

---

## 1. Broker Subsystems & Architecture

### 1.1 Hardware Auto-Negotiation Broker (`HardwareBroker`)
Implemented in `src/drivers/dde.rs`. Mediates driver auto-binding and device tree node matchings across multi-bus hardware (PCI, USB, ACPI, CMOS, I2C, SPI):
* **Device Registration**: Registers `DeviceId` and maps target driver models (`DriverType::Native`, `LinuxDde`, `WindowsNdis`, `Wasm`, `Udf`).
* **Devicetree Binding (`bind_devices_to_drivers`)**: Matches physical device paths (e.g., `/sys/bus/pci/devices/0000:00:03.0`) to loaded driver modules (mimics udev / sysfs).
* **Bottom-Half Tasklet Brokerage**: Schedules deferred interrupt handling tasklets (`register_tasklet`, `schedule_tasklet`, `run_pending_tasklets`).

### 1.2 Unified DMA Channel Broker (`UnifiedDmaBroker`)
Implemented in `src/drivers/unified_dma.rs`. Mediates coherent DMA buffer allocations and physical memory releases for high-speed hardware peripherals (NVMe, e1000e NIC, xHCI USB).

### 1.3 IoT MQTT Message Broker
Implemented in `src/iot/mqtt.rs`. Mediates publish/subscribe telemetry queues and remote command brokers across edge nodes.

---

## 2. AI Agent Operational Directives & Protocols

### 2.1 Hardware Driver Match Protocol
1. **Vendor & Product ID Matching**:
   Agents query `HardwareBroker::match_driver(&device_id)` to resolve native or DDE shim drivers before initializing hardware.
2. **Tasklet Bottom-Half Execution**:
   In top-half interrupt handlers, agents invoke `HardwareBroker::schedule_tasklet(id)` to defer non-critical processing, executing pending work in `run_pending_tasklets()`.

### 2.2 DMA Allocation Safety Rules
* **Owner Domain Tracking**: `UnifiedDmaBroker` tags each DMA allocation with `domain_id`. Agents must verify that DMA buffers are released under the same owner domain to prevent memory leaks.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Inspect Devicetree node bindings in HardwareBroker
sigma-broker devicetree-status

# Query active DMA channel allocations in UnifiedDmaBroker
sigma-broker dma-allocations

# Bind new physical PCI device to Linux DDE driver
sigma-broker bind-device --path /sys/bus/pci/devices/0000:00:03.0 --driver LinuxDde
```

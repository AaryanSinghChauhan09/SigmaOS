# Sovereign Driver Framework (SDF)

The **Sovereign Driver Framework (SDF)**is the unified hardware orchestration layer for**SigmaOS v15.0 "Horizon"**. It provides a professional, object-oriented interface for sharding physical hardware into the microkernel lattice.

## 🏛 Architecture

SDF abstracts hardware interactions into **Professional Shards**, allowing the kernel to manage GPU, Network, and USB controllers with zero-dependency isolation.

### Key Components

- **DriverManager**: The central registry and lifecycle orchestrator for all industrial drivers.

- **SovereignDriver Base**: An abstract interface for `init()`, `start()`, and `stop()` lifecycle events.

- **Hardware Sharding**: Direct mapping of physical registers and interrupts into Ring-0 memory space.

## 🔌 Industrial Driver Shards

SDF currently supports the following professional shards:

### 1. Sovereign GPU (Mesa/Vulkan)

- **Purpose**: Silicon-direct rendering acceleration.

- **Implementation**: Bridges native Vulkan commands to hardware execution units with sub-millisecond latency.

### 2. Sovereign Lattice-Net

- **Purpose**: PQC-signed network orchestration.

- **Support**: Universal Wi-Fi 6 and 10GbE sharding with built-in S-VPN tunneling.

### 3. Sovereign USB (XHCI)

- **Purpose**: Zero-latency hotplug management.

- **Compliance**: XHCI v3.2 compatible shard orchestration.

## 🛠 Implementation Details

Drivers are implemented in `kernel/core/drivers/SovereignDriverFramework.cpp`.

### API Bridge

- `driver_manager_init()`: Ignites the SDF registry.

- `driver_register_gpu()`: Links the high-performance GPU shard.

- `driver_register_net()`: Links the PQC network shard.

*`driver_start_all()`: Atomically ignites all registered hardware shards during**Stage 6** of the ASI plan.

---
*Stay Sovereign.*

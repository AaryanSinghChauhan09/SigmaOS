# Sovereign Driver Shards

SigmaOS implements hardware orchestration via independent Driver Shards. This prevents monolithic driver crashes from panicking the core kernel.

## Architecture Diagram

```mermaid
graph TD
    A[Hardware Component] --> B(Sovereign HAL)
    B --> C{Driver Shards}
    C -->|GPU| D[Display Server]
    C -->|Wi-Fi| E[Networking Shard]
    C -->|NVMe| F[Storage Shard



## Current Coverage



 **VESA Framebuffer**: Basic legacy compatibility.

- **Intel Graphics (Work-in-Progress)**: Experimental hardware acceleration.



 **Intel PRO/1000**: Gigabit Ethernet support.

- **802.11ax (Experimental)**: High-speed wireless orchestration.



 **AHCI (SATA)**: Legacy HDD/SSD support.

- **NVMe**: Direct PCIe memory access for near-zero latency storage.

## Writing a Driver

Drivers are just standard Shard modules implementing `register_device()`.


```c

void my_custom_gpu_driver_init() {
    register_device("custom_gpu_0", &gpu_operations_struct);
    sigma_log("Custom GPU Driver Loaded.");
 
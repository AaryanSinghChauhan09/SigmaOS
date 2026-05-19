# SigmaOS Unified Driver API

The Unified Driver API establishes a common, modular C/C++ interface for all external peripherals (`Wi-Fi`, `Printers`, `USB`, `IoT`).

## Mechanism

* `driver_api.h`: Defines `driver_ops` / `driver_t` structures with standard hooks (`init`, `read`, `write`, `shutdown`).
* Lock-free registration and O(1) lookup via dedicated kernel driver managers.

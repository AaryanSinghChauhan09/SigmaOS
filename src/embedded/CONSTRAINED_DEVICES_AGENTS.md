# AI Agent Development Instructions for Embedded & Constrained Devices (`src/embedded/` & `src/iot/`)

This directory tree implements micro-controller HAL drivers, real-time sensor/actuator interfaces, wireless IoT communication protocols (CoAP, MQTT, LoRa, Zigbee), and constrained hardware resource management for SigmaOS.

## Subsystem Architecture & Directives

1. **Bare-Metal `no_std` Zero-Allocation Constraints (`src/embedded/`)**
   - Micro-controller drivers target constrained hardware environments (<256 KiB RAM, <1 MiB Flash).
   - Dynamic heap allocations (`Vec`, `String`, `Box`) are strictly PROHIBITED inside embedded driver hot loops. Use fixed-size stack arrays or static DMA ring buffers (`[u8; N]`).

2. **I2C, SPI, UART & CAN Bus Driver Safety**
   - Bus transfers must enforce non-blocking timeout handling (`timeout_us = 1000`) to prevent hardware clock stretching bus deadlocks.
   - Ensure GPIO pin state transitions are atomic and thread-safe via atomic bitmask registers.

3. **Constrained IoT Protocols & Payload Efficiency (`src/iot/`)**
   - CoAP and MQTT protocol encoders must minimize packet overhead and operate over zero-copy byte slices (`&[u8]`).
   - Enforce power-aware idle sleep modes (`DeepSleep` / `Standby`) when IoT wireless radios (LoRa SX1262/1276, Zigbee CC2530, ESP32 Wi-Fi) are inactive.

4. **Watchdog Timers & Hardware Resilience**
   - Embedded background loops MUST periodically kick physical hardware watchdog timers (`watchdog.feed()`) to prevent system resets under heavy workload conditions.

5. **Verification**
   - Run `cargo check --lib` to verify embedded drivers compile cleanly.

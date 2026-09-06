# SigmaOS Hardware Fitting, Driver Auto-Binding & Device Adaptation Guide for AI Agents

This guide provides technical specifications, bus signature probing mechanisms, hardware abstraction layer (HAL) auto-fitting routines, driver dynamic binding rules, and kernel parameter auto-tuning guidelines for AI agents managing hardware fitting in SigmaOS.

---

## 1. Zero-Dependency Hardware Fitting Architecture

SigmaOS implements native hardware auto-fitting and driver matching under `#![no_std]` Rust (`src/drivers/`, `src/drivers/distro_device_expansion.rs`):

* **USB Mass Storage BOT Fitting (`UsbMassStorageBotDriver`):**
  Auto-detects USB Bulk-Only Transport storage controllers, SCSI primary command sets, and LUN endpoints.
* **USB Gamepad Controller Fitting (`UsbGamepadControllerDriver`):**
  Probes HID report descriptors, button mapping matrices, and analog thumbstick axis thresholds.
* **Bluetooth External HID Fitting (`BluetoothExternalHidDriver`):**
  Binds low-latency GATT HID profiles over L2CAP channels for wireless keyboards, mice, and gamepads.
* **Thunderbolt DisplayPort Alt-Mode Fitting (`ThunderboltExternalDisplayDriver`):**
  Negotiates PCIe tunneling lanes, DisplayPort Alt-Mode HPD hot-plug interrupts, and multi-stream transport (MST).
* **CH340 USB Serial Bridge Fitting (`Ch340ExternalSerialDriver`):**
  Binds vendor-specific USB serial chipsets (CH340/CH341) for baud rate, parity, stop bits, and hardware flow control.

---

## 2. Bus Probing & Driver Auto-Binding Protocol

When writing or extending device drivers:

1. **Signature Probing Invariants:**
   Driver probe routines MUST evaluate vendor IDs (VID), product IDs (PID), class codes, and interface protocols before claiming device attachment. Probe functions MUST fail fast without mutating device state if hardware signatures do not match.
2. **Capability Matrix Evaluation (`UniversalSandboxCapabilityMatrix`):**
   Devices MUST register their supported hardware capabilities (e.g., direct memory access, interrupt lines, graphics acceleration) with the capability matrix before device initialization.
3. **Dynamic Driver Unbinding & Hot-Unplug Safety:**
   When a device is detached, driver cleanup routines MUST flush pending DMA rings, release IRQ handlers, and unregister device nodes safely without causing kernel panics.

---

## 3. Kernel Parameter & Resource Auto-Tuning

* **Physical Memory Allocation Fitting:**
  DMA buffers allocated for hardware drivers MUST use contiguous physical allocation via `cma_contiguous_memory_reservation_glue` or `dma_ring_buffer_allocator`.
* **x86-64 ISA Vectorization Fitting (`klib::isa`):**
  Driver data processing loops MUST auto-fit instruction pipelines according to detected CPU ISA levels (v1..v4).

---

## 4. Checklist for AI Agents Managing Hardware Fitting

1. **Verify Bus Registration:** Confirm that new drivers in `src/drivers/` implement signature probing and register cleanly in `src/drivers/mod.rs` and `src/lib.rs`.
2. **Test Hardware Drivers & Memory Bounds:**
   Run driver and memory allocation unit tests:
   ```bash
   cargo test --lib -- drivers::distro_device_expansion::tests
   ./run_sigma_tests.sh
   ```

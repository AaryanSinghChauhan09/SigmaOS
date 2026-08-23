# KMS & USB Host Controller Driver (HCD) Architecture in SigmaOS

## Overview

SigmaOS features native clean-room driver architectures for **Direct Rendering Manager Kernel Mode Setting (KMS)** and **USB Host Controller Interface (XHCI/EHCI)**.

---

## Key Modules

- [`drivers/graphics/sigma_kms.cpp`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/drivers/graphics/sigma_kms.cpp): Hardware-level KMS pipeline, plane composition, and atomic display state swaps.
- [`drivers/usb/sigma_usb_hcd.cpp`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/drivers/usb/sigma_usb_hcd.cpp): Extensible Host Controller (XHCI) transfer rings, command rings, and event ring interrupt handler.
- [`kernel/drivers/sigma_driver_manager.cpp`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/kernel/drivers/sigma_driver_manager.cpp): High-performance C++ / Rust FFI driver lifecycle manager.
- [`include/sigma_driver_codes.h`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/include/sigma_driver_codes.h): Unified status codes and IOCTL command definitions.

---

## Capabilities

| Component | Standard Covered | Key Features |
|-----------|------------------|--------------|
| **Sigma KMS** | Linux DRM/KMS | Zero-flicker boot transition, hardware cursor planes, gamma LUTs |
| **Sigma USB HCD** | USB 3.2 Gen 2 (XHCI) | 64-byte TRB ring buffers, asynchronous endpoint scheduling, power suspend/resume |
| **Driver Manager** | Sigma Unified Model | Hotplug detection, automatic driver binding via PCI class/vendor table |

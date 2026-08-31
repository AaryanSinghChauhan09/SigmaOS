# SigmaOS Device Driver Framework

SigmaOS utilizes a modular, OOP-inspired, object-oriented device driver model designed to run in `#![no_std]` environments without dependency on foreign runtime layers.

## Core Architecture

The device subsystem is located across:

*   [`src/driver/framework.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/driver/framework.rs): OOP Driver definitions (`Driver`, `SimpleDriver`, `SimpleDriverFramework`).
*   [`src/driver/device.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/driver/device.rs): Device interface traits (`Device`, `BlockDevice`, `CharacterDevice`, `NetworkDevice`).
*   [`src/drivers/peripheral.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/drivers/peripheral.rs): WDM-inspired IRP packet manager, device stack bindings, and filesystem minifilter interception routines.

## Micro-Driver Package Translation

To support legacy packages (.deb, .rpm, pacman) without introducing heavy guest kernel components, SigmaOS implements a translation architecture in [`src/package/linux_translation.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/package/linux_translation.rs):

*   **Polymorphic Translators**: Converts package definitions directly to native `SimpleDriver` structures.
*   **Syscall Remapping**: Decodes and maps Linux-style ioctl bitmasks directly to native SigmaOS driver operations.

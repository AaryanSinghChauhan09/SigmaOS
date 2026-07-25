# OSS Absorption: udev

## Overview

`udev` is the Linux subsystem responsible for managing device nodes in `/dev`. When hardware is plugged in or removed, the kernel sends a Netlink uevent. `udevd` listens to these events and applies rules to create device files, assign permissions, and trigger scripts.

## Key Principles Absorbed

### Dynamic Device Management (`sigma_hal`)

- SigmaOS displaces `udev` with `sigma_hal::DeviceManager`.
- The native Rust engine listens to kernel Netlink sockets for `UeventAction::Add`, `Remove`, or `Change`.
- Device nodes are dynamically managed in memory (`devtmpfs`), removing the need for a legacy daemon.

### Security First

- Instead of complex bash scripts and `.rules` files, SigmaOS tightly integrates device creation with `sigma_security`.
- When a device node is created, it is immediately assigned the proper `SigmaContext` SELinux label natively (e.g., `system_u:object_r:input_device_t:s0`).
- This prevents vulnerabilities where unprivileged users can access hardware before `udev` finishes applying permission rules.

## Displaced Technologies

| Technology | SigmaOS Replacement |
| --- | --- |
| udev / eudev | `sigma_hal::DeviceManager` |
| .rules files | Native Rust match blocks |
| `mdev` | `DeviceManager` / `devtmpfs` logic |

## Status

**Core Absorbed** — The HAL scaffold and Netlink uevent handling structs are established natively in `userland/sigma_hal`.

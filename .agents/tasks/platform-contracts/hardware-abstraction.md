# Hardware Abstraction Contract

> **Shared SigmaOS platform contract.** Define the driver detection, support matrix, and fallback behavior shared across Linux and BSD backends.

---

## Interface

HardwareService { inventory() -> HardwareReport; detect_drivers(hw) -> [DriverRec]; support_status(device) -> SupportLevel; }

## Invariants

- Hardware inventory is non-destructive and read-only.
- Driver recommendations include a confidence score and known-issues list.
- Fallback: if the recommended driver fails, fall back to the previous working driver.
- Support matrix is generated from SigmaOS-docs and reflects tested hardware.

## Support Levels

Supported (driver available + tested), Experimental (driver available, untested), Unsupported (no driver), Unknown (device not in matrix).

## Linux Adapter

sysfs/procfs for inventory; modprobe for kernel modules; udev for hotplug.

## BSD Adapter

devinfo/pciconf for inventory; kldload for kernel modules; devd for hotplug.

## Failure Modes and Recovery

Device not detected (log + mark unknown), driver install fails (rollback to previous), hotplug race (retry with backoff).

## Open Questions

Firmware loading — current: handled by linux-firmware (Linux); needs BSD firmware loader.

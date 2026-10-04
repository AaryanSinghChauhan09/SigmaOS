# Hardware Drivers

**Capability state: Prototype.** Driver source code does not establish usable devices. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository contains driver and hardware abstraction modules, but no booted device discovery path or supported physical-device matrix has been established. USB, Wi-Fi, Ethernet, NVMe, GPU, audio, ACPI, and PCI support are unverified until a specific model is initialized and its operations pass integration tests. This page is the canonical home for driver status across these device classes.

Do not infer compatibility from protocol names, device IDs, structures, mocks, or standalone tests. No general laptop or desktop hardware support is claimed.

## Design references

Linux driver model and PCI subsystem, FreeBSD newbus, NetBSD rump kernels, and Redox driver boundaries are useful references for ownership and isolation. Mint's hardware guidance is a reference for publishing tested models and limitations.

## Roadmap

1. Choose one QEMU machine profile and record its devices.
2. Validate PCI enumeration and one console/input/storage path.
3. Add USB, network, storage, display, and audio drivers individually with bounded resource access, initialization failures, and teardown tests.
4. Publish model, revision, driver path, operations tested, and known limitations for every supported device.

**Completion evidence:** device-specific QEMU/hardware logs, integration tests, error-path results, and a maintained support matrix.

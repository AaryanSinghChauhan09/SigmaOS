---
name: Driver Hardware Submission
about: Propose or submit support for a hardware driver in SigmaOS
title: '[DRIVER]: '
labels: 'driver, hardware'
assignees: ''
---

**Device Information**
- **Device Name**:
- **Vendor ID & Product ID**:
- **Bus Type**: (PCIe, USB, ISA, VirtIO, ACPI)
- **Target Hardware / QEMU Device**:

**WDM-Style Architecture Details**
- **DriverObject Name**:
- **DeviceObject Extension Details**:
- **Paged vs NonPaged Memory Needs**:
- **Interrupt / DMA Handling Requirements**:

**Lifecycle Unit Test Plan**
Describe how driver initialization, device attach/detach, and teardown are verified in standalone tests.

**Reference Specifications**
Include links to datasheet, open-source driver reference (Linux/BSD/Tock), or specification docs.

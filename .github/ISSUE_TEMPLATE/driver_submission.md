name: Hardware Driver Submission / Request
description: Request or submit support for a new hardware device or virtualized controller.
title: "[DRIVER]: "
labels: ["driver", "hardware"]
body:
  - type: markdown
    attributes:
      value: |
        Use this template when requesting or submitting a driver for hardware devices or virtual controllers in SigmaOS.

  - type: textarea
    id: hardware_info
    attributes:
      label: Hardware / Device Details
      description: Specify vendor ID, device ID, bus architecture (PCI, USB, ISA, VirtIO, MMIO), and target hardware.
      placeholder: e.g. Intel I225-V PCIe 2.5GbE NIC (8086:15F3)
    validations:
      required: true

  - type: textarea
    id: driver_architecture
    attributes:
      label: Driver Architecture
      description: Describe the implementation conforming to DriverObject, DeviceObject, and DeviceExtension lifecycles.
    validations:
      required: true

  - type: checkboxes
    id: checklists
    attributes:
      label: Driver Safety & Lifecycle Requirements
      options:
        - label: "#![no_std] zero external dependency compliance"
          required: true
        - label: Explicit NonPagedPool memory allocation for DMA/MMIO descriptors
          required: true
        - label: Bounds-checked DMA memory transfers
          required: true
        - label: Standalone lifecycle unit test included (Create, Attach, Detach, Destroy)
          required: true

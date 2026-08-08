name: "🔌 Driver & Hardware Support Proposal"
description: "Propose a new hardware device driver or polymorphic bus adapter under the Sovereign Driver Framework (SDF)."
title: "[Driver]: <Identify target peripheral (e.g. Wi-Fi, NPU, GPU)>"
labels: ["driver", "hardware-enablement", "needs-review"]
body:
  - type: markdown
    attributes:
      value: |
        Use this template to propose addition or translation support for physical or virtual hardware peripherals, ensuring adherence to our sandboxed User-Mode Driver Ring (UMDR) guidelines.
  - type: textarea
    id: hardware-details
    attributes:
      label: "📟 Hardware details & Specifications"
      description: "Specify the exact hardware family, bus interfaces (PCIe, CXL, ISA, USB), and target vendor/device IDs."
      placeholder: "e.g., Realtek RTL8111 PCI Express Gigabit Ethernet (Vendor 0x10EC, Device 0x8168)"
    validations:
      required: true
  - type: dropdown
    id: device-era
    attributes:
      label: "⏳ Device Era / Classification"
      description: "Is this legacy/ancient hardware or cutting-edge/accelerator?"
      options:
        - "Ancient Hardware (Requires Legacy Emulation Box / Port I/O)"
        - "Modern Hardware (Requires User-Mode Driver Rings / MMIO / MSI-X)"
        - "Emerging Accelerator (Google TPU, Graphcore IPU, FPGAs, CXL Pools)"
    validations:
      required: true
  - type: textarea
    id: isolation-policies
    attributes:
      label: "🛡️ Safety & Isolation Guardrails"
      description: "How is memory protection and DMA isolation enforced? Specify IOMMU maps or volatile double-buffer allocations."
      placeholder: "Describe how malicious DMA is blocked and driver hangs are caught via watches."
    validations:
      required: true
  - type: textarea
    id: implementation-steps
    attributes:
      label: "🚀 Implementation Steps & Milestones"
      description: "Detail the development steps, from basic PCI BAR probes to full buffer packet loops."
    validations:
      required: true

# VFIO PCI Passthrough & IOMMU Virtualization Component Guide

## Component Overview
Governs Intel VT-d and AMD IOMMU page table creation, PCI device isolation into VFIO groups, MSI-X interrupt forwarding, and bare-metal GPU passthrough to virtual machines.

## Linux Inspiration
Linux VFIO (Virtual Function I/O) driver, KVM PCI passthrough, and QEMU vfio-pci device model.

## BSD Inspiration
FreeBSD bhyve passthru PCI configuration, OpenBSD vmm device emulation.

## Current SigmaOS Status
IOMMU capability detection and PCI device group enumeration implemented. VFIO container memory mapping in progress.

## Critical Missing Features
Complete IOMMU second-level page table mapping, dynamic host driver unbinding and rebinding, and virtual interrupt injection.

## Implementation Priority
MEDIUM (Phase 3)

## Key Files to Create/Improve
src/virt/vfio.rs, src/virt/iommu.rs, src/hardware/pci.rs

## Testing Strategy
Virtual device DMA isolation tests, simulated IOMMU fault containment preventing host memory corruption.

## Dependencies
VIRTUALIZATION_AGENT, HARDWARE_HAL_DRIVERS_AGENT, MEMORY_MANAGEMENT_AGENT

## Success Criteria
99.5% bare-metal GPU compute and gaming performance inside sandboxed guest virtual machines, total host isolation.

## Open Source Competitors Analysis
Offers seamless GPU passthrough without the complex kernel boot flags (`intel_iommu=on`, `vfio-pci.ids=...`) required in Linux.

## Future Enhancements
Single Root I/O Virtualization (SR-IOV) virtual GPU slicing sharing one discrete GPU across multiple microVMs.

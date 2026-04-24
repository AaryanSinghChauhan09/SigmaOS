
# Autonomous Driver Builder


The Autonomous Driver Builder (`modules/ext/plugins/auto_driver_builder.c`) is a wildcard concept realizing SigmaOS's goal of a self-organizing system.

Instead of writing thousands of lines of C code for every possible PCI or USB device, the kernel can auto-generate a generic driver using hardware metadata.


## Mechanism

1. Hardware Auto-Detection (`hw_detect.c`) scans the bus and generates a `hw_metadata_t` descriptor (Vendor ID, Device ID, MMIO address, IRQ number).
2. It passes this descriptor to `build_autonomous_driver()`.
3. The kernel dynamically creates a driver structure, binding generic memory-mapped I/O (MMIO) read/write hooks to the physical addresses specified in the metadata.
4. The driver is immediately registered as an active Service Capsule and its creation is logged in the Tamper-Proof Audit Chain.

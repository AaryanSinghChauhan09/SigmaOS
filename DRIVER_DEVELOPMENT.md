# Driver Development Guide

SigmaOS implements a userspace driver architecture inspired by Linux DDE (Device Driver Environment) and FreeBSD NetGraph frameworks.

## Driver Lifecycle

1.  **PCI Discovery**: Devices are discovered via the PCI controller daemon.
2.  **Resource Allocation**: Physical memory mapping (MMIO) and interrupts are mapped through Capabilities.
3.  **Execution Sandbox**: The driver runs inside an isolated OCI-compliant container process.
4.  **Self-Healing (TDR)**: If a driver fails, a daemon detects the heartbeat timeout and resets the device without kernel panic.\n

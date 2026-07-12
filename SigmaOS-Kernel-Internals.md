# SigmaOS Zenith: Advanced Kernel Internals (v15.2)

To push SigmaOS Zenith to industrial maturity, we have introduced a final series of advanced internal abstractions. These logic gates directly emulate the resilience, hardware traversal, and cryptographical frameworks seen in production kernels like Linux and FreeBSD, completely severing any remaining OS dependency.

All code is natively compiled in ISO C11.

---

## 23. Cryptographic API (Crypto Core)

**Inspirations:** Linux `crypto/aes_generic.c`, `crypto/sha256_generic.c`
**Implementation:** `kernel/core/security/sigma_crypto.c`

A modular kernel cryptography provider bypassing external libraries (e.g. OpenSSL). Implements stateful block ciphers and hashing algorithms (AES-256 and SHA-256 stubs) necessary for securing sovereign memory shards and validating cryptographic signatures internally.

## 24. Out-of-Memory (OOM) Killer

**Inspirations:** Linux `mm/oom_kill.c`, FreeBSD `vm_pageout.c`
**Implementation:** `kernel/core/mem/sigma_oom.c`

A ruthless survival mechanism identical to the Linux OOM killer. When physical memory is critically exhausted, it calculates a `badness` score for every running process (weighing total RAM consumed against the `oom_score_adj` heuristic) and deterministically terminates the heaviest users to prevent a kernel panic. Kernel threads are explicitly immune.

## 25. IPv4 Routing Table (FIB)

**Inspirations:** Linux `net/ipv4/fib_trie.c`, FreeBSD `in_rmx.c`
**Implementation:** `net/routing.c`

Implements a Longest Prefix Match (LPM) algorithm for routing IPv4 traffic. Defines the core Forwarding Information Base (FIB) where gateways, netmasks, and interface metrics are resolved to dictate packet traversal, removing the need for `iproute2` user-space routing daemon dependencies.

## 26. PCI / PCIe Bus Enumerator

**Inspirations:** Linux `drivers/pci/probe.c`, FreeBSD `pci.c`
**Implementation:** `kernel/core/hardware/sigma_pci.c`

A low-level hardware discovery subsystem mapping the PCI Configuration Space. Automatically probes buses, slots, and functions to resolve Vendor IDs, Device IDs, and Class Codes, building the internal hardware tree required before module initialization.

## 27. USB Core Subsystem (HCI)

**Inspirations:** Linux `drivers/usb/core/usb.c`, FreeBSD `usb_core.c`
**Implementation:** `kernel/core/hardware/sigma_usb.c`

Establishes the USB state machine (Attached, Powered, Default, Address, Configured). Manages the enumeration of device endpoints, speeds (Low, Full, High, Super), and descriptor parsing independent of the underlying Host Controller Interface (UHCI/EHCI/xHCI).

## 28. Real-Time Clock (RTC) / CMOS

**Inspirations:** Linux `drivers/rtc/rtc-cmos.c`, FreeBSD `acpi_rtc.c`
**Implementation:** `kernel/core/system/sigma_rtc.c`

Interfaces directly with port `0x70/0x71` to read the legacy PC CMOS clock. Automatically parses BCD/Binary formatting and converts the raw hardware date vectors (Year, Month, Day, Hour, Minute, Second) into standard UNIX Epoch Time for kernel timestamping.

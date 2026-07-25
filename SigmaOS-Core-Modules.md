# SigmaOS Zenith: Advanced Core Modules (v15.2)

To close the final gaps between the SigmaOS Zenith microkernel and industrial monoliths like Linux and FreeBSD, we have implemented six final advanced abstraction modules. These sub-systems empower the kernel with dynamic introspection, modern I/O interfaces, and asynchronous sandboxing.

All components adhere strictly to ISO C11.

---

## 29. ACPI Table Parser
**Inspirations:** Linux `drivers/acpi/tables.c`, FreeBSD `OsdTable.c`
**Implementation:** `kernel/core/hardware/sigma_acpi.c`

Locates the Advanced Configuration and Power Interface (ACPI) Root System Description Pointer (RSDP) in physical memory. It parses the RSDT/XSDT headers to enumerate critical hardware topologies, including the MADT (Multiple APIC) and HPET (Timers), mirroring the `acpica` framework.

## 30. Flattened Device Tree (FDT) Parser
**Inspirations:** Linux `drivers/of/fdt.c`, FreeBSD `fdt_common.c`
**Implementation:** `kernel/core/hardware/sigma_fdt.c`

Reads the Flattened Device Tree blobs provided by bootloaders (like U-Boot) to enumerate non-discoverable memory-mapped peripherals. This brings full native support for ARM SoCs and RISC-V architectures that do not possess PCI enumeration.

## 31. NVMe Host Controller Driver (Stub)
**Inspirations:** Linux `drivers/nvme/host/core.c`, FreeBSD `nvme.c`
**Implementation:** `kernel/core/hardware/sigma_nvme.c`

Provides the foundational PCIe MMIO bindings for Non-Volatile Memory Express storage. Implements the structures for the Admin Submission Queue (SQ) and Completion Queue (CQ) required to dispatch high-throughput I/O commands to NVMe hardware.

## 32. BPF (Berkeley Packet Filter) Virtual Machine
**Inspirations:** Linux `kernel/bpf/core.c`, FreeBSD `bpf_filter.c`
**Implementation:** `kernel/core/bpf/sigma_bpf.c`

An in-kernel sandbox VM executing simplified eBPF-style bytecodes. Allows user-space to upload highly optimized filtering scripts directly to the network data plane, evaluating accumulator state and jumping offsets safely without kernel panics.

## 33. Netlink Sockets (IPC)
**Inspirations:** Linux `net/netlink/af_netlink.c`
**Implementation:** `net/netlink.c`

A specialized socket family dedicated to kernel-to-user space IPC. Handles critical async routing events (`NETLINK_ROUTE`) and hardware hotplug uevents (`NETLINK_KOBJECT_UEVENT`), completely bypassing legacy `/proc` polling for modern dynamic daemons.

## 34. Inotify (Filesystem Event Notification)
**Inspirations:** Linux `fs/notify/inotify/inotify_user.c`, FreeBSD `kqueue`
**Implementation:** `fs/inotify.c`

An asynchronous monitoring subsystem that allows user-space programs to watch individual inodes or directories for specific `IN_CREATE`, `IN_DELETE`, or `IN_MODIFY` events. These are pushed to a non-blocking queue, preventing inefficient directory polling.

# SigmaOS AI Agents Memory Ballooning Management & Overcommit Guide

Welcome to the **SigmaOS AI Agents Memory Ballooning Management Guide**. This document details VirtIO memory ballooning, guest RAM inflation and deflation, hypervisor overcommit management, and dynamic memory reclamation for autonomous AI agents and virtualization engineers in SigmaOS.

---

## 1. Memory Ballooning Architecture Overview

Memory ballooning in SigmaOS (`src/virtualization/vm_manager.rs`, `src/kernel/classic_os.rs`) enables dynamic memory reallocation between host hypervisors (KVM, Xen, QEMU, Firecracker) and guest MicroVM instances without requiring reboot:

### Key Components
1. **VirtIO Memory Ballooning Driver**: Paravirtualized balloon driver (`virtio-balloon`) communicating over virtqueues.
2. **Inflation Protocol**: Steals unallocated guest physical RAM pages and yields them back to the host hypervisor during host memory pressure.
3. **Deflation Protocol**: Returns physical RAM pages from the host back to the guest virtual machine when guest workload memory demand surges.
4. **Step-Based Adjustments**: Incremental 256MB inflation/deflation steps to prevent guest kernel memory allocation panics.

---

## 2. API & Virtualization Controls

AI agents managing virtual machines or containerized helpers can configure memory balloon targets using `VmManager` (`src/virtualization/vm_manager.rs`):

```rust
use sigmaos::virtualization::vm_manager::VmManager;

let mut vm_mgr = VmManager::new();
let vm_id = "agent_microvm_01";

// Set target memory balloon size to 2048 MB
vm_mgr.set_memory_balloon(vm_id, 2048).expect("Failed to set memory balloon target");

// Verify new balloon target in VM configuration
let config = vm_mgr.get_vm_config(vm_id).expect("VM config not found");
assert_eq!(config.memory_balloon_mb, 2048);
```

### Low-Level VirtIO Balloon Manager (`src/kernel/classic_os.rs`)
```rust
use sigmaos::kernel::classic_os::VirtioBalloonManager;

let mut balloon = VirtioBalloonManager::new(1024); // Initial 1024 pages
balloon.set_target_pages(2048);

// Inflate balloon with free guest pages
let inflated_count = balloon.inflate(&[100, 101, 102, 103]);
assert_eq!(inflated_count, 4);

// Deflate balloon to reclaim pages for guest kernel
let deflated_pages = balloon.deflate(2);
assert_eq!(deflated_pages.len(), 2);
```

---

## 3. Autonomous AI Agent Memory Management Policies

Autonomous agents monitoring system memory overcommit MUST follow these operational guidelines:

1. **Host Memory Pressure Response**: When host RAM utilization exceeds 85%, trigger balloon inflation across idle guest MicroVMs.
2. **Guest Out-Of-Memory Prevention**: If guest kernel swap usage or OOM score elevates, deflate the balloon immediately to return physical RAM to the guest.
3. **Smooth Step Adjustment**: Adjust balloon sizes in 256 MB or 512 MB steps rather than abrupt multi-gigabyte changes.

---

## 4. Checklist for AI Agents Managing Memory Ballooning

- [ ] Confirmed target balloon size does not drop below guest minimum kernel footprint (256 MB).
- [ ] Verified VirtIO balloon virtqueue ring buffers release page descriptors properly.
- [ ] Tested inflation and deflation cycles under simulated host memory overcommit.
- [ ] Executed `./run_sigma_tests.sh` to confirm virtualization and kernel test suites pass cleanly.

# SigmaOS: Advanced Performance & Virtualization Roadmap

This document outlines the roadmap for introducing hardware enclaves, next-generation filesystems, and micro-virtualization techniques into SigmaOS.

## Target Repositories for Absorption

1. **`intel/linux-sgx`**
   - **Goal:** Support secure hardware enclaves to execute sensitive workloads.
   - **SigmaOS Integration:** Bridge SGX enclave initialization and EENTER/EEXIT logic directly into the scheduler, matching capability checks to enclave entries.

2. **`openzfs/zfs` & `btrfs/btrfs-progs`**
   - **Goal:** Copy-on-Write storage, snapshots, and software RAID.
   - **SigmaOS Integration:** Map ZFS pool allocation algorithms and Btrfs extent mapping patterns to our `sigmafs.rs` kernel driver.

3. **`firecracker-microvm/firecracker`**
   - **Goal:** Ultra-fast, minimal virtualization.
   - **SigmaOS Integration:** Interface Firecracker's lightweight VMM concepts (using KVM APIs) into a specialized VM capability driver inside the kernel.

### Last Updated: July 2026

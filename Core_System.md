# Core System Roadmap & Architecture Spec

## 1. Kernel Strategy & LTS Baseline
SigmaOS implements a sovereign `no_std` Rust microkernel architecture designed to achieve zero-dependency safety, low latency, and a minimal Trusted Computing Base (TCB). Rather than inheriting the legacy bloat of standard monolithic kernels, SigmaOS maintains a strict boundaries policy.

### Core Primitives
- **Scheduler**: Preemptive Round-Robin scheduler (`sigma_rr_sched.rs`) with O(1) task enqueue/dequeue.
- **Physical Memory**: Buddy physical allocator (`sigma_buddy_alloc.rs`) for coarse page grouping.
- **Virtual Memory**: 4-level page table manager (`sigma_vmm.rs`) protecting user/kernel boundaries.
- **Slab Memory**: Cache allocator (`sigma_slab_alloc.rs`) for fast, fragmented heap reclamation.

## 2. Hardware Compatibility List (HCL) Strategy
We reject blind driver insertion. Drivers must be written in bare-metal Rust with zero C dependencies.
- **Tier 1 (Fully Supported)**: Lenovo ThinkPad T14 (Gen 3/4), Framework Laptop 13, Dell XPS 13.
- **Virtualization Target**: QEMU/KVM with virtio-gpu, virtio-net, and virtio-blk.
- **Network Interface**: Intel Gigabit Ethernet (`sigma_e1000.rs`) and Basic Intel Wi-Fi.

## 3. Driver Roadmap
- **Phase 1 (0–3m)**: Stabilize memory-mapped I/O, VirtIO networking, and basic graphics framing.
- **Phase 2 (3–6m)**: Adapt Open-Source Nouveau and basic Intel/AMD KMS display controllers.
- **Phase 3 (6–9m)**: Launch the Driver Bounty Program to write community USB/PCI controllers in Rust.
- **Phase 4 (9–12m)**: Implement peripheral USB printer and Wi-Fi 6/7 adapters.

## 4. Contributor Guidelines
- No imports of standard library (`no_std` only).
- Keep code clean of unsafe blocks unless interfacing directly with memory-mapped register regions.
- Document all hardware register definitions in a `registers.toml` specification.

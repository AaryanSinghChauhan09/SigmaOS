# SigmaOS Subsystem Audit & Gap Analysis vs Linux Variants

**Status:** v15.0.0 Zenith
**Date:** 2026-07-04
**Purpose:** Identify missing subsystems, drivers, and features by comparing SigmaOS against 50+ Linux kernel variants.

---

## Executive Summary

SigmaOS is **architecturally sound but feature-starved** at Phase 0. When compared to even minimal Linux variants:

| Category | SigmaOS Status | Linux Equivalents | Criticality |
|---|---|---|---|
| **Scheduler** | Round-Robin (stub) | CFS, EDF, deadline | **BLOCKING** |
| **Memory Allocator** | Slab (TBD) | Buddy + Slab | **BLOCKING** |
| **Syscalls** | 30 planned | 400+ | **BLOCKING** |
| **Device Drivers** | e1000, virtio-net | e1000, RTL8111, r8169, iwlwifi, mt7921, i915, amdgpu | **Phase 1-2** |
| **Filesystems** | VFS layer only | ext4, btrfs, F2FS, NTFS3, Overlay | **Phase 3** |
| **Network Stack** | TCP/UDP/IPv6 (Rust) | TCP, UDP, SCTP, mptcp | **Phase 1** |
| **GPU/Graphics** | VGA framebuffer | DRM, KMS, VFIO, virtio-gpu, i915, amdgpu | **Phase 2** |
| **Virtualization** | Container stub | KVM, QEMU, VMX/SVM | **Phase 3** |
| **Security** | PQC TLS + AppArmor | SELinux, seccomp, Landlock, BPF | **Phase 4** |

---

## Part 1: Current SigmaOS Subsystems (What Exists)

### Kernel Core (kernel/core/)

- ✅ **Scheduler** (`sovereign_scheduler.rs`): Placeholder scheduler, not production-ready

- ✅ **Memory Management** (`memory/sovereign_allocator.rs`): Slab allocator, needs stress testing

- ✅ **Syscall Layer** (`syscall/gate.rs`, `sovereign_syscalls.rs`): ~30 syscalls, non-POSIX ABI

- ✅ **IPC** (`syscall/ipc.rs`): Message queue stub

- ✅ **HAL** (`hal/`): x86_64 IDT, RISCV stub, MMIO/port I/O in Zig

- ✅ **Interrupts** (IRQ handler stubs)

- ✅ **Process Management** (`sovereign_process_manager.rs`)

- ✅ **Namespace & Isolation** (`sovereign_namespace.rs`)

### Drivers (drivers/)

- ✅ **e1000** (C/C++ compat shim): Ethernet NIC

- ✅ **VirtIO-net** (Rust): Paravirtual Ethernet

- ✅ **NVMe** (Rust stub): `drivers/sigma/nvme.rs` — not fully implemented

- ✅ **WiFi** (Rust stub): `drivers/sigma/wifi.rs` — not fully implemented

- ✅ **USB** (Rust stub): `drivers/sigma/usb.rs` — not fully implemented

- ✅ **HAL/MMIO** (`drivers/hal/mmio.rs`, `port_io.zig`): Low-level I/O

### Network (kernel/net/)

- ✅ **TCP/UDP Stack** (Rust): `net/net_stack.rs`, `socket.rs`

- ✅ **Firewall** (Rust stub): `net/firewall.rs`

- ✅ **TLS 1.3** (C++): `net/tls/sigma_tls.cpp` with PQC (Kyber-1024 hybrid)

- ✅ **DNS** (C++): `net/dns/sigma_dns.cpp`, DoH support

- ✅ **DHCP** (C++): `net/dhcp/sigma_dhcp.cpp` RFC 2131/2132

- ✅ **WPA3/SAE** (C++): `net/wifi/sigma_wpa3.cpp` IEEE 802.11-2020

### Storage (kernel/storage/)

- ✅ **VFS layer** (Rust stub): `kernel/storage/mod.rs`, `shard.rs`

- ✅ **Block device abstraction**

- ⚠️ **No actual filesystems** (ext4-like, btrfs-like planned for Phase 3)

### Virtualization (kernel/virt/) — *If SIGMA_USE_HYPERVISOR=ON*

- ✅ **Hypervisor stub** (`hypervisor.rs`): Container runtime concept

- ✅ **vCPU manager** (`vcpu.rs`): Process isolation

- ✅ **Container runtime** (`container.rs`): Namespacing placeholder

### Security (security/)

- ✅ **AppArmor-like MAC** (`sovereign_sandbox_mac.rs`): Capability-based

- ✅ **PQC Crypto** (Kyber-1024, Dilithium-5): FIPS 203/204 final

- ✅ **Secure boot** (TPM2 support planned)

### Userland (userland/)

- ✅ **sigpkg** (Rust): Package manager

- ✅ **sigma-sh** (Rust): Shell

- ✅ **sigma-agent** (Nim): Workflow automation, plugin system

- ✅ **coreutils** (Rust): Basic utilities

---

## Part 2: Critical Gaps vs Linux Variants

### **PHASE 0 BLOCKERS** (Must complete before Phase 1)

#### 1. **Scheduler** — Inadequate

**Current:** Round-robin scheduler stub in `sovereign_scheduler.rs`
**Problem:** No priority levels, no preemption tuning, no latency guarantees

**Linux Ref:** `kernel/sched/core.c` (15K+ lines)

- CFS (Completely Fair Scheduler) with O(log N) complexity

- EDF (Earliest Deadline First) for real-time

- CPU affinity, load balancing, preemption classes

**Reference:** `torvalds/linux` scheduler patterns:
```c
// From kernel/sched/core.c
struct task_struct {
    int prio;                    // Priority (0-139)
    int rt_priority;             // RT priority
    enum preempt_active preempt; // Preemption state
    struct sched_class *sched_class;  // Scheduler class (fair, rt, deadline)
    struct rb_node run_node;     // RB tree for runqueue
    u64 vruntime;                // Virtual runtime (CFS)
};

// Runqueue abstraction
struct rq {
    struct rb_root tasks_timeline;  // All runnable tasks
    struct task_struct *curr;       // Current task
    unsigned long nr_running;
    unsigned long rt_avg;
};
```

### Recommendation:

- [ ] Implement **EDF scheduler** (deadline-first) for hard real-time (Phase 1)

- [ ] Add **priority levels** (0-255: kernel, 128-255: user)

- [ ] CPU affinity & **NUMA-aware** scheduling

- [ ] Preemption counter + IRQ nesting tracking

---

#### 2. **Memory Allocator** — Stress-test Incomplete

**Current:** Slab allocator stub in `kernel/core/memory/sovereign_allocator.rs`
**Problem:** No stress testing, no fragmentation metrics, no reclamation

**Linux Ref:** `mm/page_alloc.c` (8K lines) + `mm/slab.c` (5K lines)

- Buddy allocator: O(log n) page allocation/deallocation

- Per-CPU page caches (pcp lists)

- Zone-aware allocation (DMA, Normal, HighMem)

- Page reclaim (kswapd), compaction

### Key patterns from Linux:

```c
// From mm/page_alloc.c
struct zone {
    unsigned long pages_scanned;
    struct free_area free_area[MAX_ORDER];  // Buddy allocator
    struct list_head *pageset;              // Per-CPU caches
};

struct per_cpu_pages {
    int count;       // Pages in cache
    int high;        // Refill threshold
    int batch;       // Batch size
    struct list_head lists[NR_PCP_LISTS];
};

// Buddy allocator split/merge
static void __free_one_page(struct page *page, unsigned long pfn,
        struct zone *zone, unsigned int order, int migratetype, fpi_t fpi_flags)
{
    // Merge adjacent buddy blocks recursively
    struct page *buddy = find_buddy_page(page, order);
    if (buddy && page_is_buddy(buddy)) {
        merge_buddy(page, buddy, order + 1);
    }
}
```

### Recommendation:

- [ ] Implement **full Buddy allocator** with split/merge in Rust

- [ ] Per-CPU page caches (PCP)

- [ ] Zone-aware allocation (at least DMA + Normal)

- [ ] Stress test suite: `fuzz_allocator`, `bench_fragmentation`

- [ ] Memory pressure detection & kswapd-like reclaim

---

#### 3. **System Call Table** — Only ~30 Syscalls

**Current:** `kernel/syscalls/sovereign_syscalls.rs` with ~30 calls
**Problem:** Missing critical syscalls for POSIX/Linux compat

**Linux Ref:** 400+ syscalls across x86_64, ARM, RISC-V

### Critical missing:

- File I/O: `open`, `close`, `read`, `write`, `lseek`

- Process: `fork`, `exec`, `clone`, `wait`, `exit`

- Memory: `mmap`, `munmap`, `brk`

- Signals: `signal`, `sigaction`, `kill`

- Networking: `socket`, `bind`, `listen`, `accept`, `connect`

- IPC: `pipe`, `msgget`, `shmget`

- FS: `mount`, `umount`

- Device: `ioctl`

### Recommendation:

- [ ] Implement POSIX subset (200 syscalls minimum)

- [ ] Reference: `torvalds/linux arch/x86/entry/syscalls/syscall_64.tbl` (440 lines)

- [ ] Versioning strategy (v1, v2, v3 for backward compat)

- [ ] Trace/audit syscall entry points

---

### **PHASE 1 GAPS** (Blocks networking & real hardware boot)

#### 4. **Network Drivers** — Only e1000 + VirtIO

### Current:

- e1000 (legacy, 82543-82576 era)

- VirtIO-net (QEMU only)

### Linux Variants Have (Pick 3-5 key ones):

- **Realtek RTL8111** (`r8169` driver): Cheap, ubiquitous

- **iwlwifi** (`intel/iwlwifi/`): Intel WiFi, 802.11ac/ax

- **MediaTek MT7921** (`mediatek/mt7921e/`): Cheap mobile WiFi

- **Mellanox ConnectX** (`mlx4`, `mlx5`): High-performance

- **Broadcom BCM43xx** (`b43`, `brcmsmac`): Embedded WiFi

### Reference pattern (RTL8111 from AsahiLinux/linux):

```c
// drivers/net/ethernet/realtek/8169.c
struct rtl8169_private {
    void __iomem *mmio_addr;          // Memory-mapped I/O base
    struct pci_dev *pdev;
    struct napi_struct napi;
    struct sk_buff *rx_skbuff[NUM_RX_DESC];
    struct TxDesc tx_ring[NUM_TX_DESC];
};

static int rtl_open(struct net_device *dev) {
    // 1. Reset MAC, enable RX/TX
    // 2. Allocate RX descriptors + buffers
    // 3. Set MAC address
    // 4. Enable interrupts
}

static void rtl_tx(struct net_device *dev, struct sk_buff *skb) {
    // 1. Get next TX descriptor
    // 2. DMA map skb
    // 3. Fill descriptor (addr, len, ownership flag)
    // 4. Trigger TX in NIC via register write
}
```

### Recommendation:

- [ ] **RTL8111 driver** (Rust): 1500 LOC, covers 80% of cheap laptops

- [ ] **WPA3/WiFi association** refinement

- [ ] PCI device probing (vendorID/deviceID matching)

- [ ] DMA setup for RX/TX rings

- [ ] NAPI polling (not interrupt-driven for every packet)

---

#### 5. **Storage Drivers** — No NVMe, SATA, ATA

**Current:** VFS layer only, no block drivers

### Linux Variants Have:

- **NVMe** (`drivers/nvme/host/`): PCIe SSDs (most common now)

- **AHCI** (`drivers/ata/libahci.c`): SATA/SSD storage

- **mmc/SD** (`drivers/mmc/host/`): eMMC, microSD cards

### Reference (NVMe from torvalds/linux):

```c
// drivers/nvme/host/pci.c
struct nvme_queue {
    struct nvme_dev *dev;
    spinlock_t sq_lock;
    struct nvme_command *sq_cmds;    // Submission queue
    volatile struct nvme_completion *cqes;  // Completion queue
    dma_addr_t sq_dma_addr, cq_dma_addr;
};

static void nvme_submit_cmd(struct nvme_queue *nvmeq, struct nvme_command *cmd)
{
    // 1. Copy cmd to submission queue tail
    // 2. Write doorbell register to NIC (signals new commands)
    // 3. Wait for completion in IRQ handler
}
```

### Recommendation:

- [ ] **NVMe driver** (Rust): 2000 LOC, covers 90% of modern machines

- [ ] **AHCI/SATA fallback** (Rust): For older hardware

- [ ] Interrupt-driven + polling modes

- [ ] Error recovery (abort, reset, namespace loss)

---

#### 6. **GPU/Graphics** — Only VGA Framebuffer

**Current:** VESA/GOP framebuffer at 1024×768

### Linux Variants Have:

- **DRM/KMS** (Direct Rendering Manager): Unified graphics subsystem

- **i915** (Intel): UHD/Iris integrated GPUs

- **amdgpu** (AMD): RDNA/VEGA discrete

- **virtio-gpu** (QEMU): Paravirtual display

- **VFIO** (passthrough): Physical GPU assignment to VMs

### Reference (virtio-gpu driver simplified from torvalds/linux):

```c
// drivers/gpu/drm/virtio/virtgpu_drm_driver.c
struct virtio_gpu_device {
    struct virtio_device *vdev;
    struct virtqueue *controlq;
    struct sg_table vbufs;
    struct work_struct config_changed_work;
};

// Create scanout buffer (for display)
static int virtio_gpu_mode_set(struct drm_crtc *crtc,
        struct drm_display_mode *mode)
{
    struct virtio_gpu_cmd_set_scanout cmd = {
        .hdr.type = cpu_to_le32(VIRTIO_GPU_CMD_SET_SCANOUT),
        .scanout_id = crtc->index,
        .resource_id = buffer->id,
        .rect.width = mode->hdisplay,
        .rect.height = mode->vdisplay,
    };
    virtio_gpu_queue_ctrl_buffer(device, &cmd);
}
```

### Recommendation:

- [ ] **virtio-gpu driver** (Rust): 1000 LOC, QEMU display passthrough

- [ ] **DRM/KMS minimal layer** (Rust): CRTC, encoder, connector abstraction

- [ ] **i915 basic support** (Phase 2): Polling mode (no interrupts yet)

- [ ] VFIO stubs (Phase 3)

---

#### 7. **USB/HID** — Stubs Only

**Current:** `drivers/sigma/usb.rs` — non-functional

### Linux Variants Have:

- **USB core** (`drivers/usb/core/`): Host controller, device enumeration

- **USB keyboard/mouse** (`drivers/usb/input/`): HID protocol

- **USB mass storage** (`drivers/usb/storage/`): External drives

### Reference (USB keyboard driver, simplified):

```c
// drivers/usb/input/usbkbd.c
struct usb_kbd {
    struct urb *urb;                      // USB request block
    unsigned char data[32];               // Interrupt buffer
    struct hid_device *hid;               // Input device
};

static void usb_kbd_irq(struct urb *urb)
{
    struct usb_kbd *kbd = urb->context;

    // Parse HID report from interrupt endpoint
    // Decode modifier keys + key code
    // Report to input subsystem
    hid_input_report(kbd->hid, HID_INPUT_REPORT, kbd->data, 8, 0);

    // Resubmit URB for next interrupt
    usb_submit_urb(urb, GFP_ATOMIC);
}
```

### Recommendation:

- [ ] **USB core** (xHCI controller): 1500 LOC

- [ ] **HID keyboard** (500 LOC): Polling mode first

- [ ] **HID mouse** (500 LOC): Same

- [ ] **Mass storage** (Phase 2)

---

### **PHASE 2 GAPS** (Desktop & userland)

#### 8. **Desktop Environment** — Zenith Compositor Planned

**Current:** `desktop/` placeholder

### Linux Desktop References:

- **Wayland** (weston, GNOME/Wayland): Modern display server

- **X11** (xorg): Legacy but ubiquitous

- **Compositor** (Weston): Window manager + renderer

### Recommendation:

- [ ] Minimal Wayland compositor (Rust)

- [ ] GTK/Qt binding layer (Nim)

- [ ] Window manager (sovereign-wm)

---

#### 9. **Audio** — Not Started

**Current:** Not implemented

### Linux References:

- **ALSA** (`sound/core/`): Low-level audio

- **PulseAudio**: Daemon + server

- **JACK**: Professional audio

### Recommendation:

- [ ] Simple audio mixer (Phase 2 placeholder)

- [ ] HDA (High Definition Audio) driver (Phase 2)

---

### **PHASE 3+ GAPS** (Filesystems, virtualization, security hardening)

#### 10. **Filesystems** — VFS Layer Only

**Current:** `kernel/storage/` is abstract; no actual FS implementation

### Linux Variants Offer:

- **ext4**: Journaled, stable, POSIX

- **btrfs**: Copy-on-write, snapshots, RAID

- **F2FS**: Flash-optimized, wear leveling

- **NTFS3** (from Paragon-Software-Group/linux-ntfs3): Windows compat

- **Overlay**: Layered FS (container use)

### Reference (ext4 simplified from torvalds/linux):

```c
// fs/ext4/super.c
struct ext4_sb_info {
    unsigned long s_blocks_per_group;
    unsigned long s_inodes_per_group;
    struct ext4_group_desc *s_group_desc;
    struct buffer_head **s_sbh;
};

// Read inode from disk
static struct inode *ext4_iget(struct super_block *sb, unsigned long ino)
{
    // 1. Compute block group: ino / inodes_per_group
    // 2. Read inode from disk: offset = (ino % inodes_per_group) * inode_size
    // 3. Populate inode cache
    // 4. Return inode to VFS
}
```

### Recommendation:

- [ ] **Minimal ext4 support** (Phase 3): Read-only first

- [ ] **Btrfs stub** (Phase 3): Snapshots for container isolation

- [ ] **NTFS3 binding** (Phase 3): For interop

---

#### 11. **Virtualization & Containers** — Stubs Only

**Current:** `kernel/virt/` has placeholder hypervisor, container, vCPU

### Linux Variants Offer:

- **KVM** (`arch/x86/kvm/`): Type-2 hypervisor, VMX/SVM

- **QEMU** (external): Guest OS runner

- **LXC/systemd-nspawn**: Lightweight containers

- **cgroup** (`kernel/cgroup/`): Resource isolation

### Recommendation:

- [ ] **Container namespaces** refinement (Phase 3)

- [ ] **cgroup v2 support** (Phase 3)

- [ ] **KVM stubs** (Phase 4)

---

#### 12. **Security Hardening** — PQC Good, Exploit Mitigations Missing

**Current:** PQC TLS (Kyber-1024 + Dilithium-5), AppArmor-like MAC

### Linux Variants Offer (from landlock-lsm/linux, samitolvanen/linux):

- **Landlock** (`security/landlock/`): Unprivileged sandboxing

- **SELinux**: Role-based access control

- **seccomp**: Syscall filtering

- **CFI** (Control Flow Integrity): Indirect call protection

- **Shadow stack**: Return address verification

- **Stack canaries**: Buffer overflow detection

- **FORTIFY_SOURCE**: Compile-time string checks

### Reference (Landlock from landlock-lsm/linux):

```c
// security/landlock/syscalls.c
SYSCALL_DEFINE3(landlock_add_rule, int, ruleset_fd,
        enum landlock_rule_type, rule_type, const void __user *, rule_attr)
{
    // Allows unprivileged process to restrict own capabilities
    // E.g., restrict file access to /tmp, network to port 8080
}
```

### Recommendation:

- [ ] **seccomp filters** (Phase 4): Syscall whitelisting

- [ ] **Shadow stack** (Phase 4): Return address signing

- [ ] **CFI** (Phase 5): Indirect call hardening

- [ ] **Landlock-like sandboxing** (Phase 4): For untrusted userland

---

## Part 3: Driver Roadmap & Priority Matrix

### Priority Ranking (Phase 1–3)

| Driver | Complexity | Impact | Timeline | Reference |
|---|---|---|---|---|
| **RTL8111** (NIC) | Medium | HIGH | 4 weeks | `r8169` from torvalds/linux |
| **NVMe** (SSD) | Medium | HIGH | 4 weeks | `nvme/host/pci.c` from torvalds/linux |
| **AHCI** (SATA) | Low | MEDIUM | 2 weeks | `ata/libahci.c` from torvalds/linux |
| **i915** (Intel GPU) | High | MEDIUM | 8 weeks | `gpu/drm/i915/` from torvalds/linux |
| **amdgpu** (AMD GPU) | High | MEDIUM | 8 weeks | `gpu/drm/amd/amdgpu/` from torvalds/linux |
| **virtio-gpu** (QEMU) | Low | LOW | 2 weeks | `gpu/drm/virtio/` from torvalds/linux |
| **xHCI** (USB host) | High | MEDIUM | 6 weeks | `usb/host/xhci.c` from torvalds/linux |
| **HID kb/mouse** | Low | HIGH | 2 weeks | `usb/input/usbkbd.c` from torvalds/linux |
| **SDHCI** (SD/eMMC) | Low | MEDIUM | 3 weeks | `mmc/host/sdhci.c` from torvalds/linux |

---

## Part 4: Code Pattern Reference

### Scheduler Pattern (Linux → Rust)

### Linux (C):

```c
struct task_struct {
    struct sched_entity se;          // Fair scheduler entity
    struct rt_rq *rt.rq;             // Real-time runqueue
    int prio, rt_priority;
};

// Scheduler class (pluggable)
struct sched_class {
    void (*enqueue_task)(struct rq *rq, struct task_struct *p, int flags);
    void (*dequeue_task)(struct rq *rq, struct task_struct *p, int flags);
    void (*yield_task)(struct rq *rq);
    struct task_struct *(*pick_next_task)(struct rq *rq, struct task_struct *prev);
};
```

### SigmaOS (Rust pattern to implement):

```rust
#![no_std]

pub trait SchedulerClass {
    fn enqueue_task(&mut self, task: &TaskStruct) -> Result<(), SchedError>;
    fn dequeue_task(&mut self, task_id: Pid) -> Result<TaskStruct, SchedError>;
    fn pick_next_task(&mut self) -> Option<&TaskStruct>;
}

pub struct EdfScheduler {
    runqueue: Vec<(Deadline, Pid)>,  // Min-heap by deadline
}

impl SchedulerClass for EdfScheduler {
    fn enqueue_task(&mut self, task: &TaskStruct) -> Result<(), SchedError> {
        self.runqueue.push((task.deadline, task.pid));
        self.runqueue.sort_by_key(|t| t.0);  // Maintain ordering
        Ok(())
    }

    fn pick_next_task(&mut self) -> Option<&TaskStruct> {
        // Return task with earliest deadline
        self.runqueue.first().map(|t| t.1)
    }
}
```

---

## Part 5: Test & Validation Strategy

### Unit Tests (in Rust)

```bash

# Test scheduler correctness

cargo test --lib kernel::core::sched::tests

# Test allocator

cargo test --lib kernel::core::memory::tests

# Stress test allocator with fragmentation

cargo test --release -- --nocapture fragmentation_stress
```

### QEMU Integration Tests

```bash

# Boot and test e1000 driver

make PROFILE=standalone QEMU_ARGS="-net nic,model=e1000" qemu-test

# Boot and test NVMe

make PROFILE=cloud qemu-test

# GPIO/interrupt stress test

./scripts/qemu_stress_interrupts.sh
```

### Benchmark Suite

```bash

# Scheduler latency

./tools/bench/scheduler_latency.sh

# Allocator throughput

./tools/bench/allocator_throughput.sh

# Network throughput (e1000)

./tools/bench/net_throughput.sh
```

---

## Part 6: Recommended Contribution Workflow

### Phase 0 (Immediate)

1. **Scheduler hardening** (2 weeks)
   - Implement priority levels (0-255)
   - Add preemption counter
   - CPU affinity tracking

2. **Memory allocator stress test** (1 week)
   - Fuzz allocator with random alloc/free patterns
   - Measure fragmentation
   - Add kswapd-like reclaim

3. **Syscall table expansion** (3 weeks)
   - Add 50+ POSIX syscalls (open, close, read, write, mmap, etc.)
   - Add versioning layer for backward compat

### Phase 1 (Weeks 4–8)

1. **RTL8111 driver** (4 weeks): Rust + async I/O

2. **NVMe driver** (4 weeks): Rust + DMA

3. **AHCI/SATA** (2 weeks): Fallback for older hardware

### Phase 2 (Weeks 9–16)

1. **virtio-gpu** (2 weeks): Quick win for QEMU desktop

2. **i915 basic** (6 weeks): Intel GPU polling mode

3. **USB/xHCI host** (6 weeks): Keyboard + mouse

---

## Part 7: External Resources

### Linux Kernel References

- **Scheduler**: [torvalds/linux kernel/sched/core.c](https://github.com/torvalds/linux/blob/master/kernel/sched/core.c)

- **Memory**: [torvalds/linux mm/page_alloc.c](https://github.com/torvalds/linux/blob/master/mm/page_alloc.c)

- **Network Drivers**: [torvalds/linux drivers/net/ethernet/](https://github.com/torvalds/linux/tree/master/drivers/net/ethernet)

- **GPU/DRM**: [torvalds/linux drivers/gpu/drm/](https://github.com/torvalds/linux/tree/master/drivers/gpu/drm)

### Specialized Linux Variants

- **Rust Support**: [Rust-for-Linux/linux](https://github.com/Rust-for-Linux/linux)

- **Real-Time**: [zen-kernel/zen-kernel](https://github.com/zen-kernel/zen-kernel), [thesofproject/linux](https://github.com/thesofproject/linux)

- **Security**: [landlock-lsm/linux](https://github.com/landlock-lsm/linux), [samitolvanen/linux](https://github.com/samitolvanen/linux)

- **Apple Silicon**: [AsahiLinux/linux](https://github.com/AsahiLinux/linux) — excellent GPU driver examples

- **Mobile**: [msm8916-mainline/linux](https://github.com/msm8916-mainline/linux), [Icenowy/linux](https://github.com/Icenowy/linux)

### Books & Papers

- **Linux Kernel Development** (Robert Love): Chapter 3 (Processes), Chapter 8 (Memory)

- **Understanding the Linux Kernel** (Bovet & Cesati): Architecture focus

- **Linux Device Drivers** (Rubini et al.): Driver development patterns

- **OSDEV.org**: x86-64 memory management, interrupt handling

---

## Next Steps

1. **Pick Phase 0 priority**: Start with scheduler hardening OR memory allocator stress test

2. **Create GitHub Issues** for each gap (use labels: `phase-0`, `phase-1`, `driver`, `subsystem`)

3. **Assign Linux variant references** to each issue (link to torvalds/linux or variant)

4. **Establish spike timeboxes**: 1 week = feasibility study + proof-of-concept

---

### End of Audit.
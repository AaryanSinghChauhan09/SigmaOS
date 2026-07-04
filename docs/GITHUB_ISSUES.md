# GitHub Issues — SigmaOS Development Backlog

Ready-to-create GitHub issues, organized by tier and sprint.

---

## Tier 1 — Bootability (CRITICAL PATH)

### Issue #1001: [KERNEL] Complete UEFI Bootloader (sigma-boot.efi)
**Labels:** kernel, boot, critical  
**Files:** `sigma-boot/sigma_boot.zig`, `arch/x86_64/head64.asm`

**Description:**
sigma-boot.zig has a Zig UEFI implementation that loads the kernel ELF. Needs:
- Test with QEMU OVMF firmware
- Verify memory map hand-off to sigma_kernel_main
- Add GOP framebuffer setup
- Add Secure Boot signature verification (Dilithium-5)

**Acceptance Criteria:**
- [ ] QEMU boots with `-bios /usr/share/ovmf/OVMF.fd`
- [ ] Serial output: `SigmaOS Boot v15.0`
- [ ] Kernel receives valid BootInfo struct
- [ ] Memory map passed to buddy allocator

---

### Issue #1002: [KERNEL] Wire GDT + IDT to kernel main
**Labels:** kernel, boot, critical  
**Files:** `arch/x86_64/gdt.asm`, `arch/x86_64/idt.asm`, `kernel/core/sigma_irq.rs`

**Description:**
GDT, IDT, and IRQ files now exist. Need to call them from sigma_kernel_main.

**Acceptance Criteria:**
- [ ] `sigma_gdt_init(kernel_stack_top)` called at boot
- [ ] `sigma_idt_init()` called at boot
- [ ] Timer IRQ fires at 1000 Hz (verify with JIFFIES counter in serial log)
- [ ] Keyboard IRQ fires on keypress

---

### Issue #1003: [KERNEL] Complete Round-Robin Scheduler
**Labels:** kernel, scheduler, critical  
**Files:** `kernel/core/sigma_sched.rs`

**Description:**
sigma_sched.rs has MLFQ+CFS+EDF. For initial boot, needs:
- Integration with timer IRQ (call `sched_tick()` from IRQ handler)
- Process table linking (fork/exec)
- Context switch using `arch/x86_64/switch.asm`

**Acceptance Criteria:**
- [ ] 2 tasks scheduled round-robin visible in serial log
- [ ] `sched_tick()` called from PIT handler
- [ ] Context switch saves/restores all registers

---

### Issue #1004: [KERNEL] Buddy + Slab Allocator Integration
**Labels:** kernel, memory, critical  
**Files:** `kernel/core/sigma_mm.rs`

**Description:**
sigma_mm.rs has buddy+slab. Need to initialize from BootInfo memory map.

**Acceptance Criteria:**
- [ ] `sigma_slab_init()` uses memory map from UEFI
- [ ] Allocate/free 1000 objects without leak
- [ ] `sigma_mm_free_pages()` returns correct count

---

### Issue #1005: [KERNEL] VFS + Tmpfs Integration
**Labels:** kernel, vfs, critical  
**Files:** `kernel/vfs/sigma_vfs.rs`, `kernel/vfs/sigma_tmpfs.rs`

**Description:**
VFS and Tmpfs are implemented. Need to wire open/read/write syscalls to them.

**Acceptance Criteria:**
- [ ] `sigma-sh` can `echo hello > /tmp/test && cat /tmp/test`
- [ ] `mkdir /tmp/mydir` works
- [ ] `stat /tmp/test` returns correct size

---

### Issue #1006: [SYSCALL] Implement read/write/open/close
**Labels:** kernel, syscalls, critical  
**Files:** `kernel/core/syscall_dispatch.rs`

**Description:**
Syscall dispatch exists but read/write/open/close return ENOSYS.

**Acceptance Criteria:**
- [ ] `write(1, "hello\n", 6)` writes to serial/VGA
- [ ] `open("/tmp/test", O_RDONLY)` returns valid fd
- [ ] `read(fd, buf, 100)` reads tmpfs file content
- [ ] `close(fd)` frees fd slot

---

## Tier 2 — Filesystem & I/O

### Issue #1007: [FS] Ext4 Read-Only Mount
**Labels:** fs, storage  
**Files:** `kernel/fs/ext4/`

**Description:**
Implement ext4 read-only mounting from a block device.

**Acceptance Criteria:**
- [ ] Mount ext4 image: `sigma_vfs_mount("/", dev, &EXT4_OPS)`
- [ ] `cat /etc/hostname` reads from ext4
- [ ] `ls /` lists root directory

---

### Issue #1008: [DRIVER] Console Character Device
**Labels:** drivers, io  
**Files:** `drivers/char/console.rs`

**Description:**
Implement /dev/console, /dev/null, /dev/zero, /dev/random.

**Acceptance Criteria:**
- [ ] `write(1, buf, len)` → output on VGA + serial
- [ ] `read` from /dev/null returns 0
- [ ] `read` from /dev/random returns pseudo-random bytes

---

### Issue #1009: [NET] TCP State Machine
**Labels:** networking, tcp  
**Files:** `kernel/net/tcp.rs`

**Description:**
Implement full RFC 793 TCP state machine: SYN, SYN-ACK, ESTABLISHED, FIN.

**Acceptance Criteria:**
- [ ] TCP 3-way handshake completes with QEMU user networking
- [ ] `connect(fd, &addr, sizeof(addr))` works
- [ ] `send/recv` exchange data
- [ ] FIN/RST handled correctly

---

### Issue #1010: [SECURITY] Full sigma_pledge Kernel Enforcement
**Labels:** security, syscalls  
**Files:** `kernel/security/sigma_pledge.rs`

**Description:**
sigma_pledge.rs has the data structure. Need to wire it to every syscall.

**Acceptance Criteria:**
- [ ] Process calls `sigma_pledge("stdio")`, then attempts `open("/etc/passwd")` → SIGKILL
- [ ] Audit log entry written for every violation
- [ ] `getpid()` allowed under any pledge

---

## Tier 3 — Graphics & Desktop

### Issue #1011: [GPU] VirtIO-GPU DRM Driver
**Labels:** drivers, gpu, graphics  
**Files:** `drivers/gpu/sigma_virtio_gpu.zig`

**Description:**
Implement DRM/KMS mode setting for VirtIO-GPU (QEMU).

**Acceptance Criteria:**
- [ ] Framebuffer available at boot
- [ ] Can set 1024×768 resolution
- [ ] Zenith compositor can write pixels

---

### Issue #1012: [DESKTOP] Zenith Wayland Compositor
**Labels:** desktop, wayland  
**Files:** `desktop/compositor/`

**Description:**
Wire Zenith desktop compositor to VirtIO-GPU + input driver.

**Acceptance Criteria:**
- [ ] Desktop window appears in QEMU display
- [ ] Mouse input routes to windows
- [ ] Keyboard input works in terminal

---

## Tier 4 — Package Management

### Issue #1013: [PKG] sigma-pkg End-to-End
**Labels:** packaging, ecosystem  
**Files:** `sigma-pkg/`, `sigma_pkg_registry/`

**Description:**
Implement complete package install flow.

**Acceptance Criteria:**
- [ ] `sigma-pkg install htop` downloads, verifies, installs
- [ ] Dilithium-5 signature verified
- [ ] `sigma-pkg remove htop` cleans up
- [ ] Rollback works if install fails

---

### Issue #1014: [TOOLS] Core Utilities (ls, cat, grep, sed)
**Labels:** userland, tools  
**Files:** `userland/coreutils/`

**Description:**
Implement GNU coreutils equivalents in Rust.

**Acceptance Criteria:**
- [ ] `ls /tmp` lists files with permissions
- [ ] `cat /etc/hostname` reads file
- [ ] `grep pattern /tmp/file` matches lines
- [ ] `echo hello | sed s/hello/world/` outputs `world`

---

## GitHub Milestone Structure

```
Milestone: v15.1 (Sprint 0-2, 3 months)
  #1001 UEFI Bootloader
  #1002 GDT+IDT wiring
  #1003 Round-Robin Scheduler
  #1004 Memory Allocator
  #1005 VFS+Tmpfs

Milestone: v15.2 (Sprint 3-4, 6 months)
  #1006 read/write/open/close syscalls
  #1007 Ext4 read-only
  #1008 Console device
  #1009 TCP state machine
  #1010 sigma_pledge enforcement

Milestone: v16.0 Apex (Sprint 5+, 12 months)
  #1011 VirtIO-GPU
  #1012 Zenith compositor
  #1013 sigma-pkg
  #1014 Core utilities
```

---

*See also: [12-Week-Milestone-Plan](../wiki_repo/12-Week-Milestone-Plan.md) · [PHASE_A_EXECUTION_CHECKLIST.md](../PHASE_A_EXECUTION_CHECKLIST.md)*

# SigmaOS — Feature-Level Development Roadmap Per Branch

Grounded in actual code state from June 2026 codebase audit.
Every task maps to a real file with a known current state.

**Legend:** ✅ Real impl · ⚠️ Simulated/partial · ❌ Stub/missing

---

## Actual Implementation State (Ground Truth)

| Component | File | State |
|-----------|------|-------|
| Kernel boot entry | `kernel/core/sigma_kernel_main.c` | ✅ Real |
| Buddy allocator | `kernel/memory/sigma_allocator.cpp` | ✅ Real |
| MCS scheduler | `kernel/sched/sigma_mcs.cpp` | ✅ Real |
| Tiling WM | `zenith_desktop/wm/sigma_tiling_wm.cpp` | ✅ Real |
| Shell (parser + builtins) | `userland/shell/sigma_shell.cpp` | ✅ Real (no TTY read) |
| PE loader (parser) | `runtime/compat/win32/sigma_pe_loader.cpp` | ⚠️ Partial |
| NT API layer | `runtime/compat/win32/sigma_ntdll.cpp` | ⚠️ Partial |
| Compositor | `zenith_desktop/compositor/sigma_compositor.cpp` | ⚠️ Partial |
| VMM / page tables | `kernel/memory/sigma_vmm.cpp` | ⚠️ Stub |
| Kyber-1024 | `crypto/SovereignKyber.cpp` | ⚠️ Simulated |
| Dilithium-5 | `crypto/SovereignDilithium5.cpp` | ⚠️ Simulated |
| NVMe driver | `drivers/SovereignNVMe.cpp` | ⚠️ Simulated |
| NIC driver | `drivers/SovereignNIC.cpp` | ⚠️ Simulated |
| GPU driver | `drivers/SovereignGPU.cpp` | ⚠️ Simulated |
| Syscall dispatch | `kernel/syscalls/sigma_syscalls.cpp` | ❌ Stub |
| TCP/IP stack | `net/sigma_tcp_ip.cpp` | ❌ Minimal |
| QEMU CI tests | `.github/workflows/sigma_qemu.yml` | ❌ echo stubs |

---

## `kernel-exp` — Feature Roadmap

### Feature 1: Syscall Dispatch (BLOCKS EVERYTHING)

**Current:** `sigma_syscalls.cpp` — all handlers are `return 0` stubs. 8 syscall numbers defined.
**Need:** Full dispatch table wired to real subsystems.

| Task | File | Detail |
|------|------|--------|
| Wire `SIGMA_SYS_DEBUG_PRINT` to serial + VGA | `kernel/syscalls/sigma_syscalls.cpp` | Route to `serial_puts()` / `vga_putc()` |
| Wire `SIGMA_SYS_ALLOC_MEM` to buddy allocator | `kernel/syscalls/sigma_syscalls.cpp` | Call `sigma_malloc(arg1)`, return ptr |
| Wire `SIGMA_SYS_FREE_MEM` to buddy allocator | `kernel/syscalls/sigma_syscalls.cpp` | Call `sigma_free((void*)arg1)` |
| Wire `SIGMA_SYS_SPAWN_TASK` to MLFQ | `kernel/syscalls/sigma_syscalls.cpp` | Already calls `sched_add_task` — verify ABI |
| Wire `SIGMA_SYS_SEND_MSG` / `SIGMA_SYS_RECV_MSG` to IPC | `kernel/syscalls/sigma_syscalls.cpp` | Route to `SovereignIPC` |
| Wire `SIGMA_SYS_HW_IO` with capability check | `kernel/syscalls/sigma_syscalls.cpp` | Gate on capability token before port I/O |
| Add 22 more syscalls (read/write/open/close/fork/exec/exit/mmap/mprotect/munmap/wait/stat/...) | `kernel/core/sigma_syscall_dispatch.cpp` | Match Linux x86-64 ABI numbers for compat layer |
| SYSCALL/SYSRET MSR setup (x86-64) | `arch/x86_64/sigma_syscall_entry.asm` | Write `STAR`, `LSTAR`, `SFMASK` MSRs |
| Ring-3→Ring-0 entry point in assembly | `arch/x86-64/sigma_syscall_entry.asm` | Save RSP/RBP/regs, call C handler, restore |

**Exit test:** `write(1, "hello\n", 6)` from a static userland binary prints to serial.

### Feature 2: VMM — Page Table Walker (BLOCKS COMPAT LAYER, USERLAND)

**Current:** `sigma_vmm.cpp` — `sigma_vmm_map` is empty. `sigma_mmap` treats virtual = physical.
**Need:** Real 4-level page table walk with PML4→PDPT→PD→PT.

| Task | File | Detail |
|------|------|--------|
| x86-64 PML4 init at boot | `kernel/memory/sigma_vmm.cpp` | Allocate PML4 from buddy, write CR3 |
| `sigma_vmm_map(ctx, vaddr, paddr, flags)` — full 4-level walk | `kernel/memory/sigma_vmm.cpp` | Allocate PDPT/PD/PT as needed via `sigma_malloc` |
| `sigma_vmm_unmap(ctx, vaddr)` — clear PTE + TLB flush | `kernel/memory/sigma_vmm.cpp` | `invlpg` instruction |
| `sigma_mmap` use real page alloc + VMM | `kernel/memory/sigma_vmm.cpp` | Replace `sigma_malloc` with phys page alloc + map |
| CoW fault handler | `kernel/memory/sigma_vmm.cpp` | Wire to `#PF` handler in IDT |
| KASLR: randomize kernel base at boot | `kernel/memory/sigma_kaslr.cpp` | Read RDRAND, shift kernel load address |
| Separate kernel/user address spaces | `kernel/memory/sigma_vmm.cpp` | CR3 swap on context switch |

**Exit test:** Userland process at VA 0x400000 reads/writes a mapped page; CoW copies on write.

### Feature 3: MLFQ Scheduler (upgrade from round-robin)

**Current:** `sigma_kernel_main.c` calls `sigma_sched_init()` + `sigma_sched_add_task()` for two dummy tasks. MCS budget accounting exists (`sigma_mcs.cpp`). MLFQ likely exists in `kernel/scheduler/`.

| Task | File | Detail |
|------|------|--------|
| Verify MLFQ body exists and compiles | `kernel/scheduler/sigma_mlfq.cpp` (or `.h`) | Run `make PROFILE=microkernel` |
| Wire MLFQ with MCS budget accounting | `kernel/sched/sigma_mcs.cpp` | Call `sigma_mcs_check_budget` in scheduler tick |
| Priority inheritance for mutex waiters | `kernel/sched/sigma_pi.cpp` | Elevate waiter to holder's priority |
| Multi-core runqueue (one per CPU) | `kernel/sched/sigma_runqueue.cpp` | Per-CPU `sigma_sched_ctx_t` array |
| NUMA-aware task placement | `kernel/sched/sigma_numa.cpp` | Read ACPI SRAT, prefer near-memory CPUs |
| CFS vruntime red-black tree (post-MLFQ) | `kernel/sched/sigma_cfs.cpp` | `struct rb_node` + `vruntime` tracking |
| `sigma_sleep_ms` wired to PIT/APIC timer | `kernel/core/sigma_timer.cpp` | Insert task into timer wheel, wake on expiry |

**Exit test:** 8 tasks at mixed priorities run for 10 seconds — high-priority tasks get more CPU cycles measurably.

### Feature 4: IRQ / Interrupt Controller

**Current:** `idt_init()` and `sigma_pic_init()` called at boot. Timer at 1000 Hz. Keyboard IRQ handler present.

| Task | File | Detail |
|------|------|--------|
| APIC init (replace PIC for SMP) | `kernel/core/sigma_irq.cpp` | Write to APIC MMIO base; disable PIC |
| Local APIC timer (replace PIT) | `kernel/core/sigma_timer.cpp` | Calibrate against PIT, then switch |
| I/O APIC for PCI MSI-X | `hal/sigma_pci.cpp` | Read IOAPIC redirection table, enable MSI |
| IRQ routing table | `kernel/core/sigma_irq.cpp` | PCI device → IOAPIC pin → vector mapping |
| APIC spurious vector handler | `arch/x86_64/sigma_irq_entry.asm` | Handler for vector 0xFF |
| SMP IPI (inter-processor interrupt) | `kernel/core/sigma_irq.cpp` | ICR write for scheduler kick + TLB shootdown |

**Exit test:** Timer fires 1000× per second on APIC; TLB shootdown IPI works across 2 virtual CPUs.

### Feature 5: UEFI Bootloader (`sigma-boot.efi`)

**Current:** Doesn't exist. Kernel is loaded by GRUB/QEMU multiboot.

| Task | File | Detail |
|------|------|--------|
| UEFI PE binary skeleton | `sigma-boot/sigma_boot.c` | EDK2 application entry `efi_main()` |
| Load kernel ELF from ESP partition | `sigma-boot/sigma_elf_loader.c` | `EFI_FILE_PROTOCOL->Read()` |
| UEFI GOP framebuffer setup | `sigma-boot/sigma_gop.c` | `EFI_GRAPHICS_OUTPUT_PROTOCOL` |
| Pass memory map to kernel | `sigma-boot/sigma_mmap.c` | Build E820 / multiboot2 memory map |
| A/B slot selection via EFI variables | `sigma-boot/sigma_ab_slot.c` | Read `SigmaBootSlot` EFI variable |
| Dilithium3 signature verification of kernel | `sigma-boot/sigma_secboot.c` | Verify kernel ELF before loading |
| `make iso` target producing bootable ISO | `Makefile` | `grub-mkrescue` with sigma-boot fallback |

**Exit test:** `qemu-system-x86_64 -cdrom SigmaOS.iso` boots to `sigma-login:` prompt.

---

## `drivers-dev` — Feature Roadmap

### Feature 6: Real GPU / DRM / KMS Pipeline

**Current:** `SovereignGPU.cpp` — framebuffer base address is a hardcoded constant (`0xE0000000`), swap and shader functions log to serial. No real register writes.

| Task | File | Detail |
|------|------|--------|
| VirtIO-GPU device detect + init | `drivers/display/sigma_virtio_gpu.cpp` | Scan PCI for vendor 0x1AF4 device 0x1050 |
| VirtIO-GPU resource create (2D) | `drivers/display/sigma_virtio_gpu.cpp` | `VIRTIO_GPU_CMD_RESOURCE_CREATE_2D` |
| VirtIO-GPU set scanout + flush | `drivers/display/sigma_virtio_gpu.cpp` | Attach resource to scanout, flush rect |
| DRM/KMS abstraction layer | `drivers/graphics/sigma_kms.cpp` | Connector → CRTC → plane → framebuffer chain |
| Intel i915 PCI detect + MMIO map | `drivers/graphics/sigma_i915.cpp` | PCI 0x8086 0x3E9B (UHD 630) base address |
| i915 display engine init (pipe + plane) | `drivers/graphics/sigma_i915.cpp` | Program HTOTAL/VTOTAL/DPLL registers |
| AMD amdgpu DCN (display core next) | `drivers/graphics/sigma_amdgpu.cpp` | PCI 0x1002 series, GFX v9/v10 |
| Atomic modesetting API | `drivers/graphics/sigma_kms.cpp` | `sigma_kms_set_mode(connector, mode)` |
| DMA-BUF shared buffer | `drivers/graphics/sigma_dmabuf.cpp` | Share GPU buffer with Zenith compositor |
| Vulkan ICD loader stub | `drivers/graphics/sigma_vk_icd.cpp` | Point `VK_ICD_FILENAMES` to sigma driver |

**Exit test:** Zenith compositor renders a 1920×1080 desktop frame via VirtIO-GPU in QEMU at ≥60 FPS.

### Feature 7: Wi-Fi Drivers

**Current:** `drivers/SovereignWiFi.cpp` exists (not read). `sigma_iwlwifi.h` stub.

| Task | File | Detail |
|------|------|--------|
| cfg80211 nl80211 userspace framework | `drivers/net/sigma_cfg80211.cpp` | Netlink socket, `NL80211_CMD_CONNECT` |
| Intel iwlwifi firmware loader | `drivers/net/sigma_iwlwifi.cpp` | Load `.ucode` firmware blob via sigma-vfs |
| iwlwifi MVM driver (7265/8265/AX200) | `drivers/net/sigma_iwlwifi.cpp` | MMIO init, Tx/Rx queues |
| MediaTek mt7921 PCIe Wi-Fi | `drivers/net/sigma_mt7921.cpp` | Popular in JioBook + budget laptops |
| Realtek rtl8xxxu USB Wi-Fi | `drivers/net/sigma_rtl8xxxu.cpp` | USB dongle for rural/edge devices |
| WPA2/WPA3 supplicant | `userland/net/sigma_wpa.cpp` | 4-way handshake for PSK authentication |
| DHCP client via sigma-net | `net/dhcp/sigma_dhcp_client.cpp` | Broadcast DISCOVER, parse OFFER |

**Exit test:** `sigma-net connect --ssid HomeNetwork --psk password` associates and gets DHCP IP.

### Feature 8: NVMe + Storage Real DMA

**Current:** `SovereignNVMe.cpp` — DMA is `status = 0; // Assume success`. Queue depth tracked but no real submission queue.

| Task | File | Detail |
|------|------|--------|
| NVMe admin queue setup (SQ+CQ) | `drivers/storage/sigma_nvme.cpp` | Write `ASQ`, `ACQ`, `AQA` BAR registers |
| NVMe identify controller | `drivers/storage/sigma_nvme.cpp` | Submit admin cmd 0x06, read `MDTS` |
| NVMe I/O queue creation | `drivers/storage/sigma_nvme.cpp` | Create/Delete I/O Submission + Completion queues |
| NVMe read/write command submission | `drivers/storage/sigma_nvme.cpp` | Build NVM command, ring doorbell |
| Completion queue interrupt (MSI-X) | `drivers/storage/sigma_nvme.cpp` | Wire to I/O APIC vector |
| DMA-coherent buffer allocation | `kernel/mm/sigma_dma.cpp` | Allocate contiguous physical pages |
| NVMe multiqueue (per-CPU SQ) | `drivers/storage/sigma_nvme.cpp` | Reduce lock contention on SMP |

**Exit test:** `sigma-fsck /dev/nvme0n1` reads actual sectors from QEMU NVMe device.

### Feature 9: Real NIC — e1000 / VirtIO-net DMA Ring

**Current:** `SovereignNIC.cpp` — `transmit()` logs a string, `receive()` returns 0. No DMA ring.

| Task | File | Detail |
|------|------|--------|
| e1000 Tx descriptor ring setup | `kernel/core/drivers/SovereignE1000.cpp` | Write `TDBAL/TDBAH/TDLEN/TDH/TDT` MMIO |
| e1000 Rx descriptor ring setup | `kernel/core/drivers/SovereignE1000.cpp` | Write `RDBAL/RDBAH/RDLEN/RDH/RDT` MMIO |
| e1000 transmit: fill descriptor + kick TDT | `kernel/core/drivers/SovereignE1000.cpp` | Set buffer address, length, CMD byte |
| e1000 receive: poll RDT, copy buffer | `kernel/core/drivers/SovereignE1000.cpp` | Check DD bit in status, deliver to stack |
| VirtIO-net Tx/Rx virtqueue | `drivers/net/sigma_virtio_net.cpp` | Split virtqueue, VRING_DESC + AVAIL/USED |
| ARP resolution (replace stub) | `kernel/net/sigma_net_arp.cpp` | RFC 826 ARP request/reply state machine |
| DHCP client wired to NIC | `net/dhcp/sigma_dhcp_client.cpp` | Call after link up |

**Exit test:** `ping 10.0.2.2` from QEMU guest reaches QEMU host gateway and gets ICMP reply.

### Feature 10: Audio — HDA Controller

**Current:** No audio driver exists.

| Task | File | Detail |
|------|------|--------|
| HDA controller detect (PCI class 0x0403) | `drivers/audio/sigma_hda.cpp` | Map MMIO BAR0 |
| HDA CORB/RIRB setup | `drivers/audio/sigma_hda.cpp` | Codec command/response ring buffers |
| HDA codec node enumeration | `drivers/audio/sigma_hda.cpp` | `GET_PARAMETER(AFG)` → pin widgets |
| PCM output stream | `drivers/audio/sigma_hda.cpp` | BDL (Buffer Descriptor List), DMA pages |
| sigma-audio daemon API | `userland/audio/sigma_audio.cpp` | `sigma_audio_open/write/close` |
| sigma-bhashini TTS → sigma-audio | `userland/bhashini/sigma_bhashini.cpp` | Pipe TTS float32 PCM to HDA stream |

**Exit test:** `sigma-audio play test.wav` plays sound through QEMU HDA virtual device.

---

## `fs-dev` — Feature Roadmap

### Feature 11: VFS — Real File Operations

**Current:** VFS header exists. `vfs_open/read/write/close` likely partial. `POSIXShim.cpp` calls `vfs_open/read/write/close` but VMM not connected.

| Task | File | Detail |
|------|------|--------|
| `vfs_open(pid, path, flags)` — path lookup | `kernel/vfs/sigma_vfs.cpp` | Walk directory tree from root inode |
| `vfs_read(pid, fd, buf, count)` — block read | `kernel/vfs/sigma_vfs.cpp` | Translate fd → inode → block device read |
| `vfs_write(pid, fd, buf, count)` | `kernel/vfs/sigma_vfs.cpp` | Dirty page marking + write-back |
| `vfs_close(pid, fd)` | `kernel/vfs/sigma_vfs.cpp` | Flush dirty, release inode reference |
| `vfs_mkdir/unlink/rename/stat` | `kernel/vfs/sigma_vfs.cpp` | Core directory operations |
| Per-process file descriptor table | `kernel/vfs/sigma_fdtable.cpp` | Array of `struct sigma_file*` per process |
| Tmpfs (RAM-backed, no block device) | `kernel/vfs/sigma_tmpfs.cpp` | First mountable FS — enables shell I/O |
| Mount table | `kernel/vfs/sigma_mount.cpp` | `sigma_mount(dev, path, type)` + VFS lookup |

**Exit test:** Shell can `echo hello > /tmp/test.txt && cat /tmp/test.txt` via tmpfs.

### Feature 12: SigmaFS Native Filesystem

**Current:** `sigmafs.c` present. State unknown.

| Task | File | Detail |
|------|------|--------|
| On-disk layout: superblock + inode bitmap + block bitmap | `fs/sigmafs/sigma_mkfs.cpp` | Write superblock at LBA 0 |
| `sigma_mkfs /dev/nvme0n1p1` tool | `fs/sigmafs/sigma_mkfs.cpp` | Format partition with SigmaFS |
| Inode read/write | `fs/sigmafs/sigma_inode.cpp` | Inode table, direct + indirect blocks |
| Directory read/write | `fs/sigmafs/sigma_dir.cpp` | Linear dir entries with hash cache |
| Journaling (JBD2-inspired) | `fs/sigmafs/sigma_journal.cpp` | Circular log for crash recovery |
| dm-verity integration | `kernel/fs/sigma_dmverity.cpp` | Hash tree verification on read |
| `sigma-fsck` tool | `userland/tools/sigma_fsck.cpp` | Check + repair SigmaFS partition |

**Exit test:** `sigma-pkg install vim` installs to SigmaFS partition, survives reboot.

### Feature 13: Unified Buffer Cache (UBC)

**Current:** `kernel/fs/sigma_ubc.h` — header only, no implementation.

| Task | File | Detail |
|------|------|--------|
| Page cache hash table | `kernel/fs/sigma_ubc.cpp` | `(dev, block) → struct page*` radix tree |
| Dirty page list + writeback thread | `kernel/fs/sigma_ubc.cpp` | kthread flushes dirty pages every 5 s |
| Zero-copy read: map page into user VA | `kernel/fs/sigma_ubc.cpp` | Share physical page via VMM instead of copy |
| Pre-emptive read-ahead | `kernel/fs/sigma_readahead.cpp` | Sequential access → prefetch N blocks ahead |
| Adaptive read-ahead via sigma-ai | `kernel/fs/sigma_readahead.cpp` | LLM-predicted access pattern |
| Page eviction (LRU clock) | `kernel/fs/sigma_ubc.cpp` | Evict cold pages under memory pressure |

**Exit test:** Sequential file read of 1 GB runs at ≥500 MB/s (cached) on NVMe.

---

## `performance-optimized` — Feature Roadmap

### Feature 14: Real PQC Cryptographic Primitives

**Current:** Both Kyber and Dilithium use `splitmix64` PRNG and XOR — not cryptographically secure.

| Task | File | Detail |
|------|------|--------|
| Integrate liboqs Kyber-1024 NTT | `crypto/SovereignKyber.cpp` | Swap `kyber_gen_matrix` with liboqs backend |
| AVX-512 NTT for Kyber poly-mul | `crypto/sigma_kyber_avx512.cpp` | Hand-vectorized butterfly operations |
| ARM NEON NTT for Kyber poly-mul | `crypto/sigma_kyber_neon.cpp` | NEON intrinsics for poly_mul_acc |
| Integrate liboqs Dilithium-5 | `crypto/SovereignDilithium5.cpp` | Real `SHAKE-256` XOF via libshake |
| SHAKE-256 / SHA-3 implementation | `crypto/sigma_sha3.cpp` | Keccak-f[1600] state machine |
| CRYSTALS NTT benchmark | `tests/perf/bench_pqc.cpp` | Report ops/sec, compare reference vs AVX-512 |
| ML-KEM (FIPS 203 final) bindings | `crypto/sigma_mlkem.cpp` | Replace draft Kyber with FIPS 203 |
| ML-DSA (FIPS 204 final) bindings | `crypto/sigma_mldsa.cpp` | Replace draft Dilithium with FIPS 204 |
| SLH-DSA (FIPS 205) hash-based sigs | `crypto/sigma_slhdsa.cpp` | For code signing (no lattice assumptions) |

**Benchmark target:** Kyber-1024 KEM: ≥5.8 M ops/sec on AVX-512, ≥2.1 M on NEON.

### Feature 15: Silicon-Aware Scheduler + PGO

**Current:** MLFQ exists. MCS budget accounting wired. No NUMA, no CFS, no PGO.

| Task | File | Detail |
|------|------|--------|
| NUMA topology reader | `kernel/sched/sigma_numa.cpp` | Parse ACPI SRAT table at boot |
| Per-NUMA-node runqueue | `kernel/sched/sigma_numa.cpp` | Prefer tasks on their home node |
| Load balancer (steal from distant node) | `kernel/sched/sigma_lb.cpp` | `sigma_lb_tick()` every 100 ms |
| CFS vruntime (red-black tree runqueue) | `kernel/sched/sigma_cfs.cpp` | `struct rb_node` + `vruntime` sorting |
| ACPI P-state frequency scaling | `kernel/power/sigma_perf_governor.cpp` | Write `IA32_PERF_CTL` MSR |
| PGO Makefile targets | `Makefile` | `make PROFILE=pgo iso` generates PGO instrumented build |
| PGO profile collection via QEMU | `Makefile` | Run `perf record` on QEMU boot, feed to clang `-fprofile-use` |
| Lock-free CAS runqueue | `klib/sigma_lockfree.h` | Michael-Scott queue, Treiber stack |
| io_uring-style async I/O | `kernel/io/sigma_uring.cpp` | SQ/CQ ring buffers, zero-copy paths |

**Benchmark target:** Context switch < 50 ns; boot time < 2 s on NVMe.

### Feature 16: Vulkan Compositor Acceleration

**Current:** `SovereignGPU.cpp` — `swapBuffers()` just resets a watchdog counter. Compositor uses a mock framebuffer (static C array).

| Task | File | Detail |
|------|------|--------|
| Vulkan instance + physical device init | `zenith_desktop/compositor/sigma_vk_init.cpp` | `vkCreateInstance`, `vkEnumeratePhysicalDevices` |
| Vulkan swapchain for DRM output | `zenith_desktop/compositor/sigma_vk_swapchain.cpp` | `vkCreateSwapchainKHR` with KMS DRM fd |
| Vulkan command buffer per frame | `zenith_desktop/compositor/sigma_vk_frame.cpp` | Triple-buffer, pre-record draw calls |
| DMA-BUF import for window surfaces | `zenith_desktop/compositor/sigma_vk_dmabuf.cpp` | Import app buffer as Vulkan image |
| GLSL shaders for window compositing | `zenith_desktop/shaders/` | Blur, shadow, glassmorphism effects |
| SIMD matrix scaling (NEON/AVX-512) | `zenith_desktop/sigma_simd_scale.cpp` | Replace scalar loops in transform path |
| Zero-alloc hot path | `zenith_desktop/compositor/sigma_compositor.cpp` | Pre-allocate all frame resources at startup |

**Benchmark target:** 1 frame latency max at 120 Hz; zero-copy DMA-BUF path active.

---

## `tools-dev` — Feature Roadmap

### Feature 17: Shell — Connect TTY to Real I/O

**Current:** `sigma_shell.cpp` — full parser + builtins. REPL loop reads from `line[0] = '\0'` placeholder. Tokenizer, history, aliases, env vars all real. **The shell does nothing at runtime without a TTY fd.**

| Task | File | Detail |
|------|------|--------|
| TTY fd read via `sigma_sys_read(0, ...)` | `userland/shell/sigma_shell.cpp` | Replace placeholder with real syscall |
| Echo mode + raw mode toggle | `userland/shell/sigma_shell.cpp` | `ECHO`/`ICANON` terminal flags |
| Fork + exec for external commands | `userland/shell/sigma_shell.cpp` | `sigma_sys_fork()` + `sigma_sys_execve()` |
| Pipe implementation (anonymous FDs) | `userland/shell/sigma_shell.cpp` | `sigma_sys_pipe()`, dup2 for stdin/stdout |
| Redirect: `>`, `>>`, `<`, `2>` | `userland/shell/sigma_shell.cpp` | `sigma_sys_open()` + dup2 |
| Background jobs `&` + `jobs` builtin | `userland/shell/sigma_shell.cpp` | Track bg PIDs in jobs table |
| Fish-style tab completion via VFS | `userland/shell/sigma_shell.cpp` | Call `vfs_readdir(PWD)` on TAB key |
| Script mode: `sigma-sh script.sh` | `userland/shell/sigma_shell.cpp` | Read from file fd instead of stdin |

**Exit test:** `ls | grep sigma | wc -l` works end-to-end in QEMU terminal.

### Feature 18: sigma-wine — Win32 Compat (Stage W1–W2)

**Current:** PE loader parses headers but `s.mem = NULL`. ntdll has 20 NT functions. kernel32 console I/O exists. wine_loader orchestrates but VMM integration is TODO.

| Task | File | Detail |
|------|------|--------|
| sigma-vmm region allocation for PE sections | `runtime/compat/win32/sigma_pe_loader.cpp` | Call `sigma_vmm_map_region(va, size, perms)` |
| Base relocation application | `runtime/compat/win32/sigma_pe_loader.cpp` | Walk `IMAGE_BASE_RELOCATION` chain, patch addresses |
| IAT patching with sigma-ntdll stubs | `runtime/compat/win32/sigma_pe_loader.cpp` | Resolve each import by name → sigma DLL |
| sigma-kernel32 file I/O | `runtime/compat/win32/kernel32/sigma_kernel32_file.cpp` | `CreateFileA/W`, `ReadFile`, `WriteFile` |
| sigma-kernel32 process creation | `runtime/compat/win32/kernel32/sigma_kernel32_process.cpp` | `CreateProcess` → `sigma_sys_fork` + PE load |
| sigma-kernel32 sync primitives | `runtime/compat/win32/kernel32/sigma_kernel32_sync.cpp` | `CreateMutex`, `WaitForSingleObject`, `CreateEvent` |
| sigma-kernel32 memory (VirtualAlloc) | `runtime/compat/win32/kernel32/sigma_kernel32_memory.cpp` | Route to `NtAllocateVirtualMemory` |
| sigma-msvcrt printf + malloc | `runtime/compat/win32/crt/sigma_msvcrt.cpp` | `printf` → sigma serial; `malloc` → `RtlAllocateHeap` |
| sigma-reg SQLite backend | `runtime/compat/win32/registry/sigma_reg.cpp` | HKLM/HKCU open/read/write via SQLite |
| NT path normalizer (`\??\C:\` → `/sigma/wine/c/`) | `runtime/compat/win32/sigma_ntdll.cpp` | Full path translation in `NtCreateFile` |
| sigma-wine CI test | `.github/workflows/sigma_wine_ci.yml` | Run `sigma-wine tests/compat/hello.exe` in QEMU |

**Exit test:** `sigma-wine hello.exe` prints `Hello, SigmaOS!` on serial console in QEMU.

### Feature 19: sigma-cli Completions + Profile Engine

**Current:** `sigma_cli.cpp` — modular CLI with profiles/aliases. VFS profile load is `[~]` partial.

| Task | File | Detail |
|------|------|--------|
| VFS read `~/.sigma_profile` on shell start | `zenith_desktop/personalization/sigma_profile_engine.cpp` | `vfs_open("/home/user/.sigma_profile")` |
| Apply profile → WM layout + theme | `zenith_desktop/zenith_unified_init.cpp` | Call `sigma_wm_layout(mode)` from profile key |
| `sigma-cli pkg install/remove/list` | `userland/tools/sigma_cli.cpp` | Route to sigma-pkg via IPC |
| `sigma-cli wine exec <exe>` | `userland/tools/sigma_cli.cpp` | Call `sigma_wine_exec()` |
| `sigma-cli health check` | `userland/tools/sigma_cli.cpp` | Query sigma-heal daemon via sigma-bus |
| `sigma-cli boot rollback` | `userland/tools/sigma_cli.cpp` | Write rollback flag to EFI variable |
| Man page generation for sigma-cli | `docs/man/sigma-cli.1` | Pandoc from Markdown |
| sigma-observatory stub TUI | `userland/tools/sigma_observatory.cpp` | ncurses-style live stats for CPU/mem/net |

### Feature 20: sigma-pkg — Package Manager

**Current:** `sigma_pkg_registry/` exists. Deb/rpm/apk compat resolver not implemented. No live repo server.

| Task | File | Detail |
|------|------|--------|
| sigma-repo-server (Go HTTPS) | `sigmad/repo/main.go` | Serve `.spkg` files with Dilithium3-signed index |
| `sigma-pkg install <name>` end-to-end | `userland/sigma-pkg/sigma_pkg.cpp` | Fetch + Dilithium3-verify + extract to VFS |
| dm-verity on package install | `userland/sigma-pkg/sigma_pkg.cpp` | Hash tree check before extraction |
| Rollback on failed install | `userland/sigma-pkg/sigma_pkg.cpp` | Atomic: unpack to staging, swap on success |
| Bootstrap package set (50 pkgs) | `sigma_pkg_registry/recipes/` | bash, coreutils, curl, git, Python, GCC, Go |
| India CDN mirror | infra | `packages.sigmaos.dev` + NIC mirror |
| `.spkg` recipe format | `sigma_pkg_registry/sigma_pkg_recipe.c` | TOML + Dilithium3 signature manifest |

---

## `release/standalone` — Feature Roadmap

### Feature 21: Zenith Desktop — Full Pipeline

**Current:** Tiling WM is real. Compositor has render loop + mock framebuffer. `composite_window()` is empty. Input polling via IPC stub.

| Task | File | Detail |
|------|------|--------|
| Connect compositor to real DRM framebuffer | `zenith_desktop/compositor/sigma_compositor.cpp` | Replace mock 1920×1080 array with real GPU FB |
| Implement `composite_window()` alpha blend | `zenith_desktop/compositor/sigma_compositor.cpp` | Porter-Duff over operator, SIMD accelerated |
| Real input event loop (keyboard + pointer) | `zenith_desktop/compositor/sigma_compositor.cpp` | Read from `/dev/input/event0` via sigma-vfs |
| BSP tree rebuild on `remove_window` | `zenith_desktop/wm/sigma_tiling_wm.cpp` | Fix the TODO in `remove_window()` |
| Animation engine (opacity fade) | `zenith_desktop/compositor/sigma_compositor.cpp` | 200 ms fade on spawn/close via IPC stub |
| Multi-monitor support | `zenith_desktop/compositor/sigma_compositor.cpp` | Multiple `output` structs, per-output render loop |
| Wayland protocol bridge (optional) | `zenith_desktop/wayland/sigma_wayland_bridge.cpp` | Run existing Linux apps via compatibility |
| Theme hot-reload without restart | `zenith_desktop/theme/sigma_theme_engine.cpp` | Watch `~/.sigma_profile` for inotify-equiv |

### Feature 22: DID-Based Login

**Current:** `SovereignDID.cpp` exists in security/. No display manager. No QR code generator.

| Task | File | Detail |
|------|------|--------|
| `sigma-dm` display manager | `userland/display/sigma_dm.cpp` | Show QR code on Zenith framebuffer |
| QR code generator | `userland/display/sigma_qr.cpp` | Encode DID URL as QR matrix |
| sigma-ultra companion app scan | `userland/sigma_ultra.cpp` | USSD or BLE scan → DID proof |
| DID auth → session key | `security/SovereignDID.cpp` | Ed25519 / ML-DSA challenge-response |
| Session key seal to TPM2 | `security/SovereignTPM.cpp` | `TPM2_CC_Seal` with PCR 0+7 |
| Fallback PIN entry | `userland/display/sigma_dm.cpp` | TOTP / static PIN for no-camera devices |

### Feature 23: sigma-ai Local LLM

**Current:** `userland/ai/` directory exists. sigma-heal/sigma-lex reference sigma-ai but no LLM backend.

| Task | File | Detail |
|------|------|--------|
| llama.cpp integration | `userland/ai/sigma_ai_llama.cpp` | Fork llama.cpp, call `llama_eval()` |
| sigma-ai daemon with IPC API | `userland/ai/sigma_ai_daemon.cpp` | sigma-bus: `sigma_ai_ask(prompt, &response)` |
| Sarvam-1 model bundle (.gguf) | `sigma_pkg_registry/recipes/sarvam1.recipe` | Download + verify 4.1 GB model |
| sigma-heal → sigma-ai crash analysis | `userland/ai/sigma_heal_ai.cpp` | Send kernel panic dump, get diagnosis |
| sigma-lex → sigma-ai Gazette parser | `userland/ai/sigma_lex_ai.cpp` | Parse Gazette PDF, extract regulatory updates |
| AVX-512 inference acceleration | `userland/ai/sigma_ai_avx512.cpp` | Use llama.cpp AVX-512 backend |
| Indian language routing | `userland/ai/sigma_ai_lang.cpp` | Auto-detect Devanagari/Tamil/Telugu input |

### Feature 24: Indian IME

**Current:** Nothing. `sigma_locale.h` exists but no translation strings, no input method.

| Task | File | Detail |
|------|------|--------|
| Inscript keyboard layout engine | `userland/ime/sigma_inscript.cpp` | Map scan codes to Unicode code points |
| Phonetic (transliteration) input | `userland/ime/sigma_phonetic.cpp` | "namaste" → "नमस्ते" via rule table |
| 22-language Unicode support | `userland/ime/sigma_unicode.cpp` | All scheduled Indian languages |
| IME switcher in Zenith taskbar | `zenith_desktop/taskbar/sigma_ime_switch.cpp` | Language flag icon, Ctrl+Space to switch |
| sigma-bhashini voice input | `userland/bhashini/sigma_bhashini_ime.cpp` | ASR → text → IME composition |
| OpenType font rendering (HarfBuzz) | `zenith_desktop/compositor/sigma_font.cpp` | Complex script shaping for Devanagari |

---

## `release/microkernel` — Feature Roadmap

### Feature 25: Minimal Kernel Profile

**Current:** `sigma_kernel_main.c` supports `SIGMA_MINIMAL_MODE` but it just skips tasks.

| Task | File | Detail |
|------|------|--------|
| Strip kernel to 15 essential syscalls | `kernel/core/sigma_syscall_dispatch.cpp` | `#ifdef SIGMA_MICROKERNEL_PROFILE` |
| sigma-bus IPC end-to-end | `kernel/ipc/sigma_bus.cpp` | Capability token passing over IPC channel |
| Capability-based access control | `kernel/security/sigma_caps.cpp` | Deny syscalls without capability token |
| seL4-style formal verification hooks | `kernel/security/sigma_contracts.h` | Frama-C WP annotations on IPC functions |
| Under 512 KB kernel image size | `Makefile` | `size build/vmlinuz-sigma` < 524288 bytes |
| Under 8 MB boot RAM footprint | QEMU: `free` in shell | Base system at rest < 8 MB |

**Exit test:** Boots in `qemu -m 8m` with sigma-bus ping-pong working between 2 processes.

---

## `release/cloud` — Feature Roadmap

### Feature 26: Container Enforcement — Kernel Path

**Current:** `sigma-pod run-native` sends namespace/cgroup spec as IPC. Kernel orchestrator `[~]` partial. Cgroup enforcement `[~]` partial.

| Task | File | Detail |
|------|------|--------|
| `sigma_cgroup.c` CPU quota enforcement | `kernel/core/process/sigma_cgroup.c` | Throttle task when `cpu.quota` exceeded |
| `sigma_cgroup.c` memory limit | `kernel/core/process/sigma_cgroup.c` | OOM kill when `memory.limit` exceeded |
| `sigma_cgroup.c` I/O bandwidth limit | `kernel/core/process/sigma_cgroup.c` | Token bucket on block I/O submissions |
| Namespace creation (PID/NET/MNT/IPC/UTS/USER) | `kernel/core/process/sigma_namespace.cpp` | Create isolated namespace per pod |
| dm-verity .spkg image verification | `kernel/fs/sigma_dmverity.cpp` | Hash tree check before container mount |
| KVM hypervisor (VMs alongside containers) | `kernel/hypervisor/sigma_kvm.cpp` | `KVM_CREATE_VM` ioctl, vCPU management |
| sigma-fleet MDM agent | `userland/tools/sigma_fleet_agent.cpp` | Report status, receive remote commands |
| OpenTelemetry metrics export | `userland/sigma_otel_export.cpp` | Prometheus + Jaeger endpoints |

**Exit test:** `sigma-pod run-native test.spkg --cpu=100 --mem=64` — process gets OOM-killed at exactly 64 MB.

---

## `release/distributed` — Feature Roadmap

### Feature 27: SovereignCloudFS + Mesh Compute

**Current:** `net/sigma_cloudfs.cpp` and `net/sigma_mesh.cpp` exist (state unknown).

| Task | File | Detail |
|------|------|--------|
| SovereignCloudFS: Raft consensus | `net/sigma_cloudfs.cpp` | 3-node Raft leader election + log replication |
| SovereignCloudFS: block sync | `net/sigma_cloudfs.cpp` | Encrypted multi-node block replication |
| CRDT offline sync | `net/sigma_offline_sync.cpp` | LWW-element set for key-value store |
| sigma-mesh-compute job scheduler | `net/sigma_mesh.cpp` | Distribute compute tasks across BharatNet nodes |
| sigma-blockchain-lite DLT | `net/sigma_blockchain.cpp` | Append-only Dilithium3-signed ledger |
| sigma-zkvm Groth16 ZK proof | `runtime/zkvm/sigma_zkvm.cpp` | `sigma-datasov zk prove --claim "income > 500000"` |

---

## `release/rtos` — Feature Roadmap

### Feature 28: Hard Real-Time Guarantees

**Current:** EDF exists in scheduler roadmap. MCS budget accounting is real. No priority inheritance.

| Task | File | Detail |
|------|------|--------|
| PREEMPT_RT-style full kernel preemption | `kernel/core/sigma_sched.cpp` | All spinlocks → mutex, preemptible IRQ handlers |
| EDF scheduler (earliest-deadline-first) | `kernel/sched/sigma_edf.cpp` | `struct edf_task { deadline_ns; }` + priority queue |
| Priority inheritance for mutexes | `kernel/sched/sigma_pi.cpp` | Elevate holder to waiter's priority temporarily |
| Bounded IRQ latency (< 10 µs target) | `kernel/core/sigma_irq.cpp` | Measure APIC → handler latency via RDTSC |
| ROS 2 DDS porting layer | `runtime/ros2/sigma_ros2_dds.cpp` | OMG DDS XRCE over sigma-bus |
| sigma-twin IoT live data path | `userland/twin/sigma_twin_iot.cpp` | MQTT broker (eclipse-paho port) on sigma-net |
| Xenomai-style dual-kernel mode | `kernel/core/sigma_xenomai.cpp` | RT tasks bypass normal scheduler path |

---

## `release/mobile` — Feature Roadmap

### Feature 29: ARM64 Full BSP

**Current:** Stubs in `arch/arm64/`. No working cross-compile toolchain confirmed.

| Task | File | Detail |
|------|------|--------|
| ARM64 GIC-400 interrupt controller | `arch/arm64/sigma_gic.cpp` | GICD/GICC MMIO init, IRQ routing |
| ARM64 MMU (4-level TTBR0/TTBR1) | `arch/arm64/sigma_mmu.cpp` | `TCR_EL1` setup, TTBR0 for user, TTBR1 for kernel |
| BCM2711 BSP (RPi 4) | `arch/arm64/sigma_bcm2711.cpp` | UART, GENET eth, PCIe, VidCore VI |
| BCM2712 BSP (RPi 5) | `arch/arm64/sigma_bcm2712.cpp` | RP1 southbridge, PCIe 2.0, USB3 |
| ARM64 cross-compile toolchain | `toolchain-aarch64-elf.cmake` | `aarch64-linux-gnu-gcc` target |
| sigma-ultra USSD text mode UI | `userland/sigma_ultra.cpp` | 5 menus, 2G compatible, < 16 MB RAM |
| RISC-V PLIC + SV48 MMU | `arch/riscv64/sigma_plic.cpp` | StarFive VisionFive 2 target |
| Neon Kyber-1024 NTT | `crypto/sigma_kyber_neon.cpp` | 5.7× speedup over reference C on ARM |

---

## `release/dual-boot` — Feature Roadmap

### Feature 30: Dual-Boot Install

**Current:** No installer exists. No EFI entry registration. No NTFS driver.

| Task | File | Detail |
|------|------|--------|
| Partition detector | `userland/installer/sigma_part_detect.cpp` | Read GPT/MBR, list existing OS partitions |
| EFI boot entry registration | `sigma-boot/sigma_efi_entry.c` | `EFI_BOOT_MANAGER_PROTOCOL->AddBootEntry()` |
| Shrink existing NTFS partition | `userland/installer/sigma_part_resize.cpp` | libparted-style online resize |
| Windows NTFS read-only mount | `fs/ntfs/sigma_ntfs_ro.cpp` | Parse MFT, expose via sigma-VFS |
| GRUB chainload fallback | `sigma-boot/sigma_grub_chain.c` | Write GRUB2 config with SigmaOS + Windows entries |
| Installer TUI (ncurses-style) | `userland/installer/sigma_installer_ui.cpp` | Disk selection → partition → format → install |
| Uninstall (remove EFI entry + partition) | `userland/installer/sigma_uninstall.cpp` | Clean removal path |

---

## `gh-pages` / `prepare-sigmaos-launch` — Feature Roadmap

### Feature 31: CI Pipeline — Real Tests

**Current:** QEMU tests in `sigma_qemu.yml` are `echo "Simulating..."`. Many test steps use `|| true`.

| Task | File | Detail |
|------|------|--------|
| Wire QEMU boot test to real ISO | `.github/workflows/sigma_qemu.yml` | Remove `echo`, use `qemu-system-x86_64 -cdrom SigmaOS.iso` |
| Assert boot reaches shell in < 30 s | `.github/workflows/sigma_ci.yml` | `timeout 30 bash -c "..."` with exit code check |
| `make check-abi` gate on `SIGMA_STABLE` symbols | `Makefile` | `nm` diff between PRs |
| Wine compat CI | `.github/workflows/sigma_wine_ci.yml` | Run `sigma-wine hello.exe` in QEMU |
| Memory sanitizer build | `.github/workflows/sigma_ci.yml` | `-fsanitize=address,undefined` build variant |
| SPDX header enforcement (fail, not warn) | `.github/workflows/sigma_ci.yml` | Change `|| true` to real exit code check |
| Doxygen → wiki auto-publish | `.github/workflows/sigma_ci.yml` | Run `doxygen Doxyfile` + commit to wiki_repo |

### Feature 32: Website + App Store

| Task | File | Detail |
|------|------|--------|
| Branch status dashboard | `site.js` | Poll GitHub API, show per-branch CI badge |
| Interactive roadmap gantt | `roadmap.html` | D3.js Gantt based on this roadmap file |
| QEMU-in-browser demo | `browser/sigma_qemu_web.js` | `qemu.js` WebAssembly port |
| App store live sigma-pkg integration | `app_store.html` | Query sigma-repo-server for package list |
| Dark/light theme toggle | `site.css` | CSS `prefers-color-scheme` |

---

## Master Feature Checklist (all branches)

| # | Feature | Branch | Priority | Status |

|---|---------|---------|----------|--------|
| 1 | Syscall dispatch (real bodies) | `kernel-exp` | 🔴 | ❌ |
| 2 | VMM page table walker | `kernel-exp` | 🔴 | ⚠️ |
| 3 | UEFI bootloader (`sigma-boot.efi`) | `kernel-exp` | 🔴 | ❌ |
| 4 | MLFQ scheduler body | `kernel-exp` | 🔴 | ⚠️ |
| 5 | APIC + SMP interrupts | `kernel-exp` | 🔴 | ⚠️ |
| 6 | VirtIO-GPU real DMA | `drivers-dev` | 🔴 | ❌ |
| 7 | Intel i915 modesetting | `drivers-dev` | 🔴 | ❌ |
| 8 | e1000 DMA TX/RX rings | `drivers-dev` | 🔴 | ❌ |
| 9 | VFS open/read/write bodies | `fs-dev` | 🔴 | ⚠️ |
| 10 | Tmpfs | `fs-dev` | 🔴 | ❌ |
| 11 | Shell TTY read | `tools-dev` | 🟠 | ❌ |
| 12 | sigma-wine PE VMM mapping | `tools-dev` | 🟠 | ⚠️ |
| 13 | sigma-pkg live repo | `tools-dev` | 🟠 | ❌ |
| 14 | Real Kyber NTT (liboqs) | `performance-optimized` | 🟠 | ❌ |
| 15 | Real Dilithium NTT (liboqs) | `performance-optimized` | 🟠 | ❌ |
| 16 | CryptFS Argon2id (Issue #44) | `kernel-exp` | 🔴 | ❌ |
| 17 | Compositor `composite_window()` | `release/standalone` | 🟠 | ❌ |
| 18 | DID login screen | `release/standalone` | 🟠 | ❌ |
| 19 | sigma-ai llama.cpp | `release/standalone` | 🟠 | ❌ |
| 20 | Indian IME | `release/standalone` | 🟠 | ❌ |
| 21 | sigma-pod kernel cgroup enforcement | `release/cloud` | 🟠 | ⚠️ |
| 22 | ARM64 GIC + MMU | `release/mobile` | 🟡 | ❌ |
| 23 | EDF scheduler | `release/rtos` | 🟡 | ❌ |
| 24 | QEMU CI real tests (not echo) | `gh-pages` | 🟠 | ❌ |
| 25 | sigma-wine hello.exe W1 milestone | `tools-dev` | 🟡 | ❌ |
| 26 | SigmaFS journaling FS | `fs-dev` | 🟡 | ⚠️ |
| 27 | HDA audio driver | `drivers-dev` | 🟡 | ❌ |
| 28 | Wi-Fi iwlwifi/mt7921 | `drivers-dev` | 🟠 | ❌ |
| 29 | sigma-fleet MDM agent | `release/cloud` | 🟡 | ❌ |
| 30 | Dual-boot installer TUI | `release/dual-boot` | 🟡 | ❌ |
| 31 | RAFT consensus CloudFS | `release/distributed` | 🟡 | ❌ |
| 32 | PREEMPT_RT full preemption | `release/rtos` | 🟡 | ❌ |

---

*See also: [Branch Development Roadmap](Branch-Development-Roadmap) · [Windows Compatibility Layer Roadmap](Windows-Compatibility-Layer-Roadmap) · [Gap Analysis](Gap-Analysis) · [Development Roadmap](Development-Roadmap)*

# SigmaOS Kernel Boot Sequence

## Overview

SigmaOS boots through five distinct stages, from UEFI firmware to the `sigma-sh` interactive prompt. Each stage is verified before execution using Secure Boot + measured boot (TPM2 PCR extension).

---

## Stage 0: UEFI Firmware → sigma-boot.efi

**Entry point**: CPU powers on, jumps to UEFI firmware (OVMF in CI, vendor UEFI on real hardware).

**Actions**:
1. UEFI POST: memory init, PCI enumeration, ACPI table construction
2. UEFI Boot Manager loads `\EFI\SIGMA\sigma-boot.efi` from the ESP
3. Secure Boot validates `sigma-boot.efi` against the SigmaOS UEFI certificate
4. TPM2 PCR 4 is extended with the hash of `sigma-boot.efi`

**File**: `arch/boot/sigma-boot.efi` (PE32+ EFI application, built from `arch/boot/sovereign_boot.asm` + EFI stub)

**TODO**:
- [ ] Implement UEFI shim for third-party Secure Boot keys
- [ ] Add UEFI capsule update support for ESP firmware updates

---

## Stage 1: EFI Stub → Load Kernel ELF, Pass Memory Map

**Entry point**: `sigma-boot.efi` `EfiMain()` function.

**Actions**:
1. Call `GetMemoryMap()` — retrieve UEFI memory map
2. Locate `sigma-kernel.elf` on ESP (`\EFI\SIGMA\sigma-kernel.elf`)
3. Parse ELF PT_LOAD headers, allocate memory via `AllocatePages(EfiLoaderData)`
4. Load kernel ELF segments into allocated physical pages
5. Build `SigmaBootInfo` struct: memory map, ACPI RSDP pointer, framebuffer info
6. TPM2: extend PCR 8 with SHA-256 of `sigma-kernel.elf`
7. Call `ExitBootServices()` — UEFI releases control
8. Jump to kernel entry point (ELF `e_entry`)

**File**: `arch/boot/sigma-boot.efi`

**TODO**:
- [ ] Implement kernel ASLR relocation before `ExitBootServices`
- [ ] Add measured boot for initramfs (PCR 9)

---

## Stage 2: `kernel_main()` → GDT → IDT → APIC → Scheduler

**Entry point**: `kernel_main(boot_info: &SigmaBootInfo)` in `kernel/src/main.rs`

**Actions in order**:

```
kernel_main(boot_info)
  │
  ├─ 1. Setup GDT (kernel CS/SS, user CS/SS, TSS)
  │     File: kernel/src/arch/x86_64/gdt.rs
  │
  ├─ 2. Setup IDT (exception handlers 0–31, IRQ handlers 32–255)
  │     File: kernel/src/arch/x86_64/idt.rs
  │
  ├─ 3. Enable paging: identity map kernel, ASLR randomise kernel KASLR offset
  │     File: arch/x86_64/paging.asm + kernel/src/mm/paging.rs
  │
  ├─ 4. Initialise physical memory allocator (buddy allocator, from UEFI mem map)
  │     File: kernel/src/mm/buddy.rs
  │
  ├─ 5. Initialise local APIC, calibrate TSC timer
  │     File: kernel/src/arch/x86_64/apic.rs
  │
  ├─ 6. Start sigma-bus ring buffers (one per logical CPU)
  │     File: kernel/sigma_bus/src/ring.rs
  │
  ├─ 7. Initialise scheduler (EDF for RTOS profile, CFS-inspired for desktop/cloud)
  │     File: kernel/src/sched/mod.rs
  │
  └─ 8. Jump to Stage 3 (VFS mount)
```

**TODO**:
- [ ] SMP: wake secondary CPUs via INIT-SIPI-SIPI, clone GDT/IDT per CPU
- [ ] NUMA: query ACPI SRAT table, set up per-node buddy allocators

---

## Stage 3: VFS Mount → initramfs Extract → sigma-init PID 1

**Entry point**: `kernel_stage3()` called after scheduler is live.

**Actions**:
1. Mount SigmaFS root from `sigma-rootfs.img` (virtio-blk or embedded initramfs)
2. Mount `tmpfs` at `/tmp` and `/run`
3. Mount `devfs` at `/dev`
4. Extract initramfs overlay (gzip/zstd CPIO) to `/`
5. Verify initramfs SHA-256 against boot manifest (TPM2 PCR 9 check)
6. `execve("/sbin/sigma-init", ["sigma-init"], envp)` — PID 1

**Files**:
- `kernel/src/fs/vfs.rs` — VFS mount table
- `kernel/src/fs/sigma_fs.rs` — SigmaFS kernel driver
- `userland/sigma_init/src/main.rs` — PID 1 init

**TODO**:
- [ ] dm-verity: verify root filesystem hash tree at mount time
- [ ] initramfs encryption: decrypt with TPM2-unsealed key before extract

---

## Stage 4: sigma-init → sigma-sh Prompt

**Entry point**: `sigma-init` PID 1 starts as a shard.

**Actions**:
1. Read `/etc/sigma/init.toml` — shard dependency graph
2. Start system shards in dependency order:
   - `sigma-logger` (audit log)
   - `sigma-net` (networking, DHCP)
   - `sigma-audiod` (on desktop profile)
   - `sigma-otel-collector` (observability)
3. Check TPM2 boot health counter (rollback if >= 3 failures)
4. Spawn `sigma-sh` on the primary TTY
5. Print boot banner + `sigma-sh` prompt

**File**: `userland/sigma_init/src/main.rs`

**TODO**:
- [ ] Parallel shard startup (topological sort + async spawn)
- [ ] sigma-init crash recovery: restart any shard that exits non-zero

---

## Full Boot Timeline (Target: < 2s to sigma-sh prompt)

| Stage | Duration target | Milestone |
|---|---|---|
| Stage 0 (UEFI) | ~200ms | UEFI POST |
| Stage 1 (EFI stub) | ~50ms | Kernel loaded |
| Stage 2 (kernel init) | ~100ms | Scheduler running |
| Stage 3 (VFS + init) | ~200ms | PID 1 started |
| Stage 4 (shards) | ~300ms | sigma-sh prompt |
| **Total** | **< 850ms** | |

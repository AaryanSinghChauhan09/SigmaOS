# Minimal SigmaOS v0.1 — Release Specification

> This document defines exactly what "v0.1 Minimal" is, how to build it,
> and what the success criteria are. It is the bridge between architecture
> and a real, downloadable OS.

---

## Goal

Ship a bootable ISO that:

1. Boots on QEMU and real x86_64 hardware via UEFI.
2. Presents a `sigma-sh` command prompt.
3. Can install packages with `sigma-pkg install <name>`.
4. Has a working network connection (DHCP on e1000 / VirtIO-net).
5. Fits in under **150 MB** (ISO).
6. Runs in **under 256 MB RAM**.

This is the "Hello World" of SigmaOS. Everything else follows from this.

---

## What v0.1 Is NOT

- No graphical desktop (Zenith comes in v1.0).
- No Wi-Fi (wired/VirtIO only).
- No audio.
- No GPU acceleration.
- No ARM64 (x86_64 only).
- No online package registry (local sigpkg repo on ISO only).

Keep it small. Keep it honest. Ship it.

---

## Component Checklist

### Bootloader

```
[ ] sigma-boot.efi — UEFI application (C, < 32 KB)
    [ ] UEFI GOP framebuffer init
    [ ] Load kernel ELF from EFI partition
    [ ] Pass memory map to kernel (UEFI memory descriptor)
    [ ] Jump to kernel entry point

[ ] GRUB fallback config (grub.cfg) for legacy BIOS targets
```

### Kernel (minimum viable)

```
[ ] Entry point: arch/x86_64/boot.asm → kernel_main()
[ ] GDT + IDT setup
[ ] APIC init (mask PIC, enable APIC timer)
[ ] HPET timer → jiffies clock
[ ] Buddy allocator: 2^n page-frame management
[ ] Slab allocator: kmalloc / kfree
[ ] x86-64 4-level page table: map kernel + MMIO regions
[ ] Round-robin scheduler: 64 task slots, 10 ms time slice
[ ] 30 syscalls:
    read, write, open, close, exit, fork, exec, wait,
    mmap, munmap, brk, getpid, getppid, kill, signal,
    stat, fstat, lseek, dup, dup2, pipe, chdir, getcwd,
    mkdir, rmdir, unlink, rename, clock_gettime, nanosleep, ioctl
[ ] Printk → UART + framebuffer console
[ ] Kernel panic handler (dump registers, halt)
```

### Drivers

```
[ ] VESA/GOP framebuffer (text console, 80x25 minimum)
[ ] e1000 NIC (QEMU default) — DHCP via sigma-dhcp
[ ] VirtIO-net (QEMU paravirt) — same sigma-net stack
[ ] NVMe (already implemented ✅)
[ ] VirtIO-blk (already implemented ✅)
[ ] xHCI USB (already implemented ✅)
[ ] USB HID keyboard (basic scan-code → ASCII)
[ ] PS/2 keyboard fallback (i8042)
```

### Filesystem

```
[ ] VFS layer: open / read / write / close / stat / readdir
[ ] Tmpfs: RAM-backed, used for /tmp and early userland
[ ] FAT32 read-only: for EFI partition access
[ ] initramfs: cpio archive embedded in kernel, extracted to tmpfs at boot
[ ] SigmaFS read-only mount: for installed packages
```

### Userland (inside initramfs)

```
[ ] sigma-init  (PID 1)
    [ ] Mount /proc, /sys, /dev
    [ ] Run sigma-dhcp on eth0
    [ ] Exec sigma-sh

[ ] sigma-sh    (minimal shell)
    [ ] Read eval print loop
    [ ] Builtin: cd, exit, help, export, echo
    [ ] External command exec via fork/exec
    [ ] PATH lookup
    [ ] Pipes (cmd1 | cmd2)
    [ ] Redirect (>, >>)
    [ ] Signal handling (Ctrl+C, Ctrl+D)

[ ] Minimal coreutils (single static binary, busybox-style)
    [ ] ls, cat, cp, mv, rm, mkdir, rmdir, pwd, touch
    [ ] chmod, chown, ps, kill, top (basic)
    [ ] wget or curl (HTTP only, no TLS required for v0.1)
    [ ] tar, gzip

[ ] sigma-pkg (local mode only for v0.1)
    [ ] Read package index from /var/sigma/pkg/repo/
    [ ] Install: extract .sigpkg to /usr/
    [ ] Remove: delete package files
    [ ] List: show installed packages
    [ ] Commands: install, remove, list, search, info
```

### Package Repository (on ISO)

Minimum 10 packages bundled in the ISO:

```
sigma-hello      # test package: prints "Hello from SigmaOS"
sigma-neofetch   # system info display
sigma-nano       # text editor (nano port)
sigma-git-lite   # read-only git clone
sigma-curl       # HTTP client
sigma-python3    # Python 3.12 minimal
sigma-node       # Node.js 20 LTS minimal
sigma-sigma-sh   # shell updates
sigma-coreutils  # extended coreutils
sigma-nettools   # ip, ping, nslookup
```

### ISO Structure

```
SigmaOS-0.1-x86_64.iso
├── EFI/
│   └── BOOT/
│       └── BOOTX64.EFI          ← sigma-boot.efi
├── boot/
│   ├── sigma-kernel.elf         ← kernel ELF (with initramfs embedded)
│   └── grub/grub.cfg            ← fallback GRUB config
└── repo/
    └── *.sigpkg                  ← 10 bundled packages
```

---

## Build Commands

```bash
# Clone
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Prerequisites (Ubuntu/Debian)
sudo apt install -y build-essential nasm cmake qemu-system-x86 \
  xorriso mtools grub-pc-bin grub-efi-amd64-bin rustup
rustup target add x86_64-unknown-none

# Build v0.1 minimal
make PROFILE=minimal all -j$(nproc)

# Output: build/SigmaOS-0.1-x86_64.iso

# Test in QEMU
make run-qemu
# equivalent to:
qemu-system-x86_64 \
  -cdrom build/SigmaOS-0.1-x86_64.iso \
  -m 256M \
  -serial stdio \
  -net nic,model=e1000 -net user \
  -enable-kvm        # optional, Linux hosts only
```

---

## Test Matrix

| Test | Expected Output | Pass? |
|------|----------------|-------|
| QEMU UEFI boot | `sigma-sh>` prompt | ⬜ |
| `echo hello` | `hello` | ⬜ |
| `ls /` | Lists root dirs | ⬜ |
| `sigma-pkg install sigma-hello` | Downloads + installs | ⬜ |
| `sigma-hello` | `Hello from SigmaOS!` | ⬜ |
| `ping 8.8.8.8 -c 1` | `1 packet received` | ⬜ |
| QEMU shutdown | `Reached target Shutdown` | ⬜ |
| ISO < 150 MB | `du -sh SigmaOS-0.1.iso` | ⬜ |
| RAM < 256 MB | `free -m` shows < 256 used | ⬜ |
| Boots on real hardware | USB boot on x86_64 laptop | ⬜ |

---

## Parallel Work (no kernel dependency)

These can be built **right now** without the bootable kernel:

1. **sigma-pkg local mode** — write and test the package manager against a mock FS.
2. **sigma-sh unit tests** — test the REPL logic against vitest / Rust tests in userspace.
3. **sigpkg format spec** — define the `.sigpkg` archive format and metadata schema.
4. **10-package starter repo** — build and sign the 10 bundled packages.
5. **installer.html backend** — wire the existing GUI to the CLI installer logic.
6. **Community setup** — GitHub Discussions, Discord, issue templates.
7. **Quick-start guide** — `QUICKSTART.md` for contributors and early users.

---

## Success Definition

> v0.1 is done when a person who has never heard of SigmaOS can:
>
> 1. Find the ISO on the GitHub Releases page.
> 2. Boot it in QEMU by following the README.
> 3. Run `sigma-pkg install sigma-hello && sigma-hello` successfully.
> 4. Tell someone else "I ran SigmaOS."

That's it. Everything else — Zenith Desktop, Wi-Fi, ARM, cloud — comes after.

---

## Next Step After v0.1

Ship the ISO → create a GitHub Release tag `v0.1.0` → post on Hacker News / Reddit
r/selfhosted. Get 10 people to boot it and report what broke. Fix those 10 things.
That's v0.2. Repeat.

---

*See also: [ROADMAP.md](../ROADMAP.md) · [INSTALL.md](../INSTALL.md) · [docs/Competitive_Analysis.md](Competitive_Analysis.md)*

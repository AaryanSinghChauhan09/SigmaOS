# SigmaOS Phase G Roadmap — Kernel Completion + Hardware

Phase G is the most critical phase in SigmaOS history.
It turns the OS from a well-designed scaffold into a bootable system.

## Status: ACTIVE (v16.0 Apex target)

## Blocking Items (must finish in order)

### G-01: Kernel Scheduler

- **File:** `kernel/core/sigma_sched.cpp`

- **Goal:** Round-robin scheduler for 64 tasks

- **Test:** QEMU: 2 tasks interleave without deadlock

- **Blocks:** All real hardware boot

### G-02: Physical Memory Manager

- **File:** `kernel/core/sigma_mm.cpp`

- **Goal:** Buddy allocator — alloc/free pages, no leaks

- **Test:** alloc/free 100 pages, then slab 10 000 objects

- **Blocks:** All real hardware boot

### G-03: Virtual Memory Manager

- **File:** `kernel/mm/sigma_vmm.cpp`

- **Goal:** x86-64 4-level page table walker

- **Test:** map 1 MB region, read back correctly

### G-04: IRQ Controller

- **File:** `kernel/core/sigma_irq.cpp`

- **Goal:** APIC + PIC init, HPET timer → jiffies

- **Test:** timer IRQ fires in QEMU

### G-05: Syscall Dispatch

- **File:** `kernel/core/sigma_syscall_dispatch.cpp`

- **Goal:** 30 essential syscalls

- **Test:** `write(1, "hi\n", 3)` from userland works

### G-06: VESA/GOP Framebuffer

- **File:** `drivers/display/sigma_vesa.cpp`

- **Goal:** Pixels on screen in QEMU

- **Test:** A coloured rectangle appears at boot

### G-07: UEFI Bootloader

- **File:** `sigma-boot/sigma_boot.c`

- **Goal:** sigma-boot.efi loads the kernel

- **Test:** QEMU boots to kernel entry point

### G-08: Bootable ISO

- **File:** `Makefile`

- **Goal:** `make iso` produces `SigmaOS.iso`

- **Test:** `qemu-system-x86_64 -cdrom SigmaOS.iso` → sigma-sh prompt

## Phase G Secondary Items

| ID | Area | File | Blocked By |
|----|------|------|------------|
| G-09 | Wi-Fi 6 (iwlwifi) | `drivers/net/sigma_iwlwifi.cpp` | G-08 |
| G-10 | Bluetooth 5.3 (HCI) | `drivers/bt/sigma_hci_usb.cpp` | G-08 |
| G-11 | Developer SDK | `tools/sdk/` | G-08 |
| G-12 | App sandbox (sandboxctl) | `kernel/security/sigma_caps.cpp` | G-05 |
| G-13 | Multi-monitor KMS | `drivers/graphics/sigma_kms.cpp` | G-06 |
| G-14 | CryptFS real key derivation | `kernel/security/sigma_cryptfs.cpp` | G-05 |
| G-15 | Package repo server | `userland/pkg/sigma_repo_server.cpp` | G-08 |
| G-16 | Full TCP/UDP socket layer | `kernel/net/sigma_socket.cpp` | G-05 |

## Phase H — India Stack (blocked until G-08)

| ID | Area | Prerequisite |
|----|------|-------------|
| H-01 | ABDM FHIR API client | G-16 TCP stack |
| H-02 | GST IRN + e-Way Bill API | G-16 TCP stack |
| H-03 | UPI Autopay / mandate | H-02 + TCP |
| H-04 | Local LLM (llama.cpp backend) | G-08 boot |
| H-05 | Indian IME (Inscript + phonetic) | G-06 framebuffer |
| H-06 | sigma-bhashini offline models | H-04 + audio stack |
| H-07 | Federated learning coordinator | H-04 + TCP |
| H-08 | CBDC e-rupee wallet | H-03 UPI stack |

---

*Track progress: [CURRENT_PROBLEMS_MANIFEST.md](CURRENT_PROBLEMS_MANIFEST.md)*

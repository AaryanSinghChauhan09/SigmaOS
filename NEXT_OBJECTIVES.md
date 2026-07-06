# SigmaOS Next Objectives

Prioritised list of the next concrete tasks. This page tracks what's actively being worked on vs what's next in queue.

---

## 🔴 Active Right Now (Phase G — Kernel Boot)

These are blocking every other feature. In strict execution order:

1. **Round-robin scheduler** — `kernel/core/sigma_sched.cpp`
   - 64-task round-robin, QEMU test: 2 tasks interleave

2. **Buddy physical allocator** — `kernel/core/sigma_mm.cpp`
   - Alloc/free 100 pages, no leak; slab for kmalloc

3. **x86-64 page table walker** — `kernel/mm/sigma_vmm.cpp`
   - Map 1 MB region, read back correctly

4. **APIC + PIC init + HPET timer** — `kernel/core/sigma_irq.cpp`
   - Timer IRQ fires in QEMU at 100 Hz

5. **30-syscall dispatch** — `kernel/core/sigma_syscall_dispatch.cpp`
   - `write(1,"hi\n",3)` from userland shell works

6. **VESA/GOP framebuffer** — `drivers/display/sigma_vesa.cpp`
   - Pixels on screen in QEMU

7. **sigma-boot.efi** — `sigma-boot/sigma_boot.c`
   - QEMU boots to kernel via UEFI

8. **`make iso`** — `Makefile`
   - `qemu-system-x86_64 -cdrom SigmaOS.iso` → sigma-sh prompt

---

## 🟠 Next After Phase G (Immediate)

| Task | Branch | Description |
|------|--------|-------------|
| VESA SDF driver | drivers-dev | Framebuffer as a proper SDF driver |
| VirtIO-GPU | drivers-dev | QEMU accelerated GPU |
| Intel i915 modesetting | drivers-dev | Intel iGPU basic display |
| VFS bodies | fs-dev | open/read/write/close implementations |
| Tmpfs | fs-dev | RAM-backed filesystem for early boot |
| CryptFS key fix | kernel-exp | Fix #1009 — derive_key() returns zeros |
| MLFQ scheduler | kernel-exp | Upgrade from round-robin |
| CFS clone | performance-optimized | vruntime + red-black tree |
| ARM64 GIC | release/mobile | Interrupt controller for RPi 4/5 |
| ARM64 MMU | release/mobile | Page tables on ARM64 |

---

## 🟡 Phase G Secondary

| ID | Task | File |
|----|------|------|
| #851-WLAN | Wi-Fi 6 iwlwifi driver | `drivers/net/sigma_iwlwifi.cpp` |
| #851-BT | Bluetooth 5.3 HCI | `drivers/bt/sigma_hci_usb.cpp` |
| #1000 | Developer SDK | `tools/sdk/` |
| #1001 | App sandbox (sandboxctl) | `kernel/security/sigma_caps.cpp` |
| #1002 | Multi-monitor KMS | `drivers/graphics/sigma_kms.cpp` |
| #1011 | Package repo server | `userland/pkg/sigma_repo_server.cpp` |
| #1012 | Full TCP RFC 793 socket layer | `kernel/net/sigma_socket.cpp` |

---

## 🟢 Documentation & Community (Ongoing)

- [ ] Doxygen wired to CI

- [ ] Man pages for 50 CLI tools (`docs/man/*.1`)

- [ ] QEMU smoke test in CI

- [ ] `make check-abi` gate

- [ ] Interactive branch status dashboard on gh-pages

- [ ] First public contributor meetup / hackathon planning

---

## ✅ Recently Completed

- Kyber-1024 KEM + Dilithium-5 PQC signatures

- WASM/WASI runtime

- Linux ELF compat layer

- Native KMS/GPU framework

- PCIe MSI-X HAL

- Cgroup enforcement

- Sovereign Package Registry

- Offline-First CRDT sync

- Native Performance Governor

- ARCHITECTURE.md, GOVERNANCE.md, INSTALL.md, SUPPORT.md

- Complete wiki core reference pages

---

*Track bugs: [CURRENT_PROBLEMS_MANIFEST.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CURRENT_PROBLEMS_MANIFEST.md) · See also: [Development-Roadmap](Development-Roadmap)*

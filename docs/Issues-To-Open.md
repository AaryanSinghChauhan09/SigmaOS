# SigmaOS Issues To Open

Pre-written issue descriptions ready to file on GitHub. File these as they become active work.

---

## Phase G — Kernel Boot (File Immediately)

### [KERNEL] Implement round-robin scheduler — Phase G-01
**Label:** `kernel`, `phase-g`, `good first issue (advanced)`
**File:** `kernel/core/sigma_sched.cpp`
**Body:**
Implement a simple 64-task round-robin scheduler. The scheduler must:
- Maintain a circular task queue
- Context switch on timer tick (APIC, 10ms quantum)
- Pass QEMU test: 2 tasks printing alternately without deadlock
- Exit criteria: `make qemu` shows interleaved output from 2 tasks

### [KERNEL] Implement buddy physical allocator — Phase G-02
**Label:** `kernel`, `phase-g`
**File:** `kernel/core/sigma_mm.cpp`
**Body:**
Implement a buddy allocator for physical page frames:
- Support orders 0–10 (4 KB – 4 MB blocks)
- `alloc_pages(order)` and `free_pages(ptr, order)`
- No leaks: alloc 100 pages, free all, realloc should succeed
- Slab on top: `kmalloc(size)` / `kfree(ptr)` with 10k object test

### [KERNEL] APIC + HPET timer initialisation — Phase G-04
**Label:** `kernel`, `phase-g`
**File:** `kernel/core/sigma_irq.cpp`
**Body:**
Initialise APIC and HPET for timer interrupts:
- Parse ACPI MADT to locate APIC base
- Initialise local APIC timer at 100 Hz
- Test: timer IRQ fires in QEMU; `jiffies` counter increments

### [BOOT] Implement sigma-boot.efi UEFI loader — Phase G-07
**Label:** `boot`, `phase-g`, `critical`
**File:** `sigma-boot/sigma_boot.c`
**Body:**
Implement a minimal UEFI bootloader:
- Load kernel ELF from ESP
- Build memory map for physical MM init
- Set up identity-mapped page tables (first 4 GB)
- Jump to `sigma_kernel_main()`
- Test: `qemu-system-x86_64 -cdrom SigmaOS.iso` boots to kernel

---

## Phase G — Drivers

### [DRIVER] VESA/GOP framebuffer SDF driver — Phase G-06
**Label:** `driver`, `display`, `phase-g`
**File:** `drivers/display/sigma_vesa.cpp`
**Body:**
Implement framebuffer driver using UEFI GOP (Graphics Output Protocol):
- Detect framebuffer base, width, height, pitch from GOP
- Provide `sigma_fb_write_pixel(x, y, rgb)` API
- Test: coloured rectangle visible in QEMU `-display gtk`

### [DRIVER] Intel iwlwifi Wi-Fi 6 driver — Phase G-09
**Label:** `driver`, `net`, `phase-g`
**File:** `drivers/net/sigma_iwlwifi.cpp`
**Body:**
Implement 802.11ax driver for Intel wireless NICs:
- Probe via PCI (vendor 0x8086, device IDs for AX200/AX201/AX210)
- Init firmware loading via sigma-firmwared
- Implement `scan`, `connect`, `disconnect` ops
- Test: QEMU passthrough or physical Intel NIC associates with AP

---

## Security

### [SECURITY] Fix CryptFS key derivation — Issue #1009
**Label:** `security`, `critical`, `phase-g`
**File:** `kernel/security/sigma_cryptfs.cpp`
**Body:**
`derive_key()` currently returns 32 zero bytes. All CryptFS-encrypted volumes are trivially decryptable.
Fix: implement Argon2id key derivation from passphrase + salt:
```cpp
sigma_status derive_key(const char* passphrase, const uint8_t* salt, 
                         size_t salt_len, uint8_t* key_out, size_t key_len);
```
Use `security/SovereignEntropy.cpp` for salt generation.
Test: encrypted file not readable after password change.

---

## Networking

### [NET] Complete TCP RFC 793 state machine — Issue #1012
**Label:** `net`, `phase-g`
**File:** `kernel/net/sigma_socket.cpp`
**Body:**
Current TCP implementation is missing the full state machine.
Implement: SYN_SENT → ESTABLISHED → FIN_WAIT → CLOSED
Required for all application networking.
Test: `curl --unix-socket` to `sigmad-netd` returns HTTP 200.

---

*File these at: https://github.com/AaryanSinghChauhan09/SigmaOS/issues/new*

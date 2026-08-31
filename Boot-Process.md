# 🚀 SigmaOS Boot Process

## Boot Sequence Overview

```
Power On
  ↓
UEFI Firmware
  ↓ (Secure Boot verification)
Sigma Bootloader (EFI stub)
  ↓ (Kernel signature check)
Sigma Kernel (ELF64)
  ↓ (Hardware init)
sigma-init (PID 1)
  ↓ (Service activation)
System Services
  ↓
Login Manager
  ↓
User Session (Desktop/SSH)
```

## Phase 1: UEFI Firmware

1. **POST (Power-On Self Test)**: Hardware enumeration
2. **UEFI initialization**: Boot device selection, GOP (Graphics Output Protocol) init
3. **Secure Boot check**: Verify bootloader signature against UEFI DB keys
4. **Boot manager**: Load `\EFI\sigmaos\sigma.efi`

## Phase 2: Sigma Bootloader

The Sigma bootloader is an EFI stub that:
1. **Validates** kernel image using Dilithium-5 signature
2. **Sets up** memory map from UEFI `GetMemoryMap()`
3. **Loads** kernel, initramfs, and command line (Unified Kernel Image)
4. **Measures** all components into TPM PCR registers
5. **Transfers** control to kernel entry point

### TPM PCR Measurements
| PCR | Contents |
|-----|---------|
| 0 | Core firmware |
| 4 | Bootloader |
| 7 | Secure Boot state |
| 11 | Kernel + initramfs hash |
| 14 | Command line |

## Phase 3: Kernel Initialization

### Architecture Init (x86_64)
1. Switch from real mode → protected mode → long mode
2. Set up GDT (Global Descriptor Table)
3. Initialize IDT (Interrupt Descriptor Table)
4. Set up page tables (PML4 identity mapping)
5. Enable SMEP/SMAP/NX/PCID via CR4

### Memory Init
1. Parse UEFI memory map → memory zones
2. Initialize buddy allocator
3. Map kernel virtual address space (KASLR offset applied)
4. Initialize slab allocator

### CPU Init
1. Initialize per-CPU data structures
2. Set up APIC (Advanced Programmable Interrupt Controller)
3. Calibrate TSC (Time Stamp Counter)
4. Enable all CPU cores (SMP)
5. Initialize NUMA topology

### Device Init
1. Scan PCI/PCIe bus
2. Load essential drivers (storage, network)
3. Mount initramfs (EROFS or cpio)
4. Probe ACPI tables for power management

## Phase 4: sigma-init (PID 1)

sigma-init is SigmaOS's init system — compatible with systemd unit files:

### Unit Types Supported
| Unit Type | Example |
|-----------|---------|
| Service | `nginx.service` |
| Target | `multi-user.target` |
| Timer | `backup.timer` |
| Socket | `ssh.socket` |
| Mount | `home.mount` |
| Swap | `swapfile.swap` |

### Activation Phases
```
sysinit.target
  ↓
basic.target
  ↓
network.target
  ↓
multi-user.target
  ↓
graphical.target (if desktop)
```

### Features
- **Topological sort**: Resolves dependency order (detects cycles)
- **Parallel startup**: Units without dependencies start simultaneously
- **Socket activation**: Services started on first connection
- **Socket activation**: Sockets opened before service starts (zero downtime)
- **Auto-restart**: Configurable restart policy (always, on-failure, etc.)
- **systemd-analyze blame**: Boot time attribution per unit

## Boot Security Hardening

| Feature | Description |
|---------|-------------|
| Secure Boot | Only signed binaries can execute |
| TPM sealing | Disk key sealed to expected PCR values |
| KASLR | Kernel loaded at random offset |
| dm-verity | Root filesystem integrity verification |
| Initramfs hash | Initramfs integrity via PCR |
| Kernel lockdown | Restrict direct hardware access after boot |

## Troubleshooting

### Boot Hangs
1. Disable Secure Boot temporarily to isolate signature issues
2. Boot with `debug` kernel parameter for verbose output
3. Check sigma-journal logs: `journalctl -b -1`

### TPM Issues
1. Clear TPM and re-enroll in UEFI settings
2. Verify PCR values haven't changed: `tpm2_pcrread`
3. Re-seal disk key after firmware update

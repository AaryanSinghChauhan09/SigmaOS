# Boot-to-Login Path Specification

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2025-01-22  
**Purpose:** Define the smallest complete boot-to-login path for SigmaOS

---

## 1. Overview

This specification defines the minimal boot-to-login path for SigmaOS, targeting the M1 milestone (QEMU bootable preview). The goal is to establish a reproducible, testable boot sequence that can be verified in CI.

---

## 2. Boot Sequence

### 2.1 Hardware Boot

```
UEFI/BIOS
  → Load bootloader from EFI System Partition or MBR
  → Verify bootloader signature (secure boot)
  → Execute bootloader
```

**Target:** QEMU x86_64 with OVMF (UEFI) or SeaBIOS (BIOS)

### 2.2 Bootloader

```
Bootloader (systemd-boot or GRUB)
  → Load kernel image
  → Load initramfs
  → Pass kernel command line
  → Verify kernel signature
  → Jump to kernel entry point
```

**Components:**
- Bootloader configuration in `/boot/loader/entries/` or `/boot/grub/`
- Kernel image at `/boot/vmlinuz-sigmaos`
- Initramfs at `/boot/initramfs-sigmaos.img`
- Command line parameters for init system

### 2.3 Kernel Initialization

```
Kernel
  → Early boot (assembly entry)
  → Setup page tables
  → Initialize GDT/IDT
  → Initialize memory management
  → Initialize interrupt handlers
  → Initialize scheduler
  → Initialize driver subsystem
  → Mount root filesystem
  → Switch to user mode
  → Execute init (PID 1)
```

**Components:**
- `src/boot/` - Bootloader interface
- `src/kernel/` - Core kernel
- `src/arch/` - Architecture-specific code
- `src/drivers/` - Driver initialization

### 2.4 Init System

```
Init (PID 1)
  → Parse configuration
  → Mount virtual filesystems (/proc, /sys, /dev)
  → Initialize device nodes
  → Start service supervisor
  → Start essential services
  → Start login service
```

**Components:**
- `src/userland/init/` - Init system
- Service supervisor
- Service configuration in `/etc/sigma-init/`

### 2.5 Login

```
Login Service
  → Display login prompt
  → Authenticate user
  → Start user session
  → Execute shell
```

**Components:**
- Login daemon
- User authentication
- Session management
- Shell execution

---

## 3. Component Specifications

### 3.1 Bootloader

**Requirements:**
- Support UEFI and BIOS boot
- Load kernel and initramfs
- Pass kernel command line
- Verify signatures (optional for M1)

**Implementation:**
- Use systemd-boot for UEFI (simpler than GRUB)
- Generate loader entries during installation
- Store configuration in `/boot/loader/entries/sigmaos.conf`

**Example Configuration:**
```
title SigmaOS
linux /vmlinuz-sigmaos
initrd /initramfs-sigmaos.img
options root=UUID=xxxx-xxxx-xxxx-xxxx ro quiet
```

### 3.2 Kernel

**Requirements:**
- Boot on x86_64 QEMU
- Initialize basic hardware (CPU, memory, timers)
- Mount root filesystem
- Execute init

**Implementation:**
- Minimal kernel for M1
- Basic scheduler (round-robin sufficient)
- Basic memory management (buddy allocator)
- Basic driver support (virtio-blk, virtio-net)
- Simple init execution

**Source Locations:**
- `src/boot/bootloader.rs` - Bootloader interface
- `src/kernel/main.rs` - Kernel entry point
- `src/kernel/memory/` - Memory management
- `src/kernel/scheduler/` - Process scheduler
- `src/arch/x86_64/` - x86_64-specific code

### 3.3 Init System

**Requirements:**
- PID 1 process
- Mount virtual filesystems
- Start essential services
- Start login service

**Implementation:**
- Simple init for M1
- Static service list (no dynamic discovery)
- Basic service supervision
- Logging to `/var/log/sigma-init.log`

**Service List (M1):**
1. `udev` - Device node management
2. `network` - Network configuration
3. `login` - Login service

**Source Location:**
- `src/userland/init/` - Init system

### 3.4 Login Service

**Requirements:**
- Display login prompt
- Authenticate single user (root)
- Start shell
- Handle logout

**Implementation:**
- Simple login daemon for M1
- Single user (root) with no password
- Start sigma-sh shell
- Basic session management

**Source Location:**
- `src/userland/login/` - Login service

### 3.5 Shell

**Requirements:**
- Execute commands
- Basic built-ins (cd, pwd, exit)
- Simple variable expansion
- Pipeline support

**Implementation:**
- Use sigma-sh specification
- Minimal implementation for M1
- Focus on basic functionality

**Source Location:**
- `src/shell/` - Shell implementation

---

## 4. Filesystem Layout

### 4.1 Root Filesystem

```
/
├── boot/
│   ├── vmlinuz-sigmaos
│   ├── initramfs-sigmaos.img
│   └── loader/
│       └── entries/
│           └── sigmaos.conf
├── bin/
│   ├── sh (sigma-sh)
│   ├── cat
│   ├── ls
│   └── ...
├── etc/
│   ├── sigma-init/
│   │   ├── services/
│   │   └── config
│   ├── passwd
│   └── group
├── lib/
├── proc/ (procfs)
├── sys/ (sysfs)
├── dev/ (devtmpfs)
├── var/
│   └── log/
│       └── sigma-init.log
└── home/
    └── root/
```

### 4.2 Initramfs Layout

```
initramfs/
├── init (init script)
├── bin/
│   ├── mount
│   └── switch_root
└── lib/
```

---

## 5. Testing Strategy

### 5.1 QEMU Boot Test

**Test Script:**
```bash
#!/bin/bash
# QEMU boot smoke test

qemu-system-x86_64 \
  -m 2048 \
  -enable-kvm \
  -drive file=sigmaos.qcow2,format=qcow2 \
  -nographic \
  -serial mon:stdio \
  -monitor none

# Expected output:
# 1. Bootloader messages
# 2. Kernel boot messages
# 3. Init startup messages
# 4. Login prompt
```

**Success Criteria:**
- Bootloader loads kernel
- Kernel boots without panic
- Init starts successfully
- Login prompt appears
- Can log in as root
- Can execute basic commands

### 5.2 CI Integration

**GitHub Actions Workflow:**
```yaml
name: QEMU Boot Test

on: [push, pull_request]

jobs:
  qemu-boot:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build ISO
        run: make iso
      - name: Boot in QEMU
        run: scripts/qemu-boot-test.sh
      - name: Verify login
        run: scripts/verify-login.sh
```

---

## 6. Implementation Status

| Component | Status | Implementation | Tests |
|-----------|--------|----------------|-------|
| Bootloader | Not started | None | None |
| Kernel | Partially implemented | `src/kernel/` stubs | Basic compilation |
| Init | Prototype | `src/userland/init/` stubs | Basic tests |
| Login | Not started | None | None |
| Shell | Prototype | `src/shell/` stubs | Parsing tests |
| Coreutils | Partially implemented | `src/userland/coreutils/` | Unit tests |

---

## 7. Next Steps

### Phase 1: Kernel Boot
1. Implement bootloader interface
2. Implement kernel entry point
3. Implement basic memory management
4. Implement basic scheduler
5. Test kernel boot in QEMU

### Phase 2: Init System
1. Implement init (PID 1)
2. Implement virtual filesystem mounting
3. Implement service supervisor
4. Implement service configuration
5. Test init startup

### Phase 3: Login
1. Implement login daemon
2. Implement user authentication
3. Implement session management
4. Test login flow

### Phase 4: Integration
1. Integrate all components
2. Test full boot-to-login
3. Add CI tests
4. Document known issues

---

## 8. Success Metrics

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| QEMU boot success | 100% | 0% | ❌ |
| Kernel to init transition | Working | None | ❌ |
| Init to login transition | Working | None | ❌ |
| Login to shell transition | Working | None | ❌ |
| Basic command execution | Working | Partial | ⚠️ |

---

## 9. References

- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Overall project status
- [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md) - Architecture decisions
- [SUPPORT_MATRIX.md](SUPPORT_MATRIX.md) - Hardware support
- [Future Development Plan](SIGMAOS_STRATEGIC_DEVELOPMENT_PLAN_LINUX_BSD.md) - Strategic plan

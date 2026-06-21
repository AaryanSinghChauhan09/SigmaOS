---
name: 🐛 Bug Report
about: Report a defect, crash, or incorrect behavior
title: "fix(<subsystem>): <short description>"
labels: ["bug", "triage"]
assignees: []

---

## Bug Description

<!-- A clear, concise description of what the bug is. -->

## Subsystem

<!-- Which subsystem is affected? -->
- [ ] `kernel/core` — Scheduler, namespaces, syscall gate
- [ ] `kernel/memory` — VMM, page tables, buddy allocator
- [ ] `kernel/security` — Zero-Trust, PAM/ACL, audit trail
- [ ] `drivers/storage` — NVMe, AHCI
- [ ] `drivers/usb` — xHCI host controller
- [ ] `drivers/graphics` — KMS, GPU
- [ ] `net` — TCP/IP, ARP, E1000/RTL8139
- [ ] `fs` — Ext4, VFS
- [ ] `crypto` — Kyber, Dilithium
- [ ] `zenith_desktop` — Compositor, WM, settings
- [ ] `userland` — Shell, pkg manager
- [ ] `hal` — PCIe, ACPI, power
- [ ] Other: ___

## Steps to Reproduce

1. 
2. 
3. 

## Expected Behavior

<!-- What should have happened? -->

## Actual Behavior

<!-- What actually happened? Include crash output, error codes, etc. -->

```
<paste output here>
```

## Error Code

<!-- If applicable, paste the ZEN_DRV_xxxx or K_ERR_xxxx code -->

## Environment

| Field | Value |
|-------|-------|
| Profile | `standalone` / `iot-arm64` / other |
| Host OS | e.g. Ubuntu 24.04 |
| QEMU version | e.g. 8.2.1 |
| Commit | `git rev-parse HEAD` output |

## Additional Context

<!-- Any other context, logs, or screenshots -->

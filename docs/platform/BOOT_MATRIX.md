# SigmaOS Platform Boot Matrix

## Validation Policy
Passing unit tests does not establish OS bootability. Boot viability requires end-to-end emulation or hardware execution reaching an interactive or diagnostic prompt.

SigmaOS tracks bootability across multiple architectures and execution environments under three evidence gates:
- **Level 1 (Code Exists)**: Architecture stubs and bootloader interfaces are written.
- **Level 2 (Tests Pass)**: Kernel bootstrap unit tests pass in isolation.
- **Level 3 (System Behavior Verified)**: End-to-end boot to deterministic shell marker via QEMU or bare-metal execution.

---

## Target Matrix

| Target Platform | Architecture | Mode | Level 1 (Code) | Level 2 (Tests) | Level 3 (Verified) | Latest Evidence |
|---|---|---|:---:|:---:|:---:|---|
| QEMU Standard PC (i440FX + PIIX) | x86_64 | BIOS / UEFI | Yes | Yes | Yes | `build/boot/boot_smoke_result.json` |
| QEMU Q35 Chipset | x86_64 | UEFI | Yes | Yes | Yes | `scripts/ci/qemu_boot_smoke.sh` |
| QEMU Virt Machine | aarch64 | UEFI | Yes | Yes | Partial | Model verified; QEMU smoke ongoing |
| Generic x86_64 PC | x86_64 | UEFI | Yes | Yes | Untested | Pending physical hardware test run |

---

## Deterministic Boot Milestones
A verified boot sequence must traverse and log all 11 lifecycle markers without panics:
1. `firmware` (Firmware Bridge initialized)
2. `bootloader` (ACPI & TPM measured boot active)
3. `kernel_entry` (Kernel entry point executed)
4. `arch_init` (GDT/IDT/paging configured)
5. `memory_init` (Physical & virtual memory manager ready)
6. `interrupt_init` (APIC & timer interrupts enabled)
7. `scheduler_init` (Task queues and idle thread created)
8. `vfs_init` (Root filesystem mounted)
9. `driver_init` (Core console/serial drivers attached)
10. `userspace_start` (PID 1 init process spawned)
11. `shell_ready` (Interactive shell prompt active)

# SigmaOS Truthful Machine-Readable Capability Matrix
# Priority 0 Baseline: Separates implemented, partial, prototype, design proposal, and not_implemented.
# Release validation criteria: Every claim must be backed by unit tests or QEMU integration tests.

[meta]
target_architecture = "x86_64"
reference_platform = "qemu-system-x86_64"
version = "0.9.0-alpha"
zero_external_dependencies = true

# -----------------------------------------------------------------------------
# 1. Boot, CPU, and Low-Level Foundations (Inspiration: cfenollosa/os-tutorial)
# -----------------------------------------------------------------------------

[feature.gdt_idt_bootstrap]
status = "implemented"
tested_on = ["qemu-x86_64", "unit-test"]
required_for_release = true
description = "Early GDT and IDT setup with PIC 8259 remap (IRQs 32-47) preventing CPU exception collisions."

[feature.pic_8259_interrupts]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Dual 8259 PIC master/slave port I/O initialization, mask control, and EOI dispatch."

[feature.pit_8254_timer]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Programmable Interval Timer tick tracking at 100Hz using 1.193182 MHz oscillator divider."

[feature.vga_text_mode]
status = "implemented"
tested_on = ["qemu-x86_64"]
required_for_release = true
description = "VGA buffer text mode at 0xB8000 with 80x25 terminal scrolling and color attributes."

[feature.uefi_boot]
status = "prototype"
tested_on = ["ovmf-x86_64"]
required_for_release = true
description = "GOP framebuffer query and EFI memory map transition to 64-bit long mode."

# -----------------------------------------------------------------------------
# 2. Kernel Primitives & Linux ABI Compatibility (Inspiration: torvalds/linux)
# -----------------------------------------------------------------------------

[feature.linux_syscall_abi]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Direct x86_64 syscall dispatcher supporting read, write, open, close, mmap, brk, clone, exit, and io_uring."

[feature.task_scheduler_eevdf]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Earliest Eligible Virtual Deadline First (EEVDF) scheduler inspired by Linux 6.6+."

[feature.bore_scheduler]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = false
description = "Burst-Oriented Response Enhancer (BORE) interactivity-biased CPU scheduling."

[feature.memory_buddy_allocator]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Binary buddy memory allocator managing page frames across memory zones."

[feature.vfs_posix_primitives]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "POSIX-compliant VFS inode/dentry hierarchy with file descriptors, pipes, and fifos."

# -----------------------------------------------------------------------------
# 3. Desktop Ergonomics & Omakase Workflow (Inspiration: omacom/omarchy)
# -----------------------------------------------------------------------------

[feature.omakase_tiling_engine]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Automated master-stack tiling window manager configuration with configurable inner/outer gaps."

[feature.omarchy_theme_synthesis]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Zero-friction dark/light developer theme palette, font fallback, and terminal padding."

[feature.wayland_zenith_compositor]
status = "prototype"
tested_on = ["qemu-virtio-gpu"]
required_for_release = true
description = "Zenith Wayland compositor backend handling client surface commits and input focus."

# -----------------------------------------------------------------------------
# 4. Mint Userland Stability & Safe System Updates (Inspiration: linuxmint)
# -----------------------------------------------------------------------------

[feature.mint_update_level_automation]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "5-tier update classification policy preventing risky package upgrades on production systems."

[feature.timeshift_atomic_rollback]
status = "implemented"
tested_on = ["unit-test"]
required_for_release = true
description = "Automated pre-install snapshot retention and rollback triggers for Btrfs/ZFS volumes."

[feature.installer_dry_run_and_hardware_partitioning]
status = "partial"
tested_on = ["qemu-x86_64"]
required_for_release = true
description = "Guided installer with real block device probe, dry-run safety validation, and partition generation."

[feature.wifi_networking]
status = "partial"
tested_on = ["qemu-e1000"]
required_for_release = true
description = "Ethernet functional; Wi-Fi 802.11 stack in hardware testing phase."

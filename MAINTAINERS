	List of maintainers and how the SigmaOS kernel is modelled (inspired by
	torvalds/linux MAINTAINERS). Fields:

	M: Mail patches to: FullName <email>
	R: Designated reviewer: FullName <email>
	L: Mailing list / GitHub Discussion where patches are sent
	S: Status, one of the following:
	   Supported   - someone is actively maintaining this
	   Maintained  - someone actually looks after it
	   Odd Fixes   - not actively maintained; patches accepted
	   Orphan      - no current maintainer; adopt before changing
	   Obsolete    - old code, kept for compat; actively being removed
	T: SCM tree type (git)
	F: Files and directories with wildcard patterns (shell)
	X: Files excluded from the above patterns (shell)
	N: Files matching pattern (regex)
	K: Keyword patterns matching commit subject lines

Descriptions:

THE REST
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
T: git https://github.com/AaryanSinghChauhan09/SigmaOS.git main
F: *

-------------------------------------------------------------------
ARCH — X86_64
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: arch/x86_64/
F: arch/boot/
F: kernel/arch/
F: kernel/core/boot/
K: x86_64 idt gdt apic

ARCH — ARM64 / AARCH64
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: arch/arm64/
F: kernel/arch/arm64/
K: arm64 aarch64 gic

ARCH — RISC-V
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Odd Fixes
F: arch/riscv/
F: kernel/arch/riscv/
K: riscv rv64

-------------------------------------------------------------------
KERNEL CORE — SCHEDULER
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kernel/core/sovereign_overlord.rs
F: kernel/core/sigma_sched.rs
F: kernel/core/core/sigma_sched.rs
F: kernel/sched/
F: kernel/core/sched/
K: scheduler mlfq cfs edf runqueue

KERNEL CORE — MEMORY MANAGEMENT
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kernel/memory/
F: kernel/mm/
K: buddy slab paging tlb vma aslr

KERNEL CORE — SYSCALL DISPATCH
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kernel/core/sigma_syscalls_io.rs
F: kernel/core/sigma_syscalls_proc.rs
F: kernel/core/syscall_dispatch.rs
F: kernel/core/sigma_syscall_dispatch.rs
K: syscall dispatch open read write fork execve

KERNEL CORE — IRQ / INTERRUPT HANDLING
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kernel/core/irq_controller.rs
F: kernel/core/sovereign_idt.rs
F: kernel/core/sigma_irq.rs
F: kernel/core/sigma_irq.zig
K: irq pic apic pit interrupt

-------------------------------------------------------------------
SECURITY SUBSYSTEM
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: security/
F: kernel/security/
F: kernel/core/sigma_pledge.rs
F: kernel/security/sigma_seccomp.rs
F: crypto/
K: security pledge unveil seccomp sandbox pqc kyber dilithium

-------------------------------------------------------------------
FILESYSTEMS — VFS LAYER
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kernel/fs/
F: fs/
K: vfs filesystem tmpfs sigmafs ext4 fat32 vnode

FILESYSTEMS — SIGMAFS
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: kernel/fs/sigma_sigmafs.rs
F: fs/sigmafs.zig
K: sigmafs cow snapshot

FILESYSTEMS — EXT4
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: kernel/fs/sigma_ext4.rs
F: kernel/fs/ext4/
F: fs/ext4_journal.c
K: ext4 jbd2 journal

-------------------------------------------------------------------
NETWORK STACK — TCP/IP
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kernel/net/
F: net/
K: tcp udp ip socket network

NETWORK STACK — WI-FI
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: drivers/wifi/
K: wifi iwlwifi mt7921 wpa3 sae 80211

NETWORK STACK — TLS / PQC-TLS
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kernel/net/sigma_tls.rs
F: net/tls/
K: tls kyber pqc handshake

-------------------------------------------------------------------
DRIVERS — CORE FRAMEWORK (SDF)
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: sdk/driver/
F: drivers/
F: kernel/core/driver_framework.rs
K: driver sdf abi ddk hotplug

DRIVERS — STORAGE (NVMe / SATA)
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: drivers/storage/
K: nvme sata ahci blk storage

DRIVERS — USB
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: drivers/usb/
K: usb xhci hid

DRIVERS — GPU / DRM / KMS
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: drivers/gpu/
F: drivers/graphics/
K: gpu kms drm framebuffer virtio-gpu

DRIVERS — AUDIO
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Odd Fixes
F: drivers/audio/
K: audio hda alsa pipewire

-------------------------------------------------------------------
PACKAGE MANAGER — SIGPKG / SIGMA-PKG
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: sigma-pkg/
F: pkg/
F: userland/sigpkg/
K: sigpkg package install remove search

-------------------------------------------------------------------
SHELL — SIGMA-SH
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: sigma-sh/
K: sigma-sh shell repl scripting

-------------------------------------------------------------------
ZENITH DESKTOP
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: zenith_desktop/
F: desktop/
K: zenith compositor wayland desktop wm tiling

-------------------------------------------------------------------
AI SUBSYSTEM — SIGMA-AI
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: userland/ai/
F: tools/sigma_nl_cli.py
K: sigma-ai llm gguf inference nlp

-------------------------------------------------------------------
BUILD SYSTEM / CI
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: Makefile
F: CMakeLists.txt
F: Cargo.toml
F: .github/workflows/
F: cmake/
K: build cmake cargo ninja ci actions

-------------------------------------------------------------------
DOCUMENTATION
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: docs/
F: wiki_repo/
F: README.md
K: docs wiki documentation

-------------------------------------------------------------------
KABI — KERNEL ABI STABILITY
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: kabi/
K: kabi abi stability

-------------------------------------------------------------------
VIRTUALIZATION / OCI RUNTIME
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Maintained
F: virtualization/
K: virtualization oci container qemu firecracker

-------------------------------------------------------------------
INDIA STACK INTEGRATIONS
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Odd Fixes
F: userland/india/
F: tools/gst_calc*
K: gst upi abdm india bhashini

-------------------------------------------------------------------
SDK
M: Aaryan Singh Chauhan <aaryansinghchauhan09@github>
S: Supported
F: sdk/
K: sdk developer app driver

-------------------------------------------------------------------

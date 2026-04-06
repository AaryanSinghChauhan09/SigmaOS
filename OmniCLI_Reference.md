# Σ OmniCLI: The Sovereign Dispatcher Reference (v3.0)

SigmaOS operations are orchestrated via the `sigma` command. This native C11 dispatcher maps directly to kernel shards, neutralizing the need for high-level wrappers or legacy shells.

## 🏛️ Core Modules Matrix

### ⚙️ System & Kernel (`sys`, `ps`, `shard`, `cron`, `hook`)

- `sigma sys <kill|tune|irq|info>`: Direct kernel-level resource management.
- `sigma ps`: Advanced process scheduler and cgroup hierarchy control.
- `sigma shard <load|unload|reload>`: Hot-modular execution environment control.
- `sigma cron`: Native task scheduling without background daemons.
- `sigma hook`: Hardware event binding (USB, Wifi, Battery).

### 🌌 Linux USP Absorption (`proc`, `bpf`, `cg`, `ns`, `io`, `numa`, `mem`)

- `sigma proc`: Virtual filesystem mapping for kernel state.
- `sigma bpf`: eBPF/XDP program deployment and syscall tracing.
- `sigma cg`: Cgroups v2 resource isolation.
- `sigma ns`: PID/NET/MNT/USER namespace sharding.
- `sigma io`: io_uring asynchronous ring-buffer I/O.
- `sigma mem`: THP, ZRAM, KSM, and OOM-Killer priority tuning.
- `sigma linux-usps all`: Activate every unique selling proposition of the Linux kernel.

### 💾 Storage & Files (`fs`, `vcs`, `vault`)

- `sigma fs <ls|read|mount|snapshot>`: Universal filesystem dispatcher (EXT4, Btrfs, NFS, OverlayFS).
- `sigma vcs`: Memory-snapshot based version control (Git-parity).
- `sigma vault`: Sovereign "Chrono-Vault" snapshot management (Time Machine parity).

### 🌐 Networking (`net`, `http`)

- `sigma net`: Zero-Trust mesh networking and Aether handshake.
- `sigma http`: Nginx-parity proxy, load-balancing, and SSL termination.

### 🧠 Intelligence & Math (`ai`, `ml`, `ds`)

- `sigma ai <prompt|persona|predict>`: Local LLM inference and persona projection.
- `sigma ml`: Native C11 inference engine for sharded datasets.
- `sigma ds`: Tensor math and histogram analysis across mapped buffers.

### 🛡️ Security & Forensics (`sec`, `cyber`, `sandbox`, `clean`)

- `sigma sec`: PQC keygen (Lattice-based), TPM binding, and ASLR management.
- `sigma cyber <scan|nmap|metasploit>`: Native offensive security tools.
- `sigma sandbox`: Namespace-isolated application execution.
- `sigma clean`: DOD 5220.22-M compliant amnesic silicon wipe.

### 📦 Distribution & Tools (`pkg`, `container`, `distro`, `tools`)

- `sigma pkg`: Shard package manager (APT/Pacman/Nix parity).
- `sigma container`: Docker/OCI native runtime (no daemon).
- `sigma distro <absorb|activate>`: Absorb any Linux distribution's personality.
- `sigma tools`: Direct absorption of Git, Docker, Vim, VSCode, etc.

### ⚖️ Specialized Shards (`law`, `ui`, `media`, `gaming`, `qube`)

- `sigma law <section>`: Offline Indian Law (BNS/BNSS/BSA) query engine.
- `sigma ui <open|close|tile|theme>`: Framebuffer-direct window management.
- `sigma media`: FFmpeg-parity transcoding and streaming.
- `sigma gaming`: Proton/Vulkan boost and GameMode activation.
- `sigma qube`: Qubes OS style VM isolation and compartmentalization.

---

## ⚡ The God-Matrix

The ultimate mission directive for absolute system dominance.

- `sigma god-matrix`: Simultaneously invokes all distro, tool, and kernel absorbers.
  > **Result**: ∞ Absolute Sovereignty Achieved. No competitor survives. ∞

---
**Note**: All commands map to `sovereign_tools/SovereignOmniCLI.c`. Modify at shard-level for custom mission verbs.

# 12-Week Milestone Plan

Active development plan for SigmaOS, mapped to the repo structure.

---

## Sprint 0 (Week 0–2): CI Stability

**Goal:** Every PR is gated by build + QEMU smoke.

| Task | File | Status |
|------|------|--------|
| QEMU boot smoke CI | `.github/workflows/qemu-boot.yml` | ✅ Done |
| Reproducible build CI | `.github/workflows/reproducible_build.yml` | ✅ Done |
| Compatibility matrix CI | `.github/workflows/compat-matrix.yml` | ✅ Done |
| Set `SOURCE_DATE_EPOCH` | `Makefile` | 🔄 |

**Exit criteria:** CI green badge on main; ISO builds reproducibly.

---

## Sprint 1 (Week 2–6): MicroVM + Syscall Profiler

**Goal:** Run top-20 OCI images in CI microVM; have profiler data.

| Task | File | Status |
|------|------|--------|
| microVM OCI runner | `virtualization/ocirunner/ocirunner.rs` | ✅ Done |
| microVM shell script | `virtualization/ocirunner/run_in_microvm.sh` | ✅ Done |
| Syscall profiler tool | `tools/syscall_profiler/profiler.py` | ✅ Done |
| Run profiler in CI | `.github/workflows/compat-matrix.yml` | ✅ Done |

**Exit criteria:** nginx runs in CI microVM; syscall CSV generated.

---

## Sprint 2 (Week 5–10): linux_compat Prototype

**Goal:** Static Linux ELF binaries load and run basic I/O.

| Task | File | Status |
|------|------|--------|
| ELF64 loader | `kernel/linux_compat/elf_loader.rs` | ✅ Done |
| vDSO shim | `kernel/linux_compat/vdso_shim.rs` | ✅ Done |
| /proc shim | `kernel/linux_compat/proc_shim.rs` | ✅ Done |
| Syscall dispatch (50+) | `kernel/core/syscall_dispatch.rs` | ✅ Done |
| Core scheduler | `kernel/core/sigma_sched.rs` | ✅ Done |
| Memory manager | `kernel/core/sigma_mm.rs` | ✅ Done |
| e1000 NIC driver | `kernel/linux_compat/e1000_main.rs` | ✅ Done |

**Exit criteria:** Static "hello world" ELF loads; `getpid()`, `clock_gettime()`, `write()` work.

---

## Sprint 3 (Week 9–12): Driver SDK + A/B Updater

**Goal:** Vendor-ready driver SDK; transactional updates work in QEMU.

| Task | File | Status |
|------|------|--------|
| Driver SDK trait + MMIO | `sdk/driver/src/lib.rs` | ✅ Done |
| virtio-blk example driver | `sdk/driver/examples/virtio_blk.rs` | ✅ Done |
| A/B transactional updater | `sigmad/updater/main.rs` | ✅ Done |
| kabi ABI stability | `kabi/src/lib.rs` | ✅ Done |
| Hotplug manager | `drivers/core/hotplug_manager.rs` | ✅ Done |
| CUPS print system | `drivers/printing/cups.nim` | ✅ Done |
| IPC ring buffers | `kernel/core/ipc/SovereignIPC.rs` | ✅ Done |
| sigpkg spec | `docs/SIGPKG_SPEC.md` | ✅ Done |

**Exit criteria:** example driver builds + runs in QEMU; updater switches slots.

---

## Sprint 4 (Week 12–18): VFS + Networking

**Goal:** Basic file I/O and TCP sockets work natively.

| Task | File | Priority |
|------|------|----------|
| VFS open/read/write | `kernel/fs/vfs.rs` | 🔴 Critical |
| tmpfs | `kernel/fs/tmpfs.rs` | 🔴 Critical |
| TCP state machine | `kernel/net/tcp.rs` | 🔴 Critical |
| Socket syscalls | `kernel/core/syscall_dispatch.rs` | 🔴 Critical |
| ext4 read-only | `fs/ext4/` | 🟠 High |
| DNS resolver | `kernel/net/dns.rs` | 🟠 High |

**Exit criteria:** sigma-sh can `cat /etc/hostname`; TCP connect to internet works.

---

## Sprint 5 (Week 18–24): GPU + Desktop

**Goal:** Zenith desktop boots with GPU acceleration in QEMU.

| Task | File | Priority |
|------|------|----------|
| DRM/KMS mode setting | `drivers/gpu/` | 🔴 |
| virtio-gpu driver | `drivers/gpu/sigma_virtio_gpu.zig` | 🔴 |
| Wayland compositor | `desktop/compositor/` | 🟠 |
| Input stack | `drivers/input/` | 🟠 |

**Exit criteria:** Zenith boots with mouse + keyboard in QEMU virtio-gpu.

---

## KPIs to Track Weekly

| Metric | Current | Target (Week 12) |
|--------|---------|-----------------|
| CI boot pass rate | 90% | 100% |
| Syscalls implemented | 15 | 50 |
| OCI images passing | 0 | 20 |
| Driver SDK example works | ✅ | ✅ |
| A/B update works in QEMU | ✅ | ✅ |

---

*See also: [DEVELOPMENT_ROADMAP.md](../DEVELOPMENT_ROADMAP.md) · [PHASE_A_EXECUTION_CHECKLIST.md](../PHASE_A_EXECUTION_CHECKLIST.md)*

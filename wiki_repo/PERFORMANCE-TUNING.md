# SigmaOS Performance Tuning Guide

> Comprehensive guide to optimizing SigmaOS for maximum performance across all hardware profiles.

---

## ⚡ Performance Philosophy

SigmaOS takes a **profile-based performance approach** inspired by:
- **Arch Linux**: User-controlled performance tuning
- **Gentoo**: Source-level optimization with compiler flags
- **RHEL/Fedora**: Enterprise-grade performance tuning tools
- **Clear Linux**: Intel-optimized defaults
- **Fedora Workstation**: Game mode integration

---

## 🔧 Kernel Tuning

### CPU Governor
```bash
# Performance mode (max frequency always)
echo performance > /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Powersave mode (laptops)
echo powersave > /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Schedutil (recommended - responsive)
echo schedutil > /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### I/O Schedulers
```bash
# For NVMe SSDs (none = no scheduler, pass-through)
echo none > /sys/block/nvme0n1/queue/scheduler

# For SATA SSDs
echo mq-deadline > /sys/block/sda/queue/scheduler

# For HDDs
echo bfq > /sys/block/sdb/queue/scheduler
```

### Memory Tuning
```bash
# Reduce swap tendency (0-100, lower = use RAM more)
echo 10 > /proc/sys/vm/swappiness

# Enable zram (compressed swap in RAM)
sigma-ctl enable zram
sigma-ctl configure zram --size 8G --algo zstd

# Transparent Huge Pages
echo always > /sys/kernel/mm/transparent_hugepage/enabled
```

---

## 🎮 Gaming Optimization

### GameMode
```bash
# Install and enable SigmaOS GameMode (inspired by Feral's gamemode)
sigma-pkg install sigmagamemode
systemctl enable --now sigmagamemode

# Launch games with GameMode
gamemoderun %command%  # Steam launch options
```

### WINE/Proton Optimization
```bash
# Enable esync (eventfd-based synchronization)
export WINEESYNC=1

# Enable fsync (futex-based, faster)
export WINEFSYNC=1

# Use ACO shader compiler (AMD)
export RADV_PERFTEST=aco

# Vulkan async queues
export DXVK_ASYNC=1
```

### GPU Performance
```bash
# NVIDIA power management
nvidia-settings -a GPUPowerMizerMode=1  # Prefer maximum performance

# AMD GPU performance level
echo high > /sys/class/drm/card0/device/power_dpm_force_performance_level

# Intel i915 power mode
echo 0 > /sys/module/i915/parameters/enable_rc6
```

---

## 💾 Storage Performance

### Btrfs Optimization
```bash
# Mount options for performance
mount -o noatime,compress=zstd:1,space_cache=v2,discard=async /dev/nvme0n1p1 /

# Defragment and rebalance
btrfs defragment -r /
btrfs balance start -dusage=50 -musage=50 /
```

### ext4 Optimization
```bash
# Enable lazy initialization
mke2fs -E lazy_itable_init=0,lazy_journal_init=0 /dev/sda1

# Mount with performance options
mount -o noatime,data=writeback,barrier=0 /dev/sda1 /
```

### ZFS Tuning
```bash
# Set ARC cache size (50% of RAM)
echo 17179869184 > /sys/module/zfs/parameters/zfs_arc_max

# Enable prefetch
echo 1 > /sys/module/zfs/parameters/zfetch_array_rd_sz
```

---

## 📶 Network Performance

### TCP Stack Tuning
```bash
# Increase buffer sizes
sysctl -w net.core.rmem_max=134217728
sysctl -w net.core.wmem_max=134217728
sysctl -w net.ipv4.tcp_rmem='4096 87380 134217728'
sysctl -w net.ipv4.tcp_wmem='4096 65536 134217728'

# BBR congestion control (Google's algorithm)
sysctl -w net.ipv4.tcp_congestion_control=bbr
modprobe tcp_bbr

# Fast Open
sysctl -w net.ipv4.tcp_fastopen=3
```

### IRQ Affinity
```bash
# Pin network IRQ to specific CPUs
irqbalance --oneshot
echo f > /proc/irq/$(grep eth0 /proc/interrupts | cut -d: -f1)/smp_affinity
```

---

## 🧠 AI/ML Performance

### GPU Acceleration
```bash
# Enable CUDA for NVIDIA
sigma-pkg install cuda-runtime
export CUDA_VISIBLE_DEVICES=0

# Enable ROCm for AMD
sigma-pkg install rocm-runtime
export ROCR_VISIBLE_DEVICES=0

# Enable Vulkan compute
export VULKAN_SDK=/opt/vulkan
```

### LLM Inference Optimization
```bash
# Use llama.cpp with optimal threads
sigma-llm run --model llama3-8b --threads $(nproc) --ctx-size 4096

# Enable quantization (4-bit = 4x memory reduction)
sigma-llm run --model llama3-8b --quant q4_0
```

---

## 📊 Performance Monitoring

### Built-in Tools
```bash
# SigmaOS performance dashboard
sigma-perf dashboard

# CPU profiling
sigma-perf profile --cpu --duration 30s

# Memory analysis
sigma-perf memory --trace

# I/O monitoring
sigma-perf io --live
```

### Integration with Standard Tools
```bash
# perf (Linux perf events)
perf top -g
perf record -g ./my-program
perf report

# flamegraph generation
cargo flamegraph --bin sigmaos-kernel

# eBPF tracing
bpftrace -e 'kprobe:sys_read { @[comm] = count(); }'
```

---

## 🚀 Compilation Optimization

### Rust Compiler Flags
```toml
# Cargo.toml profile for maximum performance
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true

[profile.release.build-override]
opt-level = 3
```

### Target CPU Optimization
```bash
# Build for native CPU (not portable!)
RUSTFLAGS="-C target-cpu=native" cargo build --release

# For specific architecture
RUSTFLAGS="-C target-cpu=znver4" cargo build --release  # AMD Zen 4
RUSTFLAGS="-C target-cpu=sapphirerapids" cargo build --release  # Intel Sapphire Rapids
```

---

*SigmaOS Performance Tuning Guide | Updated: 2026-08-23*
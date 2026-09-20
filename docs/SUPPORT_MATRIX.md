# SigmaOS Supported Hardware Matrix

**Version:** 1.0  
**Status:** Draft  
**Last Updated:** 2025-01-22  
**Purpose:** Document supported hardware platforms and their compatibility status

---

## 1. Support Tiers

### Tier 1: Primary Support (x86_64)
- **Target:** QEMU x86_64 virtualization
- **Boot Method:** UEFI and BIOS
- **Status:** Target for M1 milestone
- **Evidence:** None yet
- **Last Tested:** N/A

### Tier 2: Secondary Support (ARM64)
- **Target:** QEMU aarch64 virtualization
- **Boot Method:** UEFI
- **Status:** Planned for M6 milestone
- **Evidence:** None yet
- **Last Tested:** N/A

### Tier 3: Future Support
- **Targets:** RISC-V, LoongArch, PowerPC64, s390x
- **Status:** Not started
- **Evidence:** None
- **Last Tested:** N/A

---

## 2. Hardware Support Matrix

### 2.1 x86_64 QEMU (Primary Target)

| Component | Status | Notes | Last Tested |
|-----------|--------|-------|-------------|
| Boot (UEFI) | Not started | Target for M1 | N/A |
| Boot (BIOS) | Not started | Target for M1 | N/A |
| CPU (x86-64-v1) | Not started | Baseline x86-64 | N/A |
| CPU (x86-64-v2) | Not started | With AVX2 | N/A |
| RAM | Not started | Minimum 2GB | N/A |
| Storage (virtio-blk) | Not started | Primary storage | N/A |
| Storage (NVMe) | Not started | Future support | N/A |
| Network (virtio-net) | Not started | Basic networking | N/A |
| Graphics (VGA) | Not started | Framebuffer | N/A |
| Graphics (virtio-gpu) | Not started | Accelerated | N/A |
| Audio (virtio-snd) | Not started | Basic audio | N/A |
| USB | Not started | Future support | N/A |
| Suspend/Resume | Not started | Future support | N/A |

### 2.2 ARM64 QEMU (Secondary Target)

| Component | Status | Notes | Last Tested |
|-----------|--------|-------|-------------|
| Boot (UEFI) | Not started | Target for M6 | N/A |
| CPU (ARMv8) | Not started | Baseline ARM64 | N/A |
| RAM | Not started | Minimum 2GB | N/A |
| Storage (virtio-blk) | Not started | Primary storage | N/A |
| Network (virtio-net) | Not started | Basic networking | N/A |
| Graphics (virtio-gpu) | Not started | Framebuffer | N/A |
| Audio (virtio-snd) | Not started | Basic audio | N/A |

### 2.3 Reference Hardware (Future)

| Device | Architecture | Status | Notes | Last Tested |
|--------|-------------|--------|-------|-------------|
| Dell XPS 13 | x86_64 | Not started | Future reference | N/A |
| Lenovo ThinkPad X1 | x86_64 | Not started | Future reference | N/A |
| Raspberry Pi 4 | ARM64 | Not started | Future reference | N/A |
| Apple M1/M2 | ARM64 | Not started | Future reference | N/A |

---

## 3. Minimum System Requirements

### 3.1 x86_64 (Target)

- **CPU:** x86-64-v1 (SSE2)
- **RAM:** 2 GB minimum, 4 GB recommended
- **Storage:** 10 GB minimum, 20 GB recommended
- **Boot:** UEFI 2.0 or BIOS
- **Graphics:** VGA or virtio-gpu

### 3.2 ARM64 (Future)

- **CPU:** ARMv8-A
- **RAM:** 2 GB minimum, 4 GB recommended
- **Storage:** 10 GB minimum, 20 GB recommended
- **Boot:** UEFI 2.0
- **Graphics:** virtio-gpu

---

## 4. Known Limitations

### 4.1 Current Limitations

- **No real hardware tested yet** - All targets are planned
- **No graphics acceleration** - Only framebuffer support planned initially
- **No Wi-Fi support** - Only wired networking planned initially
- **No suspend/resume** - Power management not implemented
- **No USB device support** - USB stack not implemented

### 4.2 Architecture Limitations

- **x86_64-v1 baseline** - No x86-64-v2/v3/v4 optimizations yet
- **ARM64 baseline** - No ARM64 extensions yet
- **No heterogeneous computing** - No GPU/CPU offload yet

---

## 5. Testing Strategy

### 5.1 Automated Testing

- **QEMU boot tests** - Verify boot to login
- **Hardware smoke tests** - Basic functionality on reference hardware
- **Regression tests** - Ensure updates don't break support

### 5.2 Manual Testing

- **Installation tests** - Verify clean installation
- **Desktop tests** - Verify Zenith desktop functionality
- **Hardware-specific tests** - Test device-specific features

---

## 6. Support Policy

### 6.1 Tier 1 Support

- **Target:** QEMU x86_64
- **Goal:** 100% boot success on supported CI images
- **Response time:** Critical bugs fixed within 1 week
- **Updates:** Tested on every release

### 6.2 Tier 2 Support

- **Target:** QEMU ARM64
- **Goal:** 95% boot success on supported CI images
- **Response time:** Critical bugs fixed within 2 weeks
- **Updates:** Tested on every release

### 6.3 Tier 3 Support

- **Target:** Future architectures
- **Goal:** Best-effort support
- **Response time:** As resources allow
- **Updates:** Tested when feasible

---

## 7. Success Metrics

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| QEMU x86_64 boot success | 100% | 0% | ❌ |
| QEMU ARM64 boot success | 95% | 0% | ❌ |
| Clean installation on reference hardware | 95%+ | 0% | ❌ |
| Graphics support | Working | None | ❌ |
| Wi-Fi support | Working | None | ❌ |
| Audio support | Working | None | ❌ |
| Suspend/Resume | Working | None | ❌ |

---

## 8. Next Steps

1. **M1 Milestone:**
   - Implement QEMU x86_64 boot
   - Implement basic graphics (framebuffer)
   - Implement basic networking
   - Test boot-to-login path

2. **M6 Milestone:**
   - Implement QEMU ARM64 boot
   - Test on reference ARM hardware
   - Expand driver support

3. **Future:**
   - Add real hardware testing
   - Expand architecture support
   - Implement advanced features

---

## 9. References

- [PROJECT_STATUS.md](PROJECT_STATUS.md) - Overall project status
- [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md) - Architecture decisions
- [Future Development Plan](SIGMAOS_STRATEGIC_DEVELOPMENT_PLAN_LINUX_BSD.md) - Hardware strategy

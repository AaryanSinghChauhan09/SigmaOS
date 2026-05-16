# SigmaOS Embedded Hardware Compatibility List

## Release: Standalone v15.0 — IoT/Edge Deployments

---

## Tier 1: Fully Validated (Primary Targets)

| Hardware | Architecture | RAM | Flash/Storage | Status |
|----------|-------------|-----|---------------|--------|
| Raspberry Pi 4 Model B | ARM Cortex-A72 | 4GB+ | MicroSD/NVMe | ✅ Fully Supported |

| Raspberry Pi Zero 2 W | ARM Cortex-A53 | 512MB | MicroSD | ✅ Fully Supported |
| BeagleBone Black | ARM Cortex-A8 | 512MB | eMMC 4GB | ✅ Fully Supported |
| STM32MP157 Discovery | ARM Cortex-A7 | 512MB | eMMC | ✅ Fully Supported |
| NXP i.MX8M | ARM Cortex-A53 | 2GB | eMMC | ✅ Fully Supported |

## Tier 2: Validated with Constraints

| Hardware | Architecture | RAM | Notes | Status |
|----------|-------------|-----|-------|--------|
| Raspberry Pi 3B+ | ARM Cortex-A53 | 1GB | GUI disabled, FastBoot required | ⚠️ Constrained |

| Jetson Nano | ARM Cortex-A57 | 4GB | CUDA shards excluded | ⚠️ Constrained |
| ODROID-C4 | ARM Cortex-A55 | 4GB | USB3 driver in beta | ⚠️ Constrained |
| SiFive HiFive Unmatched | RISC-V U74 | 16GB | Network driver beta | ⚠️ Constrained |
| VisionFive 2 | RISC-V JH7110 | 8GB | No GPU acceleration | ⚠️ Constrained |

## Tier 3: Experimental

| Hardware | Architecture | Status | Notes |
|----------|-------------|--------|-------|
| ESP32-S3 | Xtensa LX7 | 🧪 Experimental | No MMU — heap only |
| Allwinner H616 | ARM Cortex-A53 | 🧪 Experimental | Mali GPU unvalidated |
| Milk-V Mars | RISC-V StarFive | 🧪 Experimental | PCIe not yet tested |

---

## Build Flags Per Architecture

```bash

# ARM Cortex-A (32-bit)

export SIGMA_ARCH=arm
export SIGMA_FLAGS="-march=armv7-a -mfpu=neon -mfloat-abi=hard -DSIGMA_FASTBOOT=1"

# ARM64 / AArch64

export SIGMA_ARCH=aarch64
export SIGMA_FLAGS="-march=armv8-a -DSIGMA_FASTBOOT=1"

# RISC-V 64-bit

export SIGMA_ARCH=riscv64
export SIGMA_FLAGS="-march=rv64imafdc -mabi=lp64d -DSIGMA_FASTBOOT=1"
```

---

## Memory Constraints

| Configuration | Min RAM | Recommended |
|---------------|---------|-------------|
| Headless (no GUI) | **128MB** | 512MB |

| With SovereignWM | **512MB** | 2GB |

| With SovereignContainer | **1GB** | 4GB |

> **Note**: RTOS mode forbids dynamic memory allocation. All shards must use `SovereignMemoryPool::alloc()`.

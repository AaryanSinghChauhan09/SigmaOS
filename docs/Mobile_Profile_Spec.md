# SigmaOS Mobile ARM64 Profile Specification

## Overview

The SigmaOS mobile profile targets ARM64 smartphones and tablets (starting with BCM2711-based devices, expanding to Snapdragon). It boots via a UEFI EFI stub, implements ARM-specific power management, and provides a Wayland-based touch-friendly UI via the Zenith compositor.

---

## Boot: UEFI EFI Stub on ARM64

```
Device power-on → ARM TrustZone ATF (BL1/BL2/BL31) → UEFI (EDK2 ARM64)
  │
  ▼ sigma-boot-arm64.efi (PE32+ EFI application, AArch64)
  │  UEFI LoadImage → EDK2 ACPI/DTB handoff
  │  Load sigma-kernel-arm64.elf from ESP
  │  Setup SigmaBootInfo with DTB pointer + memory map
  │  ExitBootServices → jump to kernel entry
  ▼
kernel_main_arm64(boot_info)
  │  Setup MMU (4KB pages, 48-bit VA, EL1)
  │  Init ARM GIC v3 (interrupt controller)
  │  Init PSCI (power state coordination)
  │  Init BCM2711 BSP (GPIO, I2C, SPI, UART)
  ▼
sigma-init (mobile variant)
  │  Start: sigma-net, sigma-audiod, sigma-bluetooth, Zenith compositor
  │  Launch: sigma-homescreen (Wayland app)
```

---

## Drivers: ARM GIC, MMU, BCM2711 BSP

### ARM GIC v3

```rust
// kernel/src/arch/arm64/gic.rs (sketch)

const GICD_BASE: u64 = 0xFF84_1000; // BCM2711 GICv2 distributor
const GICC_BASE: u64 = 0xFF84_2000;

pub fn gic_init() {
    // Enable distributor
    unsafe {
        write_u32(GICD_BASE + 0x000, 1); // GICD_CTLR enable
    }
    // Set all SPIs to group 1 (non-secure)
    for i in 0..32 {
        unsafe { write_u32(GICD_BASE + 0x080 + i * 4, 0xFFFF_FFFF); }
    }
    // Enable CPU interface
    unsafe { write_u32(GICC_BASE + 0x000, 1); } // GICC_CTLR
    unsafe { write_u32(GICC_BASE + 0x004, 0xFF); } // GICC_PMR: all priorities
}
```

### BCM2711 GPIO

```rust
// arch/arm64/bsp/bcm2711/gpio.rs (sketch, using svd2rust-generated PAC)
use bcm2711_pac::Peripherals;

pub fn gpio_output(pin: u8) {
    let p = unsafe { Peripherals::steal() };
    // Set FSEL to output (001)
    let reg_idx = (pin / 10) as usize;
    let shift   = (pin % 10) * 3;
    p.gpio.gpfsel[reg_idx].modify(|r, w| {
        unsafe { w.bits((r.bits() & !(0b111 << shift)) | (0b001 << shift)) }
    });
}
```

---

## Power: P/C-State Scheduler + Battery Cgroup Cap

### CPU P/C-State Management

```rust
// kernel/src/power/arm64_power.rs

pub enum CpuPowerState {
    C0,  // Active
    C1,  // WFI (Wait For Interrupt) — ARM standby
    C2,  // Core power-down (PSCI CPU_SUSPEND)
    C3,  // Cluster power-down (PSCI CPU_SUSPEND, deeper)
}

pub fn enter_idle_state(state: CpuPowerState) {
    match state {
        CpuPowerState::C0 => {},
        CpuPowerState::C1 => unsafe { core::arch::asm!("wfi") },
        CpuPowerState::C2 | CpuPowerState::C3 => {
            // PSCI CPU_SUSPEND via SMC call
            psci_cpu_suspend(state as u32, 0, 0);
        }
    }
}
```

### Battery % Cgroup Cap

When battery drops below a threshold, sigma-powerd applies CPU cgroup limits:

```toml
# /etc/sigma/power-policy.toml
[[thresholds]]
battery_pct = 20
cpu_quota   = "50%"   # cap all cgroups to 50% CPU

[[thresholds]]
battery_pct = 10
cpu_quota   = "25%"
suspend_background = true
```

---

## Touch Input: I2C HID Touch Event Pipeline

```
Touchscreen IC (e.g., Goodix GT911) → I2C bus
  │  sigma-i2c-driver (kernel shard)
  ▼
I2C HID report (touch coordinates, pressure, fingers)
  │  sigma-input-shard: parse HID report, emit InputEvent
  ▼
Zenith compositor (SeatHandler) → libinput-equivalent
  │  Wayland touch events (wl_touch.down, wl_touch.motion, wl_touch.up)
  ▼
Wayland app (sigma-homescreen, browser, etc.)
```

---

## NEON-Accelerated Kyber-1024 KEM

On ARM64, the Kyber-1024 NTT (Number Theoretic Transform) is accelerated using NEON SIMD intrinsics:

```rust
// crypto/sigma-pqcrypto/src/kyber_neon.rs

#[cfg(target_arch = "aarch64")]
pub fn ntt_arm64(a: &mut [u16; 256]) {
    // Use NEON vld1q_u16 + vmull + vaddq for butterfly operations
    // ~4× speedup over scalar NTT
    unsafe {
        core::arch::asm!(
            // NEON NTT butterfly — inner loop
            "ld1 {{v0.8h}}, [{0}]",
            "ld1 {{v1.8h}}, [{1}]",
            // ... butterfly operations ...
            in(reg) a.as_ptr(),
            in(reg) a[128..].as_ptr(),
            options(nostack)
        );
    }
}
```

Benchmark target: Kyber-1024 keygen in < 500 µs on Cortex-A72 (BCM2711).

---

## Mobile Profile Image

| Image | Size | Target |
|---|---|---|
| sigma-mobile-bcm2711.img | ~300 MB | Raspberry Pi 4 / CM4 |
| sigma-mobile-snapdragon865.img | ~400 MB | Snapdragon 865 phones |

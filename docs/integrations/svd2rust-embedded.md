# svd2rust for ARM / RISC-V BSP Driver Generation

## Overview

SigmaOS uses [svd2rust](https://github.com/rust-embedded/svd2rust) (MIT / Apache-2.0) to auto-generate type-safe Rust register bindings from vendor-supplied SVD (System View Description) files. This covers the RP2040 (Raspberry Pi), STM32F4, and nRF52840 BSPs.

---

## Supported BSPs

| BSP | SVD Source | Generated crate |
|---|---|---|
| RP2040 | `arch/arm64/bsp/rp2040/rp2040.svd` | `arch/arm64/bsp/rp2040/src/lib.rs` |
| STM32F411 | `arch/arm64/bsp/stm32f411/stm32f411.svd` | `arch/arm64/bsp/stm32f411/src/lib.rs` |
| nRF52840 | `arch/arm64/bsp/nrf52840/nrf52840.svd` | `arch/arm64/bsp/nrf52840/src/lib.rs` |
| RISC-V GD32VF103 | `arch/riscv64/bsp/gd32vf103/gd32vf103.svd` | `arch/riscv64/bsp/gd32vf103/src/lib.rs` |

---

## Makefile Target

```makefile
# arch/arm64/bsp/Makefile

SVD2RUST := svd2rust
FORM     := form

.PHONY: bsp-rp2040 bsp-stm32 bsp-nrf52840

bsp-rp2040:
	cd arch/arm64/bsp/rp2040 && \
	$(SVD2RUST) -i rp2040.svd && \
	$(FORM) -i lib.rs -o src && \
	cargo fmt

bsp-stm32:
	cd arch/arm64/bsp/stm32f411 && \
	$(SVD2RUST) -i stm32f411.svd && \
	$(FORM) -i lib.rs -o src && \
	cargo fmt

bsp-nrf52840:
	cd arch/arm64/bsp/nrf52840 && \
	$(SVD2RUST) -i nrf52840.svd && \
	$(FORM) -i lib.rs -o src && \
	cargo fmt
```

---

## arch/arm64/bsp/rp2040/Cargo.toml

```toml
[package]
name    = "rp2040-pac"
version = "0.1.0"
edition = "2021"

[dependencies]
vcell       = "=0.1.3"
cortex-m    = "=0.7.7"

[features]
default = ["rt"]
rt      = ["cortex-m/critical-section-single-core"]
```

---

## Generated BSP Usage: GPIO Toggle

```rust
// Example: toggle GPIO25 (onboard LED) on RP2040

#![no_std]
#![no_main]

use rp2040_pac::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };
    let sio = &peripherals.SIO;
    let io_bank0 = &peripherals.IO_BANK0;
    let pads_bank0 = &peripherals.PADS_BANK0;

    // Configure GPIO25 as output
    pads_bank0.gpio(25).modify(|_, w| {
        w.ie().set_bit()
         .od().clear_bit()
    });
    io_bank0.gpio(25).gpio_ctrl().modify(|_, w| {
        unsafe { w.funcsel().bits(5) } // SIO function
    });
    sio.gpio_oe_set().write(|w| unsafe { w.bits(1 << 25) });

    // Toggle LED in loop
    loop {
        sio.gpio_out_xor().write(|w| unsafe { w.bits(1 << 25) });
        cortex_m::asm::delay(1_000_000);
    }
}
```

---

## nRF52840 UART Example

```rust
// Example: initialize UART0 on nRF52840 at 115200 baud

use nrf52840_pac::Peripherals;

fn init_uart(p: &Peripherals) {
    let uarte = &p.UARTE0;
    // Set baud rate (using svd2rust-generated type-safe field write)
    uarte.baudrate.write(|w| w.baudrate().baud115200());
    // Enable TX
    uarte.enable.write(|w| w.enable().enabled());
    uarte.tasks_starttx.write(|w| unsafe { w.bits(1) });
}
```

---

## CI Job

```yaml
# In .github/workflows/bsp-generate.yml:
- name: Regenerate BSP bindings
  run: |
    cargo install svd2rust --version 0.33.0 --locked
    cargo install form --version 0.10.0 --locked
    make bsp-rp2040 bsp-stm32 bsp-nrf52840

- name: Check no diff in generated files
  run: git diff --exit-code arch/arm64/bsp/
```

---

## Exit Criteria

- `make bsp-rp2040` generates `arch/arm64/bsp/rp2040/src/lib.rs` without errors.
- `cargo build --target thumbv6m-none-eabi -p rp2040-pac` compiles successfully.
- GPIO toggle example blinks the onboard LED on real RP2040 hardware.

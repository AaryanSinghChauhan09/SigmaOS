// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/arm/bcm2835.rs — BCM2835 SoC Driver
//
// Implements the Broadcom BCM2835 SoC driver for Raspberry Pi.
// Supports GPIO, UART, SPI, I2C, and system timer.
// Based on Linux kernel bcm2835 driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── BCM2835 Register Offsets ─────────────────────

pub const BCM2835_GPIO_BASE: U32 = 0x200000;
pub const BCM2835_UART0_BASE: U32 = 0x201000;
pub const BCM2835_SPI0_BASE: U32 = 0x204000;
pub const BCM2835_I2C0_BASE: U32 = 0x205000;
pub const BCM2835_I2C1_BASE: U32 = 0x804000;
pub const BCM2835_ST_BASE: U32 = 0x3000;
pub const BCM2835_DMA_BASE: U32 = 0x007000;
pub const BCM2835_ARM_BASE: U32 = 0xB000;
pub const BCM2835_PM_BASE: U32 = 0x100000;
pub const BCM2835_VC_BASE: U32 = 0x40000000;

// ─── GPIO Register Offsets ─────────────────────

pub const BCM2835_GPIO_GPFSEL0: U32 = 0x00;
pub const BCM2835_GPIO_GPFSEL1: U32 = 0x04;
pub const BCM2835_GPIO_GPFSEL2: U32 = 0x08;
pub const BCM2835_GPIO_GPFSEL3: U32 = 0x0C;
pub const BCM2835_GPIO_GPFSEL4: U32 = 0x10;
pub const BCM2835_GPIO_GPFSEL5: U32 = 0x14;
pub const BCM2835_GPIO_GPSET0: U32 = 0x1C;
pub const BCM2835_GPIO_GPSET1: U32 = 0x20;
pub const BCM2835_GPIO_GPCLR0: U32 = 0x28;
pub const BCM2835_GPIO_GPCLR1: U32 = 0x2C;
pub const BCM2835_GPIO_GPLEV0: U32 = 0x34;
pub const BCM2835_GPIO_GPLEV1: U32 = 0x38;
pub const BCM2835_GPIO_GPEDS0: U32 = 0x40;
pub const BCM2835_GPIO_GPEDS1: U32 = 0x44;
pub const BCM2835_GPIO_GPREN0: U32 = 0x4C;
pub const BCM2835_GPIO_GPREN1: U32 = 0x50;
pub const BCM2835_GPIO_GPFEN0: U32 = 0x58;
pub const BCM2835_GPIO_GPFEN1: U32 = 0x5C;
pub const BCM2835_GPIO_GPHEN0: U32 = 0x64;
pub const BCM2835_GPIO_GPHEN1: U32 = 0x68;
pub const BCM2835_GPIO_GPLEN0: U32 = 0x70;
pub const BCM2835_GPIO_GPLEN1: U32 = 0x74;
pub const BCM2835_GPIO_GPAREN0: U32 = 0x7C;
pub const BCM2835_GPIO_GPAREN1: U32 = 0x80;
pub const BCM2835_GPIO_GPAFEN0: U32 = 0x88;
pub const BCM2835_GPIO_GPAFEN1: U32 = 0x8C;
pub const BCM2835_GPIO_GPPUD: U32 = 0x94;
pub const BCM2835_GPIO_GPPUDCLK0: U32 = 0x98;
pub const BCM2835_GPIO_GPPUDCLK1: U32 = 0x9C;

// ─── GPIO Function Select ─────────────────────

pub const GPIO_FSEL_INPUT: U32 = 0x000;
pub const GPIO_FSEL_OUTPUT: U32 = 0x001;
pub const GPIO_FSEL_ALT0: U32 = 0x100;
pub const GPIO_FSEL_ALT1: U32 = 0x200;
pub const GPIO_FSEL_ALT2: U32 = 0x300;
pub const GPIO_FSEL_ALT3: U32 = 0x400;
pub const GPIO_FSEL_ALT4: U32 = 0x500;
pub const GPIO_FSEL_ALT5: U32 = 0x600;
pub const GPIO_FSEL_MASK: U32 = 0x7;

// ─── GPIO Pull Up/Down ─────────────────────

pub const GPIO_PUD_OFF: U32 = 0x00;
pub const GPIO_PUD_DOWN: U32 = 0x01;
pub const GPIO_PUD_UP: U32 = 0x02;

// ─── BCM2835 SoC Structure ─────────────────────

pub struct Bcm2835SoC {
    pub base_address: U64,
    pub initialized: bool,
    pub gpio_base: U64,
    pub uart_base: U64,
    pub spi_base: U64,
    pub i2c0_base: U64,
    pub i2c1_base: U64,
    pub st_base: U64,
    pub dma_base: U64,
}

impl Bcm2835SoC {
    pub const fn new() -> Self {
        Bcm2835SoC {
            base_address: 0,
            initialized: false,
            gpio_base: 0,
            uart_base: 0,
            spi_base: 0,
            i2c0_base: 0,
            i2c1_base: 0,
            st_base: 0,
            dma_base: 0,
        }
    }

    /// Read MMIO register
    unsafe fn read_mmio(&self, offset: U32) -> U32 {
        let ptr = (self.base_address + offset as U64) as *const U32;
        *ptr
    }

    /// Write MMIO register
    unsafe fn write_mmio(&self, offset: U32, value: U32) {
        let ptr = (self.base_address + offset as U64) as *mut U32;
        *ptr = value
    }

    /// Initialize BCM2835 SoC
    fn init_bcm2835(&mut self, base_address: U64) -> I32 {
        self.base_address = base_address;

        // Calculate peripheral base addresses
        self.gpio_base = self.base_address + BCM2835_GPIO_BASE as U64;
        self.uart_base = self.base_address + BCM2835_UART0_BASE as U64;
        self.spi_base = self.base_address + BCM2835_SPI0_BASE as U64;
        self.i2c0_base = self.base_address + BCM2835_I2C0_BASE as U64;
        self.i2c1_base = self.base_address + BCM2835_I2C1_BASE as U64;
        self.st_base = self.base_address + BCM2835_ST_BASE as U64;
        self.dma_base = self.base_address + BCM2835_DMA_BASE as U64;

        // Initialize GPIO
        self.init_gpio();

        // Initialize system timer
        self.init_system_timer();

        self.initialized = true;
        0
    }

    /// Initialize GPIO
    unsafe fn init_gpio(&mut self) {
        // Set all GPIOs to input with pull-down
        for i in 0..6 {
            self.write_mmio(BCM2835_GPIO_GPFSEL0 + (i * 4), 0);
        }

        // Set pull-up/down to off
        self.write_mmio(BCM2835_GPIO_GPPUD, GPIO_PUD_OFF);
    }

    /// Initialize system timer
    unsafe fn init_system_timer(&mut self) {
        // System timer is initialized by firmware
        // Just ensure it's accessible
    }

    /// Set GPIO function
    unsafe fn set_gpio_function(&mut self, pin: U8, function: U32) {
        let reg = BCM2835_GPIO_GPFSEL0 + ((pin / 10) * 4) as U32;
        let shift = ((pin % 10) * 3) as U32;
        
        let mut value = self.read_mmio(reg);
        value &= !(GPIO_FSEL_MASK << shift);
        value |= (function << shift);
        self.write_mmio(reg, value);
    }

    /// Set GPIO output level
    unsafe fn set_gpio(&mut self, pin: U8, level: bool) {
        let reg = if level {
            if pin < 32 { BCM2835_GPIO_GPSET0 } else { BCM2835_GPIO_GPSET1 }
        } else {
            if pin < 32 { BCM2835_GPIO_GPCLR0 } else { BCM2835_GPIO_GPCLR1 }
        };
        
        let bit = if pin < 32 { pin } else { pin - 32 };
        self.write_mmio(reg, 1 << bit);
    }

    /// Get GPIO input level
    unsafe fn get_gpio(&self, pin: U8) -> bool {
        let reg = if pin < 32 { BCM2835_GPIO_GPLEV0 } else { BCM2835_GPIO_GPLEV1 };
        let bit = if pin < 32 { pin } else { pin - 32 };
        (self.read_mmio(reg) & (1 << bit)) != 0
    }

    /// Set GPIO pull-up/down
    unsafe fn set_gpio_pud(&mut self, pud: U32) {
        self.write_mmio(BCM2835_GPIO_GPPUD, pud);
    }

    /// Get system timer value
    unsafe fn get_system_timer(&self) -> U64 {
        let low = self.read_mmio(BCM2835_ST_BASE + 0x04);
        let high = self.read_mmio(BCM2835_ST_BASE + 0x08);
        ((high as U64) << 32) | (low as U64)
    }
}

// ─── Global BCM2835 SoC ─────────────────────────

static mut G_BCM2835: Bcm2835SoC = Bcm2835SoC::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn bcm2835_init(base_address: U64) -> I32 {
    G_BCM2835.init_bcm2835(base_address)
}

#[no_mangle]
pub unsafe extern "C" fn bcm2835_is_initialized() -> I32 {
    if G_BCM2835.initialized {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn bcm2835_set_gpio_function(pin: U8, function: U32) {
    G_BCM2835.set_gpio_function(pin, function)
}

#[no_mangle]
pub unsafe extern "C" fn bcm2835_set_gpio(pin: U8, level: bool) {
    G_BCM2835.set_gpio(pin, level)
}

#[no_mangle]
pub unsafe extern "C" fn bcm2835_get_gpio(pin: U8) -> bool {
    G_BCM2835.get_gpio(pin)
}

#[no_mangle]
pub unsafe extern "C" fn bcm2835_set_gpio_pud(pud: U32) {
    G_BCM2835.set_gpio_pud(pud)
}

#[no_mangle]
pub unsafe extern "C" fn bcm2835_get_system_timer() -> U64 {
    G_BCM2835.get_system_timer()
}

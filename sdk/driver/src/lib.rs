// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// sdk/driver/src/lib.rs — SigmaOS Driver SDK (userspace, std)
//
// Provides safe, ergonomic building blocks for writing SigmaOS drivers
// in Rust userspace (ring-3 isolated drivers).
//
// Components:
//   - Driver trait (lifecycle: probe/init/irq/shutdown)
//   - MMIO mapping helpers
//   - DMA buffer allocation
//   - IRQ binding via eventfd-style API
//   - sigma-bus IPC channel integration
//   - PCI device discovery

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::os::unix::io::RawFd;

// ── Driver trait ──────────────────────────────────────────────────────────

/// Core lifecycle trait every SigmaOS driver must implement.
pub trait Driver: Send + Sync {
    /// Check if this driver handles the given device.
    /// Return true if the driver claims the device.
    fn probe(&self, device: &DeviceInfo) -> bool;

    /// Initialize hardware. Called after probe() returns true.
    fn init(&mut self, ctx: &mut DriverContext) -> DriverResult<()>;

    /// Handle a hardware interrupt. Return true if the IRQ was ours.
    fn handle_irq(&mut self) -> bool { false }

    /// Suspend the device (power management).
    fn suspend(&mut self) -> DriverResult<()> { Ok(()) }

    /// Resume the device after suspend.
    fn resume(&mut self) -> DriverResult<()> { Ok(()) }

    /// Shut down and release all hardware resources.
    fn shutdown(&mut self);

    /// Human-readable driver name.
    fn name(&self) -> &str;

    /// Driver version string.
    fn version(&self) -> &str { "0.1.0" }
}

// ── Device info ───────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub vendor_id:  u16,
    pub device_id:  u16,
    pub class:      DriverClass,
    pub bar0:       u64,  // BAR0 physical address
    pub bar1:       u64,
    pub irq:        u8,
    pub subsystem:  u32,
    pub pci_addr:   PciAddr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PciAddr {
    pub bus:  u8,
    pub dev:  u8,
    pub func: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverClass {
    Network,
    Storage,
    Display,
    Audio,
    Input,
    Usb,
    Serial,
    Wireless,
    Gpu,
    Generic,
}

// ── Driver context ────────────────────────────────────────────────────────
pub struct DriverContext {
    pub device:   DeviceInfo,
    pub mmio:     Option<MmioMapping>,
    pub dma_bufs: Vec<DmaBuffer>,
    pub irq_fd:   Option<RawFd>,
    pub bus_ch:   u32,
}

impl DriverContext {
    pub fn new(device: DeviceInfo, bus_channel: u32) -> Self {
        Self {
            device, bus_ch: bus_channel,
            mmio: None, dma_bufs: Vec::new(), irq_fd: None,
        }
    }

    /// Map BAR0 MMIO region into driver's address space.
    pub fn map_bar0(&mut self, size: usize) -> DriverResult<()> {
        let phys = self.device.bar0;
        self.mmio = Some(MmioMapping::new(phys, size)?);
        Ok(())
    }

    /// Allocate a DMA-coherent buffer.
    pub fn alloc_dma(&mut self, size: usize) -> DriverResult<&DmaBuffer> {
        let buf = DmaBuffer::alloc(size)?;
        self.dma_bufs.push(buf);
        Ok(self.dma_bufs.last().unwrap())
    }

    /// Bind IRQ line for this device.
    pub fn bind_irq(&mut self) -> DriverResult<()> {
        // In production: open /dev/sigma/irq/{irq_num}
        // For now: create an eventfd-style fd
        let irq = self.device.irq;
        let path = format!("/dev/sigma/irq/{}", irq);
        match fs::OpenOptions::new().read(true).open(&path) {
            Ok(f) => {
                use std::os::unix::io::IntoRawFd;
                self.irq_fd = Some(f.into_raw_fd());
                Ok(())
            }
            Err(_) => {
                // IRQ subsystem not yet available — log and continue
                eprintln!("[ddk] IRQ {} bind: /dev/sigma/irq/ not available (Phase C)", irq);
                Ok(())
            }
        }
    }

    /// Send an event to the sigma-bus channel.
    pub fn bus_send(&self, kind: u32, data: &[u8]) -> DriverResult<()> {
        // In production: write to /run/sigma/bus/{channel}
        let path = format!("/run/sigma/bus/{}", self.bus_ch);
        match fs::OpenOptions::new().write(true).create(true).append(true).open(&path) {
            Ok(mut f) => {
                let header = [kind.to_le_bytes(), (data.len() as u32).to_le_bytes()].concat();
                f.write_all(&header)?;
                f.write_all(data)?;
                Ok(())
            }
            Err(_) => Ok(()) // sigma-bus not yet available
        }
    }
}

// ── MMIO mapping ──────────────────────────────────────────────────────────
pub struct MmioMapping {
    pub virt_base: *mut u8,
    pub size:      usize,
    pub phys_base: u64,
}

unsafe impl Send for MmioMapping {}
unsafe impl Sync for MmioMapping {}

impl MmioMapping {
    fn new(phys_base: u64, size: usize) -> DriverResult<Self> {
        // In production: mmap /dev/sigma/mmio or /dev/mem
        // For development: allocate a heap buffer as a stand-in
        let layout = std::alloc::Layout::from_size_align(size, 4096)
            .map_err(|_| DriverError::AllocationFailed)?;
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if ptr.is_null() { return Err(DriverError::AllocationFailed); }
        Ok(Self { virt_base: ptr, size, phys_base })
    }

    /// Volatile 32-bit MMIO read at byte offset.
    pub unsafe fn read32(&self, offset: usize) -> u32 {
        debug_assert!(offset + 4 <= self.size);
        let ptr = self.virt_base.add(offset) as *const u32;
        std::ptr::read_volatile(ptr)
    }

    /// Volatile 32-bit MMIO write at byte offset.
    pub unsafe fn write32(&self, offset: usize, val: u32) {
        debug_assert!(offset + 4 <= self.size);
        let ptr = self.virt_base.add(offset) as *mut u32;
        std::ptr::write_volatile(ptr, val);
    }

    /// Volatile 64-bit MMIO read.
    pub unsafe fn read64(&self, offset: usize) -> u64 {
        let ptr = self.virt_base.add(offset) as *const u64;
        std::ptr::read_volatile(ptr)
    }

    /// Volatile 64-bit MMIO write.
    pub unsafe fn write64(&self, offset: usize, val: u64) {
        let ptr = self.virt_base.add(offset) as *mut u64;
        std::ptr::write_volatile(ptr, val);
    }
}

impl Drop for MmioMapping {
    fn drop(&mut self) {
        if !self.virt_base.is_null() {
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(self.size, 4096);
                std::alloc::dealloc(self.virt_base, layout);
            }
        }
    }
}

// ── DMA buffer ────────────────────────────────────────────────────────────
pub struct DmaBuffer {
    pub virt_addr: *mut u8,
    pub phys_addr: u64,
    pub size:      usize,
}

unsafe impl Send for DmaBuffer {}
unsafe impl Sync for DmaBuffer {}

impl DmaBuffer {
    fn alloc(size: usize) -> DriverResult<Self> {
        let layout = std::alloc::Layout::from_size_align(size, 4096)
            .map_err(|_| DriverError::AllocationFailed)?;
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if ptr.is_null() { return Err(DriverError::AllocationFailed); }
        // Physical address: in production, query kernel for DMA address.
        // Here: use virtual address as stand-in (identity-mapped in dev env).
        let phys = ptr as u64;
        Ok(Self { virt_addr: ptr, phys_addr: phys, size })
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.virt_addr, self.size) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.virt_addr, self.size) }
    }
}

impl Drop for DmaBuffer {
    fn drop(&mut self) {
        if !self.virt_addr.is_null() {
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(self.size, 4096);
                std::alloc::dealloc(self.virt_addr, layout);
            }
        }
    }
}

// ── Error types ───────────────────────────────────────────────────────────
#[derive(Debug)]
pub enum DriverError {
    AllocationFailed,
    DeviceNotFound,
    HardwareError(String),
    IrqBindFailed,
    Io(io::Error),
}

pub type DriverResult<T> = Result<T, DriverError>;

impl From<io::Error> for DriverError {
    fn from(e: io::Error) -> Self { DriverError::Io(e) }
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::AllocationFailed  => write!(f, "Memory allocation failed"),
            DriverError::DeviceNotFound    => write!(f, "Device not found"),
            DriverError::HardwareError(s)  => write!(f, "Hardware error: {}", s),
            DriverError::IrqBindFailed     => write!(f, "IRQ bind failed"),
            DriverError::Io(e)             => write!(f, "I/O error: {}", e),
        }
    }
}

// ── PCI device discovery ──────────────────────────────────────────────────
/// Enumerate PCI devices from /sys/bus/pci/devices (Linux host or sigma-compat).
pub fn pci_enumerate() -> Vec<DeviceInfo> {
    let mut devices = Vec::new();
    let pci_path = "/sys/bus/pci/devices";

    if let Ok(entries) = fs::read_dir(pci_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let vendor_id = read_hex_file(&path.join("vendor")).unwrap_or(0) as u16;
            let device_id = read_hex_file(&path.join("device")).unwrap_or(0) as u16;
            let class_raw = read_hex_file(&path.join("class")).unwrap_or(0);

            let class = match (class_raw >> 16) as u8 {
                0x02 => DriverClass::Network,
                0x01 => DriverClass::Storage,
                0x03 => DriverClass::Display,
                0x04 => DriverClass::Audio,
                0x0C => DriverClass::Usb,
                _ => DriverClass::Generic,
            };

            if vendor_id != 0 && vendor_id != 0xFFFF {
                devices.push(DeviceInfo {
                    vendor_id, device_id, class,
                    bar0: 0, bar1: 0, irq: 0,
                    subsystem: 0,
                    pci_addr: PciAddr { bus: 0, dev: 0, func: 0 },
                });
            }
        }
    }
    devices
}

fn read_hex_file(path: &std::path::Path) -> Option<u64> {
    let content = fs::read_to_string(path).ok()?;
    let stripped = content.trim().trim_start_matches("0x");
    u64::from_str_radix(stripped, 16).ok()
}

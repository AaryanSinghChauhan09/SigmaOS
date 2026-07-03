// SPDX-License-Identifier: MIT
// sdk/driver/examples/virtio_blk.rs — Example virtio-blk userspace driver
//
// Demonstrates the SigmaOS Driver SDK for a VirtIO block device.
// Run in QEMU: qemu-system-x86_64 -drive ...,if=virtio
//
// cargo run --example virtio_blk

use sigma_driver_sdk::{
    Driver, DeviceInfo, DriverClass, DriverContext, DriverResult, DriverError,
    PciAddr, pci_enumerate,
};

// ── VirtIO PCI IDs ────────────────────────────────────────────────────────
const VIRTIO_VENDOR:       u16 = 0x1AF4;
const VIRTIO_BLK_DEVICE:   u16 = 0x1001; // legacy VirtIO-BLK
const VIRTIO_BLK_DEVICE2:  u16 = 0x1042; // modern VirtIO-BLK

// ── VirtIO block driver ───────────────────────────────────────────────────
struct VirtioBlkDriver {
    sector_count: u64,
    block_size:   u32,
}

impl Driver for VirtioBlkDriver {
    fn name(&self) -> &str { "sigma-virtio-blk" }
    fn version(&self) -> &str { "0.1.0" }

    fn probe(&self, device: &DeviceInfo) -> bool {
        device.vendor_id == VIRTIO_VENDOR
            && (device.device_id == VIRTIO_BLK_DEVICE
                || device.device_id == VIRTIO_BLK_DEVICE2)
    }

    fn init(&mut self, ctx: &mut DriverContext) -> DriverResult<()> {
        println!("[virtio-blk] Initializing device {:04x}:{:04x}",
                 ctx.device.vendor_id, ctx.device.device_id);

        // Map BAR0 (VirtIO legacy config registers, 256 bytes)
        ctx.map_bar0(256)?;

        let mmio = ctx.mmio.as_ref().ok_or(DriverError::HardwareError(
            "BAR0 not mapped".into()
        ))?;

        // Read device features (VirtIO spec 4.1.4)
        let features = unsafe { mmio.read32(0x00) };
        println!("[virtio-blk] Device features: 0x{:08x}", features);

        // Acknowledge device + driver
        unsafe {
            mmio.write32(0x12, 0x00);  // reset
            mmio.write32(0x12, 0x01);  // ACKNOWLEDGE
            mmio.write32(0x12, 0x03);  // ACKNOWLEDGE | DRIVER
        }

        // Read capacity from config space (offset 0x14 in legacy VirtIO)
        self.sector_count = unsafe { mmio.read64(0x14) };
        self.block_size   = unsafe { mmio.read32(0x1C) };
        if self.block_size == 0 { self.block_size = 512; }

        println!("[virtio-blk] Capacity: {} sectors ({} MB)",
                 self.sector_count,
                 self.sector_count * self.block_size as u64 / 1_048_576);

        // Set FEATURES_OK
        unsafe { mmio.write32(0x12, 0x0B); } // ACKNOWLEDGE|DRIVER|FEATURES_OK

        // Allocate a DMA buffer for the request ring
        let _ring_buf = ctx.alloc_dma(4096)?;

        // Bind IRQ
        ctx.bind_irq()?;

        // Set DRIVER_OK
        unsafe { mmio.write32(0x12, 0x0F); }

        println!("[virtio-blk] ✓ Initialized successfully");
        Ok(())
    }

    fn handle_irq(&mut self) -> bool {
        println!("[virtio-blk] IRQ received — I/O completion");
        true
    }

    fn shutdown(&mut self) {
        println!("[virtio-blk] Shutting down");
    }
}

fn main() {
    println!("Σ SigmaOS VirtIO-BLK Example Driver");

    // Enumerate PCI devices
    let devices = pci_enumerate();
    println!("Found {} PCI device(s)", devices.len());

    let mut driver = VirtioBlkDriver {
        sector_count: 0,
        block_size: 512,
    };

    // Find a matching device
    let found = devices.into_iter().find(|d| driver.probe(d));

    match found {
        Some(device) => {
            println!("[main] Claiming device {:04x}:{:04x} (class {:?})",
                     device.vendor_id, device.device_id, device.class);
            let mut ctx = DriverContext::new(device, 0x01); // bus channel 1 = storage
            match driver.init(&mut ctx) {
                Ok(()) => println!("[main] Driver initialized successfully"),
                Err(e) => eprintln!("[main] Init failed: {}", e),
            }
        }
        None => {
            println!("[main] No VirtIO-BLK device found");
            println!("[main] To test with QEMU:");
            println!("  qemu-system-x86_64 \\ ");
            println!("    -drive file=test.img,if=virtio,format=qcow2 \\ ");
            println!("    -device virtio-blk-pci,drive=hd0");
        }
    }
}

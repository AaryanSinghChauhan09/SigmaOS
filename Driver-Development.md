# SigmaOS Driver Development

## Driver Framework

All SigmaOS drivers implement the `SigmaDriver` trait:

```rust
pub trait SigmaDriver: Send + Sync {
    fn name(&self) -> &str;
    fn probe(&mut self, device: &DeviceInfo) -> Result<(), DriverError>;
    fn remove(&mut self) -> Result<(), DriverError>;
    fn read(&self, offset: usize, buf: &mut [u8]) -> Result<usize, DriverError>;
    fn write(&mut self, offset: usize, buf: &[u8]) -> Result<usize, DriverError>;
    fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<usize, DriverError>;
}
```

## Writing a PCI Driver

```rust
use sigma::driver::{SigmaDriver, DeviceInfo, DriverError};
use sigma::driver::pci::PciDevice;

pub struct MyNicDriver {
    base_addr: u64,
    irq: u8,
}

impl SigmaDriver for MyNicDriver {
    fn name(&self) -> &str { "my-nic" }
    
    fn probe(&mut self, device: &DeviceInfo) -> Result<(), DriverError> {
        // 1. Map MMIO region
        // 2. Reset device
        // 3. Configure interrupts
        // 4. Bring up device
        Ok(())
    }
    
    fn read(&self, _offset: usize, buf: &mut [u8]) -> Result<usize, DriverError> {
        // Read packet from RX ring
        Ok(0)
    }
    
    fn write(&mut self, _offset: usize, buf: &[u8]) -> Result<usize, DriverError> {
        // Write packet to TX ring
        Ok(buf.len())
    }
    
    fn remove(&mut self) -> Result<(), DriverError> {
        // Disable device and release resources
        Ok(())
    }
}
```

## x86 Port I/O

```rust
// Read from I/O port
pub fn inb(port: u16) -> u8 {
    let v: u8;
    unsafe { core::arch::asm!("in al, dx", in("dx") port, out("al") v); }
    v
}

// Write to I/O port  
pub fn outb(port: u16, value: u8) {
    unsafe { core::arch::asm!("out dx, al", in("dx") port, in("al") value); }
}
```

## Registering Your Driver

Add to `src/driver/mod.rs`:
```rust
pub mod my_nic_driver;
use my_nic_driver::MyNicDriver;

pub fn register_drivers() {
    DRIVER_REGISTRY.register(
        PciId::new(0x8086, 0x1533),  // Intel I210
        || Box::new(MyNicDriver { base_addr: 0, irq: 0 })
    );
}
```

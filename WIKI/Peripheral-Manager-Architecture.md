# Peripheral Manager Architecture

SigmaOS utilizes an Object-Oriented Programming (OOP) driven architecture to unify the handling of peripheral devices, regardless of their hardware generation.

## The `PeripheralDevice` Trait

By relying on the `PeripheralDevice` trait, the operating system kernel is completely decoupled from device-specific initialization or I/O. This means that:
- **Legacy Devices** (e.g., PS/2 Keyboards, Serial Mice)
- **Modern Devices** (e.g., USB 3.0, PCIe devices, Thunderbolt peripherals)

...are all handled via the exact same polymorphic interface in Rust.

## Disk Space and Binary Footprint

To satisfy our constraints of low disk usage, SigmaOS employs **dynamic dispatch** (`Box<dyn PeripheralDevice>`) rather than massive, aggressively monomorphized generics.
This limits binary bloat, ensuring that adding countless user-defined driver functions for custom hardware takes minimal extra storage space.

## Implementing a User-Defined Driver

Writing a custom driver is simple. Create a struct and implement the trait:

```rust
use crate::drivers::peripheral::{PeripheralDevice, DeviceGeneration, PowerState};

pub struct MyCustomPeripheral { ... }

impl PeripheralDevice for MyCustomPeripheral {
    fn name(&self) -> &'static str { "Custom Device" }
    fn generation(&self) -> DeviceGeneration { DeviceGeneration::Modern }
    fn initialize(&mut self) -> Result<(), &'static str> { Ok(()) }
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> { Ok(0) }
    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> { Ok(0) }
    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> { Ok(()) }
    fn shutdown(&mut self) -> Result<(), &'static str> { Ok(()) }
}
```

Once implemented, simply pass it into `PeripheralManager::register_device`. The unified manager automatically handles dynamic routing, sleep states, and graceful shutdowns.

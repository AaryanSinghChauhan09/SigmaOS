# Driver Development Guide

Writing drivers in SigmaOS differs from traditional monolithic kernels due to our strict safety guarantees.

## 1. Overview

Drivers must implement standard traits mapped to hardware subsystems. Most device drivers can run in user-space using `vfio` or specific capability tokens.

## 2. Example: Simple Storage Driver

```rust
use sigmaos::driver::StorageDriver;

pub struct MyDriver;

impl StorageDriver for MyDriver {
    fn read_block(&self, lba: u64, buf: &mut [u8]) -> Result<(), DriverError> {
        // Read logic
        Ok(())
    }
    
    fn write_block(&mut self, lba: u64, buf: &[u8]) -> Result<(), DriverError> {
        // Write logic
        Ok(())
    }
}
```

## 3. Registering a Driver

Drivers are registered at boot or dynamically loaded via signed modules.

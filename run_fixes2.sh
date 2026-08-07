#!/bin/bash
sed -i 's/pub fn get_size_by_type(&self, path: &Path)/pub fn get_size_by_type(\&self, _path: \&Path)/g' src/filesystem/disk_usage.rs
sed -i 's/pub fn swap_active_generation(&mut self, generation_id: u32)/pub fn swap_active_generation(\&mut self, _generation_id: u32)/g' src/filesystem/linux_package_parity.rs
sed -i 's/fn send(&self, message: &dyn CANMessage)/fn send(\&self, _message: \&dyn CANMessage)/g' src/embedded/can.rs
sed -i 's/pub fn write_byte(&mut self, byte: u8)/pub fn write_byte(\&mut self, _byte: u8)/g' src/drivers/kernel_io_suite.rs
sed -i 's/for &(ref cat, ref ids) in/for \&(ref _cat, ref ids) in/g' src/desktop/settings.rs
sed -i 's/for &id in ids/for \&_id in ids/g' src/desktop/settings.rs
sed -i 's/pub fn write_memory(&self, address: u64, data: &\[u8\])/pub fn write_memory(\&self, _address: u64, _data: \&[u8])/g' src/debugger/mod.rs

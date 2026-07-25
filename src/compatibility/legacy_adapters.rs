// SigmaOS OOP Legacy & Ancient Subsystem Compatibility Adapters
// Supports running ancient applications, devices, network protocol stacks,
// security permissions, filesystems, and X11/Motif user interfaces.

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::CapabilityToken;

/// 1. Legacy Kernel Adapter
/// Emulates Linux kernel system calls (e.g. Linux 2.x, 3.x, 4.x, 5.x, 6.x)
pub struct LegacyKernelAdapter {
    target_version: &'static str,
}

impl LegacyKernelAdapter {
    pub fn new(target_version: &'static str) -> Self {
        Self { target_version }
    }

    /// Translates ancient Linux syscall numbers to native microkernel primitives
    pub fn translate_syscall(&self, sys_num: usize, args: &[usize]) -> Result<usize, &'static str> {
        // Mock translation logic for traditional sys_read, sys_write, sys_open
        if sys_num == 3 { // sys_read
            return Ok(args[2]); // returns bytes read simulated
        }
        if sys_num == 4 { // sys_write
            return Ok(args[2]); // returns bytes written simulated
        }
        Err("Unsupported ancient syscall version")
    }

    pub fn target_version(&self) -> &'static str {
        self.target_version
    }
}

/// 2. Legacy Driver Adapter
/// Wraps legacy hardware interfaces (ISA bus registers, parallel ports, floppy controllers)
pub struct LegacyDriverAdapter {
    device_name: &'static str,
    base_port: u16,
    is_mapped: bool,
}

impl LegacyDriverAdapter {
    pub fn new(device_name: &'static str, base_port: u16) -> Self {
        Self {
            device_name,
            base_port,
            is_mapped: false,
        }
    }

    pub fn map_isa_registers(&mut self) -> bool {
        self.is_mapped = true;
        true
    }

    pub fn base_port(&self) -> u16 {
        self.base_port
    }
}

impl PeripheralDevice for LegacyDriverAdapter {
    fn name(&self) -> &'static str {
        self.device_name
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_mapped = true;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_mapped {
            return Err("Legacy driver: Registers not mapped");
        }
        if !buffer.is_empty() {
            buffer[0] = 0x55; // Diagnostic byte
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_mapped {
            return Err("Legacy driver: Registers not mapped");
        }
        Ok(data.len())
    }

    fn set_power_state(&mut self, _state: PowerState) -> Result<(), &'static str> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_mapped = false;
        Ok(())
    }
}

/// 3. Legacy Package Adapter
/// Translates older packaging metadata formats (.deb, .rpm, .tgz) into native .spkg spec
pub struct LegacyPackageAdapter {
    pkg_type: &'static str,
}

impl LegacyPackageAdapter {
    pub fn new(pkg_type: &'static str) -> Self {
        Self { pkg_type }
    }

    pub fn convert_package_metadata(&self, raw_metadata: &[u8]) -> Result<[u8; 64], &'static str> {
        let mut mock_spkg_name = [0u8; 64];
        let len = raw_metadata.len().min(63);
        for i in 0..len {
            mock_spkg_name[i] = raw_metadata[i];
        }
        Ok(mock_spkg_name)
    }

    pub fn pkg_type(&self) -> &'static str {
        self.pkg_type
    }
}

/// 4. Legacy Filesystem Adapter
/// Mounts and parses ancient filesystems (FAT32, MinixFS, ReiserFS)
pub struct LegacyFSAdapter {
    fs_type: &'static str,
    total_blocks: u64,
}

impl LegacyFSAdapter {
    pub fn new(fs_type: &'static str, total_blocks: u64) -> Self {
        Self { fs_type, total_blocks }
    }

    pub fn read_file_entry(&self, cluster_idx: u32, out_buf: &mut [u8]) -> Result<usize, &'static str> {
        if cluster_idx as u64 >= self.total_blocks {
            return Err("Cluster index out of filesystem bounds");
        }
        if !out_buf.is_empty() {
            out_buf[0] = b'F'; // Mock file content
            Ok(1)
        } else {
            Ok(0)
        }
    }

    pub fn fs_type(&self) -> &'static str {
        self.fs_type
    }
}

/// 5. Legacy Protocol Adapter
/// Handles ancient dial-up, serial, or IPv4-only stack packetization (PPP, SLIP)
pub struct LegacyProtocolAdapter {
    protocol: &'static str,
}

impl LegacyProtocolAdapter {
    pub fn new(protocol: &'static str) -> Self {
        Self { protocol }
    }

    pub fn encapsulate_packet(&self, raw_data: &[u8], out_buf: &mut [u8]) -> Result<usize, &'static str> {
        if out_buf.len() < raw_data.len() + 2 {
            return Err("Output buffer too small for framing");
        }
        // PPP / SLIP framing: e.g. wraps data with 0xC0 marker
        out_buf[0] = 0xC0;
        for i in 0..raw_data.len() {
            out_buf[i + 1] = raw_data[i];
        }
        out_buf[raw_data.len() + 1] = 0xC0;
        Ok(raw_data.len() + 2)
    }

    pub fn protocol(&self) -> &'static str {
        self.protocol
    }
}

/// 6. Legacy Security Adapter
/// Integrates older Linux DAC (Discretionary Access Control) permissions into Zero-Trust microkernel capability tokens
pub struct LegacySecurityAdapter {
    allow_suid: bool,
}

impl LegacySecurityAdapter {
    pub fn new(allow_suid: bool) -> Self {
        Self { allow_suid }
    }

    /// Converts ancient 9-bit octal file modes (e.g. 0o755) to secure microkernel `CapabilityToken`
    pub fn mode_to_capability(&self, mode: u32) -> CapabilityToken {
        if mode == 0o777 || (self.allow_suid && (mode & 0o4000) != 0) {
            CapabilityToken::from_bits(0xFFFF) // Master override
        } else {
            CapabilityToken::from_bits(0x04) // Limited read token
        }
    }
}

/// 7. Legacy UI Adapter
/// Transparently handles ancient graphical layers (X11 client requests, Motif, early GTK/Qt widgets)
pub struct LegacyUIAdapter {
    client_name: &'static str,
}

impl LegacyUIAdapter {
    pub fn new(client_name: &'static str) -> Self {
        Self { client_name }
    }

    /// Intercepts legacy X11 protocol packets (e.g. CreateWindow) and renders them natively on the Zenith Compositor
    pub fn translate_x11_event(&self, event_code: u32, out_render_cmd: &mut [u8]) -> Result<usize, &'static str> {
        if out_render_cmd.is_empty() {
            return Ok(0);
        }
        if event_code == 1 { // Create Window
            out_render_cmd[0] = 0xFF; // Zenith native draw window byte
            return Ok(1);
        }
        Ok(0)
    }

    pub fn client_name(&self) -> &'static str {
        self.client_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_kernel_and_driver_adapters() {
        let kernel_adapter = LegacyKernelAdapter::new("2.6.32");
        assert_eq!(kernel_adapter.target_version(), "2.6.32");

        let args = [0, 0, 100];
        let bytes_read = kernel_adapter.translate_syscall(3, &args).unwrap();
        assert_eq!(bytes_read, 100);

        let mut driver_adapter = LegacyDriverAdapter::new("Parallel Port LPT1", 0x378);
        assert_eq!(driver_adapter.base_port(), 0x378);
        assert!(driver_adapter.initialize().is_ok());

        let mut read_buf = [0u8; 10];
        assert_eq!(driver_adapter.read(&mut read_buf).unwrap(), 1);
        assert_eq!(read_buf[0], 0x55);
    }

    #[test]
    fn test_legacy_package_and_fs_adapters() {
        let pkg_adapter = LegacyPackageAdapter::new(".deb");
        assert_eq!(pkg_adapter.pkg_type(), ".deb");

        let metadata = b"ancient-lib-v1";
        let spkg_meta = pkg_adapter.convert_package_metadata(metadata).unwrap();
        assert_eq!(&spkg_meta[..metadata.len()], metadata);

        let fs_adapter = LegacyFSAdapter::new("FAT32", 1000);
        assert_eq!(fs_adapter.fs_type(), "FAT32");

        let mut file_buf = [0u8; 10];
        assert_eq!(fs_adapter.read_file_entry(5, &mut file_buf).unwrap(), 1);
        assert_eq!(file_buf[0], b'F');
    }

    #[test]
    fn test_legacy_network_security_and_ui_adapters() {
        let proto_adapter = LegacyProtocolAdapter::new("PPP");
        assert_eq!(proto_adapter.protocol(), "PPP");

        let mut out_pkt = [0u8; 20];
        let len = proto_adapter.encapsulate_packet(b"Hello", &mut out_pkt).unwrap();
        assert_eq!(len, 7);
        assert_eq!(out_pkt[0], 0xC0);
        assert_eq!(out_pkt[6], 0xC0);

        let sec_adapter = LegacySecurityAdapter::new(true);
        let cap_suid = sec_adapter.mode_to_capability(0o4755);
        assert_eq!(cap_suid.bits(), 0xFFFF);

        let ui_adapter = LegacyUIAdapter::new("xterm");
        assert_eq!(ui_adapter.client_name(), "xterm");

        let mut render_cmd = [0u8; 10];
        let bytes_written = ui_adapter.translate_x11_event(1, &mut render_cmd).unwrap();
        assert_eq!(bytes_written, 1);
        assert_eq!(render_cmd[0], 0xFF);
    }
}

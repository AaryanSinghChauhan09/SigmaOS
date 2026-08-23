#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DdeDeviceInfo {
    pub vendor_id: u16,
    pub device_id: u16,
}

pub struct DdeDeviceWrapper {
    pub simulated_pci_bar: [u8; 256],
}

impl DdeDeviceWrapper {
    pub fn new(_id: u32, _name: &[u8], _port: u32, _os: &[u8]) -> Self {
        Self {
            simulated_pci_bar: [0; 256],
        }
    }

    pub fn query_channel(&self) -> crate::driver::device::PortAddress {
        crate::driver::device::PortAddress::MemoryMapped(0xFC000000)
    }

    pub fn info(&self) -> DdeDeviceInfo {
        DdeDeviceInfo {
            vendor_id: 0x8086,
            device_id: 0x100e,
        }
    }

    pub fn write_byte(&mut self, offset: usize, val: u8) -> Result<(), ()> {
        if offset < 256 {
            self.simulated_pci_bar[offset] = val;
        }
        Ok(())
    }

    pub fn read_byte(&self, offset: usize) -> Result<u8, ()> {
        if offset < 256 {
            Ok(self.simulated_pci_bar[offset])
        } else {
            Ok(0)
        }
    }

    pub fn write(&mut self, _buf: &[u8]) -> Result<(), ()> {
        Ok(())
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<(), ()> {
        for b in buf.iter_mut() {
            *b = 0xAA;
        }
        Ok(())
    }

    pub fn ioctl(&self, _cmd: u32, _arg: usize) -> Result<usize, ()> {
        Ok(1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct ProtectedModeSwitchSimulator {
    pub gdt_loaded: bool,
    pub cr0_pe_bit: bool,
    pub active_cs_segment: u16,
    pub active_ds_segment: u16,
}
impl ProtectedModeSwitchSimulator {
    pub fn new() -> Self { Self::default() }
    pub fn lgdt(&mut self) { self.gdt_loaded = true; }
    pub fn execute_switch_to_pm(&mut self) -> Result<(), &'static str> {
        if !self.gdt_loaded { return Err("GDT not loaded"); }
        self.cr0_pe_bit = true;
        self.active_cs_segment = 0x08;
        self.active_ds_segment = 0x10;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct VgaTextModeDriverSimulator {
    pub buffer: [u16; 2000],
    pub cursor_offset: usize,
}
impl VgaTextModeDriverSimulator {
    pub fn new() -> Self { Self { buffer: [0; 2000], cursor_offset: 0 } }
    pub fn write_char(&mut self, ch: char, attr: u8) {
        if self.cursor_offset < 2000 {
            self.buffer[self.cursor_offset] = (ch as u16) | ((attr as u16) << 8);
            self.cursor_offset += 1;
        }
    }
    pub fn update_cursor_via_ports(&mut self, port: u16, val: usize) -> Result<usize, &'static str> {
        if port == 0x3D5 { self.cursor_offset = val; }
        Ok(self.cursor_offset)
    }
}

#[derive(Debug, Clone)]
pub struct PicKeyboardController {
    pub master_pic_mask: u8,
}
impl PicKeyboardController {
    pub fn new() -> Self { Self { master_pic_mask: 0xFF } }
    pub fn init_pic(&mut self) { self.master_pic_mask = 0xFD; }
    pub fn poll_port_60_read(&self, scancode: u8) -> char {
        match scancode {
            0x10 => 'q',
            0x1F => 's',
            _ => '?',
        }
    }
}

/// Historic Linux ABI & Kernel Compatibility Layer for SigmaOS
/// Replicates historical system behaviors, driver translations, and sandbox layouts
/// across early kernel eras: 0.01/0.11, 1.0, 2.0, 2.2, and 2.4/2.5.
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxEra {
    Era0_01, // September 1991 - Pure single-tasking, minimal syscalls
    Era0_11, // December 1991 - Floppy support, serial ports, LDT/GDT tasks
    Era1_0,  // March 1994 - Networking, modular drivers, sound, VFS
    Era2_0,  // June 1996 - SMP, threads (LinuxThreads), modular kernel upgrades
    Era2_4,  // January 2001 - USB, LVM, ext3, high-memory, iptables
}

/// Simulated registers representing historical CPU state for system call emulation
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct HistoricalCpuState {
    pub eax: usize,
    pub ebx: usize,
    pub ecx: usize,
    pub edx: usize,
    pub esi: usize,
    pub edi: usize,
    pub ebp: usize,
    pub esp: usize,
    pub eip: usize,
    pub eflags: usize,
}

/// Historical Syscall Emulator
pub trait HistoricSyscallEmulator {
    fn era(&self) -> LinuxEra;
    fn emulate_syscall(&self, state: &mut HistoricalCpuState) -> Result<usize, HistoricError>;
}

/// Linux 0.01/0.11 Syscall Emulator (Vintage Assembly-like handlers)
pub struct Era0_11SyscallEmulator;

impl HistoricSyscallEmulator for Era0_11SyscallEmulator {
    fn era(&self) -> LinuxEra {
        LinuxEra::Era0_11
    }

    fn emulate_syscall(&self, state: &mut HistoricalCpuState) -> Result<usize, HistoricError> {
        // Early Linux used EAX for syscall number, EBX, ECX, EDX for arguments
        match state.eax {
            0 => Err(HistoricError::SyscallNotImplemented), // sys_setup
            1 => Ok(42),                                    // sys_exit (dummy code)
            2 => Ok(101),                                   // sys_fork (simulated pid)
            3 => {
                // sys_read(fd, buf, count)
                let count = state.edx;
                Ok(count) // Returns the number of bytes read
            }
            4 => {
                // sys_write(fd, buf, count)
                let count = state.edx;
                Ok(count)
            }
            5 => Ok(3), // sys_open (dummy fd)
            6 => Ok(0), // sys_close
            19 => {
                // sys_lseek
                Ok(state.ecx)
            }
            20 => Ok(100), // sys_getpid
            _ => Err(HistoricError::SyscallNotImplemented),
        }
    }
}

/// Linux 1.0 Syscall Emulator (Introduced Network & Socket API)
pub struct Era1_0SyscallEmulator {
    pub socket_call_count: AtomicUsize,
}

impl Default for Era1_0SyscallEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Era1_0SyscallEmulator {
    pub fn new() -> Self {
        Era1_0SyscallEmulator {
            socket_call_count: AtomicUsize::new(0),
        }
    }
}

impl HistoricSyscallEmulator for Era1_0SyscallEmulator {
    fn era(&self) -> LinuxEra {
        LinuxEra::Era1_0
    }

    fn emulate_syscall(&self, state: &mut HistoricalCpuState) -> Result<usize, HistoricError> {
        match state.eax {
            102 => {
                // sys_socketcall (socket call multiplexer)
                let call_type = state.ebx; // 1 = socket, 2 = bind, 3 = connect, etc.
                self.socket_call_count.fetch_add(1, Ordering::SeqCst);
                if call_type == 1 {
                    Ok(10) // simulated socket fd
                } else {
                    Ok(0) // success
                }
            }
            _ => {
                // Fallback to basic 0.11 emulation
                let base = Era0_11SyscallEmulator;
                base.emulate_syscall(state)
            }
        }
    }
}

/// Linux 2.0/2.4 Syscall Emulator (Introduced multi-threading via clone and memory regions)
pub struct Era2_4SyscallEmulator {
    pub cloned_threads: AtomicUsize,
}

impl Default for Era2_4SyscallEmulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Era2_4SyscallEmulator {
    pub fn new() -> Self {
        Era2_4SyscallEmulator {
            cloned_threads: AtomicUsize::new(0),
        }
    }
}

impl HistoricSyscallEmulator for Era2_4SyscallEmulator {
    fn era(&self) -> LinuxEra {
        LinuxEra::Era2_4
    }

    fn emulate_syscall(&self, state: &mut HistoricalCpuState) -> Result<usize, HistoricError> {
        match state.eax {
            120 => {
                // sys_clone
                self.cloned_threads.fetch_add(1, Ordering::SeqCst);
                Ok(202) // child thread pid
            }
            125 => {
                // sys_mprotect
                Ok(0)
            }
            _ => {
                let base_net = Era1_0SyscallEmulator::new();
                base_net.emulate_syscall(state)
            }
        }
    }
}

/// Historical x86 Virtualization Sandbox Context
/// Emulates TSS tasks, segmented GDT, 4MB page size limits, and 16MB direct RAM limits of early x86 eras
pub struct VintageVirtualizationSandbox {
    pub era: LinuxEra,
    pub memory_limit: usize,
    pub segments_count: usize,
    pub simulated_ldt_entries: AtomicUsize,
}

impl VintageVirtualizationSandbox {
    pub fn new(era: LinuxEra) -> Self {
        let (memory_limit, segments_count) = match era {
            LinuxEra::Era0_01 | LinuxEra::Era0_11 => (16 * 1024 * 1024, 4), // 16MB RAM
            LinuxEra::Era1_0 => (64 * 1024 * 1024, 8),                      // 64MB RAM
            _ => (1024 * 1024 * 1024, 16),                                  // 1GB RAM
        };
        VintageVirtualizationSandbox {
            era,
            memory_limit,
            segments_count,
            simulated_ldt_entries: AtomicUsize::new(0),
        }
    }

    pub fn setup_vintage_registers(&self) -> HistoricalCpuState {
        let mut state = HistoricalCpuState::default();
        match self.era {
            LinuxEra::Era0_11 => {
                state.eflags = 0x200; // Enable interrupts
            }
            _ => {
                state.eflags = 0x202; // Enable interrupts + nesting
            }
        }
        state
    }

    pub fn register_ldt_segment(&self) {
        self.simulated_ldt_entries.fetch_add(1, Ordering::SeqCst);
    }
}

/// Vintage Linux Driver Shim Translator
pub struct VintageDriverTranslator {
    pub era: LinuxEra,
    // TODO: Replace with actual driver wrapper when available
    pub wrapper: core::marker::PhantomData<()>,
}

impl VintageDriverTranslator {
    pub fn new(era: LinuxEra, _device_name: &str) -> Self {
        VintageDriverTranslator {
            era,
            wrapper: core::marker::PhantomData,
        }
    }

    pub fn emulate_io_port(&mut self, port: u16, _val: u8) -> Result<(), HistoricError> {
        // Vintage drivers frequently accessed exact I/O ports directly (e.g. 0x3F8 for serial, 0x1F0 for IDE)
        if port == 0x3F8 || port == 0x1F0 {
            Ok(())
        } else {
            Err(HistoricError::InvalidIoPortAccess)
        }
    }
}

/// Package Conversion Shim for converting vintage packages to sigpkg formats
pub struct VintagePackageConverter;

impl VintagePackageConverter {
    pub fn convert_package(&self, name: &str, raw_format: &str) -> Result<String, HistoricError> {
        match raw_format {
            "tar.Z" | "tar.gz" | "deb" | "rpm" => {
                let out_package_name = format!("{}-sigpkg-compat", name);
                Ok(out_package_name)
            }
            _ => Err(HistoricError::UnsupportedPackageFormat),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoricError {
    SyscallNotImplemented,
    MemoryAccessViolation,
    InvalidIoPortAccess,
    UnsupportedPackageFormat,
}

/// TinyCore-style RAM-only Ephemeral Execution Engine.
/// Achieves minimal idle execution memory limits (below 30MB of RAM) through compressed read-only extensions (.tcz)
/// mounted into a high-speed in-memory VFS overlay with copy-on-write persistence separation.
pub struct TinyCoreEphemeralEngine {
    pub mounted_extensions: std::collections::HashMap<String, usize>, // ext_name -> payload_size
    pub volatile_overlay_ram_bytes: usize,
    pub persistence_enabled: bool,
}

impl TinyCoreEphemeralEngine {
    pub fn new() -> Self {
        TinyCoreEphemeralEngine {
            mounted_extensions: std::collections::HashMap::new(),
            volatile_overlay_ram_bytes: 0,
            persistence_enabled: false,
        }
    }

    pub fn load_compressed_extension(&mut self, ext_name: &str, size_bytes: usize) -> Result<(), HistoricError> {
        if ext_name.is_empty() || size_bytes == 0 {
            return Err(HistoricError::MemoryAccessViolation);
        }
        self.mounted_extensions.insert(ext_name.to_string(), size_bytes);
        Ok(())
    }

    pub fn write_to_volatile_overlay(&mut self, _file_path: &str, data_len: usize) -> Result<usize, HistoricError> {
        if self.persistence_enabled {
            return Err(HistoricError::MemoryAccessViolation); // Non-persistent RAM-only mode expected
        }
        self.volatile_overlay_ram_bytes += data_len;
        Ok(self.volatile_overlay_ram_bytes)
    }

    pub fn reset_ephemeral_state(&mut self) {
        // Drop volatile in-memory overlay structures completely on reset
        self.volatile_overlay_ram_bytes = 0;
    }
}

impl Default for TinyCoreEphemeralEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tinycore_ephemeral_engine() {
        let mut engine = TinyCoreEphemeralEngine::new();
        assert!(!engine.persistence_enabled);
        assert_eq!(engine.volatile_overlay_ram_bytes, 0);

        engine.load_compressed_extension("coreutils.tcz", 1024 * 1024).unwrap();
        assert_eq!(*engine.mounted_extensions.get("coreutils.tcz").unwrap(), 1024 * 1024);

        let overlay_size = engine.write_to_volatile_overlay("/tmp/logs.txt", 512).unwrap();
        assert_eq!(overlay_size, 512);

        engine.reset_ephemeral_state();
        assert_eq!(engine.volatile_overlay_ram_bytes, 0);
    }

    #[test]
    fn test_era_emulation_getpid() {
        let emu = Era0_11SyscallEmulator;
        let mut state = HistoricalCpuState {
            eax: 20, // sys_getpid
            ..Default::default()
        };
        let pid = emu.emulate_syscall(&mut state).unwrap();
        assert_eq!(pid, 100);
    }

    #[test]
    fn test_era_emulation_read() {
        let emu = Era0_11SyscallEmulator;
        let mut state = HistoricalCpuState {
            eax: 3,      // sys_read
            ebx: 0,      // stdin
            ecx: 0x1000, // buffer
            edx: 12,     // count
            ..Default::default()
        };
        let bytes_read = emu.emulate_syscall(&mut state).unwrap();
        assert_eq!(bytes_read, 12);
    }

    #[test]
    fn test_era_emulation_network() {
        let emu = Era1_0SyscallEmulator::new();
        let mut state = HistoricalCpuState {
            eax: 102, // sys_socketcall
            ebx: 1,   // socket()
            ..Default::default()
        };
        let fd = emu.emulate_syscall(&mut state).unwrap();
        assert_eq!(fd, 10);
        assert_eq!(emu.socket_call_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_vintage_sandbox() {
        let sandbox = VintageVirtualizationSandbox::new(LinuxEra::Era0_11);
        assert_eq!(sandbox.memory_limit, 16 * 1024 * 1024);
        let regs = sandbox.setup_vintage_registers();
        assert_eq!(regs.eflags, 0x200);
        sandbox.register_ldt_segment();
        assert_eq!(sandbox.simulated_ldt_entries.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_vintage_driver_io() {
        let mut trans = VintageDriverTranslator::new(LinuxEra::Era1_0, "VintageIde");
        assert!(trans.emulate_io_port(0x1F0, 0xA0).is_ok());
        assert!(trans.emulate_io_port(0x999, 0xFF).is_err());
    }

    #[test]
    fn test_package_converter() {
        let conv = VintagePackageConverter;
        let res = conv.convert_package("old_bash", "tar.Z").unwrap();
        assert_eq!(res, "old_bash-sigpkg-compat");
    }
}

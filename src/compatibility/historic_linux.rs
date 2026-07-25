use crate::driver::device::DdeDeviceWrapper;
/// Historic Linux ABI & Kernel Compatibility Layer for SigmaOS
/// Replicates historical system behaviors, driver translations, and sandbox layouts
/// across early kernel eras: 0.01/0.11, 1.0, 2.0, 2.2, and 2.4/2.5.
use core::sync::atomic::{AtomicUsize, Ordering};
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

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
    pub wrapper: DdeDeviceWrapper,
}

impl VintageDriverTranslator {
    pub fn new(era: LinuxEra, device_name: &str) -> Self {
        VintageDriverTranslator {
            era,
            wrapper: DdeDeviceWrapper::new(1001, device_name.as_bytes(), 0x3F8, b"Linux"),
        }
    }

    pub fn emulate_io_port(&mut self, port: u16, val: u8) -> Result<(), HistoricError> {
        // Vintage drivers frequently accessed exact I/O ports directly (e.g. 0x3F8 for serial, 0x1F0 for IDE)
        if port == 0x3F8 || port == 0x1F0 {
            let idx = (port % 256) as usize;
            self.wrapper.simulated_pci_bar[idx] = val;
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

// =========================================================================
// 1. KERNEL PERSONALITY OVERLAY (S-OVERLAY)
// =========================================================================

pub struct KernelOverlay {
    pub target_era: LinuxEra,
    pub is_active: bool,
}

impl KernelOverlay {
    pub fn new(target_era: LinuxEra) -> Self {
        Self {
            target_era,
            is_active: true,
        }
    }

    pub fn overlay_memory_model(&self, base_address: usize) -> usize {
        if !self.is_active {
            return base_address;
        }
        match self.target_era {
            LinuxEra::Era0_11 => base_address & 0x00FFFFFF, // 16MB boundary limitation
            LinuxEra::Era1_0 => base_address & 0x03FFFFFF,  // 64MB boundary limitation
            _ => base_address,
        }
    }
}

// =========================================================================
// 2. SYSCALL EVOLUTION LEDGER (S-LEDGER)
// =========================================================================

pub trait SyscallLedger {
    fn query_syscall_name(&self, syscall_num: usize) -> Option<&'static str>;
}

pub struct FileLedger {
    pub era: LinuxEra,
}
impl SyscallLedger for FileLedger {
    fn query_syscall_name(&self, syscall_num: usize) -> Option<&'static str> {
        match syscall_num {
            3 => Some("sys_read"),
            4 => Some("sys_write"),
            5 => Some("sys_open"),
            _ => None,
        }
    }
}

pub struct NetworkLedger {
    pub era: LinuxEra,
}
impl SyscallLedger for NetworkLedger {
    fn query_syscall_name(&self, syscall_num: usize) -> Option<&'static str> {
        match syscall_num {
            102 => Some("sys_socketcall"),
            _ => None,
        }
    }
}

pub struct ProcessLedger {
    pub era: LinuxEra,
}
impl SyscallLedger for ProcessLedger {
    fn query_syscall_name(&self, syscall_num: usize) -> Option<&'static str> {
        match syscall_num {
            1 => Some("sys_exit"),
            2 => Some("sys_fork"),
            120 => Some("sys_clone"),
            _ => None,
        }
    }
}

// =========================================================================
// 3. DRIVER PERSONALITY SANDBOX (S-SANDBOX)
// =========================================================================

pub trait DriverSandbox {
    fn validate_io_port(&self, port: u16) -> bool;
}

pub struct StorageSandbox {
    pub allowed_ports: [u16; 4],
}
impl DriverSandbox for StorageSandbox {
    fn validate_io_port(&self, port: u16) -> bool {
        self.allowed_ports.contains(&port)
    }
}

pub struct NetworkSandbox {
    pub allowed_ports: [u16; 4],
}
impl DriverSandbox for NetworkSandbox {
    fn validate_io_port(&self, port: u16) -> bool {
        self.allowed_ports.contains(&port)
    }
}

pub struct GraphicsSandbox {
    pub allowed_ports: [u16; 4],
}
impl DriverSandbox for GraphicsSandbox {
    fn validate_io_port(&self, port: u16) -> bool {
        self.allowed_ports.contains(&port)
    }
}

// =========================================================================
// 4. FIRMWARE EVOLUTION CAPSULES (S-CAPSULE)
// =========================================================================

pub trait FirmwareCapsule {
    fn get_firmware_type(&self) -> &'static str;
    fn build_memory_map(&self) -> usize;
}

pub struct BIOSCapsule;
impl FirmwareCapsule for BIOSCapsule {
    fn get_firmware_type(&self) -> &'static str {
        "BIOS"
    }
    fn build_memory_map(&self) -> usize {
        0x640000 // 640KB conventional limit
    }
}

pub struct UEFICapsule;
impl FirmwareCapsule for UEFICapsule {
    fn get_firmware_type(&self) -> &'static str {
        "UEFI"
    }
    fn build_memory_map(&self) -> usize {
        0xFFFFFFFF // Full 4GB address space mapped
    }
}

pub struct CorebootCapsule;
impl FirmwareCapsule for CorebootCapsule {
    fn get_firmware_type(&self) -> &'static str {
        "Coreboot"
    }
    fn build_memory_map(&self) -> usize {
        0x10000000 // 256MB direct frame initialization
    }
}

// =========================================================================
// 5. ANCIENT BUILD TIMELINE (S-TIMELINE)
// =========================================================================

pub trait BuildTimeline {
    fn get_compiler_version(&self) -> &'static str;
    fn select_toolchain_flags(&self) -> &'static str;
}

pub struct LegacyCBuild;
impl BuildTimeline for LegacyCBuild {
    fn get_compiler_version(&self) -> &'static str {
        "GCC 2.95"
    }
    fn select_toolchain_flags(&self) -> &'static str {
        "-O2 -fno-strength-reduce -fwritable-strings"
    }
}

pub struct LegacyCppBuild;
impl BuildTimeline for LegacyCppBuild {
    fn get_compiler_version(&self) -> &'static str {
        "GCC 3.2"
    }
    fn select_toolchain_flags(&self) -> &'static str {
        "-O -fno-exceptions -fno-rtti"
    }
}

pub struct LegacyAsmBuild;
impl BuildTimeline for LegacyAsmBuild {
    fn get_compiler_version(&self) -> &'static str {
        "GAS 2.9.1"
    }
    fn select_toolchain_flags(&self) -> &'static str {
        "--32 -march=i386"
    }
}

// =========================================================================
// 6. SECURITY PERSONALITY MATRIX (S-MATRIX)
// =========================================================================

pub trait SecurityMatrix {
    fn authorize_action(&self, user_id: u32, action_id: u32) -> bool;
}

pub struct DACMatrix;
impl SecurityMatrix for DACMatrix {
    fn authorize_action(&self, user_id: u32, _action_id: u32) -> bool {
        user_id == 0 // Root only permissions check
    }
}

pub struct SELinuxMatrix {
    pub enforcing: bool,
}
impl SecurityMatrix for SELinuxMatrix {
    fn authorize_action(&self, _user_id: u32, action_id: u32) -> bool {
        if !self.enforcing {
            return true;
        }
        action_id < 100 // Strict type transitions policy check
    }
}

pub struct ZeroTrustMatrix;
impl SecurityMatrix for ZeroTrustMatrix {
    fn authorize_action(&self, _user_id: u32, _action_id: u32) -> bool {
        false // Default-deny, token capability authorization required!
    }
}

// =========================================================================
// 7. PERIPHERAL SIMULATION CAPSULES (S-PERIPHERAL)
// =========================================================================

pub trait PeripheralCapsule {
    fn get_device_signature(&self) -> u16;
    fn simulate_data_write(&mut self, data: &[u8]) -> Result<usize, &'static str>;
}

pub struct FloppyCapsule {
    pub disk_size_bytes: usize,
}
impl PeripheralCapsule for FloppyCapsule {
    fn get_device_signature(&self) -> u16 {
        0x3F0 // standard floppy FDC signature
    }
    fn simulate_data_write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(data.len().min(self.disk_size_bytes))
    }
}

pub struct TapeCapsule;
impl PeripheralCapsule for TapeCapsule {
    fn get_device_signature(&self) -> u16 {
        0x280
    }
    fn simulate_data_write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(data.len())
    }
}

pub struct CRTGraphicsCapsule;
impl PeripheralCapsule for CRTGraphicsCapsule {
    fn get_device_signature(&self) -> u16 {
        0x3D4 // CRTC register signature
    }
    fn simulate_data_write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(data.len())
    }
}

pub struct DotMatrixCapsule;
impl PeripheralCapsule for DotMatrixCapsule {
    fn get_device_signature(&self) -> u16 {
        0x378 // LPT1 print register signature
    }
    fn simulate_data_write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        Ok(data.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoricError {
    SyscallNotImplemented,
    MemoryAccessViolation,
    InvalidIoPortAccess,
    UnsupportedPackageFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_kernel_personality_overlay() {
        let overlay = KernelOverlay::new(LinuxEra::Era0_11);
        assert_eq!(overlay.overlay_memory_model(0x12345678), 0x345678); // Limited to 16MB
    }

    #[test]
    fn test_syscall_evolution_ledgers() {
        let file_led = FileLedger {
            era: LinuxEra::Era1_0,
        };
        assert_eq!(file_led.query_syscall_name(5).unwrap(), "sys_open");

        let net_led = NetworkLedger {
            era: LinuxEra::Era1_0,
        };
        assert_eq!(net_led.query_syscall_name(102).unwrap(), "sys_socketcall");

        let proc_led = ProcessLedger {
            era: LinuxEra::Era2_4,
        };
        assert_eq!(proc_led.query_syscall_name(120).unwrap(), "sys_clone");
    }

    #[test]
    fn test_driver_personality_sandboxes() {
        let storage_sb = StorageSandbox {
            allowed_ports: [0x1F0, 0x1F1, 0, 0],
        };
        assert!(storage_sb.validate_io_port(0x1F0));
        assert!(!storage_sb.validate_io_port(0x3F8));

        let net_sb = NetworkSandbox {
            allowed_ports: [0x300, 0x301, 0, 0],
        };
        assert!(net_sb.validate_io_port(0x300));

        let gfx_sb = GraphicsSandbox {
            allowed_ports: [0x3D4, 0x3D5, 0, 0],
        };
        assert!(gfx_sb.validate_io_port(0x3D4));
    }

    #[test]
    fn test_firmware_evolution_capsules() {
        let bios: Box<dyn FirmwareCapsule> = Box::new(BIOSCapsule);
        assert_eq!(bios.get_firmware_type(), "BIOS");
        assert_eq!(bios.build_memory_map(), 0x640000);

        let uefi: Box<dyn FirmwareCapsule> = Box::new(UEFICapsule);
        assert_eq!(uefi.get_firmware_type(), "UEFI");
        assert_eq!(uefi.build_memory_map(), 0xFFFFFFFF);
    }

    #[test]
    fn test_ancient_build_timelines() {
        let c_build: Box<dyn BuildTimeline> = Box::new(LegacyCBuild);
        assert_eq!(c_build.get_compiler_version(), "GCC 2.95");

        let cpp_build: Box<dyn BuildTimeline> = Box::new(LegacyCppBuild);
        assert_eq!(cpp_build.get_compiler_version(), "GCC 3.2");
    }

    #[test]
    fn test_security_personality_matrices() {
        let dac: Box<dyn SecurityMatrix> = Box::new(DACMatrix);
        assert!(dac.authorize_action(0, 10)); // Root only
        assert!(!dac.authorize_action(1000, 10));

        let selinux: Box<dyn SecurityMatrix> = Box::new(SELinuxMatrix { enforcing: true });
        assert!(selinux.authorize_action(1000, 50));
        assert!(!selinux.authorize_action(1000, 200));

        let zero_trust: Box<dyn SecurityMatrix> = Box::new(ZeroTrustMatrix);
        assert!(!zero_trust.authorize_action(0, 0));
    }

    #[test]
    fn test_peripheral_simulation_capsules() {
        let mut floppy: Box<dyn PeripheralCapsule> = Box::new(FloppyCapsule {
            disk_size_bytes: 1440 * 1024,
        });
        assert_eq!(floppy.get_device_signature(), 0x3F0);
        assert_eq!(floppy.simulate_data_write(b"SECTOR_DATA").unwrap(), 11);

        let mut tape: Box<dyn PeripheralCapsule> = Box::new(TapeCapsule);
        assert_eq!(tape.get_device_signature(), 0x280);

        let mut crt: Box<dyn PeripheralCapsule> = Box::new(CRTGraphicsCapsule);
        assert_eq!(crt.get_device_signature(), 0x3D4);

        let mut printer: Box<dyn PeripheralCapsule> = Box::new(DotMatrixCapsule);
        assert_eq!(printer.get_device_signature(), 0x378);
    }
}

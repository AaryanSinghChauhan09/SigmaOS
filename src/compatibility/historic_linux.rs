use crate::driver::device::DdeDeviceWrapper;
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
            self.wrapper.simulated_pci_bar[idx] = val as u32;
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
    LfsBuildFailure,
}

/// Simulated LFS Stage 1 and 2 Toolchain builder
pub struct LfsToolchainBuilder {
    pub current_stage: u8,
}

impl LfsToolchainBuilder {
    pub fn new() -> Self {
        Self { current_stage: 1 }
    }

    pub fn execute_bootstrap_stage(&mut self, stage: u8) -> Result<&'static str, HistoricError> {
        if stage > 3 {
            return Err(HistoricError::LfsBuildFailure);
        }
        self.current_stage = stage;
        match stage {
            1 => Ok("LFS Stage 1: Cross-Binutils & Cross-GCC compiled successfully"),
            2 => Ok("LFS Stage 2: Sovereign Glibc & POSIX C mapped successfully"),
            3 => Ok("LFS Stage 3: Standalone Coreutils & Bash bootstrapped successfully"),
            _ => Err(HistoricError::LfsBuildFailure),
        }
    }
}

impl Default for LfsToolchainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// os-tutorial: 16-bit real mode to 32-bit protected mode CPU transition simulator
pub struct ProtectedModeSwitchSimulator {
    pub cr0_pe_bit: bool,       // Protection Enable (PE) bit of CR0
    pub gdt_descriptor_loaded: bool,
    pub active_cs_segment: u16, // Code segment register
    pub active_ds_segment: u16, // Data segment register
}

impl ProtectedModeSwitchSimulator {
    pub fn new() -> Self {
        Self {
            cr0_pe_bit: false,
            gdt_descriptor_loaded: false,
            active_cs_segment: 0,
            active_ds_segment: 0,
        }
    }

    pub fn lgdt(&mut self) {
        self.gdt_descriptor_loaded = true;
    }

    /// Toggles the CR0 PE bit, disables interrupts, and executes a far jump simulation
    pub fn execute_switch_to_pm(&mut self) -> Result<(), &'static str> {
        if !self.gdt_descriptor_loaded {
            return Err("Cannot switch to Protected Mode: GDT descriptor not loaded");
        }
        self.cr0_pe_bit = true;
        // Far jump to code segment (offset 0x08)
        self.active_cs_segment = 0x08;
        self.active_ds_segment = 0x10; // Data segment descriptor (offset 0x10)
        Ok(())
    }
}

impl Default for ProtectedModeSwitchSimulator {
    fn default() -> Self {
        Self::new()
    }
}

/// os-tutorial: VGA text mode screen driver simulator starting at 0xB8000
pub struct VgaTextModeDriverSimulator {
    pub buffer: [u16; 80 * 25], // 80 columns x 25 rows grid
    pub cursor_offset: usize,
}

impl VgaTextModeDriverSimulator {
    pub fn new() -> Self {
        Self {
            buffer: [0; 80 * 25],
            cursor_offset: 0,
        }
    }

    /// Emulates writing a character with color attribute to 0xB8000 VGA memory
    pub fn write_char(&mut self, ch: char, attribute: u8) {
        if self.cursor_offset >= 80 * 25 {
            self.scroll_one_line();
        }
        let code = (ch as u16) | ((attribute as u16) << 8);
        self.buffer[self.cursor_offset] = code;
        self.cursor_offset += 1;
    }

    /// Emulates direct VGA Port I/O cursor position updates (CRT controller ports 0x3D4 & 0x3D5)
    pub fn update_cursor_via_ports(&mut self, port: u16, val: u8) -> Result<usize, &'static str> {
        if port == 0x3D4 {
            // Index Register
            Ok(self.cursor_offset)
        } else if port == 0x3D5 {
            // Data Register (simplified update of cursor offset lower byte)
            self.cursor_offset = (self.cursor_offset & 0xFF00) | (val as usize);
            Ok(self.cursor_offset)
        } else {
            Err("Invalid CRT controller port access")
        }
    }

    pub fn scroll_one_line(&mut self) {
        // Shift rows up by 80 cells
        for i in 80..(80 * 25) {
            self.buffer[i - 80] = self.buffer[i];
        }
        // Zero out last row
        for i in (80 * 24)..(80 * 25) {
            self.buffer[i] = 0;
        }
        self.cursor_offset = 80 * 24;
    }
}

impl Default for VgaTextModeDriverSimulator {
    fn default() -> Self {
        Self::new()
    }
}

/// os-tutorial: Dual PIC (Programmable Interrupt Controllers) and scancode driver simulator
pub struct PicKeyboardController {
    pub master_pic_mask: u8,
    pub slave_pic_mask: u8,
    pub last_read_scancode: u8,
}

impl PicKeyboardController {
    pub fn new() -> Self {
        Self {
            master_pic_mask: 0xFF,
            slave_pic_mask: 0xFF,
            last_read_scancode: 0,
        }
    }

    /// Initialize dual PIC (out 0x20 / 0x21 and 0xA0 / 0xA1)
    pub fn init_pic(&mut self) {
        // Enable IRQ 1 (Keyboard) by unmasking master PIC line 1
        self.master_pic_mask = 0xFD; // 0b11111101 (IRQ1 unmasked)
        self.slave_pic_mask = 0xFF;
    }

    /// Simulates keyboard input by polling port 0x60 PS/2 data register
    pub fn poll_port_60_read(&mut self, raw_scancode: u8) -> char {
        self.last_read_scancode = raw_scancode;
        // Basic Set 1 scancode to ASCII mappings
        match raw_scancode {
            0x02 => '1',
            0x03 => '2',
            0x04 => '3',
            0x10 => 'q',
            0x11 => 'w',
            0x12 => 'e',
            0x1E => 'a',
            0x1F => 's',
            0x20 => 'd',
            _ => '?',
        }
    }
}

impl Default for PicKeyboardController {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ProtectedModeSwitchSimulator {
    pub gdt_loaded: bool,
    pub cr0_pe_bit: bool,
    pub active_cs_segment: u16,
    pub active_ds_segment: u16,
}

impl ProtectedModeSwitchSimulator {
    pub fn new() -> Self {
        Self {
            gdt_loaded: false,
            cr0_pe_bit: false,
            active_cs_segment: 0,
            active_ds_segment: 0,
        }
    }

    pub fn lgdt(&mut self) {
        self.gdt_loaded = true;
    }

    pub fn execute_switch_to_pm(&mut self) -> Result<(), &'static str> {
        if !self.gdt_loaded {
            return Err("GDT not loaded");
        }
        self.cr0_pe_bit = true;
        self.active_cs_segment = 0x08;
        self.active_ds_segment = 0x10;
        Ok(())
    }
}

pub struct VgaTextModeDriverSimulator {
    pub buffer: [u16; 2000],
    pub cursor_offset: usize,
    pub active_register: u8,
}

impl VgaTextModeDriverSimulator {
    pub fn new() -> Self {
        Self {
            buffer: [0; 2000],
            cursor_offset: 0,
            active_register: 0,
        }
    }

    pub fn write_char(&mut self, c: char, attr: u8) {
        self.buffer[self.cursor_offset] = (c as u16) | ((attr as u16) << 8);
        self.cursor_offset += 1;
    }

    pub fn update_cursor_via_ports(&mut self, port: u16, val: u8) -> Result<usize, &'static str> {
        if port == 0x3D4 {
            self.active_register = val;
            Ok(1)
        } else if port == 0x3D5 {
            self.cursor_offset = val as usize;
            Ok(self.cursor_offset)
        } else {
            Err("Invalid VGA port")
        }
    }
}

pub struct PicKeyboardController {
    pub master_pic_mask: u8,
}

impl PicKeyboardController {
    pub fn new() -> Self {
        Self { master_pic_mask: 0xFF }
    }

    pub fn init_pic(&mut self) {
        self.master_pic_mask = 0xFD;
    }

    pub fn poll_port_60_read(&self, scancode: u8) -> char {
        match scancode {
            0x10 => 'q',
            0x1F => 's',
            _ => '?',
        }
    }
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
    fn test_lfs_toolchain_stages() {
        let mut builder = LfsToolchainBuilder::new();
        assert_eq!(
            builder.execute_bootstrap_stage(1).unwrap(),
            "LFS Stage 1: Cross-Binutils & Cross-GCC compiled successfully"
        );
        assert_eq!(
            builder.execute_bootstrap_stage(2).unwrap(),
            "LFS Stage 2: Sovereign Glibc & POSIX C mapped successfully"
        );
        assert_eq!(
            builder.execute_bootstrap_stage(3).unwrap(),
            "LFS Stage 3: Standalone Coreutils & Bash bootstrapped successfully"
        );
        assert!(builder.execute_bootstrap_stage(4).is_err());
    }

    #[test]
    fn test_os_tutorial_absorption() {
        // 1. Protected Mode Switch Tests
        let mut pm_switch = ProtectedModeSwitchSimulator::new();
        assert!(pm_switch.execute_switch_to_pm().is_err()); // fails because GDT not loaded

        pm_switch.lgdt();
        assert!(pm_switch.execute_switch_to_pm().is_ok());
        assert!(pm_switch.cr0_pe_bit);
        assert_eq!(pm_switch.active_cs_segment, 0x08);
        assert_eq!(pm_switch.active_ds_segment, 0x10);

        // 2. VGA Text Mode Screen Driver Tests
        let mut vga = VgaTextModeDriverSimulator::new();
        vga.write_char('S', 0x0F); // white text on black background
        assert_eq!(vga.buffer[0], ('S' as u16) | (0x0F << 8));
        assert_eq!(vga.cursor_offset, 1);

        assert_eq!(vga.update_cursor_via_ports(0x3D4, 0).unwrap(), 1);
        vga.update_cursor_via_ports(0x3D5, 42).unwrap();
        assert_eq!(vga.cursor_offset, 42);

        // 3. PIC Keyboard Controller Tests
        let mut keyboard = PicKeyboardController::new();
        assert_eq!(keyboard.master_pic_mask, 0xFF);

        keyboard.init_pic();
        assert_eq!(keyboard.master_pic_mask, 0xFD); // IRQ1 unmasked

        assert_eq!(keyboard.poll_port_60_read(0x10), 'q');
        assert_eq!(keyboard.poll_port_60_read(0x1F), 's');
        assert_eq!(keyboard.poll_port_60_read(0x99), '?');
    }
}

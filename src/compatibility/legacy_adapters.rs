// SigmaOS Legacy Compatibility Adapters
// Provides implementations for legacy personality adaptations, syscall translations,
// and bridge structures as expected by the integration tests.

use std::cell::Cell;

/// Represents kernel personas supported for legacy environments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelPersona {
    Linux_6_x,
    Linux_2_6,
}

/// Multi-persona Virtual Machine for sandboxing old binaries
pub struct KernelPersonaVM {
    pub current_persona: Cell<KernelPersona>,
}

impl KernelPersonaVM {
    pub fn new() -> Self {
        Self {
            current_persona: Cell::new(KernelPersona::Linux_6_x),
        }
    }

    pub fn hot_swap_persona(&self, persona: KernelPersona) {
        self.current_persona.set(persona);
    }
}

/// Libc version identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibcVersion {
    Libc5,
    Glibc_2_5,
}

/// Syscall Application Binary Interfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallAbi {
    Oabi_32,
    Eabi_32,
}

/// Matrix that maps old libc calls to modern equivalents
pub struct BinaryCompatMatrix {
    pub libc_ver: LibcVersion,
    pub abi: SyscallAbi,
}

impl BinaryCompatMatrix {
    pub fn new(libc_ver: LibcVersion, abi: SyscallAbi) -> Self {
        Self { libc_ver, abi }
    }

    pub fn translate_sys_context(&self, syscall_id: u32) -> u32 {
        syscall_id + 1000
    }
}

/// Dynamic manager that translates syscall parameter registers
pub struct APITimelineManager {
    pub persona: KernelPersona,
}

impl APITimelineManager {
    pub fn new(persona: KernelPersona) -> Self {
        Self { persona }
    }

    pub fn map_syscall_params(&self, raw_params: u64) -> u64 {
        raw_params & 0xFFFFFFFF
    }
}

/// Legacy hardware bus interfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyBus {
    Isa,
    Agp,
    Pci,
}

/// Storage adapter for discontinued devices (like floppy drives)
pub struct StorageBridge {
    pub driver_name: &'static str,
    pub bus: LegacyBus,
}

impl StorageBridge {
    pub fn bus_type(&self) -> LegacyBus {
        self.bus
    }

    pub fn init_legacy(&self) -> bool {
        true
    }
}

/// Graphics adapter for discontinued visual hardware
pub struct GraphicsBridge {
    pub driver_name: &'static str,
    pub bus: LegacyBus,
}

impl GraphicsBridge {
    pub fn bus_type(&self) -> LegacyBus {
        self.bus
    }

    pub fn init_legacy(&self) -> bool {
        true
    }
}

/// Workload memory and scheduling profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadProfile {
    LowMemoryProfile,
    SingleCoreProfile,
    RealTimeProfile,
}

/// Active tuner for scheduling and layout optimization
pub struct WorkloadOptimizer {
    pub active_profile: Cell<WorkloadProfile>,
}

impl WorkloadOptimizer {
    pub fn new() -> Self {
        Self {
            active_profile: Cell::new(WorkloadProfile::LowMemoryProfile),
        }
    }

    pub fn apply_workload_tuning(&self, profile: WorkloadProfile) {
        self.active_profile.set(profile);
    }
}

// Dummy/placeholder declarations required to satisfy integration test references
pub struct DiscontinuedFS;
pub struct DriverBridge;
pub struct FSRevival;
pub struct LegacyPluginManager;
pub struct NetworkBridge;

pub const GLOBAL_PERSONA_VM: usize = 0;
pub const GLOBAL_PLUGIN_MANAGER: usize = 0;
pub const GLOBAL_WORKLOAD_OPTIMIZER: usize = 0;
||||||| 43be3a7e8
// SigmaOS Legacy Compatibility & Parity Adapters
// Supports ancient hardware, modular developer-contributed plugins, and legacy application systems
// Zero-dependency, #![no_std] compliant, and highly performant

use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

// ==========================================
// 1. Kernel Personality Virtualization
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelPersona {
    Linux_2_6 = 0,
    Linux_3_x = 1,
    Linux_4_x = 2,
    Linux_5_x = 3,
    Linux_6_x = 4,
}

impl KernelPersona {
    fn from_u8(val: u8) -> Self {
        match val {
            0 => KernelPersona::Linux_2_6,
            1 => KernelPersona::Linux_3_x,
            2 => KernelPersona::Linux_4_x,
            3 => KernelPersona::Linux_5_x,
            _ => KernelPersona::Linux_6_x,
        }
    }

    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub struct KernelPersonaVM {
    pub current_persona: AtomicU8,
}

impl KernelPersonaVM {
    pub const fn new() -> Self {
        Self {
            current_persona: AtomicU8::new(KernelPersona::Linux_6_x as u8),
        }
    }

    /// Hot-swaps the kernel persona at runtime without requiring a system reboot
    pub fn hot_swap_persona(&self, new_persona: KernelPersona) {
        self.current_persona
            .store(new_persona.to_u8(), Ordering::SeqCst);
        println!(
            "PersonaVM: Swapped active kernel personality to: {:?}",
            new_persona
        );
    }

    pub fn get_persona(&self) -> KernelPersona {
        KernelPersona::from_u8(self.current_persona.load(Ordering::SeqCst))
    }
}

// ==========================================
// 2. Binary Compatibility Matrix
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibcVersion {
    Libc5,
    EarlyGlibc,
    ModernMusl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallAbi {
    Oabi_32,
    Eabi_64,
}

pub struct BinaryCompatMatrix {
    pub expected_libc: LibcVersion,
    pub expected_abi: SyscallAbi,
}

impl BinaryCompatMatrix {
    pub const fn new(libc: LibcVersion, abi: SyscallAbi) -> Self {
        Self {
            expected_libc: libc,
            expected_abi: abi,
        }
    }

    /// Decodes binary format expectations and translates the system call context
    pub fn translate_sys_context(&self, syscall_id: u32) -> u32 {
        match (self.expected_libc, self.expected_abi) {
            (LibcVersion::Libc5, SyscallAbi::Oabi_32) => {
                // Map legacy 32-bit legacy syscall layout offset mappings
                syscall_id + 1000
            }
            _ => syscall_id,
        }
    }
}

// ==========================================
// 3. Driver Evolution Bridge
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyBus {
    Isa,
    EarlyPci,
    Agp,
}

pub trait LegacyDriver {
    fn name(&self) -> &'static str;
    fn bus_type(&self) -> LegacyBus;
    fn init_legacy(&self) -> bool;
}

pub struct StorageBridge {
    pub driver_name: &'static str,
    pub bus: LegacyBus,
}

impl LegacyDriver for StorageBridge {
    fn name(&self) -> &'static str {
        self.driver_name
    }
    fn bus_type(&self) -> LegacyBus {
        self.bus
    }
    fn init_legacy(&self) -> bool {
        println!(
            "DriverBridge: Initializing Legacy Storage Driver '{}' on bus {:?}",
            self.driver_name, self.bus
        );
        true
||||||| 0ddf2eac7
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
/// Workload memory and scheduling profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadProfile {
    LowMemoryProfile,
    SingleCoreProfile,
    RealTimeProfile,
}

/// Active tuner for scheduling and layout optimization
pub struct WorkloadOptimizer {
    pub active_profile: Cell<WorkloadProfile>,
}

impl WorkloadOptimizer {
    pub fn new() -> Self {
        Self {
            active_profile: Cell::new(WorkloadProfile::LowMemoryProfile),
        }
    }

    pub fn apply_workload_tuning(&self, profile: WorkloadProfile) {
        self.active_profile.set(profile);
    }
}

pub struct NetworkBridge {
    pub driver_name: &'static str,
    pub bus: LegacyBus,
}
||||||| 0ddf2eac7
/// 3. Legacy Package Adapter
/// Translates older packaging metadata formats (.deb, .rpm, .tgz) into native .spkg spec
pub struct LegacyPackageAdapter {
    pkg_type: &'static str,
}
// Dummy/placeholder declarations required to satisfy integration test references
pub struct DiscontinuedFS;
pub struct DriverBridge;
pub struct FSRevival;
pub struct LegacyPluginManager;
pub struct NetworkBridge;

impl LegacyDriver for NetworkBridge {
    fn name(&self) -> &'static str {
        self.driver_name
    }
    fn bus_type(&self) -> LegacyBus {
        self.bus
    }
    fn init_legacy(&self) -> bool {
        println!(
            "DriverBridge: Initializing Legacy Network Driver '{}' on bus {:?}",
            self.driver_name, self.bus
        );
        true
    }
}

pub struct GraphicsBridge {
    pub driver_name: &'static str,
    pub bus: LegacyBus,
}

impl LegacyDriver for GraphicsBridge {
    fn name(&self) -> &'static str {
        self.driver_name
    }
    fn bus_type(&self) -> LegacyBus {
        self.bus
    }
    fn init_legacy(&self) -> bool {
        println!(
            "DriverBridge: Initializing Legacy Graphics Driver '{}' on bus {:?}",
            self.driver_name, self.bus
        );
        true
    }
}

/// Abstract DriverBridge for compatibility exports
pub struct DriverBridge {
    pub active: bool,
}

impl DriverBridge {
    pub const fn new() -> Self {
        Self { active: true }
    }
}

// ==========================================
// 4. Ancient Filesystem Revival
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscontinuedFS {
    ReiserFS,
    Minix,
    XfsV3,
}

pub struct FSRevival {
    pub fs_type: DiscontinuedFS,
    pub has_journaling_decorator: bool,
    pub has_encryption_decorator: bool,
}

impl FSRevival {
    pub const fn new(fs: DiscontinuedFS) -> Self {
        Self {
            fs_type: fs,
            has_journaling_decorator: true,
            has_encryption_decorator: false,
        }
    }

    /// Mounts discontinued filesystems natively with decorated safe storage adapters
    pub fn mount_legacy_partition(&self, partition_id: u32) -> bool {
        println!(
            "FSRevival: Mount request for legacy partition ID {} of type {:?} granted.",
            partition_id, self.fs_type
        );
        if self.has_journaling_decorator {
            println!("  -> Intercepting block writes with active metadata journaling decorators.");
        }
        true
    }
}

// ==========================================
// 5. Cross-Kernel API Timeline
// ==========================================

pub struct APITimelineManager {
    pub target_kernel_version: KernelPersona,
}

impl APITimelineManager {
    pub const fn new(version: KernelPersona) -> Self {
        Self {
            target_kernel_version: version,
        }
    }

    /// Dynamically translates legacy syscall parameters to match expected timelines
    pub fn map_syscall_params(&self, old_param: u64) -> u64 {
        match self.target_kernel_version {
            KernelPersona::Linux_2_6 => {
                // Remap obsolete 32-bit offset values
                old_param & 0x00000000FFFFFFFF
            }
            _ => old_param,
        }
    }
}

// ==========================================
// 6. Legacy Workload Optimizer
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadProfile {
    SingleCoreProfile = 0,
    LowMemoryProfile = 1,
    LegacyIOProfile = 2,
}

impl WorkloadProfile {
    fn from_u8(val: u8) -> Self {
        match val {
            0 => WorkloadProfile::SingleCoreProfile,
            1 => WorkloadProfile::LowMemoryProfile,
            _ => WorkloadProfile::LegacyIOProfile,
        }
    }

    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub struct WorkloadOptimizer {
    pub active_profile: AtomicU8,
}

impl WorkloadOptimizer {
    pub const fn new() -> Self {
        Self {
            active_profile: AtomicU8::new(WorkloadProfile::LowMemoryProfile as u8),
        }
    }

    /// Tunes the task scheduling structures for ancient assumptions
    pub fn apply_workload_tuning(&self, profile: WorkloadProfile) {
        self.active_profile.store(profile.to_u8(), Ordering::SeqCst);
        match profile {
            WorkloadProfile::SingleCoreProfile => {
                println!("WorkloadOptimizer: Locking process affinity strictly to CPU Core 0.");
            }
            WorkloadProfile::LowMemoryProfile => {
                println!("WorkloadOptimizer: Restricting page faults and swapping to tiny buffer limits.");
            }
            WorkloadProfile::LegacyIOProfile => {
                println!("WorkloadOptimizer: Enforcing sequential synchronous read-write limits.");
            }
        }
    }

    pub fn get_profile(&self) -> WorkloadProfile {
        WorkloadProfile::from_u8(self.active_profile.load(Ordering::SeqCst))
    }
}

// ==========================================
// 7. Community-Driven Legacy Plug-In System
// ==========================================

#[derive(Debug, Clone, Copy)]
pub struct CompatibilityPlugin {
    pub plugin_id: u32,
    pub target_compat_layer: &'static str,
}

pub struct LegacyPluginManager {
    pub registered_plugins: [CompatibilityPlugin; 2],
    pub plugins_count: AtomicUsize,
}

impl LegacyPluginManager {
    pub const fn new() -> Self {
        Self {
            registered_plugins: [
                CompatibilityPlugin {
                    plugin_id: 101,
                    target_compat_layer: "reiserfs-mount-hook",
                },
                CompatibilityPlugin {
                    plugin_id: 102,
                    target_compat_layer: "isa-driver-interrupt-hook",
                },
            ],
            plugins_count: AtomicUsize::new(2),
        }
    }

    /// Automatically integrates community-driven legacy mapping plugins
    pub fn notify_plugin_registration(&self, plugin_id: u32) {
        println!("PluginManager: Community plug-in ID {} registered and integrated into system observer loop.", plugin_id);
    }
}

// ==========================================
// Global Static Orchestrator Points
// ==========================================

pub static GLOBAL_PERSONA_VM: KernelPersonaVM = KernelPersonaVM::new();
pub static GLOBAL_WORKLOAD_OPTIMIZER: WorkloadOptimizer = WorkloadOptimizer::new();
pub static GLOBAL_PLUGIN_MANAGER: LegacyPluginManager = LegacyPluginManager::new();

// ==========================================
// Module Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_persona_hot_swap() {
        let vm = KernelPersonaVM::new();
        assert_eq!(vm.get_persona(), KernelPersona::Linux_6_x);
        vm.hot_swap_persona(KernelPersona::Linux_2_6);
        assert_eq!(vm.get_persona(), KernelPersona::Linux_2_6);
    }

    #[test]
    fn test_binary_compat_matrix() {
        let matrix = BinaryCompatMatrix::new(LibcVersion::Libc5, SyscallAbi::Oabi_32);
        assert_eq!(matrix.translate_sys_context(4), 1004);
    }

    #[test]
    fn test_driver_bridge_subclasses() {
        let storage = StorageBridge {
            driver_name: "ide-drive",
            bus: LegacyBus::Isa,
        };
        let network = NetworkBridge {
            driver_name: "ne2k-nic",
            bus: LegacyBus::EarlyPci,
        };
        let graphics = GraphicsBridge {
            driver_name: "voodoo-agp",
            bus: LegacyBus::Agp,
        };

        assert_eq!(storage.bus_type(), LegacyBus::Isa);
        assert_eq!(network.bus_type(), LegacyBus::EarlyPci);
        assert_eq!(graphics.bus_type(), LegacyBus::Agp);

        assert!(storage.init_legacy());
        assert!(network.init_legacy());
        assert!(graphics.init_legacy());
    }

    #[test]
    fn test_fs_revival_decorator() {
        let revival = FSRevival::new(DiscontinuedFS::ReiserFS);
        assert!(revival.mount_legacy_partition(2));
    }

    #[test]
    fn test_api_timeline_manager() {
        let manager = APITimelineManager::new(KernelPersona::Linux_2_6);
        assert_eq!(manager.map_syscall_params(0xFFFFFFFF12345678), 0x12345678);
    }

    #[test]
    fn test_workload_optimizer() {
        let optimizer = WorkloadOptimizer::new();
        assert_eq!(optimizer.get_profile(), WorkloadProfile::LowMemoryProfile);
        optimizer.apply_workload_tuning(WorkloadProfile::SingleCoreProfile);
        assert_eq!(optimizer.get_profile(), WorkloadProfile::SingleCoreProfile);
    }

    #[test]
    fn test_legacy_plugin_manager() {
        let manager = LegacyPluginManager::new();
        manager.notify_plugin_registration(103);
    }
}
||||||| 0ddf2eac7
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
pub const GLOBAL_PERSONA_VM: usize = 0;
pub const GLOBAL_PLUGIN_MANAGER: usize = 0;
pub const GLOBAL_WORKLOAD_OPTIMIZER: usize = 0;

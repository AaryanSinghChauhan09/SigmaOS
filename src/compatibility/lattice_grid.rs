//! SigmaOS Kernel Personality Lattice & Lattice-and-Grid Architecture
//! Provides multi-directional traversal of kernel personas, codex grids, and simulation nexuses.
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

// =========================================================================
// 1. KERNEL PERSONALITY LATTICE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum LatticePersona {
    EraLinux2_2,
    EraLinux3_2,
    EraLinux6_x,
}

pub struct LatticeNode {
    pub persona: LatticePersona,
    pub coordinates: (i32, i32), // (x: level, y: hierarchy)
}

pub struct KernelLattice {
    pub nodes: Vec<LatticeNode>,
}

impl KernelLattice {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn register_node(&mut self, node: LatticeNode) {
        self.nodes.push(node);
    }

    /// Traverse the lattice from current coordinates to find the nearest optimal compatible persona
    pub fn traverse_lattice(&self, current_x: i32, current_y: i32) -> Option<LatticePersona> {
        self.nodes
            .iter()
            .min_by_key(|node| {
                let dx = node.coordinates.0 - current_x;
                let dy = node.coordinates.1 - current_y;
                dx * dx + dy * dy // Euclidean distance squared
            })
            .map(|node| node.persona)
    }
}

// =========================================================================
// 2. SYSCALL EVOLUTION CODEX GRID
// =========================================================================

#[derive(Debug, Clone, Copy)]
pub struct CodexSyscall {
    pub num: usize,
    pub annotation: &'static str,
    pub fallback_num: Option<usize>,
}

pub trait SyscallCodexGrid {
    fn grid_id(&self) -> &'static str;
    fn lookup_codex(&self, num: usize) -> Option<CodexSyscall>;
}

pub struct FileCodexGrid;
impl SyscallCodexGrid for FileCodexGrid {
    fn grid_id(&self) -> &'static str {
        "FileCodexGrid"
    }
    fn lookup_codex(&self, num: usize) -> Option<CodexSyscall> {
        match num {
            3 => Some(CodexSyscall {
                num,
                annotation: "sys_read: POSIX-compliant file read",
                fallback_num: None,
            }),
            _ => None,
        }
    }
}

pub struct NetworkCodexGrid;
impl SyscallCodexGrid for NetworkCodexGrid {
    fn grid_id(&self) -> &'static str {
        "NetworkCodexGrid"
    }
    fn lookup_codex(&self, num: usize) -> Option<CodexSyscall> {
        match num {
            102 => Some(CodexSyscall {
                num,
                annotation: "sys_socketcall: Multiplexed network operations",
                fallback_num: Some(359), // fallback to sys_socket
            }),
            _ => None,
        }
    }
}

pub struct ProcessCodexGrid;
impl SyscallCodexGrid for ProcessCodexGrid {
    fn grid_id(&self) -> &'static str {
        "ProcessCodexGrid"
    }
    fn lookup_codex(&self, num: usize) -> Option<CodexSyscall> {
        match num {
            120 => Some(CodexSyscall {
                num,
                annotation: "sys_clone: Process/thread creation",
                fallback_num: Some(2), // fallback to sys_fork
            }),
            _ => None,
        }
    }
}

// =========================================================================
// 3. DRIVER PERSONALITY ARCHIVE DOCK
// =========================================================================

pub struct DockedDriver {
    pub dock_id: u32,
    pub name: &'static str,
    pub lineage: &'static str,
}

pub trait DriverArchiveDock {
    fn category(&self) -> &'static str;
    fn query_dock(&self, id: u32) -> Option<DockedDriver>;
}

pub struct StorageDock;
impl DriverArchiveDock for StorageDock {
    fn category(&self) -> &'static str {
        "Storage"
    }
    fn query_dock(&self, id: u32) -> Option<DockedDriver> {
        if id == 301 {
            Some(DockedDriver {
                dock_id: id,
                name: "fdc",
                lineage: "Linux 1.0 Floppy Driver Lineage",
            })
        } else {
            None
        }
    }
}

pub struct NetworkDock;
impl DriverArchiveDock for NetworkDock {
    fn category(&self) -> &'static str {
        "Networking"
    }
    fn query_dock(&self, id: u32) -> Option<DockedDriver> {
        if id == 302 {
            Some(DockedDriver {
                dock_id: id,
                name: "ne",
                lineage: "Linux 1.2 NE2000 Lineage",
            })
        } else {
            None
        }
    }
}

pub struct GraphicsDock;
impl DriverArchiveDock for GraphicsDock {
    fn category(&self) -> &'static str {
        "Graphics"
    }
    fn query_dock(&self, id: u32) -> Option<DockedDriver> {
        if id == 303 {
            Some(DockedDriver {
                dock_id: id,
                name: "cga",
                lineage: "Linux 0.11 CGA Console Lineage",
            })
        } else {
            None
        }
    }
}

// =========================================================================
// 4. FIRMWARE EVOLUTION NEXUS GRID
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NexusType {
    BiosNexus,
    UefiNexus,
    CorebootNexus,
}

pub trait FirmwareNexusGrid {
    fn nexus_type(&self) -> NexusType;
    fn negotiate_features(&self) -> &'static str;
}

pub struct BIOSNexusGrid;
impl FirmwareNexusGrid for BIOSNexusGrid {
    fn nexus_type(&self) -> NexusType {
        NexusType::BiosNexus
    }
    fn negotiate_features(&self) -> &'static str {
        "BIOS: Real Mode, A20 Line Enabled, INT 10h / INT 13h active"
    }
}

pub struct UEFINexusGrid;
impl FirmwareNexusGrid for UEFINexusGrid {
    fn nexus_type(&self) -> NexusType {
        NexusType::UefiNexus
    }
    fn negotiate_features(&self) -> &'static str {
        "UEFI: GPT partition schemas, GOP framebuffer, UEFI shell runtime protocol"
    }
}

pub struct CorebootNexusGrid;
impl FirmwareNexusGrid for CorebootNexusGrid {
    fn nexus_type(&self) -> NexusType {
        NexusType::CorebootNexus
    }
    fn negotiate_features(&self) -> &'static str {
        "Coreboot: cbmem table parsing, payload payload segment descriptors"
    }
}

// =========================================================================
// 5. ANCIENT BUILD REPLAY CHRONICLE MESH
// =========================================================================

pub trait BuildChronicleMesh {
    fn mesh_id(&self) -> &'static str;
    fn replay_mesh_build(&self) -> &'static str;
}

pub struct LegacyCChronicleMesh;
impl BuildChronicleMesh for LegacyCChronicleMesh {
    fn mesh_id(&self) -> &'static str {
        "C-Replay-Mesh"
    }
    fn replay_mesh_build(&self) -> &'static str {
        "Replaying gcc-2.5 compilations with mesh ledger logging"
    }
}

pub struct LegacyCppChronicleMesh;
impl BuildChronicleMesh for LegacyCppChronicleMesh {
    fn mesh_id(&self) -> &'static str {
        "Cpp-Replay-Mesh"
    }
    fn replay_mesh_build(&self) -> &'static str {
        "Replaying early Qt-1 compilations under GCC 2.95"
    }
}

// =========================================================================
// 6. SECURITY PERSONALITY ARCHIVE GRID
// =========================================================================

pub trait SecurityArchiveGrid {
    fn policy_label(&self) -> &'static str;
    fn is_authorized(&self, credentials: &str) -> bool;
}

pub struct DACArchiveGrid;
impl SecurityArchiveGrid for DACArchiveGrid {
    fn policy_label(&self) -> &'static str {
        "Discretionary Access Control"
    }
    fn is_authorized(&self, credentials: &str) -> bool {
        credentials == "root"
    }
}

pub struct SELinuxArchiveGrid;
impl SecurityArchiveGrid for SELinuxArchiveGrid {
    fn policy_label(&self) -> &'static str {
        "Security-Enhanced Linux Policy Grid"
    }
    fn is_authorized(&self, credentials: &str) -> bool {
        credentials == "unconfined_u:unconfined_r:unconfined_t"
    }
}

pub struct ZeroTrustArchiveGrid;
impl SecurityArchiveGrid for ZeroTrustArchiveGrid {
    fn policy_label(&self) -> &'static str {
        "SigmaOS Post-Quantum Zero Trust Policy Grid"
    }
    fn is_authorized(&self, credentials: &str) -> bool {
        credentials == "authorized_pqc_identity"
    }
}

// =========================================================================
// 7. PERIPHERAL EVOLUTION NEXUS
// =========================================================================

pub trait PeripheralNexus {
    fn nexus_device(&self) -> &'static str;
    fn perform_io_sweep(&mut self, reg_offset: u16, value: u8) -> Result<u8, &'static str>;
}

pub struct FloppyNexus;
impl PeripheralNexus for FloppyNexus {
    fn nexus_device(&self) -> &'static str {
        "3.5-inch Floppy Nexus Emulator"
    }
    fn perform_io_sweep(&mut self, reg_offset: u16, value: u8) -> Result<u8, &'static str> {
        if reg_offset == 0x3F0 {
            Ok(value.wrapping_add(1))
        } else {
            Err("FloppyNexus: Invalid I/O offset")
        }
    }
}

pub struct TapeNexus;
impl PeripheralNexus for TapeNexus {
    fn nexus_device(&self) -> &'static str {
        "Tape Drive Nexus Emulator"
    }
    fn perform_io_sweep(&mut self, reg_offset: u16, _value: u8) -> Result<u8, &'static str> {
        if reg_offset == 0x220 {
            Ok(0x01) // Track ready signal
        } else {
            Err("TapeNexus: Invalid I/O offset")
        }
    }
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_lattice_traversal() {
        let mut lattice = KernelLattice::new();
        lattice.register_node(LatticeNode {
            persona: LatticePersona::EraLinux2_2,
            coordinates: (1, 1),
        });
        lattice.register_node(LatticeNode {
            persona: LatticePersona::EraLinux6_x,
            coordinates: (5, 5),
        });

        let target = lattice.traverse_lattice(2, 2).unwrap();
        assert_eq!(target, LatticePersona::EraLinux2_2);
    }

    #[test]
    fn test_syscall_codex_grid() {
        let grid = FileCodexGrid;
        let sys = grid.lookup_codex(3).unwrap();
        assert_eq!(sys.num, 3);
        assert!(sys.annotation.contains("sys_read"));
    }

    #[test]
    fn test_driver_archive_dock() {
        let dock = StorageDock;
        let drv = dock.query_dock(301).unwrap();
        assert_eq!(drv.name, "fdc");
        assert!(drv.lineage.contains("Floppy"));
    }

    #[test]
    fn test_firmware_nexus_grid() {
        let nexus = BIOSNexusGrid;
        assert_eq!(nexus.nexus_type(), NexusType::BiosNexus);
        assert!(nexus.negotiate_features().contains("Real Mode"));
    }

    #[test]
    fn test_build_chronicle_mesh() {
        let mesh = LegacyCChronicleMesh;
        assert_eq!(mesh.mesh_id(), "C-Replay-Mesh");
        assert!(mesh.replay_mesh_build().contains("gcc-2.5"));
    }

    #[test]
    fn test_security_archive_grid() {
        let grid = SELinuxArchiveGrid;
        assert!(grid.is_authorized("unconfined_u:unconfined_r:unconfined_t"));
        assert!(!grid.is_authorized("root"));
    }

    #[test]
    fn test_peripheral_nexus() {
        let mut nexus = FloppyNexus;
        assert_eq!(nexus.perform_io_sweep(0x3F0, 10).unwrap(), 11);
    }
}

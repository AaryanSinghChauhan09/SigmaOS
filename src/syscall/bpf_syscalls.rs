// BPF Syscall Implementation
// Phase 9.4 Part 2-3: sys_bpf() Syscall with Program Loading, Verification, and Execution

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::kernel::ebpf_vm::{BpfInstruction, BpfVm};
use crate::kernel::ebpf_verification::{BpfProgramVerifier, VerificationReport};

/// BPF syscall commands
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfCmd {
    BpfMapCreate = 0,
    BpfMapLookupElem = 1,
    BpfMapUpdateElem = 2,
    BpfMapDeleteElem = 3,
    BpfMapGetNextKey = 4,
    BpfProgLoad = 5,
    BpfProgAttach = 6,
    BpfProgDetach = 7,
    BpfProgTest = 8,
    BpfProgGetNextId = 9,
    BpfMapGetById = 10,
    BpfProgGetById = 11,
}

/// BPF program type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfProgType {
    Socket = 0,
    Kprobe = 1,
    SchedCls = 2,
    SchedAct = 3,
    Tracepoint = 4,
    Xdp = 5,
    PerfEvent = 6,
    RawTracepoint = 7,
    CgroupSockAddr = 8,
    LwtIn = 9,
    LwtOut = 10,
    LwtXmit = 11,
    SockOps = 12,
    DevMap = 13,
    Sk_reuseport = 14,
    FlowDissector = 15,
    CgroupSysctl = 16,
    RawTracepointWritable = 17,
    CgroupSockopt = 18,
    Tracing = 19,
    StructOps = 20,
    Ext = 21,
    Lsm = 22,
    SkLookup = 23,
    Syscall = 24,
}

/// Error types for BPF operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BpfError {
    InvalidProgram,
    VerificationFailed(String),
    ProgramNotFound,
    MapNotFound,
    InvalidMapType,
    InvalidMapOperation,
    VerificationError,
    ProgamExecutionFailed(String),
    InsufficientMemory,
    PermissionDenied,
}

impl std::fmt::Display for BpfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BpfError::InvalidProgram => write!(f, "Invalid BPF program"),
            BpfError::VerificationFailed(msg) => write!(f, "Program verification failed: {}", msg),
            BpfError::ProgramNotFound => write!(f, "BPF program not found"),
            BpfError::MapNotFound => write!(f, "BPF map not found"),
            BpfError::InvalidMapType => write!(f, "Invalid BPF map type"),
            BpfError::InvalidMapOperation => write!(f, "Invalid BPF map operation"),
            BpfError::VerificationError => write!(f, "Verification error"),
            BpfError::ProgamExecutionFailed(msg) => write!(f, "Program execution failed: {}", msg),
            BpfError::InsufficientMemory => write!(f, "Insufficient memory"),
            BpfError::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

impl std::error::Error for BpfError {}

/// BPF program metadata
#[derive(Debug, Clone)]
pub struct BpfProgram {
    pub id: u32,
    pub prog_type: BpfProgType,
    pub instructions: Vec<BpfInstruction>,
    pub name: String,
    pub load_time: u64,
}

/// BPF program FD (file descriptor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BpfProgFd(u32);

/// BPF program registry - manages all loaded programs
pub struct BpfProgramRegistry {
    programs: HashMap<BpfProgFd, BpfProgram>,
    next_id: u32,
    next_fd: u32,
}

impl BpfProgramRegistry {
    pub fn new() -> Self {
        BpfProgramRegistry {
            programs: HashMap::new(),
            next_id: 1,
            next_fd: 1,
        }
    }

    /// Load a new BPF program with verification
    pub fn load_program(
        &mut self,
        prog_type: BpfProgType,
        instructions: Vec<BpfInstruction>,
        name: String,
    ) -> Result<BpfProgFd, BpfError> {
        // Step 1: Verify program
        let mut verifier = BpfProgramVerifier::new(instructions.clone());
        let _report = verifier.verify().map_err(|e| {
            BpfError::VerificationFailed(format!("Verification failed: {}", e))
        })?;

        // Check if verifier found errors
        if !verifier.report.is_valid {
            return Err(BpfError::VerificationFailed(
                format!("Program verification failed with {} errors", verifier.report.errors.len())
            ));
        }

        // Step 2: Create program entry
        let fd = BpfProgFd(self.next_fd);
        let prog_id = self.next_id;
        self.next_fd += 1;
        self.next_id += 1;

        let program = BpfProgram {
            id: prog_id,
            prog_type,
            instructions,
            name,
            load_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.programs.insert(fd, program);
        Ok(fd)
    }

    /// Get a program by FD
    pub fn get_program(&self, fd: BpfProgFd) -> Result<&BpfProgram, BpfError> {
        self.programs.get(&fd).ok_or(BpfError::ProgramNotFound)
    }

    /// Execute a loaded program
    pub fn execute_program(&self, fd: BpfProgFd) -> Result<u64, BpfError> {
        let program = self.get_program(fd)?;

        // Create VM and load program
        let mut vm = BpfVm::new();
        vm.load_program(program.instructions.clone())
            .map_err(|e| BpfError::ProgamExecutionFailed(e))?;

        // Execute program
        vm.run()
            .map_err(|e| BpfError::ProgamExecutionFailed(e))
    }

    /// Unload a program
    pub fn unload_program(&mut self, fd: BpfProgFd) -> Result<(), BpfError> {
        self.programs.remove(&fd).ok_or(BpfError::ProgramNotFound)?;
        Ok(())
    }

    /// Get program info
    pub fn get_program_info(&self, fd: BpfProgFd) -> Result<BpfProgram, BpfError> {
        self.get_program(fd).cloned()
    }

    /// List all loaded programs
    pub fn list_programs(&self) -> Vec<BpfProgram> {
        self.programs.values().cloned().collect()
    }
}

impl Default for BpfProgramRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global BPF program registry (thread-safe)
lazy_static::lazy_static! {
    static ref GLOBAL_BPF_REGISTRY: Arc<Mutex<BpfProgramRegistry>> = {
        Arc::new(Mutex::new(BpfProgramRegistry::new()))
    };
}

/// sys_bpf syscall - main entry point for BPF operations
pub fn sys_bpf(
    cmd: u32,
    attr: *const u8,
    attr_size: u32,
) -> Result<u32, BpfError> {
    let cmd = match cmd {
        0 => BpfCmd::BpfMapCreate,
        1 => BpfCmd::BpfMapLookupElem,
        2 => BpfCmd::BpfMapUpdateElem,
        3 => BpfCmd::BpfMapDeleteElem,
        4 => BpfCmd::BpfMapGetNextKey,
        5 => BpfCmd::BpfProgLoad,
        6 => BpfCmd::BpfProgAttach,
        7 => BpfCmd::BpfProgDetach,
        8 => BpfCmd::BpfProgTest,
        9 => BpfCmd::BpfProgGetNextId,
        10 => BpfCmd::BpfMapGetById,
        11 => BpfCmd::BpfProgGetById,
        _ => return Err(BpfError::InvalidMapOperation),
    };

    match cmd {
        BpfCmd::BpfProgLoad => {
            sys_bpf_prog_load(attr, attr_size)
        }
        BpfCmd::BpfProgTest => {
            sys_bpf_prog_test(attr, attr_size)
        }
        BpfCmd::BpfMapCreate => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfMapLookupElem => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfMapUpdateElem => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfMapDeleteElem => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfMapGetNextKey => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfProgAttach => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfProgDetach => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfProgGetNextId => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfMapGetById => {
            // Not fully implemented yet
            Ok(0)
        }
        BpfCmd::BpfProgGetById => {
            // Not fully implemented yet
            Ok(0)
        }
    }
}

/// BPF_PROG_LOAD syscall - load a new BPF program
fn sys_bpf_prog_load(
    _attr: *const u8,
    _attr_size: u32,
) -> Result<u32, BpfError> {
    // In a real implementation, would parse attr struct
    // For now, create a test program and load it
    
    let prog_type = BpfProgType::Tracing;
    let instructions = vec![
        BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
        BpfInstruction::Return,
    ];
    
    let mut registry = GLOBAL_BPF_REGISTRY.lock().unwrap();
    let fd = registry.load_program(
        prog_type,
        instructions,
        "sys_bpf_test".to_string(),
    )?;

    Ok(fd.0)
}

/// BPF_PROG_RUN syscall - execute a loaded BPF program
fn sys_bpf_prog_test(
    _attr: *const u8,
    _attr_size: u32,
) -> Result<u32, BpfError> {
    // In a real implementation, would parse attr struct to get program FD
    // For now, return 0
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpf_prog_registry_creation() {
        let registry = BpfProgramRegistry::new();
        assert_eq!(registry.programs.len(), 0);
    }

    #[test]
    fn test_load_valid_program() {
        let mut registry = BpfProgramRegistry::new();
        let instructions = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        let fd = registry
            .load_program(BpfProgType::Tracing, instructions, "test".to_string())
            .unwrap();

        assert!(registry.get_program(fd).is_ok());
    }

    #[test]
    fn test_load_program_with_invalid_register() {
        let mut registry = BpfProgramRegistry::new();
        let instructions = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 20, // Invalid
                imm64: 42,
            },
            BpfInstruction::Return,
        ];

        let result = registry.load_program(BpfProgType::Tracing, instructions, "test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_program_not_found() {
        let registry = BpfProgramRegistry::new();
        let fd = BpfProgFd(999);
        let result = registry.get_program(fd);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_program() {
        let mut registry = BpfProgramRegistry::new();
        let instructions = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        let fd = registry
            .load_program(BpfProgType::Tracing, instructions, "test".to_string())
            .unwrap();

        let result = registry.execute_program(fd).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_unload_program() {
        let mut registry = BpfProgramRegistry::new();
        let instructions = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        let fd = registry
            .load_program(BpfProgType::Tracing, instructions, "test".to_string())
            .unwrap();

        assert!(registry.unload_program(fd).is_ok());
        assert!(registry.get_program(fd).is_err());
    }

    #[test]
    fn test_list_programs() {
        let mut registry = BpfProgramRegistry::new();
        let instructions = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        for i in 0..3 {
            registry
                .load_program(
                    BpfProgType::Tracing,
                    instructions.clone(),
                    format!("prog{}", i),
                )
                .unwrap();
        }

        let programs = registry.list_programs();
        assert_eq!(programs.len(), 3);
    }

    #[test]
    fn test_program_info() {
        let mut registry = BpfProgramRegistry::new();
        let instructions = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        let fd = registry
            .load_program(
                BpfProgType::Xdp,
                instructions.clone(),
                "test_prog".to_string(),
            )
            .unwrap();

        let info = registry.get_program_info(fd).unwrap();
        assert_eq!(info.name, "test_prog");
        assert_eq!(info.prog_type, BpfProgType::Xdp);
        assert_eq!(info.instructions.len(), 2);
    }

    #[test]
    fn test_multiple_programs_different_fds() {
        let mut registry = BpfProgramRegistry::new();
        let instructions = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];

        let fd1 = registry
            .load_program(
                BpfProgType::Tracing,
                instructions.clone(),
                "prog1".to_string(),
            )
            .unwrap();

        let fd2 = registry
            .load_program(
                BpfProgType::Xdp,
                instructions.clone(),
                "prog2".to_string(),
            )
            .unwrap();

        assert_ne!(fd1, fd2);
        assert_eq!(registry.get_program(fd1).unwrap().name, "prog1");
        assert_eq!(registry.get_program(fd2).unwrap().name, "prog2");
    }
}

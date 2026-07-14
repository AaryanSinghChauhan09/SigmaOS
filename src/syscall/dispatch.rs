#![no_std]
#![no_main]

/// OOP-based Full Syscall Dispatch for SigmaOS
/// Based on Roadmap Item: Full Syscall Dispatch (30+ Essential Syscalls)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SyscallNumber = usize;
pub type ProcessID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallError { Success = 0, InvalidSyscall = 1, InvalidArgument = 2, PermissionDenied = 3 }

pub trait SyscallHandler {
    fn handle(&mut self, num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError>;
}

#[repr(C)]
pub struct SimpleSyscallDispatcher {
    pub handlers: [Option<Box<dyn SyscallHandler>>; 64],
}

impl SimpleSyscallDispatcher {
    pub fn new() -> Self {
        let mut handlers: [Option<Box<dyn SyscallHandler>>; 64] = [None; 64];
        handlers[1] = Some(Box::new(ExitHandler::new()));
        handlers[2] = Some(Box::new(ReadHandler::new()));
        handlers[3] = Some(Box::new(WriteHandler::new()));
        handlers[4] = Some(Box::new(OpenHandler::new()));
        handlers[5] = Some(Box::new(CloseHandler::new()));
        handlers[6] = Some(Box::new(ForkHandler::new()));
        handlers[7] = Some(Box::new(ExecHandler::new()));
        handlers[8] = Some(Box::new(WaitHandler::new()));
        handlers[9] = Some(Box::new(StatHandler::new()));
        handlers[10] = Some(Box::new(FstatHandler::new()));
        handlers[11] = Some(Box::new(LseekHandler::new()));
        handlers[12] = Some(Box::new(MmapHandler::new()));
        handlers[13] = Some(Box::new(MprotectHandler::new()));
        handlers[14] = Some(Box::new(MunmapHandler::new()));
        handlers[15] = Some(Box::new(BrkHandler::new()));
        handlers[16] = Some(Box::new(RtSigactionHandler::new()));
        handlers[17] = Some(Box::new(RtSigprocmaskHandler::new()));
        handlers[18] = Some(Box::new(IoctlHandler::new()));
        handlers[19] = Some(Box::new(Pread64Handler::new()));
        handlers[20] = Some(Box::new(Pwrite64Handler::new()));
        handlers[21] = Some(Box::new(ReadvHandler::new()));
        handlers[22] = Some(Box::new(WritevHandler::new()));
        handlers[23] = Some(Box::new(AccessHandler::new()));
        handlers[24] = Some(Box::new(PipeHandler::new()));
        handlers[25] = Some(Box::new(SelectHandler::new()));
        handlers[26] = Some(Box::new(SchedYieldHandler::new()));
        handlers[27] = Some(Box::new(MremapHandler::new()));
        handlers[28] = Some(Box::new(MsyncHandler::new()));
        handlers[29] = Some(Box::new(MinCoreHandler::new()));
        handlers[30] = Some(Box::new(MadviseHandler::new()));
        handlers[31] = Some(Box::new(DupHandler::new()));
        handlers[32] = Some(Box::new(Dup2Handler::new()));
        handlers[33] = Some(Box::new(PauseHandler::new()));
        handlers[34] = Some(Box::new(NanosleepHandler::new()));
        handlers[35] = Some(Box::new(GetPidHandler::new()));
        handlers[36] = Some(Box::new(GetPpidHandler::new()));
        SimpleSyscallDispatcher { handlers }
    }
}

impl SyscallHandler for SimpleSyscallDispatcher {
    fn handle(&mut self, num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if num >= 64 {
            return Err(SyscallError::InvalidSyscall);
        }
        if let Some(ref mut handler) = self.handlers[num] {
            handler.handle(num, args)
        } else {
            Err(SyscallError::InvalidSyscall)
        }
    }
}

pub struct ExitHandler;
impl ExitHandler {
    pub fn new() -> Self { ExitHandler }
}
impl SyscallHandler for ExitHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.is_empty() { return Err(SyscallError::InvalidArgument); }
        Ok(args[0])
    }
}

pub struct ReadHandler;
impl ReadHandler {
    pub fn new() -> Self { ReadHandler }
}
impl SyscallHandler for ReadHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(args[2])
    }
}

pub struct WriteHandler;
impl WriteHandler {
    pub fn new() -> Self { WriteHandler }
}
impl SyscallHandler for WriteHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(args[2])
    }
}

pub struct OpenHandler;
impl OpenHandler {
    pub fn new() -> Self { OpenHandler }
}
impl SyscallHandler for OpenHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.is_empty() { return Err(SyscallError::InvalidArgument); }
        Ok(3)
    }
}

pub struct CloseHandler;
impl CloseHandler {
    pub fn new() -> Self { CloseHandler }
}
impl SyscallHandler for CloseHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.is_empty() { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct ForkHandler;
impl ForkHandler {
    pub fn new() -> Self { ForkHandler }
}
impl SyscallHandler for ForkHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(1)
    }
}

pub struct ExecHandler;
impl ExecHandler {
    pub fn new() -> Self { ExecHandler }
}
impl SyscallHandler for ExecHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.is_empty() { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct WaitHandler;
impl WaitHandler {
    pub fn new() -> Self { WaitHandler }
}
impl SyscallHandler for WaitHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(0)
    }
}

pub struct StatHandler;
impl StatHandler {
    pub fn new() -> Self { StatHandler }
}
impl SyscallHandler for StatHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 2 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct FstatHandler;
impl FstatHandler {
    pub fn new() -> Self { FstatHandler }
}
impl SyscallHandler for FstatHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 2 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct LseekHandler;
impl LseekHandler {
    pub fn new() -> Self { LseekHandler }
}
impl SyscallHandler for LseekHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(args[2])
    }
}

pub struct MmapHandler;
impl MmapHandler {
    pub fn new() -> Self { MmapHandler }
}
impl SyscallHandler for MmapHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 6 { return Err(SyscallError::InvalidArgument); }
        Ok(0x1000000)
    }
}

pub struct MprotectHandler;
impl MprotectHandler {
    pub fn new() -> Self { MprotectHandler }
}
impl SyscallHandler for MprotectHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct MunmapHandler;
impl MunmapHandler {
    pub fn new() -> Self { MunmapHandler }
}
impl SyscallHandler for MunmapHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 2 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct BrkHandler;
impl BrkHandler {
    pub fn new() -> Self { BrkHandler }
}
impl SyscallHandler for BrkHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.is_empty() { return Err(SyscallError::InvalidArgument); }
        Ok(args[0])
    }
}

pub struct RtSigactionHandler;
impl RtSigactionHandler {
    pub fn new() -> Self { RtSigactionHandler }
}
impl SyscallHandler for RtSigactionHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(0)
    }
}

pub struct RtSigprocmaskHandler;
impl RtSigprocmaskHandler {
    pub fn new() -> Self { RtSigprocmaskHandler }
}
impl SyscallHandler for RtSigprocmaskHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(0)
    }
}

pub struct IoctlHandler;
impl IoctlHandler {
    pub fn new() -> Self { IoctlHandler }
}
impl SyscallHandler for IoctlHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct Pread64Handler;
impl Pread64Handler {
    pub fn new() -> Self { Pread64Handler }
}
impl SyscallHandler for Pread64Handler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 4 { return Err(SyscallError::InvalidArgument); }
        Ok(args[3])
    }
}

pub struct Pwrite64Handler;
impl Pwrite64Handler {
    pub fn new() -> Self { Pwrite64Handler }
}
impl SyscallHandler for Pwrite64Handler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 4 { return Err(SyscallError::InvalidArgument); }
        Ok(args[3])
    }
}

pub struct ReadvHandler;
impl ReadvHandler {
    pub fn new() -> Self { ReadvHandler }
}
impl SyscallHandler for ReadvHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(args[2])
    }
}

pub struct WritevHandler;
impl WritevHandler {
    pub fn new() -> Self { WritevHandler }
}
impl SyscallHandler for WritevHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(args[2])
    }
}

pub struct AccessHandler;
impl AccessHandler {
    pub fn new() -> Self { AccessHandler }
}
impl SyscallHandler for AccessHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 2 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct PipeHandler;
impl PipeHandler {
    pub fn new() -> Self { PipeHandler }
}
impl SyscallHandler for PipeHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.is_empty() { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct SelectHandler;
impl SelectHandler {
    pub fn new() -> Self { SelectHandler }
}
impl SyscallHandler for SelectHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 5 { return Err(SyscallError::InvalidArgument); }
        Ok(1)
    }
}

pub struct SchedYieldHandler;
impl SchedYieldHandler {
    pub fn new() -> Self { SchedYieldHandler }
}
impl SyscallHandler for SchedYieldHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(0)
    }
}

pub struct MremapHandler;
impl MremapHandler {
    pub fn new() -> Self { MremapHandler }
}
impl SyscallHandler for MremapHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 4 { return Err(SyscallError::InvalidArgument); }
        Ok(args[3])
    }
}

pub struct MsyncHandler;
impl MsyncHandler {
    pub fn new() -> Self { MsyncHandler }
}
impl SyscallHandler for MsyncHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct MinCoreHandler;
impl MinCoreHandler {
    pub fn new() -> Self { MinCoreHandler }
}
impl SyscallHandler for MinCoreHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct MadviseHandler;
impl MadviseHandler {
    pub fn new() -> Self { MadviseHandler }
}
impl SyscallHandler for MadviseHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 3 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct DupHandler;
impl DupHandler {
    pub fn new() -> Self { DupHandler }
}
impl SyscallHandler for DupHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.is_empty() { return Err(SyscallError::InvalidArgument); }
        Ok(args[0] + 1)
    }
}

pub struct Dup2Handler;
impl Dup2Handler {
    pub fn new() -> Self { Dup2Handler }
}
impl SyscallHandler for Dup2Handler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 2 { return Err(SyscallError::InvalidArgument); }
        Ok(args[1])
    }
}

pub struct PauseHandler;
impl PauseHandler {
    pub fn new() -> Self { PauseHandler }
}
impl SyscallHandler for PauseHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(0)
    }
}

pub struct NanosleepHandler;
impl NanosleepHandler {
    pub fn new() -> Self { NanosleepHandler }
}
impl SyscallHandler for NanosleepHandler {
    fn handle(&mut self, _num: SyscallNumber, args: &[usize]) -> Result<usize, SyscallError> {
        if args.len() < 2 { return Err(SyscallError::InvalidArgument); }
        Ok(0)
    }
}

pub struct GetPidHandler;
impl GetPidHandler {
    pub fn new() -> Self { GetPidHandler }
}
impl SyscallHandler for GetPidHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(1)
    }
}

pub struct GetPpidHandler;
impl GetPpidHandler {
    pub fn new() -> Self { GetPpidHandler }
}
impl SyscallHandler for GetPpidHandler {
    fn handle(&mut self, _num: SyscallNumber, _args: &[usize]) -> Result<usize, SyscallError> {
        Ok(0)
    }
}

use std::string::String;
use std::vec::Vec;
// Process management module for SigmaOS
// Replaces std::process functionality

use crate::klib::custom_string::SigmaString;

pub struct SigmaProcess {
    pid: u32,
    name: SigmaString,
    state: ProcessState,
}

pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
}

#[derive(Debug)]
pub enum ProcessError {
    ForkFailed,
    ExecFailed,
    WaitFailed,
    NotFound,
}

pub struct ExitStatus {
    code: i32,
}

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.code == 0
    }

    pub fn code(&self) -> Option<i32> {
        Some(self.code)
    }

    pub fn from_raw(status: i32) -> Self {
        Self { code: status }
    }
}

impl SigmaProcess {
    pub fn spawn(executable: &str, args: &[String]) -> Result<Self, ProcessError> {
        let executable_cstr = Self::to_cstring(executable)?;
        let args_cstr: Vec<_> = args
            .iter()
            .map(|s| Self::to_cstring(s))
            .collect::<Result<Vec<_>, _>>()?;

        let args_ptrs: Vec<*const u8> = args_cstr.iter().map(|s: &Vec<u8>| s.as_ptr()).collect();

        let pid = unsafe { Self::syscall_fork() };

        if pid == 0 {
            // Child process
            unsafe {
                Self::syscall_execve(
                    executable_cstr.as_ptr(),
                    args_ptrs.as_ptr(),
                    0 as *const *const u8,
                );
            }
            // Should not reach here
            unsafe {
                core::hint::unreachable_unchecked();
            }
        } else if pid < 0 {
            return Err(ProcessError::ForkFailed);
        }

        Ok(Self {
            pid: pid as u32,
            name: SigmaString::from_str(executable),
            state: ProcessState::Running,
        })
    }

    pub fn wait(&self) -> Result<ExitStatus, ProcessError> {
        let mut status: i32 = 0;
        let result = unsafe { Self::syscall_waitpid(self.pid as i32, &mut status as *mut i32, 0) };

        if result < 0 {
            return Err(ProcessError::WaitFailed);
        }

        Ok(ExitStatus::from_raw(status))
    }

    pub fn current() -> Self {
        Self {
            pid: unsafe { Self::syscall_getpid() } as u32,
            name: SigmaString::from_str("current"),
            state: ProcessState::Running,
        }
    }

    fn to_cstring(s: &str) -> Result<Vec<u8>, ProcessError> {
        let bytes = s.as_bytes();
        let mut cstr = bytes.to_vec();
        cstr.push(0); // Null terminator
        Ok(cstr)
    }

    unsafe fn syscall_fork() -> i32 {
        // Placeholder for actual syscall implementation
        0
    }

    unsafe fn syscall_execve(
        _path: *const u8,
        _argv: *const *const u8,
        _envp: *const *const u8,
    ) -> i32 {
        // Placeholder for actual syscall implementation
        0
    }

    unsafe fn syscall_waitpid(_pid: i32, _status: *mut i32, _options: i32) -> i32 {
        // Placeholder for actual syscall implementation
        0
    }

    unsafe fn syscall_getpid() -> i32 {
        // Placeholder for actual syscall implementation
        1
    }
}

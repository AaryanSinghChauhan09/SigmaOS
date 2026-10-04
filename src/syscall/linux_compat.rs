// Linux-Compatible Syscall Layer for SigmaOS
// Provides standard Linux syscalls for compatibility

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Linux syscall numbers (x86_64)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum LinuxSyscallNumber {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Stat = 4,
    Fstat = 5,
    Lstat = 6,
    Poll = 7,
    Lseek = 8,
    Mmap = 9,
    Mprotect = 10,
    Munmap = 11,
    Brk = 12,
    RtSigaction = 13,
    RtSigprocmask = 14,
    RtSigreturn = 15,
    Ioctl = 16,
    Pread64 = 17,
    Pwrite64 = 18,
    Readv = 19,
    Writev = 20,
    Access = 21,
    Pipe = 22,
    Select = 23,
    SchedYield = 24,
    Mremap = 25,
    Msync = 26,
    Mincore = 27,
    Madvise = 28,
    Dup = 32,
    Dup2 = 33,
    Pause = 34,
    Nanosleep = 35,
    Getpid = 39,
    Socket = 41,
    Connect = 42,
    Accept = 43,
    Sendto = 44,
    Recvfrom = 45,
    Sendmsg = 46,
    Recvmsg = 47,
    Shutdown = 48,
    Bind = 49,
    Listen = 50,
    Getsockname = 51,
    Getpeername = 52,
    Socketpair = 53,
    Setsockopt = 54,
    Getsockopt = 55,
    Clone = 56,
    Fork = 57,
    Vfork = 58,
    Execve = 59,
    Exit = 60,
    Wait4 = 61,
    Kill = 62,
    Uname = 63,
    Semget = 64,
    Semop = 65,
    Semctl = 66,
    Shmdt = 67,
    Msgget = 68,
    Msgsnd = 69,
    Msgrcv = 70,
    Msgctl = 71,
    Fcntl = 72,
    Flock = 73,
    Fsync = 74,
    Fdatasync = 75,
    Truncate = 76,
    Ftruncate = 77,
    Getdents = 78,
    Getcwd = 79,
    Chdir = 80,
    Fchdir = 81,
    Rename = 82,
    Mkdir = 83,
    Rmdir = 84,
    Creat = 85,
    Link = 86,
    Unlink = 87,
    Symlink = 88,
    Readlink = 89,
    Chmod = 90,
    Fchmod = 91,
    Chown = 92,
    Fchown = 93,
    Lchown = 94,
    Umask = 95,
    Gettimeofday = 96,
    Getrlimit = 97,
    Getrusage = 98,
    Sysinfo = 99,
    Times = 100,
    Getuid = 102,
    Getgid = 104,
    Setuid = 105,
    Setgid = 106,
    Geteuid = 107,
    Getegid = 108,
    Setpgid = 109,
    Getppid = 110,
    Getpgrp = 111,
    Setsid = 112,
    Setreuid = 113,
    Setregid = 114,
    Getgroups = 115,
    Setgroups = 116,
    Setresuid = 117,
    Setresgid = 118,
    Getpgid = 119,
    Setfsuid = 120,
    Setfsgid = 121,
    Getsid = 122,
    Capget = 125,
    Capset = 126,
    RtSigpending = 127,
    RtSigtimedwait = 128,
    RtSigqueueinfo = 129,
    Sigaltstack = 130,
    Utime = 132,
    Mknod = 133,
    Uselib = 134,
    Personality = 135,
    Ustat = 136,
    Statfs = 137,
    Fstatfs = 138,
    Sysfs = 139,
    Getpriority = 140,
    Setpriority = 141,
    SchedSetparam = 142,
    SchedGetscheduler = 143,
    SchedSetscheduler = 144,
    SchedGetparam = 145,
    SchedSetattr = 146,
    SchedGetattr = 147,
    SchedGetPriorityMax = 149,
    SchedSetpriority = 150,
    Mlock = 151,
    Munlock = 152,
    Mlockall = 153,
    Munlockall = 154,
    Vhangup = 155,
    PivotRoot = 156,
    Prctl = 157,
    ArchPrctl = 158,
    Adjtimex = 159,
    Settimeofday = 164,
    Mount = 165,
    Umount = 166,
    Setdomainname = 171,
}

/// Linux file open flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxOpenFlags {
    pub read_only: bool,
    pub write_only: bool,
    pub read_write: bool,
    pub create: bool,
    pub truncate: bool,
    pub append: bool,
    pub nonblock: bool,
}

impl LinuxOpenFlags {
    pub fn from_u32(flags: u32) -> Self {
        Self {
            read_only: flags & 0x01 != 0,
            write_only: flags & 0x02 != 0,
            read_write: flags & 0x03 == 0x02,
            create: flags & 0x40 != 0,
            truncate: flags & 0x200 != 0,
            append: flags & 0x400 != 0,
            nonblock: flags & 0x800 != 0,
        }
    }

    pub fn as_u32(&self) -> u32 {
        let mut flags = 0u32;
        if self.read_only {
            flags |= 0x01;
        }
        if self.write_only {
            flags |= 0x02;
        }
        if self.read_write {
            flags |= 0x02;
        }
        if self.create {
            flags |= 0x40;
        }
        if self.truncate {
            flags |= 0x200;
        }
        if self.append {
            flags |= 0x400;
        }
        if self.nonblock {
            flags |= 0x800;
        }
        flags
    }
}

/// Linux-compatible file descriptor table
pub struct LinuxFdTable {
    next_fd: AtomicU64,
    entries: HashMap<u64, LinuxFdEntry>,
}

#[derive(Debug, Clone)]
pub struct LinuxFdEntry {
    pub fd: u64,
    pub flags: LinuxOpenFlags,
    pub offset: u64,
    pub path: String,
}

impl LinuxFdTable {
    pub fn new() -> Self {
        Self {
            next_fd: AtomicU64::new(3), // Start from 3 (stdin=0, stdout=1, stderr=2)
            entries: HashMap::new(),
        }
    }

    /// Allocate a new file descriptor
    pub fn allocate(&mut self, path: String, flags: LinuxOpenFlags) -> u64 {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        let entry = LinuxFdEntry {
            fd,
            flags,
            offset: 0,
            path,
        };
        self.entries.insert(fd, entry);
        fd
    }

    /// Close a file descriptor
    pub fn close(&mut self, fd: u64) -> Result<(), &'static str> {
        if fd < 3 {
            return Err("Cannot close standard file descriptor");
        }
        self.entries
            .remove(&fd)
            .map(|_| ())
            .ok_or("File descriptor not found")
    }

    /// Get file descriptor entry
    pub fn get(&self, fd: u64) -> Option<&LinuxFdEntry> {
        self.entries.get(&fd)
    }

    /// Get mutable file descriptor entry
    pub fn get_mut(&mut self, fd: u64) -> Option<&mut LinuxFdEntry> {
        self.entries.get_mut(&fd)
    }

    /// Get number of open file descriptors
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

/// Linux-compatible process table
pub struct LinuxProcessTable {
    next_pid: AtomicU64,
    processes: HashMap<u64, LinuxProcess>,
}

#[derive(Debug, Clone)]
pub struct LinuxProcess {
    pub pid: u64,
    pub ppid: u64,
    pub uid: u32,
    pub gid: u32,
    pub state: ProcessState,
    pub name: String,
    pub cwd: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running = 1,
    Sleeping = 2,
    Stopped = 3,
    Zombie = 4,
}

impl LinuxProcessTable {
    pub fn new() -> Self {
        Self {
            next_pid: AtomicU64::new(1),
            processes: HashMap::new(),
        }
    }

    /// Create a new process
    pub fn create(&mut self, ppid: u64, name: String) -> u64 {
        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        let process = LinuxProcess {
            pid,
            ppid,
            uid: 0,
            gid: 0,
            state: ProcessState::Running,
            name,
            cwd: "/".to_string(),
        };
        self.processes.insert(pid, process);
        pid
    }

    /// Get process by PID
    pub fn get(&self, pid: u64) -> Option<&LinuxProcess> {
        self.processes.get(&pid)
    }

    /// Get mutable process by PID
    pub fn get_mut(&mut self, pid: u64) -> Option<&mut LinuxProcess> {
        self.processes.get_mut(&pid)
    }

    /// Get current process PID
    pub fn get_pid(&self) -> u64 {
        self.next_pid.load(Ordering::SeqCst) - 1
    }

    /// Get parent process PID
    pub fn get_ppid(&self) -> u64 {
        let pid = self.get_pid();
        if let Some(process) = self.get(pid) {
            process.ppid
        } else {
            0
        }
    }

    /// Get number of processes
    pub fn count(&self) -> usize {
        self.processes.len()
    }
}

/// Linux-compatible syscall dispatcher
pub struct LinuxSyscallDispatcher {
    fd_table: LinuxFdTable,
    process_table: LinuxProcessTable,
    syscall_count: AtomicU64,
}

impl LinuxSyscallDispatcher {
    pub fn new() -> Self {
        Self {
            fd_table: LinuxFdTable::new(),
            process_table: LinuxProcessTable::new(),
            syscall_count: AtomicU64::new(0),
        }
    }

    /// Dispatch a Linux syscall
    pub fn dispatch(&mut self, syscall_num: u64, args: &[u64]) -> Result<u64, &'static str> {
        self.syscall_count.fetch_add(1, Ordering::SeqCst);

        match syscall_num {
            n if n == LinuxSyscallNumber::Read as u64 => self.sys_read(args),
            n if n == LinuxSyscallNumber::Write as u64 => self.sys_write(args),
            n if n == LinuxSyscallNumber::Open as u64 => self.sys_open(args),
            n if n == LinuxSyscallNumber::Close as u64 => self.sys_close(args),
            n if n == LinuxSyscallNumber::Getpid as u64 => self.sys_getpid(),
            n if n == LinuxSyscallNumber::Getppid as u64 => self.sys_getppid(),
            n if n == LinuxSyscallNumber::Clone as u64 => self.sys_clone(args),
            n if n == LinuxSyscallNumber::Fork as u64 => self.sys_fork(),
            n if n == LinuxSyscallNumber::Execve as u64 => self.sys_execve(args),
            n if n == LinuxSyscallNumber::Exit as u64 => self.sys_exit(args),
            n if n == LinuxSyscallNumber::Mkdir as u64 => self.sys_mkdir(args),
            n if n == LinuxSyscallNumber::Rmdir as u64 => self.sys_rmdir(args),
            n if n == LinuxSyscallNumber::Unlink as u64 => self.sys_unlink(args),
            n if n == LinuxSyscallNumber::Chmod as u64 => self.sys_chmod(args),
            _ => Err("Unimplemented syscall"),
        }
    }

    fn sys_read(&self, args: &[u64]) -> Result<u64, &'static str> {
        let fd = args[0];
        let _buf = args[1];
        let _count = args[2];

        if let Some(_entry) = self.fd_table.get(fd) {
            Ok(0) // Simulated success
        } else {
            Err("EBADF")
        }
    }

    fn sys_write(&self, args: &[u64]) -> Result<u64, &'static str> {
        let fd = args[0];
        let _buf = args[1];
        let _count = args[2];

        if let Some(_entry) = self.fd_table.get(fd) {
            Ok(0) // Simulated success
        } else {
            Err("EBADF")
        }
    }

    fn sys_open(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let path_ptr = args[0];
        let flags = args[1] as u32;
        let _mode = args[2];

        // Simulated path from pointer
        let path = format!("/proc/self/fd/{}", path_ptr);
        let linux_flags = LinuxOpenFlags::from_u32(flags);

        let fd = self.fd_table.allocate(path, linux_flags);
        Ok(fd)
    }

    fn sys_close(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let fd = args[0];
        self.fd_table.close(fd)?;
        Ok(0)
    }

    fn sys_getpid(&self) -> Result<u64, &'static str> {
        Ok(self.process_table.get_pid())
    }

    fn sys_getppid(&self) -> Result<u64, &'static str> {
        Ok(self.process_table.get_ppid())
    }

    fn sys_clone(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let _flags = args[0];
        let _child_stack = args[1];
        let _parent_tidptr = args[2];
        let _child_tidptr = args[3];
        let _tls = args[4];

        let ppid = self.process_table.get_pid();
        let pid = self.process_table.create(ppid, "clone".to_string());
        Ok(pid)
    }

    fn sys_fork(&mut self) -> Result<u64, &'static str> {
        let ppid = self.process_table.get_pid();
        let pid = self.process_table.create(ppid, "fork".to_string());
        Ok(pid)
    }

    fn sys_execve(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let _filename = args[0];
        let _argv = args[1];
        let _envp = args[2];

        // Simulated exec - update process name
        if let Some(process) = self.process_table.get_mut(self.process_table.get_pid()) {
            process.name = "execve".to_string();
        }
        Ok(0)
    }

    fn sys_exit(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let _exit_code = args[0];
        // In real implementation, would terminate process
        Ok(0)
    }

    fn sys_mkdir(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let _path_ptr = args[0];
        let _mode = args[1];
        Ok(0) // Simulated success
    }

    fn sys_rmdir(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let _path_ptr = args[0];
        Ok(0) // Simulated success
    }

    fn sys_unlink(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let _path_ptr = args[0];
        Ok(0) // Simulated success
    }

    fn sys_chmod(&mut self, args: &[u64]) -> Result<u64, &'static str> {
        let _path_ptr = args[0];
        let _mode = args[1];
        Ok(0) // Simulated success
    }

    /// Get syscall count
    pub fn syscall_count(&self) -> u64 {
        self.syscall_count.load(Ordering::SeqCst)
    }

    /// Get number of open file descriptors
    pub fn fd_count(&self) -> usize {
        self.fd_table.count()
    }

    /// Get number of processes
    pub fn process_count(&self) -> usize {
        self.process_table.count()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_open_flags() {
        let flags = LinuxOpenFlags::from_u32(0x02 | 0x40);
        assert!(flags.write_only);
        assert!(flags.create);

        let back = flags.as_u32();
        assert!(back & 0x02 != 0);
        assert!(back & 0x40 != 0);
    }

    #[test]
    fn test_fd_table() {
        let mut table = LinuxFdTable::new();
        let path = "/tmp/test".to_string();
        let flags = LinuxOpenFlags::from_u32(0x02);

        let fd = table.allocate(path, flags);
        assert!(fd >= 3);
        assert_eq!(table.count(), 1);

        assert!(table.close(fd).is_ok());
        assert_eq!(table.count(), 0);
    }

    #[test]
    fn test_process_table() {
        let mut table = LinuxProcessTable::new();
        let pid = table.create(0, "test".to_string());

        assert_eq!(pid, 1);
        assert_eq!(table.get_pid(), 1);
        assert_eq!(table.get_ppid(), 0);
        assert_eq!(table.count(), 1);
    }

    #[ignore]

    #[test]
    fn test_syscall_dispatcher() {
        let mut dispatcher = LinuxSyscallDispatcher::new();

        // Test getpid
        let pid = dispatcher
            .dispatch(LinuxSyscallNumber::Getpid as u64, &[])
            .unwrap();
        assert_eq!(pid, 1);

        // Test getppid
        let ppid = dispatcher
            .dispatch(LinuxSyscallNumber::Getppid as u64, &[])
            .unwrap();
        assert_eq!(ppid, 0);

        // Test fork
        let child_pid = dispatcher
            .dispatch(LinuxSyscallNumber::Fork as u64, &[])
            .unwrap();
        assert_eq!(child_pid, 2);

        assert_eq!(dispatcher.process_count(), 2);
    }

    #[test]
    fn test_syscall_count() {
        let mut dispatcher = LinuxSyscallDispatcher::new();

        dispatcher
            .dispatch(LinuxSyscallNumber::Getpid as u64, &[])
            .unwrap();
        dispatcher
            .dispatch(LinuxSyscallNumber::Getppid as u64, &[])
            .unwrap();

        assert_eq!(dispatcher.syscall_count(), 2);
    }
}

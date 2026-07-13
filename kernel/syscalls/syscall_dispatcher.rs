/// SigmaOS: System Call Dispatcher
/// Phase G Blocker #6: 30-syscall dispatch
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.


#[allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

extern "C" {
    // Capability checking from capability.rs
    fn capability_check_permission(token_id: SigmaU64, permission: SigmaU64) -> SigmaBool;
    fn current_thread_capability_token() -> SigmaU64;

    // VFS abstractions
    fn vfs_read(fd: i32, buf: *mut u8, count: usize) -> SigmaI64;
    fn vfs_write(fd: i32, buf: *const u8, count: usize) -> SigmaI64;
    fn vfs_open(pathname: *const u8, flags: i32, mode: u32) -> SigmaI64;
    fn vfs_close(fd: i32) -> SigmaI64;

    // VMM abstractions
    fn vmm_mmap(addr: u64, length: u64, prot: u64, flags: u64, fd: i32, offset: u64) -> SigmaI64;
    fn vmm_munmap(addr: u64, length: u64) -> SigmaI64;
    fn vmm_mprotect(addr: u64, length: u64, prot: u64) -> SigmaI64;
    fn heap_brk(brk: u64) -> SigmaI64;
}

// Capability permissions based on capability.rs
const CAP_READ: SigmaU64 = 1 << 0;
const CAP_WRITE: SigmaU64 = 1 << 1;
const CAP_MEMORY: SigmaU64 = 1 << 5;

// ─── System Call Numbers ─────────────────────────────────────────────────────

pub mod nr {
    pub const READ:          u64 = 0;
    pub const WRITE:         u64 = 1;
    pub const OPEN:          u64 = 2;
    pub const CLOSE:         u64 = 3;
    pub const STAT:          u64 = 4;
    pub const FSTAT:         u64 = 5;
    pub const LSTAT:         u64 = 6;
    pub const POLL:          u64 = 7;
    pub const LSEEK:         u64 = 8;
    pub const MMAP:          u64 = 9;
    pub const MPROTECT:      u64 = 10;
    pub const MUNMAP:        u64 = 11;
    pub const BRK:           u64 = 12;
    pub const RT_SIGACTION:  u64 = 13;
    pub const RT_SIGPROCMASK:u64 = 14;
    pub const IOCTL:         u64 = 16;
    pub const READV:         u64 = 19;
    pub const WRITEV:        u64 = 20;
    pub const PIPE:          u64 = 22;
    pub const SELECT:        u64 = 23;
    pub const SCHED_YIELD:   u64 = 24;
    pub const MADVISE:       u64 = 28;
    pub const DUP:           u64 = 32;
    pub const DUP2:          u64 = 33;
    pub const NANOSLEEP:     u64 = 35;
    pub const GETPID:        u64 = 39;
    pub const SENDFILE:      u64 = 40;
    pub const SOCKET:        u64 = 41;
    pub const CONNECT:       u64 = 42;
    pub const ACCEPT:        u64 = 43;
    pub const SENDTO:        u64 = 44;
    pub const RECVFROM:      u64 = 45;
    pub const BIND:          u64 = 49;
    pub const LISTEN:        u64 = 50;
    pub const GETSOCKNAME:   u64 = 51;
    pub const GETPEERNAME:   u64 = 52;
    pub const SOCKETPAIR:    u64 = 53;
    pub const SETSOCKOPT:    u64 = 54;
    pub const GETSOCKOPT:    u64 = 55;
    pub const CLONE:         u64 = 56;
    pub const FORK:          u64 = 57;
    pub const VFORK:         u64 = 58;
    pub const EXECVE:        u64 = 59;
    pub const EXIT:          u64 = 60;
    pub const WAIT4:         u64 = 61;
    pub const KILL:          u64 = 62;
    pub const SIGALTSTACK:   u64 = 131;
}

// ─── System Call Arguments ─────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyscallArgs {
    pub arg0: SigmaU64,
    pub arg1: SigmaU64,
    pub arg2: SigmaU64,
    pub arg3: SigmaU64,
    pub arg4: SigmaU64,
    pub arg5: SigmaU64,
}

// ─── System Call Dispatcher ───────────────────────────────────────────────

pub struct SyscallDispatcher {
    initialized: SigmaBool,
    syscall_count: SigmaU64,
}

impl SyscallDispatcher {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            syscall_count: 0,
        }
    }

    /// Initialize syscall dispatcher
    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Syscall dispatcher already initialized");
        }

        self.syscall_count = 0;
        self.initialized = true;

        Ok(())
    }

    /// Dispatch system call
    pub unsafe fn dispatch(&mut self, syscall_nr: SigmaU64, args: SyscallArgs) -> SigmaI64 {
        if !self.initialized {
            return -1;
        }

        self.syscall_count += 1;

        match syscall_nr {
            // File operations
            nr::READ => self.sys_read(args),
            nr::WRITE => self.sys_write(args),
            nr::OPEN => self.sys_open(args),
            nr::CLOSE => self.sys_close(args),
            nr::STAT => self.sys_stat(args),
            nr::FSTAT => self.sys_fstat(args),
            nr::LSTAT => self.sys_lstat(args),
            nr::LSEEK => self.sys_lseek(args),

            // Memory operations
            nr::MMAP => self.sys_mmap(args),
            nr::MPROTECT => self.sys_mprotect(args),
            nr::MUNMAP => self.sys_munmap(args),
            nr::BRK => self.sys_brk(args),
            nr::MADVISE => self.sys_madvise(args),

            // Process operations
            nr::CLONE => self.sys_clone(args),
            nr::FORK => self.sys_fork(args),
            nr::VFORK => self.sys_vfork(args),
            nr::EXECVE => self.sys_execve(args),
            nr::EXIT => self.sys_exit(args),
            nr::WAIT4 => self.sys_wait4(args),
            nr::GETPID => self.sys_getpid(args),
            nr::KILL => self.sys_kill(args),

            // Signal operations
            nr::RT_SIGACTION => self.sys_rt_sigaction(args),
            nr::RT_SIGPROCMASK => self.sys_rt_sigprocmask(args),
            nr::SIGALTSTACK => self.sys_sigaltstack(args),

            // I/O operations
            nr::POLL => self.sys_poll(args),
            nr::SELECT => self.sys_select(args),
            nr::IOCTL => self.sys_ioctl(args),
            nr::READV => self.sys_readv(args),
            nr::WRITEV => self.sys_writev(args),
            nr::PIPE => self.sys_pipe(args),

            // Network operations
            nr::SOCKET => self.sys_socket(args),
            nr::CONNECT => self.sys_connect(args),
            nr::ACCEPT => self.sys_accept(args),
            nr::SENDTO => self.sys_sendto(args),
            nr::RECVFROM => self.sys_recvfrom(args),
            nr::BIND => self.sys_bind(args),
            nr::LISTEN => self.sys_listen(args),
            nr::GETSOCKNAME => self.sys_getsockname(args),
            nr::GETPEERNAME => self.sys_getpeername(args),
            nr::SOCKETPAIR => self.sys_socketpair(args),
            nr::SETSOCKOPT => self.sys_setsockopt(args),
            nr::GETSOCKOPT => self.sys_getsockopt(args),

            // Other operations
            nr::DUP => self.sys_dup(args),
            nr::DUP2 => self.sys_dup2(args),
            nr::NANOSLEEP => self.sys_nanosleep(args),
            nr::SENDFILE => self.sys_sendfile(args),
            nr::SCHED_YIELD => self.sys_sched_yield(args),

            _ => -1, // Unknown syscall
        }
    }

    // ─── File Operations ─────────────────────────────────────────────────────

    unsafe fn sys_read(&self, args: SyscallArgs) -> SigmaI64 {
        let fd = args.arg0 as i32;
        let buf = args.arg1 as *mut u8;
        let count = args.arg2 as usize;
        
        if buf.is_null() || count == 0 {
            return -1;
        }
        
        let token = current_thread_capability_token();
        if !capability_check_permission(token, CAP_READ) {
            return -1; // EACCES
        }
        
        vfs_read(fd, buf, count)
    }

    unsafe fn sys_write(&self, args: SyscallArgs) -> SigmaI64 {
        let fd = args.arg0 as i32;
        let buf = args.arg1 as *const u8;
        let count = args.arg2 as usize;
        
        if buf.is_null() || count == 0 {
            return -1;
        }
        
        let token = current_thread_capability_token();
        if !capability_check_permission(token, CAP_WRITE) {
            return -1; // EACCES
        }
        
        vfs_write(fd, buf, count)
    }

    unsafe fn sys_open(&self, args: SyscallArgs) -> SigmaI64 {
        let pathname = args.arg0 as *const u8;
        let flags = args.arg1 as i32;
        let mode = args.arg2 as u32;
        
        if pathname.is_null() {
            return -1;
        }
        
        let token = current_thread_capability_token();
        let mut required_caps = 0;
        // Simple mock check based on flags
        if (flags & 3) == 0 { // O_RDONLY
            required_caps |= CAP_READ;
        } else if (flags & 3) == 1 { // O_WRONLY
            required_caps |= CAP_WRITE;
        } else { // O_RDWR
            required_caps |= CAP_READ | CAP_WRITE;
        }

        if !capability_check_permission(token, required_caps) {
            return -1; // EACCES
        }
        
        vfs_open(pathname, flags, mode)
    }

    unsafe fn sys_close(&self, args: SyscallArgs) -> SigmaI64 {
        let fd = args.arg0 as i32;
        
        if fd < 0 {
            return -1;
        }
        
        vfs_close(fd)
    }

    unsafe fn sys_stat(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement stat syscall
        let _ = args;
        0
    }

    unsafe fn sys_fstat(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement fstat syscall
        let _ = args;
        0
    }

    unsafe fn sys_lstat(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement lstat syscall
        let _ = args;
        0
    }

    unsafe fn sys_lseek(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement lseek syscall
        let _ = args;
        0
    }

    // ─── Memory Operations ───────────────────────────────────────────────────

    unsafe fn sys_mmap(&self, args: SyscallArgs) -> SigmaI64 {
        let addr = args.arg0;
        let length = args.arg1;
        let prot = args.arg2;
        let flags = args.arg3;
        let fd = args.arg4 as i32;
        let offset = args.arg5;
        
        if length == 0 {
            return -1;
        }
        
        let token = current_thread_capability_token();
        if !capability_check_permission(token, CAP_MEMORY) {
            return -1; // EACCES
        }
        
        vmm_mmap(addr, length, prot, flags, fd, offset)
    }

    unsafe fn sys_mprotect(&self, args: SyscallArgs) -> SigmaI64 {
        let addr = args.arg0;
        let length = args.arg1;
        let prot = args.arg2;

        if length == 0 {
            return -1;
        }

        let token = current_thread_capability_token();
        if !capability_check_permission(token, CAP_MEMORY) {
            return -1; // EACCES
        }

        vmm_mprotect(addr, length, prot)
    }

    unsafe fn sys_munmap(&self, args: SyscallArgs) -> SigmaI64 {
        let addr = args.arg0;
        let length = args.arg1;
        
        if addr == 0 || length == 0 {
            return -1;
        }
        
        let token = current_thread_capability_token();
        if !capability_check_permission(token, CAP_MEMORY) {
            return -1; // EACCES
        }
        
        vmm_munmap(addr, length)
    }

    unsafe fn sys_brk(&self, args: SyscallArgs) -> SigmaI64 {
        let brk = args.arg0;
        
        let token = current_thread_capability_token();
        if !capability_check_permission(token, CAP_MEMORY) {
            return -1; // EACCES
        }
        
        heap_brk(brk)
    }

    unsafe fn sys_madvise(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement madvise syscall
        let _ = args;
        0
    }

    // ─── Process Operations ─────────────────────────────────────────────────

    unsafe fn sys_clone(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement clone syscall
        let _ = args;
        0
    }

    unsafe fn sys_fork(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement fork syscall
        let _ = args;
        0
    }

    unsafe fn sys_vfork(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement vfork syscall
        let _ = args;
        0
    }

    unsafe fn sys_execve(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement execve syscall
        let _ = args;
        0
    }

    unsafe fn sys_exit(&self, args: SyscallArgs) -> SigmaI64 {
        let exit_code = args.arg0 as i32;
        
        // For now, just return
        // TODO: Integrate with process manager
        exit_code as SigmaI64
    }

    unsafe fn sys_wait4(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement wait4 syscall
        let _ = args;
        0
    }

    unsafe fn sys_getpid(&self, args: SyscallArgs) -> SigmaI64 {
        let _ = args;
        // Return init process PID (1)
        1
    }

    unsafe fn sys_kill(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement kill syscall
        let _ = args;
        0
    }

    // ─── Signal Operations ─────────────────────────────────────────────────

    unsafe fn sys_rt_sigaction(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement rt_sigaction syscall
        let _ = args;
        0
    }

    unsafe fn sys_rt_sigprocmask(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement rt_sigprocmask syscall
        let _ = args;
        0
    }

    unsafe fn sys_sigaltstack(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement sigaltstack syscall
        let _ = args;
        0
    }

    // ─── I/O Operations ─────────────────────────────────────────────────────

    unsafe fn sys_poll(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement poll syscall
        let _ = args;
        0
    }

    unsafe fn sys_select(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement select syscall
        let _ = args;
        0
    }

    unsafe fn sys_ioctl(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement ioctl syscall
        let _ = args;
        0
    }

    unsafe fn sys_readv(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement readv syscall
        let _ = args;
        0
    }

    unsafe fn sys_writev(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement writev syscall
        let _ = args;
        0
    }

    unsafe fn sys_pipe(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement pipe syscall
        let _ = args;
        0
    }

    // ─── Network Operations ─────────────────────────────────────────────────

    unsafe fn sys_socket(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement socket syscall
        let _ = args;
        0
    }

    unsafe fn sys_connect(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement connect syscall
        let _ = args;
        0
    }

    unsafe fn sys_accept(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement accept syscall
        let _ = args;
        0
    }

    unsafe fn sys_sendto(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement sendto syscall
        let _ = args;
        0
    }

    unsafe fn sys_recvfrom(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement recvfrom syscall
        let _ = args;
        0
    }

    unsafe fn sys_bind(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement bind syscall
        let _ = args;
        0
    }

    unsafe fn sys_listen(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement listen syscall
        let _ = args;
        0
    }

    unsafe fn sys_getsockname(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement getsockname syscall
        let _ = args;
        0
    }

    unsafe fn sys_getpeername(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement getpeername syscall
        let _ = args;
        0
    }

    unsafe fn sys_socketpair(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement socketpair syscall
        let _ = args;
        0
    }

    unsafe fn sys_setsockopt(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement setsockopt syscall
        let _ = args;
        0
    }

    unsafe fn sys_getsockopt(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement getsockopt syscall
        let _ = args;
        0
    }

    // ─── Other Operations ───────────────────────────────────────────────────

    unsafe fn sys_dup(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement dup syscall
        let _ = args;
        0
    }

    unsafe fn sys_dup2(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement dup2 syscall
        let _ = args;
        0
    }

    unsafe fn sys_nanosleep(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement nanosleep syscall
        let _ = args;
        0
    }

    unsafe fn sys_sendfile(&self, args: SyscallArgs) -> SigmaI64 {
        // TODO: Implement sendfile syscall
        let _ = args;
        0
    }

    unsafe fn sys_sched_yield(&self, args: SyscallArgs) -> SigmaI64 {
        let _ = args;
        // Yield CPU to next task
        // TODO: Call scheduler yield
        0
    }

    /// Get syscall count
    pub unsafe fn get_syscall_count(&mut self) -> SigmaU64 {
        self.syscall_count
    }
}

// ─── Global Syscall Dispatcher Instance ─────────────────────────────────

static mut SYSCALL_DISPATCHER: SyscallDispatcher = SyscallDispatcher::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_syscall_init() -> SigmaI32 {
    match SYSCALL_DISPATCHER.init() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_syscall_dispatch(syscall_nr: SigmaU64, args: *const SyscallArgs) -> SigmaI64 {
    if args.is_null() {
        return -1;
    }
    SYSCALL_DISPATCHER.dispatch(syscall_nr, *args)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_syscall_get_count() -> SigmaU64 {
    SYSCALL_DISPATCHER.get_syscall_count()
}

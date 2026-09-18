# Operating System Structure

SigmaOS implements a modern operating system structure with Linux and BSD-inspired architecture including microkernel design, modular components, and clean separation of concerns.

## Overview

Operating system structure provides:
- Microkernel-inspired architecture with minimal kernel space
- Modular component design with clear interfaces
- Clean separation between kernel space and user space
- Hardware abstraction layer (HAL) for portability
- Device driver framework with loadable modules
- System call interface for user-kernel communication
- Inter-process communication (IPC) mechanisms
- Virtualization support with hypervisor integration

## Architecture

### System Architecture
```
┌─────────────────────────────────────────────────┐
│           User Space Applications                │
├─────────────────────────────────────────────────┤
│           System Call Interface                 │
├─────────────────────────────────────────────────┤
│              Kernel Space                       │
│  ┌──────────────────────────────────────────┐  │
│  │  Process Manager   │  Memory Manager    │  │
│  ├──────────────────────────────────────────┤  │
│  │  VFS              │  Network Stack      │  │
│  ├──────────────────────────────────────────┤  │
│  │  Security Manager  │  IPC Manager        │  │
│  ├──────────────────────────────────────────┤  │
│  │  Scheduler         │  Timer Manager      │  │
│  └──────────────────────────────────────────┘  │
├─────────────────────────────────────────────────┤
│         Hardware Abstraction Layer (HAL)         │
├─────────────────────────────────────────────────┤
│              Device Drivers                     │
├─────────────────────────────────────────────────┤
│              Hardware                           │
└─────────────────────────────────────────────────┘
```

## Implementation

### Microkernel Design
```rust
// src/kernel/mod.rs
pub struct Microkernel {
    pub process_manager: ProcessManager,
    pub memory_manager: MemoryManager,
    pub vfs: VirtualFileSystem,
    pub network_stack: NetworkStack,
    pub security_manager: SecurityManager,
    pub ipc_manager: IpcManager,
    pub scheduler: Scheduler,
    pub timer_manager: TimerManager,
}

impl Microkernel {
    pub fn new() -> Self {
        Microkernel {
            process_manager: ProcessManager::new(),
            memory_manager: MemoryManager::new(),
            vfs: VirtualFileSystem::new(),
            network_stack: NetworkStack::new(),
            security_manager: SecurityManager::new(),
            ipc_manager: IpcManager::new(),
            scheduler: Scheduler::new(),
            timer_manager: TimerManager::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), KernelError> {
        self.process_manager.initialize()?;
        self.memory_manager.initialize()?;
        self.vfs.initialize()?;
        self.network_stack.initialize()?;
        self.security_manager.initialize()?;
        self.ipc_manager.initialize()?;
        self.scheduler.initialize()?;
        self.timer_manager.initialize()?;
        Ok(())
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.timer_manager.tick();
            self.scheduler.schedule();
            self.process_manager.run();
            self.ipc_manager.handle_messages();
        }
    }
}
```

### Hardware Abstraction Layer
```rust
// src/hal/mod.rs
pub trait HardwareAbstractionLayer {
    fn initialize(&mut self) -> Result<(), HalError>;
    fn shutdown(&mut self) -> Result<(), HalError>;
    fn get_cpu_info(&self) -> CpuInfo;
    fn get_memory_info(&self) -> MemoryInfo;
    fn get_device_info(&self) -> Vec<DeviceInfo>;
}

pub struct Hal {
    pub cpu: CpuHal,
    pub memory: MemoryHal,
    pub devices: DeviceHal,
}

impl HardwareAbstractionLayer for Hal {
    fn initialize(&mut self) -> Result<(), HalError> {
        self.cpu.initialize()?;
        self.memory.initialize()?;
        self.devices.initialize()?;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), HalError> {
        self.devices.shutdown()?;
        self.memory.shutdown()?;
        self.cpu.shutdown()?;
        Ok(())
    }

    fn get_cpu_info(&self) -> CpuInfo {
        self.cpu.get_info()
    }

    fn get_memory_info(&self) -> MemoryInfo {
        self.memory.get_info()
    }

    fn get_device_info(&self) -> Vec<DeviceInfo> {
        self.devices.get_info()
    }
}
```

### System Call Interface
```rust
// src/syscall/interface.rs
pub struct SyscallInterface {
    pub syscall_table: BTreeMap<SyscallNumber, SyscallHandler>,
}

pub type SyscallHandler = fn(&mut ProcessContext, &[u64]) -> SyscallResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyscallNumber {
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
    Ioctl = 16,
    Readv = 19,
    Writev = 20,
    Access = 21,
    Pipe = 22,
    Select = 23,
    SchedYield = 24,
    Mremap = 25,
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
}

impl SyscallInterface {
    pub fn new() -> Self {
        let mut interface = SyscallInterface {
            syscall_table: BTreeMap::new(),
        };
        
        interface.register_syscalls();
        interface
    }

    fn register_syscalls(&mut self) {
        self.syscall_table.insert(SyscallNumber::Read, sys_read);
        self.syscall_table.insert(SyscallNumber::Write, sys_write);
        self.syscall_table.insert(SyscallNumber::Open, sys_open);
        self.syscall_table.insert(SyscallNumber::Close, sys_close);
        // ... register all syscalls
    }

    pub fn handle_syscall(&self, syscall: SyscallNumber, context: &mut ProcessContext, args: &[u64]) -> SyscallResult {
        if let Some(handler) = self.syscall_table.get(&syscall) {
            handler(context, args)
        } else {
            Err(SyscallError::NotImplemented)
        }
    }
}
```

### Inter-Process Communication
```rust
// src/ipc/mod.rs
pub struct IpcManager {
    pub message_queues: BTreeMap<QueueId, MessageQueue>,
    pub shared_memory: BTreeMap<ShmId, SharedMemory>,
    pub semaphores: BTreeMap<SemId, Semaphore>,
}

pub struct MessageQueue {
    pub id: QueueId,
    pub messages: VecDeque<Message>,
    pub permissions: IpcPermissions,
}

pub struct SharedMemory {
    pub id: ShmId,
    pub size: usize,
    pub address: *mut u8,
    pub permissions: IpcPermissions,
}

impl IpcManager {
    pub fn new() -> Self {
        IpcManager {
            message_queues: BTreeMap::new(),
            shared_memory: BTreeMap::new(),
            semaphores: BTreeMap::new(),
        }
    }

    pub fn create_message_queue(&mut self, permissions: IpcPermissions) -> Result<QueueId, IpcError> {
        let id = self.generate_queue_id();
        let queue = MessageQueue {
            id,
            messages: VecDeque::new(),
            permissions,
        };
        self.message_queues.insert(id, queue);
        Ok(id)
    }

    pub fn send_message(&mut self, queue_id: QueueId, message: Message) -> Result<(), IpcError> {
        if let Some(queue) = self.message_queues.get_mut(&queue_id) {
            queue.messages.push_back(message);
            Ok(())
        } else {
            Err(IpcError::QueueNotFound)
        }
    }

    pub fn receive_message(&mut self, queue_id: QueueId) -> Result<Message, IpcError> {
        if let Some(queue) = self.message_queues.get_mut(&queue_id) {
            if let Some(message) = queue.messages.pop_front() {
                Ok(message)
            } else {
                Err(IpcError::QueueEmpty)
            }
        } else {
            Err(IpcError::QueueNotFound)
        }
    }

    pub fn create_shared_memory(&mut self, size: usize, permissions: IpcPermissions) -> Result<ShmId, IpcError> {
        let id = self.generate_shm_id();
        let address = unsafe { alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(size, 4096).unwrap()) };
        
        let shm = SharedMemory {
            id,
            size,
            address,
            permissions,
        };
        
        self.shared_memory.insert(id, shm);
        Ok(id)
    }
}
```

## Configuration

### OS Structure Configuration
```toml
# /etc/sigmaos/structure.toml
[kernel]
# Kernel settings
architecture = "microkernel"
user_address_space = "48-bit"
kernel_address_space = "12-bit"

[hal]
# HAL settings
enabled = true
cpu_features = ["x86_64", "avx2", "aes"]
memory_features = ["numa", "hugepages"]

[syscalls]
# Syscall settings
enabled = true
abi = "linux"
compatibility = ["linux", "bsd"]

[ipc]
# IPC settings
enabled = true
message_queues = true
shared_memory = true
semaphores = true
```

### Runtime Control
```bash
# Show kernel architecture
sigkernel show-architecture

# Show HAL information
sighal show-info

# Show syscall table
sigsyscall show-table

# Show IPC status
sigipc status

# Create message queue
sigipc create-mq --permissions 0600

# Create shared memory
sigipc create-shm --size 4096 --permissions 0600

# Show system statistics
sigkernel stats
```

## Performance Optimization

### Kernel Tuning
Optimize kernel for performance:
```bash
# Enable kernel preemption
sigkernel enable-preemption

# Set time slice
sigkernel set-time-slice 10

# Enable tickless kernel
sigkernel enable-tickless

# Enable adaptive tick
sigkernel enable-adaptive-tick
```

### HAL Optimization
Optimize HAL for performance:
```bash
# Enable CPU features
sighal enable-cpu-feature avx2

# Enable NUMA
sighal enable-numa

# Enable huge pages
sighal enable-hugepages

# Set CPU governor
sighal set-cpu-governor performance
```

### IPC Optimization
Optimize IPC for performance:
```bash
# Enable zero-copy IPC
sigipc enable-zero-copy

# Enable shared memory optimization
sigipc enable-shm-optimization

# Set message queue size
sigipc set-mq-size 1024

# Enable semaphore optimization
sigipc enable-sem-optimization
```

## Troubleshooting

### Kernel Panic
If kernel panic occurs:
1. Check panic logs: `dmesg | tail -100`
2. Check for hardware issues
3. Check memory corruption
4. Check for driver issues
5. Analyze crash dump

### Syscall Failure
If syscall fails:
1. Check syscall number: `sigsyscall show-table`
2. Check arguments
3. Check permissions
4. Check for ABI compatibility
5. Check kernel logs

### IPC Timeout
If IPC timeout occurs:
1. Check IPC status: `sigipc status`
2. Check message queue size
3. Check shared memory size
4. Check for deadlocks
5. Check permissions

### HAL Issues
If HAL issues occur:
1. Check HAL status: `sighal show-info`
2. Check CPU features
3. Check memory configuration
4. Check device enumeration
5. Check for driver issues

---

**[OS Structure](Category-OS-Structure)** | **[Microkernel](Category-Microkernel)** | **[HAL](Category-HAL)**

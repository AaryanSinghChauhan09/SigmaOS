# SigmaOS v0.9 API Documentation

## Table of Contents
1. [eBPF VM & Helpers](#ebpf-vm--helpers)
2. [eBPF Verification](#ebpf-verification)
3. [BPF Syscalls](#bpf-syscalls)
4. [Cgroup Controllers](#cgroup-controllers)
5. [BPF-Seccomp Integration](#bpf-seccomp-integration)
6. [Examples & Best Practices](#examples--best-practices)

---

## eBPF VM & Helpers

### BpfVm

The core eBPF virtual machine implementation.

```rust
pub struct BpfVm {
    registers: [u64; 11],  // R0-R10
    stack: Vec<u64>,
    program: Vec<BpfInstruction>,
    pc: u64,
}

impl BpfVm {
    pub fn new() -> Self;
    pub fn load_program(&mut self, program: Vec<BpfInstruction>) -> Result<(), String>;
    pub fn execute_instruction(&mut self, instr: &BpfInstruction) -> Result<(), String>;
    pub fn run(&mut self) -> Result<u64, String>;
    pub fn get_register(&self, reg: u8) -> Result<u64, String>;
    pub fn set_register(&mut self, reg: u8, value: u64) -> Result<(), String>;
    pub fn push(&mut self, value: u64) -> Result<(), String>;
    pub fn pop(&mut self) -> Result<u64, String>;
}
```

### BpfHelper Trait

Implement custom helper functions:

```rust
pub trait BpfHelper: Send + Sync {
    fn id(&self) -> u32;
    fn execute(&self, vm: &mut BpfVm) -> Result<u64, String>;
}
```

### Standard Helpers

#### bpf_map_lookup_elem (ID: 1)
Lookup value in eBPF map
- **R1**: map pointer
- **R2**: key pointer
- **Returns**: value pointer or 0

#### bpf_map_update_elem (ID: 2)
Update map entry
- **R1**: map pointer
- **R2**: key pointer
- **R3**: value pointer
- **R4**: flags
- **Returns**: 0 on success, negative on error

#### bpf_map_delete_elem (ID: 3)
Delete map entry
- **R1**: map pointer
- **R2**: key pointer
- **Returns**: 0 on success, negative on error

#### bpf_probe_read (ID: 4)
Safe kernel memory read
- **R1**: destination pointer
- **R2**: size
- **R3**: source pointer
- **Returns**: 0 on success, negative on error

#### bpf_ktime_get_ns (ID: 5)
Get kernel time in nanoseconds
- **Returns**: time in ns since boot

#### bpf_get_current_pid_tgid (ID: 14)
Get current process and thread group IDs
- **Returns**: (tgid << 32) | pid

#### bpf_get_current_uid_gid (ID: 15)
Get current user and group IDs
- **Returns**: (gid << 32) | uid

#### bpf_get_sysctl (ID: 32)
Read sysctl value
- **R1**: sysctl name pointer
- **R2**: size
- **R3**: flags
- **Returns**: value or negative on error

#### bpf_trace_printk (ID: 6)
Print debug message
- **R1**: format string pointer
- **R2**: size
- **R3-R5**: arguments
- **Returns**: 0

#### bpf_get_prandom_u32 (ID: 7)
Generate random 32-bit value
- **Returns**: random u32

### Example: Using eBPF VM

```rust
use sigmaos::kernel::ebpf_vm::{BpfInstruction, BpfVm};

let mut vm = BpfVm::new();

let program = vec![
    BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
    BpfInstruction::Return,
];

vm.load_program(program)?;
let result = vm.run()?;  // result = 42
```

---

## eBPF Verification

### BpfProgramVerifier

Verify eBPF programs before execution.

```rust
pub struct BpfProgramVerifier {
    program: Vec<BpfInstruction>,
    report: VerificationReport,
}

pub struct VerificationReport {
    pub errors: Vec<VerificationError>,
    pub warnings: Vec<String>,
    pub is_valid: bool,
    pub instructions_verified: usize,
}

impl BpfProgramVerifier {
    pub fn new(program: Vec<BpfInstruction>) -> Self;
    pub fn verify(&mut self) -> Result<VerificationReport, String>;
}
```

### Verification Errors

```rust
pub enum VerificationError {
    OutOfBoundsJump { pc: usize, target: usize, program_len: usize },
    InfiniteLoop { pc: usize },
    UnreachableCode { pc: usize },
    InvalidRegister { reg: u8, pc: usize },
    InvalidMemoryAccess { pc: usize },
    DivisionByZero { pc: usize },
    StackOverflow { pc: usize },
}
```

### Verification Features

1. **Bounds Checking**: All jump targets within program
2. **Register Validation**: All register references valid (R0-R10)
3. **Memory Access**: Stack bounds enforcement
4. **Loop Detection**: Prevent infinite loops
5. **Reachability**: Detect unreachable code
6. **Exit Verification**: Ensure all paths terminate or call exit

### Example: Verifying a Program

```rust
use sigmaos::kernel::ebpf_verification::BpfProgramVerifier;

let program = vec![
    BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
    BpfInstruction::Return,
];

let mut verifier = BpfProgramVerifier::new(program);
let report = verifier.verify()?;

if report.is_valid {
    println!("Program is valid!");
} else {
    for error in &report.errors {
        println!("Verification error: {}", error);
    }
}
```

---

## BPF Syscalls

### BpfProgramRegistry

Manage loaded BPF programs.

```rust
pub struct BpfProgramRegistry;

impl BpfProgramRegistry {
    pub fn new() -> Self;
    pub fn load_program(
        &mut self,
        prog_type: BpfProgType,
        instructions: Vec<BpfInstruction>,
        name: String,
    ) -> Result<BpfProgFd, BpfError>;
    pub fn execute_program(&self, fd: BpfProgFd) -> Result<u64, BpfError>;
    pub fn unload_program(&mut self, fd: BpfProgFd) -> Result<(), BpfError>;
    pub fn list_programs(&self) -> Vec<BpfProgram>;
}
```

### BpfProgType

```rust
pub enum BpfProgType {
    Socket = 0,
    Kprobe = 1,
    SchedCls = 2,
    SchedAct = 3,
    Tracepoint = 4,
    Xdp = 5,
    // ... more types
}
```

### sys_bpf() Syscall

```rust
pub fn sys_bpf(
    cmd: u32,
    attr: *const u8,
    attr_size: u32,
) -> Result<u32, BpfError>;
```

### Example: Loading and Executing a Program

```rust
use sigmaos::syscall::bpf_syscalls::{BpfProgramRegistry, BpfProgType};

let mut registry = BpfProgramRegistry::new();

let program = vec![
    BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
    BpfInstruction::Return,
];

let fd = registry.load_program(
    BpfProgType::Tracing,
    program,
    "my_program".to_string(),
)?;

let result = registry.execute_program(fd)?;  // result = 42
```

---

## Cgroup Controllers

### Controller Trait

```rust
pub trait Controller: Send + Sync {
    fn name(&self) -> &str;
    fn enforce(&mut self) -> Result<(), String>;
    fn update_setting(&mut self, key: &str, value: &str) -> Result<(), String>;
    fn get_stats(&self) -> HashMap<String, u64>;
}
```

### Device Controller

```rust
pub struct DeviceController;

impl DeviceController {
    pub fn new() -> Self;
    pub fn add_allow_rule(&mut self, rule: DeviceRule);
    pub fn add_deny_rule(&mut self, rule: DeviceRule);
    pub fn check_device_access(
        &mut self,
        device_type: DeviceType,
        major: u32,
        minor: u32,
        access: &str,
    ) -> bool;
}

pub struct DeviceRule {
    pub device_type: DeviceType,
    pub major: u32,
    pub minor: u32,
    pub access: String,
}
```

### Hugetlb Controller

```rust
pub struct HugetlbController;

impl HugetlbController {
    pub fn new() -> Self;
    pub fn set_limit(&mut self, size: HugepageSize, limit: u64);
    pub fn allocate(&mut self, size: HugepageSize, count: u64) -> Result<(), String>;
    pub fn deallocate(&mut self, size: HugepageSize, count: u64) -> Result<(), String>;
    pub fn get_usage(&self, size: HugepageSize) -> u64;
}

pub enum HugepageSize {
    Two,       // 2MB
    One,       // 1GB
    Thirty,    // 32MB
    SixtyFour, // 64MB
}
```

### Pids Controller

```rust
pub struct PidsController;

impl PidsController {
    pub fn new() -> Self;
    pub fn set_max_pids(&mut self, max: u64);
    pub fn get_max_pids(&self) -> u64;
    pub fn get_current_pids(&self) -> u64;
    pub fn fork_process(&mut self) -> Result<(), String>;
    pub fn exit_process(&mut self) -> Result<(), String>;
}
```

### RDMA Controller

```rust
pub struct RdmaController;

impl RdmaController {
    pub fn new() -> Self;
    pub fn set_qp_limit(&mut self, limit: u32);
    pub fn set_cq_limit(&mut self, limit: u32);
    pub fn allocate_qp(&mut self) -> Result<(), String>;
    pub fn deallocate_qp(&mut self) -> Result<(), String>;
    pub fn allocate_cq(&mut self) -> Result<(), String>;
}
```

### Net_cls Controller

```rust
pub struct NetClsController;

impl NetClsController {
    pub fn new() -> Self;
    pub fn set_class_id(&mut self, id: u32);
    pub fn get_class_id(&self) -> u32;
    pub fn classify_packet(&mut self, bytes: u64);
}
```

### Example: Using Cgroup Controllers

```rust
use sigmaos::kernel::cgroup_controllers::{
    PidsController, HugetlbController, HugepageSize,
};

let mut pids = PidsController::new();
pids.set_max_pids(1000);

for _ in 0..100 {
    pids.fork_process()?;
}

pids.enforce()?;  // Verify limits

let mut hugetlb = HugetlbController::new();
hugetlb.set_limit(HugepageSize::Two, 100 * 1024 * 1024);
hugetlb.allocate(HugepageSize::Two, 10)?;
```

---

## BPF-Seccomp Integration

### BpfSeccompFilter

```rust
pub struct BpfSeccompFilter;

impl BpfSeccompFilter {
    pub fn new(program: Vec<BpfInstruction>, name: String) -> Result<Self, String>;
    pub fn is_loaded(&self) -> bool;
    pub fn unload(&mut self);
    pub fn execute_filter(&mut self, syscall_info: &SyscallInfo) 
        -> Result<BpfFilterResult, String>;
    pub fn get_stats(&self) -> HashMap<String, u64>;
}
```

### SyscallInfo

```rust
pub struct SyscallInfo {
    pub syscall_number: u32,
    pub args: [u64; 6],
}

impl SyscallInfo {
    pub fn new(syscall_number: u32) -> Self;
    pub fn with_args(syscall_number: u32, args: [u64; 6]) -> Self;
}
```

### BpfFilterResult

```rust
pub struct BpfFilterResult {
    pub action: SeccompAction,
    pub error_code: i32,
}

pub enum SeccompAction {
    Allow = 0,
    Deny = 1,
    Trace = 2,
    Kill = 3,
    Log = 4,
    ErrorNo = 5,
}
```

### SyscallArgumentInspector

```rust
pub struct SyscallArgumentInspector;

impl SyscallArgumentInspector {
    pub fn extract_arg(syscall_info: &SyscallInfo, arg_num: usize) -> Option<u64>;
    pub fn compare_arg(syscall_info: &SyscallInfo, arg_num: usize, value: u64) -> bool;
    pub fn arg_in_range(
        syscall_info: &SyscallInfo,
        arg_num: usize,
        min: u64,
        max: u64,
    ) -> bool;
}
```

### Example: Seccomp Filtering

```rust
use sigmaos::security::seccomp_ebpf::{BpfSeccompFilter, SyscallInfo};

let program = vec![
    BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
    BpfInstruction::Return,
];

let mut filter = BpfSeccompFilter::new(program, "allow_all".to_string())?;

let syscall = SyscallInfo::with_args(
    1,  // sys_write
    [1, 0x1000, 100, 0, 0, 0],  // fd, buf, size
);

let result = filter.execute_filter(&syscall)?;
println!("Filter result: {:?}", result.action);
```

---

## Examples & Best Practices

### Example 1: Complex eBPF Program

```rust
let program = vec![
    BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 100 },
    BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 50 },
    BpfInstruction::Jeq {
        dst_reg: 0,
        src_reg: 1,
        offset: 2,
    },
    BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
    BpfInstruction::Ja { offset: 1 },
    BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 1 },
    BpfInstruction::Return,
];

let mut registry = BpfProgramRegistry::new();
let fd = registry.load_program(BpfProgType::Tracing, program, "cond".to_string())?;
let result = registry.execute_program(fd)?;
```

### Example 2: Integrated Cgroup Management

```rust
use sigmaos::kernel::cgroup_controllers::{
    PidsController, DeviceController, DeviceRule, DeviceType,
};

let mut pids = PidsController::new();
let mut devices = DeviceController::new();

pids.set_max_pids(100);

let rule = DeviceRule {
    device_type: DeviceType::Block,
    major: 8,
    minor: 0,
    access: "rw".to_string(),
};

devices.add_allow_rule(rule);

// Enforce both controllers
pids.enforce()?;
devices.enforce()?;

let pids_stats = pids.get_stats();
let device_stats = devices.get_stats();
```

### Best Practices

1. **Always verify programs** before loading
   ```rust
   let mut verifier = BpfProgramVerifier::new(program);
   verifier.verify()?;
   ```

2. **Use proper cgroup limits** to prevent resource exhaustion
   ```rust
   pids.set_max_pids(1000);  // Reasonable default
   hugetlb.set_limit(HugepageSize::Two, 1000 * 1024 * 1024);
   ```

3. **Monitor statistics** for resource tracking
   ```rust
   let stats = controller.get_stats();
   for (key, value) in stats {
       println!("{}: {}", key, value);
   }
   ```

4. **Handle errors gracefully**
   ```rust
   match registry.load_program(...) {
       Ok(fd) => println!("Program loaded: {:?}", fd),
       Err(e) => eprintln!("Loading failed: {}", e),
   }
   ```

5. **Clean up resources**
   ```rust
   registry.unload_program(fd)?;
   filter.unload();
   ```

---

## Troubleshooting

### Program Verification Failures
- Check all register numbers are 0-10
- Ensure all jumps target valid instructions
- Verify no infinite loops (use DFS analysis)
- Check for unreachable code

### Cgroup Allocation Failures
- Verify limits are set before allocation
- Check current usage < limit
- Ensure proper initialization

### Syscall Filtering Issues
- Verify filter program is valid
- Check argument indices are correct (0-5)
- Ensure filter is loaded before execution

---

**SigmaOS v0.9 API - Complete and Production Ready**

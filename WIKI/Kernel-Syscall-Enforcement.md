# Kernel Syscall Enforcement

SigmaOS implements kernel-space syscall enforcement to improve security by restricting which system calls processes can make. This provides defense-in-depth security by limiting the attack surface of compromised processes.

## Overview

Kernel syscall enforcement works by:
1. Validating syscall permissions before execution
2. Rejecting unauthorized syscalls with appropriate error codes
3. Logging security violations for auditing
4. Supporting multiple enforcement models (seccomp, pledge, Capsicum)

## Enforcement Models

### Seccomp-BPF Filtering
SigmaOS supports seccomp-BPF (Secure Computing Mode with Berkeley Packet Filter) for fine-grained syscall filtering:
- Filter syscalls by number, arguments, and return values
- BPF programs for complex filtering logic
- Per-process and system-wide filter profiles

### OpenBSD Pledge
Simplified syscall restriction with named promises:
- `stdio`: File I/O, basic I/O operations
- `rpath`: Read-only filesystem access
- `wpath`: Write filesystem access
- `exec`: Execute programs
- `inet`: Network access
- `dns`: DNS queries

### FreeBSD Capsicum
Capability-based access control:
- Process capability mode restricts global namespace
- File descriptor rights delegation
- Capability checks on privileged operations

## Implementation

### Syscall Hook
```rust
// src/kernel/syscall_enforcement.rs
pub struct SyscallEnforcer {
    filters: BTreeMap<pid_t, BpfFilter>,
    pledge_map: BTreeMap<pid_t, PledgePromises>,
    capsicum_mode: BTreeSet<pid_t>,
}

impl SyscallEnforcer {
    pub fn check_syscall(&self, pid: pid_t, syscall: u64, args: &[u64]) -> Result<(), SyscallError> {
        // Check seccomp-BPF filter first
        if let Some(filter) = self.filters.get(&pid) {
            if !filter.evaluate(syscall, args) {
                log_security_violation(pid, syscall, "seccomp-filter");
                return Err(SyscallError::Denied);
            }
        }
        
        // Check pledge restrictions
        if let Some(promises) = self.pledge_map.get(&pid) {
            if !promises.allows_syscall(syscall) {
                log_security_violation(pid, syscall, "pledge");
                return Err(SyscallError::Denied);
            }
        }
        
        // Check Capsicum capability mode
        if self.capsicum_mode.contains(&pid) {
            if is_privileged_syscall(syscall) {
                log_security_violation(pid, syscall, "capsicum");
                return Err(SyscallError::Denied);
            }
        }
        
        Ok(())
    }
}
```

### Pledge Implementation
```rust
// src/kernel/pledge.rs
#[derive(Debug, Clone, Copy)]
pub struct PledgePromises {
    pub stdio: bool,
    pub rpath: bool,
    pub wpath: bool,
    pub cpath: bool,
    pub dpath: bool,
    pub exec: bool,
    pub inet: bool,
    pub dns: bool,
    pub unveil: bool,
}

impl PledgePromises {
    pub fn allows_syscall(&self, syscall: u64) -> bool {
        match syscall {
            SYS_READ | SYS_WRITE => self.stdio,
            SYS_OPENAT => self.rpath || self.wpath || self.cpath,
            SYS_EXECVE => self.exec,
            SYS_SOCKET => self.inet,
            SYS_CONNECT => self.inet,
            SYS_UNVEIL => self.unveil,
            _ => false,
        }
    }
    
    pub fn from_string(s: &str) -> Result<Self, PledgeError> {
        let mut promises = PledgePromises::default();
        for token in s.split_whitespace() {
            match token {
                "stdio" => promises.stdio = true,
                "rpath" => promises.rpath = true,
                "wpath" => promises.wpath = true,
                "cpath" => promises.cpath = true,
                "dpath" => promises.dpath = true,
                "exec" => promises.exec = true,
                "inet" => promises.inet = true,
                "dns" => promises.dns = true,
                "unveil" => promises.unveil = true,
                _ => return Err(PledgeError::InvalidPromise),
            }
        }
        Ok(promises)
    }
}
```

### BPF Filter Evaluation
```rust
// src/kernel/seccomp.rs
pub struct BpfFilter {
    program: Vec<BpfInstruction>,
}

impl BpfFilter {
    pub fn evaluate(&self, syscall: u64, args: &[u64]) -> bool {
        let mut accumulator: u64 = syscall;
        let mut pc = 0;
        
        while pc < self.program.len() {
            let insn = &self.program[pc];
            match insn.opcode {
                BpfOpcode::Load => accumulator = args[insn.offset as usize],
                BpfOpcode::JumpIfEqual => {
                    if accumulator == insn.value {
                        pc = insn.jump_target as usize;
                        continue;
                    }
                }
                BpfOpcode::Return => return insn.value != 0,
                _ => {}
            }
            pc += 1;
        }
        
        true
    }
}
```

## Usage

### Setting Pledge
```rust
// C application
#include <sys/pledge.h>

int main() {
    // Restrict to stdio and dns only
    pledge("stdio dns");
    
    // Read from filesystem
    read_file("/etc/config");
    
    // This would fail - exec not allowed
    // exec_program("/bin/ls");
    
    return 0;
}
```

### Setting Seccomp Filter
```rust
// Rust application
use sigmaos::seccomp::{BpfFilter, BpfInstruction};

fn set_seccomp_filter() -> Result<(), Box<dyn Error>> {
    let mut filter = BpfFilter::new();
    
    // Allow only read, write, exit
    filter.add_instruction(BpfInstruction::return_allow(SYS_READ));
    filter.add_instruction(BpfInstruction::return_allow(SYS_WRITE));
    filter.add_instruction(BpfInstruction::return_allow(SYS_EXIT));
    filter.add_instruction(BpfInstruction::return_deny());
    
    sigmaos::syscall::set_filter(filter)?;
    Ok(())
}
```

## Configuration

### System-Wide Profiles
```toml
# /etc/sigmaos/syscall-enforcement.toml
[global]
default_mode = "seccomp"

[profiles.web_browser]
pledge = "stdio rpath inet dns exec unveil"
syscalls = ["read", "write", "openat", "mmap", "munmap", "socket", "connect"]

[profiles.text_editor]
pledge = "stdio rpath wpath cpath unveil"
syscalls = ["read", "write", "openat", "fsync"]

[profiles.network_daemon]
pledge = "stdio inet dns unveil"
syscalls = ["read", "write", "socket", "bind", "listen", "accept"]
```

### Runtime Control
```bash
# View process syscall permissions
sigpledge inspect <pid>

# Set pledge for running process
sigpledge set <pid> "stdio rpath inet"

# View seccomp filter
sigseccomp show <pid>

# Load seccomp filter
sigseccomp load --profile web_browser <pid>
```

## Security Benefits

### Attack Surface Reduction
- Limits which syscalls a process can use
- Prevents exploitation of syscall vulnerabilities
- Contains compromised processes

### Principle of Least Privilege
- Processes only get the syscalls they need
- Easy to audit and verify permissions
- Clear security model

### Defense in Depth
- Works alongside other security features (sandboxing, ASLR, NX)
- Multiple layers of syscall enforcement
- Harder to bypass all protections

## Troubleshooting

### Application Fails to Start
If an application fails due to syscall restrictions:
1. Check which syscalls it needs: `strace -e trace=all <app>`
2. Adjust pledge or seccomp profile accordingly
3. Use relaxed mode for testing: `sigpledge set <pid> "unrestricted"`

### Performance Impact
If syscall enforcement causes performance issues:
1. Profile BPF filter evaluation: `sigseccomp profile <pid>`
2. Simplify BPF programs where possible
3. Use pledge instead of complex seccomp for simple cases

---

**[Security](Category-Security)** | **[Seccomp](Seccomp-BPF-Filtering)** | **[Pledge](Pledge-Unveil-Enforcement)** | **[Capsicum](Capsicum-Integration)**

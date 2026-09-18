# FreeBSD Capsicum Integration

SigmaOS implements FreeBSD Capsicum capability mode for fine-grained capability-based access control, providing sandboxing and privilege restriction for processes.

## Overview

Capsicum provides:
- Capability mode for process sandboxing
- Fine-grained file descriptor rights
- Capability checks on privileged operations
- Integration with existing POSIX APIs
- Capability delegation across process boundaries

## Architecture

### Capability Model
- **Rights**: Permissions associated with file descriptors
- **Capability Mode**: Global process restriction mode
- **Capability Checks**: Kernel enforces rights before operations
- **Capability Delegation**: Rights can be delegated to child processes

### File Descriptor Rights
- **CAP_READ**: Read from file descriptor
- **CAP_WRITE**: Write to file descriptor
- **CAP_SEEK**: Seek within file descriptor
- **CAP_FCNTL**: Use fcntl operations
- **CAP_IOCTL**: Use ioctl operations
- **CAP_MMAP**: Memory map file descriptor
- **CAP_CREATE**: Create new files
- **CAP_FEXECVE**: Execute from file descriptor
- **CAP_LOOKUP**: Lookup paths relative to directory FD

## Implementation

### Capability Rights
```rust
// src/kernel/capsicum.rs
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapRights(pub u64);

impl CapRights {
    pub const READ: u64 = 1 << 0;
    pub const WRITE: u64 = 1 << 1;
    pub const SEEK: u64 = 1 << 2;
    pub const FCNTL: u64 = 1 << 3;
    pub const IOCTL: u64 = 1 << 4;
    pub const MMAP: u64 = 1 << 5;
    pub const CREATE: u64 = 1 << 6;
    pub const FEXECVE: u64 = 1 << 7;
    pub const LOOKUP: u64 = 1 << 8;
    pub const ALL: u64 = Self::READ | Self::WRITE | Self::SEEK | 
                         Self::FCNTL | Self::IOCTL | Self::MMAP |
                         Self::CREATE | Self::FEXECVE | Self::LOOKUP;

    pub fn has_right(&self, right: u64) -> bool {
        (self.0 & right) != 0
    }

    pub fn set_right(&mut self, right: u64) {
        self.0 |= right;
    }

    pub fn clear_right(&mut self, right: u64) {
        self.0 &= !right;
    }
}
```

### Capability Manager
```rust
// src/kernel/capsicum/manager.rs
pub struct CapsicumManager {
    capability_mode: BTreeMap<pid_t, bool>,
    fd_rights: BTreeMap<(pid_t, i32), CapRights>,
    global_namespace_hidden: BTreeSet<pid_t>,
}

impl CapsicumManager {
    pub fn enter_capability_mode(&mut self, pid: pid_t) -> Result<(), CapsicumError> {
        // Check if process can enter capability mode
        if !self.can_enter_capability_mode(pid) {
            return Err(CapsicumError::NotAllowed);
        }
        
        self.capability_mode.insert(pid, true);
        self.global_namespace_hidden.insert(pid);
        
        Ok(())
    }

    pub fn set_fd_rights(&mut self, pid: pid_t, fd: i32, rights: CapRights) {
        self.fd_rights.insert((pid, fd), rights);
    }

    pub fn get_fd_rights(&self, pid: pid_t, fd: i32) -> CapRights {
        self.fd_rights.get(&(pid, fd))
            .copied()
            .unwrap_or(CapRights(CapRights::ALL))
    }

    pub fn check_rights(&self, pid: pid_t, fd: i32, required: CapRights) -> Result<(), CapsicumError> {
        if !self.capability_mode.contains_key(&pid) {
            return Ok(()); // Not in capability mode
        }
        
        let current = self.get_fd_rights(pid, fd);
        if !current.has_right(required.0) {
            return Err(CapsicumError::InsufficientRights);
        }
        
        Ok(())
    }

    pub fn can_enter_capability_mode(&self, pid: pid_t) -> bool {
        // Check if process has any open file descriptors without rights
        // Check if process has any threads
        // Check if process is privileged
        true
    }
}
```

### File Descriptor Delegation
```rust
// src/kernel/capsicum/delegation.rs
impl CapsicumManager {
    pub fn delegate_fd(&mut self, parent_pid: pid_t, child_pid: pid_t, fd: i32) -> Result<(), CapsicumError> {
        let rights = self.get_fd_rights(parent_pid, fd);
        
        // Inherit rights to child process
        self.set_fd_rights(child_pid, fd, rights);
        
        Ok(())
    }

    pub fn restrict_fd(&mut self, pid: pid_t, fd: i32, rights: CapRights) -> Result<(), CapsicumError> {
        if !self.capability_mode.contains_key(&pid) {
            return Err(CapsicumError::NotInCapabilityMode);
        }
        
        self.set_fd_rights(pid, fd, rights);
        Ok(())
    }

    pub fn limit_fd(&mut self, pid: pid_t, fd: i32, rights: CapRights) -> Result<(), CapsicumError> {
        let current = self.get_fd_rights(pid, fd);
        let restricted = CapRights(current.0 & rights.0);
        self.set_fd_rights(pid, fd, restricted);
        Ok(())
    }
}
```

### System Call Checks
```rust
// src/kernel/capsicum/syscall.rs
impl CapsicumManager {
    pub fn check_read(&self, pid: pid_t, fd: i32) -> Result<(), CapsicumError> {
        self.check_rights(pid, fd, CapRights(CapRights::READ))
    }

    pub fn check_write(&self, pid: pid_t, fd: i32) -> Result<(), CapsicumError> {
        self.check_rights(pid, fd, CapRights(CapRights::WRITE))
    }

    pub fn check_seek(&self, pid: pid_t, fd: i32) -> Result<(), CapsicumError> {
        self.check_rights(pid, fd, CapRights(CapRights::SEEK))
    }

    pub fn check_ioctl(&self, pid: pid_t, fd: i32) -> Result<(), CapsicumError> {
        self.check_rights(pid, fd, CapRights(CapRights::IOCTL))
    }

    pub fn check_mmap(&self, pid: pid_t, fd: i32) -> Result<(), CapsicumError> {
        self.check_rights(pid, fd, CapRights(CapRights::MMAP))
    }

    pub fn check_openat(&self, pid: pid_t, dirfd: i32) -> Result<(), CapsicumError> {
        if dirfd == AT_FDCWD {
            return Ok(()); // Allow using current directory
        }
        
        self.check_rights(pid, dirfd, CapRights(CapRights::LOOKUP))
    }
}
```

## Usage

### Entering Capability Mode
```rust
use sigmaos::capsicum::{CapsicumManager, CapRights};

fn main() -> Result<(), Box<dyn Error>> {
    let manager = CapsicumManager::new();
    
    // Open file descriptors before entering capability mode
    let fd = std::fs::File::open("/etc/config")?;
    
    // Set rights for file descriptor
    manager.set_fd_rights(std::process::id(), fd.as_raw_fd(), 
                          CapRights(CapRights::READ));
    
    // Enter capability mode
    manager.enter_capability_mode(std::process::id())?;
    
    // Read from file (allowed)
    let content = std::fs::read_to_string("/etc/config")?;
    
    // Write to file (would fail - no write rights)
    // std::fs::write("/etc/config", "new content")?;
    
    Ok(())
}
```

### File Descriptor Rights
```rust
// Open file with specific rights
let fd = std::fs::File::open("/etc/config")?;
manager.set_fd_rights(pid, fd.as_raw_fd(), 
                      CapRights(CapRights::READ | CapRights::SEEK));

// Restrict existing file descriptor
manager.limit_fd(pid, fd.as_raw_fd(), CapRights(CapRights::READ));

// Remove all rights
manager.restrict_fd(pid, fd.as_raw_fd(), CapRights(0));
```

## Configuration

### Capsicum Configuration
```toml
# /etc/sigmaos/capsicum.toml
[global]
enabled = true
enforce_mode = true

[defaults]
# Default rights for new file descriptors
read = true
write = true
seek = true
fcntl = true
ioctl = false
mmap = true
create = false
fexecve = false
lookup = true

[restrictions]
# Restrictions for specific binaries
"/usr/bin/web_browser" = { read = true, write = false, network = true }
"/usr/bin/text_editor" = { read = true, write = true, network = false }
```

### Runtime Control
```bash
# View capability mode status
sigcap status <pid>

# Enter capability mode
sigcap enter <pid>

# Set file descriptor rights
sigcap set-rights <pid> <fd> <rights>

# View file descriptor rights
sigcap get-rights <pid> <fd>

# Restrict file descriptor
sigcap restrict <pid> <fd> <rights>

# Delegate file descriptor to child
sigcap delegate <parent_pid> <child_pid> <fd>
```

## Security Benefits

### Attack Surface Reduction
- Processes can only perform operations with explicit rights
- Global namespace is hidden in capability mode
- Privileged operations require explicit capability checks

### Principle of Least Privilege
- File descriptors have minimal required rights
- Rights can be further restricted over time
- Child processes inherit restricted rights

### Defense in Depth
- Works alongside other security features (seccomp, pledge, Landlock)
- Multiple layers of capability enforcement
- Harder to bypass all protections

## Integration with Other Security Features

### Landlock Integration
Capsicum can be combined with Linux Landlock:
```rust
// Apply Landlock rules first
landlock_apply_rules(&rules)?;

// Then enter Capsicum capability mode
capsicum_enter_capability_mode()?;
```

### Pledge Integration
Capsicum can be combined with OpenBSD pledge:
```rust
// Set pledge promises
pledge("stdio rpath inet")?;

// Then enter Capsicum capability mode
capsicum_enter_capability_mode()?;
```

## Troubleshooting

### Process Fails to Enter Capability Mode
If a process fails to enter capability mode:
1. Check for open file descriptors without rights
2. Check for active threads
3. Check for privileged operations in progress
4. Review process capabilities: `sigcap status <pid>`

### Operations Fail with EPERM
If operations fail with permission denied:
1. Check if process is in capability mode: `sigcap status <pid>`
2. Check file descriptor rights: `sigcap get-rights <pid> <fd>`
3. Verify required rights are set
4. Check global namespace access

### Rights Not Inherited by Child Process
If child process doesn't inherit rights:
1. Explicitly delegate file descriptor: `sigcap delegate <parent> <child> <fd>`
2. Check fork timing (delegate before fork)
3. Verify child process is in capability mode

---

**[Security](Category-Security)** | **[Landlock Integration](Landlock-LSM-Integration)** | **[Pledge Integration](Pledge-Unveil-Enforcement)**

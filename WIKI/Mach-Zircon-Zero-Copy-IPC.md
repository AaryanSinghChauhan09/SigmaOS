# Mach/Zircon Zero-Copy IPC

SigmaOS implements Mach and Zircon-style zero-copy Inter-Process Communication (IPC) for high-performance message passing with minimal CPU overhead and memory copies.

## Overview

Mach/Zircon IPC provides:
- Zero-copy message passing
- Capability-based access control
- Handle-based resource management
- Channel-based communication
- Out-of-line (OOL) data transfer
- Virtual memory remapping
- Gigabyte-scale IPC
- Kernel-enforced rights verification

## Architecture

### IPC Pipeline
```
Process A → Channel → Kernel → Channel → Process B
                   ↓
              Zero-Copy Transfer
                   ↓
              Virtual Memory Remap
```

### Message Types
- **Inline Messages**: Small messages embedded in channel
- **OOL Messages**: Large messages transferred via memory mapping
- **Handle Messages**: Capability handles for resources
- **Combined Messages**: Mix of inline, OOL, and handles

## Implementation

### IPC Channel
```rust
// src/ipc/mach/channel.rs
pub struct IpcChannel {
    pub id: u64,
    pub capacity: usize,
    pub messages: Vec<IpcMessage>,
    pub handles: BTreeMap<u64, Handle>,
    pub zero_copy_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct IpcMessage {
    pub data: Vec<u8>,
    pub handles: Vec<Handle>,
    pub ool_data: Option<Vec<u8>>,
}

impl IpcChannel {
    pub fn new(capacity: usize) -> Self {
        IpcChannel {
            id: generate_channel_id(),
            capacity,
            messages: Vec::new(),
            handles: BTreeMap::new(),
            zero_copy_enabled: true,
        }
    }

    pub fn send(&mut self, message: IpcMessage) -> Result<(), IpcError> {
        if self.messages.len() >= self.capacity {
            return Err(IpcError::ChannelFull);
        }
        
        // Zero-copy transfer if enabled
        if self.zero_copy_enabled {
            self.send_zero_copy(message)?;
        } else {
            self.messages.push(message);
        }
        
        Ok(())
    }

    pub fn receive(&mut self) -> Result<IpcMessage, IpcError> {
        if self.messages.is_empty() {
            return Err(IpcError::ChannelEmpty);
        }
        
        Ok(self.messages.remove(0))
    }

    fn send_zero_copy(&mut self, message: IpcMessage) -> Result<(), IpcError> {
        // Remap virtual memory for OOL data
        if let Some(ool_data) = &message.ool_data {
            self.remap_memory(ool_data)?;
        }
        
        self.messages.push(message);
        Ok(())
    }

    fn remap_memory(&self, data: &[u8]) -> Result<(), IpcError> {
        // Remap memory pages for zero-copy transfer
        let pages = self.memory_to_pages(data);
        
        for page in pages {
            self.remap_page(page)?;
        }
        
        Ok(())
    }
}
```

### Handle Management
```rust
// src/ipc/mach/handle.rs
pub struct Handle {
    pub id: u64,
    pub rights: HandleRights,
    pub resource: Resource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandleRights {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub duplicate: bool,
}

#[derive(Debug, Clone)]
pub enum Resource {
    Channel(IpcChannel),
    MemoryRegion(MemoryRegion),
    Semaphore(Semaphore),
    Port(Port),
}

impl Handle {
    pub fn new(resource: Resource, rights: HandleRights) -> Self {
        Handle {
            id: generate_handle_id(),
            rights,
            resource,
        }
    }

    pub fn check_rights(&self, required: HandleRights) -> Result<(), IpcError> {
        if required.read && !self.rights.read {
            return Err(IpcError::InsufficientRights);
        }
        if required.write && !self.rights.write {
            return Err(IpcError::InsufficientRights);
        }
        if required.execute && !self.rights.execute {
            return Err(IpcError::InsufficientRights);
        }
        if required.duplicate && !self.rights.duplicate {
            return Err(IpcError::InsufficientRights);
        }
        Ok(())
    }

    pub fn duplicate(&self) -> Result<Handle, IpcError> {
        self.check_rights(HandleRights { duplicate: true, ..Default::default() })?;
        Ok(Handle {
            id: generate_handle_id(),
            rights: self.rights,
            resource: self.resource.clone(),
        })
    }
}
```

### Port-based IPC
```rust
// src/ipc/mach/port.rs
pub struct Port {
    pub id: u64,
    pub queue: Vec<IpcMessage>,
    pub capacity: usize,
    pub waiting: BTreeSet<pid_t>,
}

impl Port {
    pub fn new(capacity: usize) -> Self {
        Port {
            id: generate_port_id(),
            queue: Vec::new(),
            capacity,
            waiting: BTreeSet::new(),
        }
    }

    pub fn send(&mut self, message: IpcMessage) -> Result<(), IpcError> {
        if self.queue.len() >= self.capacity {
            return Err(IpcError::PortFull);
        }
        
        self.queue.push(message);
        
        // Wake up waiting process
        if let Some(&pid) = self.waiting.iter().next() {
            self.wake_process(pid);
        }
        
        Ok(())
    }

    pub fn receive(&mut self, pid: pid_t) -> Result<IpcMessage, IpcError> {
        if self.queue.is_empty() {
            self.waiting.insert(pid);
            return Err(IpcError::NoMessage);
        }
        
        Ok(self.queue.remove(0))
    }

    fn wake_process(&self, pid: pid_t) {
        // Wake up process
        wake_process(pid);
    }
}
```

## Configuration

### IPC Configuration
```toml
# /etc/sigmaos/ipc.toml
[channel]
# Channel settings
default_capacity = 1024
zero_copy = true
max_message_size = "1G"

[port]
# Port settings
default_capacity = 256
timeout_ms = 1000

[handle]
# Handle settings
max_handles = 4096
rights_checking = true
```

### Runtime Control
```bash
# Create channel
sigipc create-channel --capacity 1024

# Create port
sigipc create-port --capacity 256

# Send message
sigipc send <channel_id> <message>

# Receive message
sigipc receive <channel_id>

# View channel status
sigipc status <channel_id>

# Enable zero-copy
sigipc set-zero-copy <channel_id> true

# View handle information
sigipc handle-info <handle_id>
```

## Performance Optimization

### Zero-Copy
Enable zero-copy for best performance:
```bash
# Enable zero-copy
sigipc set-zero-copy <channel_id> true

# Set large message size
sigipc set-max-message-size <channel_id> 1G
```

### Channel Capacity
Optimize channel capacity:
```bash
# Increase capacity
sigipc set-capacity <channel_id> 4096

# Decrease capacity
sigipc set-capacity <channel_id> 256
```

### Handle Management
Optimize handle management:
```bash
# Set max handles
sigipc set-max-handles 8192

# Enable rights checking
sigipc set-rights-checking true
```

## Troubleshooting

### Channel Full
If channel is full:
1. Check capacity: `sigipc status <channel_id>`
2. Increase capacity: `sigipc set-capacity <channel_id> 4096`
3. Receive messages: `sigipc receive <channel_id>`
4. Check for slow consumer
5. Enable zero-copy

### Rights Violation
If rights violation occurs:
1. Check handle rights: `sigipc handle-info <handle_id>`
2. Verify required rights
3. Update handle rights if needed
4. Check rights checking: `sigipc get-rights-checking`

### Poor Performance
If IPC performance is poor:
1. Enable zero-copy: `sigipc set-zero-copy <channel_id> true`
2. Increase capacity: `sigipc set-capacity <channel_id> 4096`
3. Check message size
4. Optimize message structure
5. Use OOL for large data

### Handle Exhaustion
If handles are exhausted:
1. Check handle count: `sigipc handle-count`
2. Increase max handles: `sigipc set-max-handles 8192`
3. Close unused handles
4. Check for handle leaks

---

**[IPC](Category-IPC)** | **[Message Passing](Message-Passing)** | **[Zero-Copy](Zero-Copy)**
